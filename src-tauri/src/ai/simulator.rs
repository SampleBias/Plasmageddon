use super::glm5::{ChatMsg, GLM5Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatorInput {
    pub construct_name: String,
    pub sequence: String,
    pub parts_summary: Vec<String>,
    pub host: String,
    pub copy_number: i32,
    pub time_hours: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatorOutput {
    pub mrna_level: f64,
    pub protein_level: f64,
    pub bottlenecks: Vec<String>,
    pub developability_score: f64,
    pub time_course: Vec<TimeCoursePoint>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeCoursePoint {
    pub time_h: f64,
    pub mrna: f64,
    pub protein: f64,
}

pub async fn run_simulator(client: &GLM5Client, input: &SimulatorInput) -> Result<SimulatorOutput, String> {
    let system_msg = ChatMsg {
        role: "system".to_string(),
        content: "You are Plasmageddon Simulator, a molecular biology expression simulation AI. \
            Given a construct with its parts and host, predict expression levels over time.\n\n\
            Respond ONLY with valid JSON matching this schema:\n\
            {\n  \"mrna_level\": number (relative units),\n  \"protein_level\": number (relative units),\n  \
            \"bottlenecks\": [\"string\"],\n  \"developability_score\": number (0-100),\n  \
            \"time_course\": [{\"time_h\": number, \"mrna\": number, \"protein\": number}],\n  \
            \"notes\": \"string\"\n}".to_string(),
    };

    let user_msg = ChatMsg {
        role: "user".to_string(),
        content: format!(
            "Construct: {}\nHost: {}\nCopy number: {}\nTime course: {} hours\n\nParts: {}\n\nSequence length: {} bp",
            input.construct_name,
            input.host,
            input.copy_number,
            input.time_hours,
            input.parts_summary.join(", "),
            input.sequence.len()
        ),
    };

    let response = client.chat(vec![system_msg, user_msg], 0.3).await?;

    let cleaned = response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    serde_json::from_str::<SimulatorOutput>(cleaned)
        .map_err(|e| format!("Failed to parse simulator response: {e}\nRaw: {cleaned}"))
}
