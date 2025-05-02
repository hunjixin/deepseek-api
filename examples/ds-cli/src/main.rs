mod chat_history;
use anyhow::{anyhow, Result};
use chat_history::ChatHistory;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use deepseek_api::{
    request::{MessageRequest, UserMessageRequest}, response::AssistantMessage, Client
};
use ratatui::{
    layout::{ Constraint, Direction, Layout},
    style::{Color,Style},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use std::sync::{
        mpsc::{channel, Sender},
        Arc, RwLock,
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
            //request thread
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
                    .push(MessageRequest::Assistant(AssistantMessage::new(
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
    // Set up terminal draw thread
    let result = app.run(terminal);
    ratatui::restore();
    result.map_err(|err| anyhow!("Program exit abnormally {err}"))
}

struct App {
    req_sender: Sender<String>,
    req_state: Arc<RwLock<ReqStat>>,
    input: Input,
    scroll_offset: usize,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
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
                            return Ok(());
                        }
                        _ => {
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) && (key.code == KeyCode::Enter || key.code == KeyCode::Char('j')) {
                                //if got controller + enter its enter for input
                                println!("control enter ");
                                let state = self.req_state.read().unwrap();
                                if !state.is_requesting {
                                    self.input.handle_event(&Event::Key(KeyEvent::new_with_kind(KeyCode::Enter, event::KeyModifiers::NONE, KeyEventKind::Press)));
                                }
                            } else {
                                // dispatch other keys to input
                                let state = self.req_state.read().unwrap();
                                if !state.is_requesting {
                                    self.input.handle_event(&Event::Key(key));
                                }
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

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(f.area());

    let (history, is_requesting) = {
        let state = app.req_state.read().unwrap();
        (state.history.clone(), state.is_requesting)
    };

    ChatHistory::render(f, chunks[0], &history, &mut app.scroll_offset, is_requesting);

    //render inputs
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Input Question?  Press ESC to exit");
    
    let width = chunks[1].width.max(3) - 3;
    let scroll = app.input.visual_scroll(width as usize);
    let mut input = Paragraph::new(app.input.value())
        .scroll((0, scroll as u16))
        .block(input_block);

    input = match is_requesting {
        true => input.style(Style::default().fg(Color::Gray)),
        false => input.style(Style::default().fg(Color::Green)),
    };
    
    f.render_widget(input, chunks[1]);

    if !is_requesting {
        f.set_cursor_position((
            chunks[1].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            chunks[1].y + 1,
        ));
    }
}
