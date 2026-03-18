use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct GLM5Client {
    client: Client,
    api_key: String,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMsg>,
    temperature: f64,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMsg {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMsg,
}

#[derive(Debug, Deserialize)]
pub struct StreamChunk {
    pub choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
pub struct StreamChoice {
    pub delta: Option<StreamDelta>,
}

#[derive(Debug, Deserialize)]
pub struct StreamDelta {
    pub content: Option<String>,
}

impl GLM5Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            base_url: "https://open.bigmodel.cn/api/paas/v4".to_string(),
        }
    }

    pub async fn chat(
        &self,
        messages: Vec<ChatMsg>,
        temperature: f64,
    ) -> Result<String, String> {
        let request = ChatRequest {
            model: "glm-4-plus".to_string(),
            messages,
            temperature,
            stream: false,
        };

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("API request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("GLM-5 API error {status}: {body}"));
        }

        let body: ChatResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {e}"))?;

        body.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| "Empty response from GLM-5".to_string())
    }

    pub async fn chat_stream(
        &self,
        messages: Vec<ChatMsg>,
        temperature: f64,
    ) -> Result<reqwest::Response, String> {
        let request = ChatRequest {
            model: "glm-4-plus".to_string(),
            messages,
            temperature,
            stream: true,
        };

        self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("API request failed: {e}"))
    }
}
