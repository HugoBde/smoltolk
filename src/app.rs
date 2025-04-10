use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Position};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};

use crate::chat::Chat;
use crate::gemini;
use crate::message::{Author, Message};

enum AppMode {
  Normal,
  Insert,
}

pub struct App {
  username: String,

  mode: AppMode,

  chats:     Vec<Chat>,
  curr_chat: usize,

  input_msg:        Option<String>,
  input_msg_length: usize,

  quit: bool,
}

impl App {
  pub fn new() -> Self {
    let username = match std::process::Command::new("whoami").output() {
      Ok(out) if out.status.success() => String::from_utf8(out.stdout).unwrap(),
      _ => String::from("you"),
    };

    return Self {
      username,
      mode: AppMode::Normal,
      chats: vec![Chat::new(String::from("test chat"))],
      curr_chat: 0,
      input_msg: Some(String::new()),
      input_msg_length: 0,
      quit: false,
    };
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
      terminal.draw(|frame| self.render(frame))?;
      self.handle_events()?;

      if self.quit {
        return Ok(());
      }
    }
  }

  fn handle_events(&mut self) -> Result<()> {
    match self.mode {
      AppMode::Normal => self.handle_events_normal()?,
      AppMode::Insert => self.handle_events_insert()?,
    }
    return Ok(());
  }

  fn handle_events_normal(&mut self) -> Result<()> {
    if let Event::Key(key) = event::read()? {
      match key.code {
        KeyCode::Char('q') => self.quit = true,

        KeyCode::Char('i') => self.mode = AppMode::Insert,

        KeyCode::Enter => {
          let text = Option::take(&mut self.input_msg).unwrap();
          self.input_msg = Some(String::new());
          self.input_msg_length = 0;
          let msg = Message::new(text, Author::USER(self.username.clone()));
          self.chats[self.curr_chat].messages.push(msg);

          let gemini_response = gemini::send_req(&self.chats[self.curr_chat])?;
          let msg = Message::new(gemini_response, Author::AI);
          self.chats[self.curr_chat].messages.push(msg);
        }

        _ => {}
      }
    }

    return Ok(());
  }

  fn handle_events_insert(&mut self) -> Result<()> {
    if let Event::Key(key) = event::read()? {
      match key.code {
        KeyCode::Esc => self.mode = AppMode::Normal,

        KeyCode::Char(c) => {
          self.input_msg.as_mut().unwrap().push(c);
          self.input_msg_length += 1;
        }

        KeyCode::Backspace => {
          let _ = self.input_msg.as_mut().unwrap().pop();
          self.input_msg_length = self.input_msg_length.saturating_sub(1);
        }

        _ => {}
      }
    }
    return Ok(());
  }

  fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
    match key.code {
      _ => {}
    }

    return Ok(());
  }

  fn render(&self, frame: &mut Frame) {
    match self.mode {
      AppMode::Normal => self.render_normal(frame),
      AppMode::Insert => self.render_insert(frame),
    }
  }
  fn render_normal(&self, frame: &mut Frame) {
    let layout = Layout::new(
      Direction::Vertical,
      vec![
        Constraint::Percentage(100),
        Constraint::Min(4),
        Constraint::Min(1),
      ],
    );

    let [messages_area, input_area, help_area] = layout.areas(frame.area());

    let messages: Vec<Line> = self.chats[self.curr_chat]
      .messages
      .iter()
      .map(|msg| msg.render())
      .flatten()
      .collect();

    let messages = Paragraph::new(messages)
      .block(
        Block::new()
          .title(self.chats[self.curr_chat].title.as_str())
          .title_alignment(Alignment::Center)
          .border_type(BorderType::Double)
          .borders(Borders::ALL),
      )
      .wrap(Wrap { trim: true });

    let input = Paragraph::new(self.input_msg.as_ref().unwrap().as_str())
      .block(
        Block::new()
          .border_type(BorderType::Double)
          .borders(Borders::ALL),
      )
      .wrap(Wrap { trim: true });

    let help = Paragraph::new("<i> to insert | <Enter> to submit | <q> to exit")
      .alignment(Alignment::Center);

    frame.render_widget(messages, messages_area);

    frame.render_widget(input, input_area);

    frame.render_widget(help, help_area);
  }

  fn render_insert(&self, frame: &mut Frame) {
    let layout = Layout::new(
      Direction::Vertical,
      vec![
        Constraint::Percentage(100),
        Constraint::Min(4),
        Constraint::Min(1),
      ],
    );

    let [messages_area, input_area, help_area] = layout.areas(frame.area());

    let messages: Vec<Line> = self.chats[self.curr_chat]
      .messages
      .iter()
      .map(|msg| msg.render())
      .flatten()
      .collect();

    let messages = Paragraph::new(messages)
      .block(
        Block::new()
          .title(self.chats[self.curr_chat].title.as_str())
          .title_alignment(Alignment::Center)
          .border_type(BorderType::Double)
          .borders(Borders::ALL),
      )
      .wrap(Wrap { trim: true });

    let input = Paragraph::new(self.input_msg.as_ref().unwrap().as_str())
      .block(
        Block::new()
          .border_type(BorderType::Double)
          .borders(Borders::ALL),
      )
      .wrap(Wrap { trim: true });

    let help = Paragraph::new("<Esc> to exit insert mode").alignment(Alignment::Center);

    frame.render_widget(messages, messages_area);

    frame.render_widget(input, input_area);

    frame.render_widget(help, help_area);

    frame.set_cursor_position(Position::new(
      input_area.x + self.input_msg_length as u16 + 1,
      input_area.y + 1,
    ));
  }
}
