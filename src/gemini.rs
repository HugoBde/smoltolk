use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::chat::Chat;
use crate::message::{Author, Message};

lazy_static! {
  pub static ref GEMINI_API_ADDRESS: String = {
    let gemini_api_key = std::env::var("GEMINI_API_KEY").unwrap();
    return format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}", gemini_api_key);
  };
}

#[derive(Serialize)]
struct GeminiRequest {
  contents: Vec<GeminiMessage>,
}

#[derive(Serialize)]
struct GeminiMessage {
  role:  &'static str,
  parts: Vec<MessagePart>,
}

#[derive(Deserialize)]
struct GeminiResponse {
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

pub fn to_req_body(chat: &Chat) -> Result<String> {
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
      return GeminiMessage { role, parts };
    })
    .collect();

  let req = GeminiRequest { contents };

  let req_body = serde_json::to_string(&req)?;

  return Ok(req_body);
}

pub fn to_response(res: String) -> Result<Message> {
  let res = serde_json::from_str::<GeminiResponse>(&res)?;

  let msg = res.candidates[0].content.parts[0].text.clone();

  let msg = Message::new(msg, Author::AI);

  return Ok(msg);
}
