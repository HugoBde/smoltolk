use color_eyre::Result;
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::chat::Chat;
use crate::message::Author;

lazy_static! {
  static ref GEMINI_API_KEY: String = std::env::var("GEMINI_API_KEY").unwrap();
}

#[derive(Serialize)]
struct Request {
  contents: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
  role:  &'static str,
  parts: Vec<MessagePart>,
}

#[derive(Deserialize)]
struct Response {
  candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
  content: Content,
}

#[derive(Deserialize)]
struct Content {
  parts: Vec<MessagePart>,
}

#[derive(Serialize, Deserialize)]
struct MessagePart {
  text: String,
}

pub fn send_req(chat: &Chat) -> Result<String> {
  let client = Client::new();

  let contents = chat
    .messages
    .iter()
    .map(|m| {
      let role = match m.author {
        Author::USER(_) => "user",
        Author::AI => "model",
      };
      let parts = vec![MessagePart {
        text: m.text.clone(),
      }];
      return Message { role, parts };
    })
    .collect();

  let req_body = Request { contents };

  let req_body = serde_json::to_string(&req_body)?;

  let req = client.post(
    &format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}", 
      *GEMINI_API_KEY
      )
    )
    .header("Content-Type", "application/json")
    .body(req_body);

  let res = req.send()?.text()?;

  let res = serde_json::from_str::<Response>(&res)?;

  let res = res.candidates[0].content.parts[0].text.clone();

  return Ok(res);
}
