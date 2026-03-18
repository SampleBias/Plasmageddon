use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct GroqClient {
    client: Client,
    api_key: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMsg>,
    temperature: f64,
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

impl GroqClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn chat(
        &self,
        messages: Vec<ChatMsg>,
        model: &str,
        temperature: f64,
    ) -> Result<String, String> {
        let request = ChatRequest {
            model: model.to_string(),
            messages,
            temperature,
        };

        let resp = self
            .client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Groq API request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Groq API error {status}: {body}"));
        }

        let body: ChatResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse Groq response: {e}"))?;

        body.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| "Empty response from Groq".to_string())
    }
}
