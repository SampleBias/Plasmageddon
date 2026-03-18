use super::groq::{ChatMsg, GroqClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartSuggestion {
    pub part_name: String,
    pub part_type: String,
    pub reason: String,
    pub score: f64,
}

pub async fn suggest_parts(
    client: &GroqClient,
    current_parts: &[String],
    library_parts: &[String],
) -> Result<Vec<PartSuggestion>, String> {
    let system_msg = ChatMsg {
        role: "system".to_string(),
        content: "You suggest genetic parts from a library that would complement a construct. \
            Respond ONLY with a JSON array of suggestions: \
            [{\"part_name\": \"string\", \"part_type\": \"string\", \"reason\": \"string\", \"score\": 0.0-1.0}]"
            .to_string(),
    };

    let user_msg = ChatMsg {
        role: "user".to_string(),
        content: format!(
            "Current construct parts:\n{}\n\nAvailable library parts:\n{}",
            current_parts.join("\n"),
            library_parts.join("\n")
        ),
    };

    let response = client
        .chat(vec![system_msg, user_msg], "llama-3.3-70b-versatile", 0.3)
        .await?;

    let cleaned = response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    serde_json::from_str::<Vec<PartSuggestion>>(cleaned)
        .map_err(|e| format!("Failed to parse part suggestions: {e}"))
}
