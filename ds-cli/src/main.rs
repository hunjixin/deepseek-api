mod chat_history;
use anyhow::{anyhow, Result};
use chat_history::{ChatHistory, DisplayContent};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use deepseek_api::{
    request::{MessageRequest, SystemMessageRequest, UserMessageRequest},
    response::{AssistantMessage, ModelType},
    ClientBuilder,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use std::sync::{
    mpsc::{channel, Sender},
    Arc, RwLock,
};
use std::{thread, time::Duration};
use tui_textarea::{Input, Key, TextArea};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}

#[derive(Clone, Debug, Default)]
struct BtnState {
    use_reasoning_model: bool,
    send_system_msg: bool,
}

#[derive(Default)]
struct ShareState {
    btn_state: BtnState,
    is_requesting: bool,

    history: Vec<MessageRequest>,
    display_message: Vec<DisplayContent>,
}

fn main() -> Result<()> {
    color_eyre::install().map_err(|err| anyhow!("{err}"))?;
    let args = Args::parse();
    let (req_sender, req_receiver) = channel();
    let req_state = Arc::new(RwLock::new(ShareState::default()));

    {
        let client: deepseek_api::Client = ClientBuilder::new(args.api_key.clone()).build()?;
        let req_state = req_state.clone();
        thread::spawn(move || loop {
            //request thread
            let msg: String = req_receiver.recv().unwrap();
            let (req_msgs, btn_state) = {
                let mut req_state = req_state.write().unwrap();
                if req_state.btn_state.send_system_msg {
                    req_state
                        .history
                        .push(MessageRequest::System(SystemMessageRequest::new(
                            msg.as_str(),
                        )));
                } else {
                    req_state
                        .history
                        .push(MessageRequest::User(UserMessageRequest::new(msg.as_str())));
                }

                req_state.display_message.push(DisplayContent {
                    is_user: true,
                    content: Some(msg.clone()),
                    reasoning_content: None,
                });
                (req_state.history.clone(), req_state.btn_state.clone())
            };

            let model = if btn_state.use_reasoning_model {
                ModelType::DeepSeekReasoner
            } else {
                ModelType::DeepSeekChat
            };
            let mut completions = client.chat();
            let builder = completions
                .chat_builder(req_msgs)
                .stream(true)
                .use_model(model);
            let resp = completions.create(builder).unwrap().must_stream();

            let mut content_buf = String::new();
            let mut reasoning_content_buf = String::new();
            let mut first = true;
            for item in resp {
                let chat = item.unwrap();
                for msg in chat.choices.iter() {
                    if let Some(ref reasoning_content) = msg.delta.reasoning_content {
                        reasoning_content_buf.push_str(reasoning_content);
                    }
                    if let Some(ref content) = msg.delta.content {
                        content_buf.push_str(content);
                    }
                }

                let mut req_state = req_state.write().unwrap();
                if first {
                    first = false;
                } else {
                    req_state.history.pop();
                    req_state.display_message.pop();
                }

                req_state
                    .history
                    .push(MessageRequest::Assistant(AssistantMessage {
                        content: content_buf.clone(),
                        reasoning_content: None,
                        ..Default::default()
                    }));

                req_state.display_message.push(DisplayContent {
                    is_user: false,
                    content: (!content_buf.is_empty()).then(|| content_buf.clone()),
                    reasoning_content: (!reasoning_content_buf.is_empty())
                        .then(|| reasoning_content_buf.clone()),
                });
            }

            let mut req_state = req_state.write().unwrap();
            req_state.is_requesting = false;
        });
    }

    let app = App {
        req_sender,
        req_state,
        textarea: TextArea::default(),
        scroll_offset: 0,
        cursor: Cursor::Input,
    };
    let terminal = ratatui::init();
    // Set up terminal draw thread
    let result = app.run(terminal);
    ratatui::restore();
    result.map_err(|err| anyhow!("Program exit abnormally {err}"))
}

#[derive(Debug, Clone, PartialEq)]
enum Cursor {
    Chat,
    DeepThinkButton,
    UseSystemButton,
    Input,
}

impl Cursor {
    fn next(self) -> Self {
        match self {
            Cursor::Chat => Cursor::Input,
            Cursor::Input => Cursor::DeepThinkButton,
            Cursor::DeepThinkButton => Cursor::UseSystemButton,
            Cursor::UseSystemButton => Cursor::Chat,
        }
    }
}

struct App<'a> {
    req_sender: Sender<String>,
    req_state: Arc<RwLock<ShareState>>,
    textarea: TextArea<'a>,
    cursor: Cursor,
    scroll_offset: usize,
}

impl App<'_> {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|f| ui(f, &mut self))?;
            if event::poll(Duration::from_millis(0))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Release {
                        continue;
                    }

                    if key.code == KeyCode::Esc {
                        return Ok(());
                    }
                    if key.code == KeyCode::Tab {
                        self.cursor = self.cursor.next();
                        continue;
                    }

                    match self.cursor {
                        Cursor::Chat => {
                            if key.code == KeyCode::Up {
                                self.step(-1);
                            } else if key.code == KeyCode::Down {
                                self.step(1);
                            }
                        }
                        Cursor::DeepThinkButton => {
                            // Handle button click
                            if key.code == KeyCode::Enter {
                                let mut req_state = self.req_state.write().unwrap();
                                req_state.btn_state.use_reasoning_model =
                                    !req_state.btn_state.use_reasoning_model;
                            }
                        }
                        Cursor::UseSystemButton => {
                            // Handle button click
                            if key.code == KeyCode::Enter {
                                let mut req_state = self.req_state.write().unwrap();
                                req_state.btn_state.send_system_msg =
                                    !req_state.btn_state.send_system_msg;
                            }
                        }
                        Cursor::Input => {
                            let mut req_state = self.req_state.write().unwrap();
                            if req_state.is_requesting {
                                continue;
                            }

                            if key.modifiers == KeyModifiers::CONTROL
                                && (key.code == KeyCode::Enter || key.code == KeyCode::Char('j'))
                            {
                                self.textarea.input(Input {
                                    key: Key::Enter,
                                    ..Default::default()
                                });
                                continue;
                            }

                            if key.code == KeyCode::Enter {
                                let msg: String = self.textarea.lines().join("\n");
                                if msg.is_empty() {
                                    continue;
                                }

                                req_state.is_requesting = true;
                                self.textarea = TextArea::default();
                                self.req_sender.send(msg).unwrap();
                            } else {
                                self.textarea.input(Event::Key(key));
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
        .constraints([
            Constraint::Min(1),
            Constraint::Length(4),
            Constraint::Length(3),
        ])
        .split(f.area());

    let (display_message, is_requesting, btn_state) = {
        let state = app.req_state.read().unwrap();
        (
            state.display_message.clone(),
            state.is_requesting,
            state.btn_state.clone(),
        )
    };

    ChatHistory::render(
        f,
        chunks[0],
        app.cursor == Cursor::Chat,
        &display_message,
        &mut app.scroll_offset,
        is_requesting,
    );

    render_input(
        f,
        chunks[1],
        app.cursor == Cursor::Input,
        &mut app.textarea,
        is_requesting,
    );

    render_button_row(f, chunks[2], &app.cursor, &btn_state);
}

fn render_button_row(f: &mut Frame, area: Rect, cursor: &Cursor, btn_config: &BtnState) {
    let button_constraints = [Constraint::Length(12), Constraint::Length(12)];
    let chunks = Layout::horizontal(button_constraints)
        .spacing(1)
        .split(area);

    {
        //deepthink button
        let mut block = Block::default().borders(Borders::ALL);
        if let Cursor::DeepThinkButton = cursor {
            block = block.border_style(Style::default().fg(Color::LightBlue));
        }
        let style = if btn_config.use_reasoning_model {
            Style::default().bg(Color::Rgb(112, 128, 174))
        } else {
            Style::default()
        };
        let para = Paragraph::new("Deep Think")
            .block(block)
            .style(style)
            .alignment(Alignment::Center);
        f.render_widget(para, chunks[0]);
    }

    {
        //use system message button
        let mut block = Block::default().borders(Borders::ALL);
        if let Cursor::UseSystemButton = cursor {
            block = block.border_style(Style::default().fg(Color::LightBlue));
        }
        let style = if btn_config.send_system_msg {
            Style::default().bg(Color::Rgb(112, 128, 174))
        } else {
            Style::default()
        };
        let para = Paragraph::new("System Msg")
            .block(block)
            .style(style)
            .alignment(Alignment::Center);
        f.render_widget(para, chunks[1]);
    }
}

fn render_input(
    f: &mut Frame,
    area: Rect,
    is_cursor: bool,
    text_area: &mut TextArea,
    is_requesting: bool,
) {
    //render inputs
    let mut input_block = Block::default()
        .borders(Borders::all())
        .title("Input Question?  Press ESC to exit");

    input_block = match (is_requesting, is_cursor) {
        (_, false) => input_block.border_style(Style::default().fg(Color::Gray)),
        (true, true) => input_block.border_style(Style::default().fg(Color::Gray)),
        (false, true) => input_block.border_style(Style::default().fg(Color::LightBlue)),
    };

    text_area.set_block(input_block);
    f.render_widget(&*text_area, area);
}
