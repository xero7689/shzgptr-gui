use std::sync::mpsc;
use std::thread;
use tokio::runtime::Runtime;

use structs::{ChatCompletionsRequest, ChatCompletionsResponse, Message};

pub mod structs;

#[derive(Copy, Clone)]
pub enum ModelId {
    Gpt4o,
    Gpt4oMini,
}

impl ModelId {
    pub fn to_string(&self) -> String {
        match self {
            ModelId::Gpt4o => "gpt-4o".to_string(),
            ModelId::Gpt4oMini => "gpt-4o-mini".to_string(),
        }
    }
}

pub struct ModelConfig {
    pub max_tokens: i32,
    pub temperature: f32,
}

pub struct OpenAIClient {
    pub api_key: String,
    pub model_id: ModelId,
    pub model_config: ModelConfig,
}

const DEFAULT_SYSTEM_PROMPT: &str = r#"
You are an expert software developer with extensive knowledge in various programming languages, frameworks, and best practices.

When responding to user queries, you should break down complex problems into manageable steps, carefully consider each step, and provide clear, precise, and well-explained answers.

Your goal is to help the user solve their technical problems efficiently, while also providing educational value by explaining the reasoning behind your solutions.

Strive to simplify your explanations as much as possible, making complex concepts accessible without losing accuracy. Whenever necessary, offer additional context or alternatives to ensure the user fully understands the topic at hand."#;

impl OpenAIClient {
    pub fn new(
        api_key: String,
        model_id: Option<ModelId>,
        max_tokens: Option<i32>,
        temperature: Option<f32>,
    ) -> Self {
        Self {
            api_key,
            model_id: model_id.unwrap_or(ModelId::Gpt4oMini),
            model_config: ModelConfig {
                max_tokens: max_tokens.unwrap_or(1024),
                temperature: temperature.unwrap_or(1.0),
            },
        }
    }

    pub fn chat_completions_in_thread(
        &self,
        user_message: String,
    ) -> Result<ChatCompletionsResponse, reqwest::Error> {
        let (tx, rx) = mpsc::channel();

        let api_key = self.api_key.clone();
        let model_id = Some(self.model_id);
        let max_tokens = Some(self.model_config.max_tokens);
        let temperature = Some(self.model_config.temperature);

        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            let result = rt.block_on(async {
                let client = OpenAIClient::new(api_key, model_id, max_tokens, temperature);
                client.chat_completions(user_message).await
            });
            tx.send(result).unwrap();
        });

        let response = rx.recv().unwrap();
        response
    }

    pub async fn chat_completions(
        &self,
        user_message: String,
    ) -> Result<ChatCompletionsResponse, reqwest::Error> {
        let messages = vec![
            Message {
                role: "system".into(),
                content: DEFAULT_SYSTEM_PROMPT.into(),
            },
            Message {
                role: "user".into(),
                content: user_message,
            },
        ];

        let request = ChatCompletionsRequest {
            model: self.model_id.to_string(),
            messages,
            max_tokens: self.model_config.max_tokens,
            temperature: self.model_config.temperature,
        };
        let response_json = reqwest::Client::new()
            .post("https://api.openai.com/v1/chat/completions")
            .json(&request)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .text()
            .await?;

        let response: ChatCompletionsResponse = serde_json::from_str(&response_json).unwrap();
        Ok(response)
    }
}
