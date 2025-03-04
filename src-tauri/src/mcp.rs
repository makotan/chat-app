use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    id: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: Message,
}

pub struct McpClient {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl McpClient {
    pub fn new(api_key: String, base_url: String, model: String) -> Self {
        let client = Client::new();
        Self {
            client,
            api_key,
            base_url,
            model,
        }
    }
    
    // ゲッターメソッドを追加
    pub fn get_api_key(&self) -> String {
        self.api_key.clone()
    }
    
    pub fn get_base_url(&self) -> String {
        self.base_url.clone()
    }
    
    pub fn get_model(&self) -> String {
        self.model.clone()
    }

    pub async fn send_message(
        &self,
        messages: Vec<Message>,
    ) -> Result<String, Box<dyn Error>> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages,
            max_tokens: Some(2000),
            temperature: Some(0.7),
        };

        let response = self
            .client
            .post(&format!("{}/v1/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API error: {}", error_text).into());
        }

        let chat_response: ChatResponse = response.json().await?;
        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No response from model".into())
        }
    }
    
    // 静的メソッドを追加して、MutexGuardの問題を回避
    pub async fn send_message_static(
        api_key: String,
        base_url: String,
        model: String,
        messages: Vec<Message>,
    ) -> Result<String, Box<dyn Error>> {
        let client = Client::new();
        let request = ChatRequest {
            model,
            messages,
            max_tokens: Some(2000),
            temperature: Some(0.7),
        };

        let response = client
            .post(&format!("{}/v1/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("API error: {}", error_text).into());
        }

        let chat_response: ChatResponse = response.json().await?;
        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("No response from model".into())
        }
    }
}