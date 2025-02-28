use clap::Parser;
use color_eyre::{owo_colors::colors::css::MediumSpringGreen, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Offset, Rect},
    style::{palette::tailwind, Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use seep_seek_api::{
    request::{MessageRequest, UserMessageRequest},
    response::ModelType,
    Client,
};
use serde::Serialize;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use tui_input::{backend::crossterm::EventHandler, Input};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub api_key: String,
}
fn main() -> Result<()> {
    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    color_eyre::install()?;
    let args = Args::parse();
    let client = Client::new(&args.api_key);

    let app = App {
        client,
        runtime,
        history: vec![],
        show: vec![],
        input: Input::default(),
        list_state: ListState::default(),
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
    client: Client,
    runtime: Runtime,
    history: Vec<MessageRequest>,
    input: Input,

    show: Vec<ListItem<'static>>,
    list_state: ListState,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<Option<()>> {
        loop {
            terminal.draw(|f| ui(f, &mut self))?;

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
                        self.input.reset();

                        let client = self.client.clone();
                        self.history
                            .push(MessageRequest::User(UserMessageRequest::new(&msg)));
                        let history = self.history.clone();
                        let resp = self
                            .runtime
                            .block_on(async move {
                                let mut completions =
                                    client.completions().set_model(ModelType::DeepSeekChat);
                                let builder =
                                    completions.chat_builder(history).append_user_message(&msg);
                                completions.create(builder).await
                            })
                            .unwrap();

                        for msg in resp.choices.iter() {
                            let resp_msg = MessageRequest::from_message(
                                msg.message.as_ref().expect("message"),
                            )
                            .unwrap();
                            self.history.push(resp_msg);
                        }
                    }
                    KeyCode::Esc => {
                        return Ok(None);
                    }
                    _ => {
                        self.input.handle_event(&Event::Key(key));
                    }
                }
            }
        }
    }

    fn step(&mut self, amount: isize) {
        if amount == 1 {
            self.list_state.select_next();
        } else {
            self.list_state.select_previous();
        }
    }
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

    let history = app.history.clone();

    let list_items: Vec<ListItem> = history
        .iter()
        .map(|req| req.get_content())
        .enumerate()
        .map(|(index, msg)| {
            let text = if index % 2 == 0 {
                Text::raw(msg.clone()).alignment(Alignment::Right)
            } else {
                Text::raw(msg.clone()).alignment(Alignment::Left)
            };
            ListItem::new(text)
        })
        .collect();
    app.show = list_items.clone();

    let list = List::new(list_items.clone())
        .highlight_style(Style::default().bg(Color::Indexed(237)))
        .scroll_padding(4);
    f.render_stateful_widget(list, chunks[0], &mut app.list_state);

    let width = chunks[1].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Question?"));
    f.render_widget(input, chunks[2]);

    f.set_cursor_position((
        chunks[2].x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
        chunks[2].y + 1,
    ));
}
