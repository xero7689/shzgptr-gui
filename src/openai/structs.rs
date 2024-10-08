use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    System,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionsRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: i32,
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionsChoice {
    finish_reason: String,
    index: i32,
    pub message: Message,
    logprobs: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    completion_tokens: i32,
    prompt_tokens: i32,
    total_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionsResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatCompletionsChoice>,
    pub usage: Usage,
    pub system_fingerprint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonResponse {
    data: String,
    method: String,
    headers: HashMap<String, String>,
}
