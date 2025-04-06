use ratatui::style::Stylize;
use ratatui::text::Line;

pub struct Message {
  pub text:   String,
  pub author: Author,
}

impl Message {
  pub fn new(text: String, author: Author) -> Self {
    return Self { text, author };
  }

  pub fn render(&self) -> Vec<Line> {
    let author = self.author.render();
    let text = Line::from(self.text.as_ref());
    return vec![author, text];
  }
}

pub enum Author {
  USER(String),
  AI,
}

impl Author {
  fn render(&self) -> Line {
    match self {
      Self::USER(s) => Line::from(s.clone()).red(),
      Self::AI => Line::from("gemini").blue(),
    }
  }
}
