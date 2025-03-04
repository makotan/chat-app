use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

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
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        // 静的メソッドを利用して実装を共有
        Self::send_message_static(
            self.api_key.clone(),
            self.base_url.clone(),
            self.model.clone(),
            messages
        ).await
    }
    
    // 静的メソッドを追加して、MutexGuardの問題を回避
    pub async fn send_message_static(
        api_key: String,
        base_url: String,
        model: String,
        messages: Vec<Message>,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let client = Client::new();
        let request = ChatRequest {
            model,
            messages,
            max_tokens: Some(2000),
            temperature: Some(0.7),
        };

        // リトライ設定
        let max_retries = 3;
        let mut retry_count = 0;
        let mut last_error = None;

        while retry_count < max_retries {
            match Self::try_send_request(&client, &base_url, &api_key, &request).await {
                Ok(content) => return Ok(content),
                Err(e) => {
                    // エラーを保存
                    last_error = Some(e.to_string());
                    
                    // 一時的なエラーの場合のみリトライ
                    if retry_count < max_retries - 1 {
                        // 指数バックオフ（1秒、2秒、4秒...）
                        let backoff_duration = Duration::from_secs(2u64.pow(retry_count as u32));
                        sleep(backoff_duration).await;
                        retry_count += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        // すべてのリトライが失敗した場合
        Err(format!("Failed after {} retries. Last error: {}",
            max_retries, last_error.unwrap_or_else(|| "Unknown error".to_string())).into())
    }
    
    // リクエスト送信を試行する内部メソッド
    async fn try_send_request(
        client: &Client,
        base_url: &str,
        api_key: &str,
        request: &ChatRequest,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let response = client
            .post(&format!("{}/v1/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            
            // ステータスコードに基づいてエラーメッセージをカスタマイズ
            let error_message = match status {
                StatusCode::UNAUTHORIZED => format!("認証エラー: APIキーが無効です。 ({})", error_text),
                StatusCode::TOO_MANY_REQUESTS => format!("レート制限エラー: リクエストが多すぎます。 ({})", error_text),
                StatusCode::BAD_REQUEST => format!("リクエストエラー: リクエストの形式が正しくありません。 ({})", error_text),
                StatusCode::INTERNAL_SERVER_ERROR => format!("サーバーエラー: APIサーバーで問題が発生しました。 ({})", error_text),
                _ => format!("API エラー ({}): {}", status.as_u16(), error_text),
            };
            
            return Err(error_message.into());
        }

        let chat_response: ChatResponse = response.json().await?;
        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err("モデルからの応答がありません".into())
        }
    }
}