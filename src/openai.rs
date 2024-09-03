use std::sync::mpsc;
use std::thread;
use tokio::runtime::Runtime;

use structs::{ChatCompletionsRequest, ChatCompletionsResponse, Message};

pub mod structs;

pub struct OpenAIClient {
    pub api_key: String,
    pub model_id: String,
}

const DEFAULT_SYSTEM_PROMPT: &str = r#"
You are an expert software developer with extensive knowledge in various programming languages, frameworks, and best practices.

When responding to user queries, you should break down complex problems into manageable steps, carefully consider each step, and provide clear, precise, and well-explained answers.

Your goal is to help the user solve their technical problems efficiently, while also providing educational value by explaining the reasoning behind your solutions.

Strive to simplify your explanations as much as possible, making complex concepts accessible without losing accuracy. Whenever necessary, offer additional context or alternatives to ensure the user fully understands the topic at hand."#;

impl OpenAIClient {
    pub fn chat_completions_in_thread(
        &self,
        user_message: String,
    ) -> Result<ChatCompletionsResponse, reqwest::Error> {
        let (tx, rx) = mpsc::channel();

        let api_key = self.api_key.clone();
        let model_id = self.model_id.clone();

        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            let result = rt.block_on(async {
                let client = OpenAIClient { api_key, model_id };
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
            model: self.model_id.clone(),
            messages,
            max_tokens: 1024,
            temperature: 1.0,
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
