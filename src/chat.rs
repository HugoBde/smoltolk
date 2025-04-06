use crate::message::Message;

pub struct Chat {
  pub title:    String,
  pub messages: Vec<Message>,
}

impl Chat {
  pub fn new(title: String) -> Self {
    return Self {
      title,
      messages: vec![],
    };
  }
}
