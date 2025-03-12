use anyhow::{anyhow, Result};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use deepseek_api::{
    request::{AssistantMessageRequest, MessageRequest, UserMessageRequest},
    Client,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Color,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarState, Wrap},
    DefaultTerminal, Frame,
};
use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc, RwLock,
    },
    vec,
};
use std::{thread, time::Duration};
use tui_input::{backend::crossterm::EventHandler, Input};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}

#[derive(Default)]
struct ReqStat {
    history: Vec<MessageRequest>,
    is_requesting: bool,
}
fn main() -> Result<()> {
    color_eyre::install().map_err(|err| anyhow!("{err}"))?;
    let args = Args::parse();
    let (req_sender, req_receiver) = channel();
    let req_state = Arc::new(RwLock::new(ReqStat::default()));

    {
        let client = Client::new(&args.api_key);
        let req_state = req_state.clone();
        thread::spawn(move || loop {
            let msg: String = req_receiver.recv().unwrap();
            let req_msgs = { req_state.read().unwrap().history.clone() };

            let mut completions = client.chat();
            let builder = completions
                .chat_builder(req_msgs)
                .stream(true)
                .append_user_message(&msg);
            let resp = completions.create(builder).unwrap().must_stream();

            let mut msg_buf = String::new();
            let mut first = true;
            for item in resp {
                let chat = item.unwrap();
                for msg in chat.choices.iter() {
                    msg_buf.push_str(&msg.delta.content);
                }

                let mut req_state = req_state.write().unwrap();
                if first {
                    first = false;
                } else {
                    req_state.history.pop();
                }
                req_state
                    .history
                    .push(MessageRequest::Assistant(AssistantMessageRequest::new(
                        &msg_buf,
                    )));
            }

            let mut req_state = req_state.write().unwrap();
            req_state.is_requesting = false;
        });
    }

    let app = App {
        req_sender,
        req_state,
        input: Input::default(),
        scroll_offset: 0,
    };
    let terminal = ratatui::init();
    let result = app.run(terminal);
    ratatui::restore();

    match result {
        Ok(None) => println!("Canceled"),
        Err(err) => eprintln!("{err}"),
        _ => {}
    }
    Ok(())
}

struct App {
    req_sender: Sender<String>,
    req_state: Arc<RwLock<ReqStat>>,
    input: Input,
    scroll_offset: usize,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<Option<()>> {
        loop {
            terminal.draw(|f| ui(f, &mut self))?;

            if event::poll(Duration::from_millis(0))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Up => {
                            self.step(-1);
                        }
                        KeyCode::Down => {
                            self.step(1);
                        }
                        KeyCode::Enter => {
                            let msg: String = self.input.value().into();
                            if msg.is_empty() {
                                continue;
                            }
                            let mut req_state = self.req_state.write().unwrap();
                            if req_state.is_requesting {
                                continue;
                            }
                            req_state.is_requesting = true;
                            req_state
                                .history
                                .push(MessageRequest::User(UserMessageRequest::new(&msg)));
                            self.input.reset();
                            self.req_sender.send(msg).unwrap();
                        }
                        KeyCode::Esc => {
                            return Ok(None);
                        }
                        _ => {
                            let state = self.req_state.read().unwrap();
                            if !state.is_requesting {
                                self.input.handle_event(&Event::Key(key));
                            }
                        }
                    }
                }
            } else {
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }

    fn step(&mut self, amount: isize) {
        if amount == 1 {
            self.scroll_offset = self.scroll_offset.saturating_add(1);
        } else {
            self.scroll_offset = self.scroll_offset.saturating_sub(1);
        }
    }
}

fn total_lines(history: &[MessageRequest], content_width: usize) -> usize {
    history
        .iter()
        .map(|msg| {
            let lines = textwrap::wrap(&msg.get_content(), content_width)
                .into_iter()
                .map(|cow| cow.into_owned()) // 转换为 String
                .collect::<Vec<String>>();
            lines.len()
        })
        .sum()
}

fn visible_lines(
    history: &[MessageRequest],
    scroll_offset: usize,
    content_width: usize,
    height: usize,
) -> Vec<(String, Alignment)> {
    let mut lines = vec![];
    let mut current_line = 0;
    let start_line = scroll_offset;
    let end_line = start_line + height;

    for (i, msg) in history.iter().enumerate() {
        let wrapped = textwrap::wrap(&msg.get_content(), content_width)
            .into_iter()
            .map(|cow| cow.into_owned())
            .collect::<Vec<String>>();

        for line in wrapped {
            if current_line >= start_line && current_line < end_line {
                let alignment = if i % 2 == 0 {
                    Alignment::Right
                } else {
                    Alignment::Left
                };
                lines.push((line, alignment));
            }
            current_line += 1;
            if current_line >= end_line {
                break;
            }
        }
    }
    lines
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    let (history, is_requesting) = {
        let state = app.req_state.read().unwrap();
        (state.history.clone(), state.is_requesting)
    };

    let history_layout =
        Layout::horizontal([Constraint::Min(1), Constraint::Length(1)]).split(chunks[0]);
    let (content_area, scrollbar_area) = (history_layout[0], history_layout[1]);

    let content_width = content_area.width.saturating_sub(1);

    let total_lines = total_lines(&history, content_width as usize);
    let visible_height = content_area.height as usize;
    if is_requesting {
        app.scroll_offset = total_lines.saturating_sub(visible_height);
    }

    let visible_lines = visible_lines(
        &history,
        app.scroll_offset,
        content_width as usize,
        visible_height,
    );

    let constraints = vec![Constraint::Length(1); visible_lines.len()];
    let layout = Layout::vertical(constraints).split(content_area);

    for (i, (line, alignment)) in visible_lines.into_iter().enumerate() {
        let para = Paragraph::new(line)
            .alignment(alignment)
            .wrap(Wrap { trim: true });
        f.render_widget(para, layout[i]);
    }

    let scrollbar = Scrollbar::default()
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let mut scroll_state = ScrollbarState::new(total_lines)
        .position(app.scroll_offset)
        .viewport_content_length(visible_height);

    f.render_stateful_widget(scrollbar, scrollbar_area, &mut scroll_state);

    let width = chunks[1].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = app.input.visual_scroll(width as usize);
    let mut input = Paragraph::new(app.input.value())
        .scroll((0, scroll as u16))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Input Question?  Press ESC to exit"),
        );
    if is_requesting {
        input = input.style(Color::Gray);
    } else {
        input = input.style(Color::Green);
    }
    f.render_widget(input, chunks[2]);

    if !is_requesting {
        f.set_cursor_position((
            chunks[2].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            chunks[2].y + 1,
        ));
    }
}
