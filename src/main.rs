#![allow(dead_code)]

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::DefaultTerminal;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();
    let result = app.run(&mut terminal);
    ratatui::restore();
    return result;
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello, World!", frame.area());
}

struct Chat {
    messages: Vec<Message>,
}

impl Chat {
    fn new() -> Self {
        return Self { messages: vec![] };
    }
}

enum Author {
    USER,
    AI,
}

impl Author {
    fn render(&self) -> Line {
        match self {
            Self::USER => Line::from("bde").red(),
            Self::AI => Line::from("gemini").blue(),
        }
    }
}

struct Message {
    text: String,
    author: Author,
}

impl Message {
    fn new(text: String, author: Author) -> Self {
        return Self { text, author };
    }

    fn render(&self) -> [Line; 3] {
        let author = self.author.render();
        let text = Line::from(self.text.as_ref());
        let pad = Line::from("  ");
        return [author, text, pad];
    }
}

struct App {
    chats: Vec<Chat>,
    curr_chat: usize,
    input_msg: Option<String>,
    input_msg_length: usize,
    // display_help: bool,
    quit: bool,
}

impl App {
    fn new() -> Self {
        return Self {
            chats: vec![Chat {
                messages: vec![
                    Message {
                        text: String::from("Hello"),
                        author: Author::USER,
                    },
                    Message {
                        text: String::from("Hey! How can I help you today?"),
                        author: Author::AI,
                    },
                    Message {
                        text: String::from("Suck on my ballzzzz"),
                        author: Author::USER,
                    },
                ],
            }],
            curr_chat: 0,
            // display_help: false,
            input_msg: Some(String::new()),
            input_msg_length: 0,
            quit: false,
        };
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;

            if self.quit {
                return Ok(());
            }
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(k) => self.handle_key(k),
            _ => {}
        }
        return Ok(());
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            // KeyCode::Char('?') => self.display_help = true,
            KeyCode::Esc => self.quit = true,
            KeyCode::Char(c) => {
                self.input_msg.as_mut().unwrap().push(c);
                self.input_msg_length += 1;
            }
            KeyCode::Enter => {
                let text = Option::take(&mut self.input_msg).unwrap();
                self.input_msg = Some(String::new());
                self.input_msg_length = 0;
                let msg = Message::new(text, Author::USER);
                self.chats[self.curr_chat].messages.push(msg);
            }
            KeyCode::Backspace => {
                let _ = self.input_msg.as_mut().unwrap().pop();
                self.input_msg_length = self.input_msg_length.saturating_sub(1);
            }
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::new(
            Direction::Vertical,
            vec![Constraint::Percentage(100), Constraint::Min(4)],
        );

        let [messages_area, input_area] = layout.areas(frame.area());

        let messages: Vec<Line> = self.chats[self.curr_chat]
            .messages
            .iter()
            .map(|msg| msg.render())
            .flatten()
            .collect();

        let messages = Paragraph::new(messages).block(
            Block::new()
                .title(" new chat ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Double)
                .borders(Borders::ALL),
        );

        let input = Paragraph::new(self.input_msg.as_ref().unwrap().as_str()).block(
            Block::new()
                .border_type(BorderType::Double)
                .borders(Borders::ALL),
        );

        // let help = Line::from("? for help").alignment(Alignment::Center);

        frame.render_widget(messages, messages_area);

        frame.render_widget(input, input_area);

        frame.set_cursor_position(Position::new(
            input_area.x + self.input_msg_length as u16 + 1,
            input_area.y + 1,
        ));

        // frame.render_widget(help, help_area);
    }
}
