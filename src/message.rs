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
    let mut lines = vec![];
    lines.push(self.author.render());
    self.text.lines().for_each(|line| {
      lines.push(Line::from(line));
    });
    return lines;
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
