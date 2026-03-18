use tauri::{AppHandle, Emitter, State};

use crate::ai::compiler::{self, CompilerInput, CompilerOutput};
use crate::ai::glm5::{ChatMsg, GLM5Client, StreamChunk};
use crate::ai::groq::GroqClient;
use crate::ai::part_match::{self, PartSuggestion};
use crate::ai::simulator::{self, SimulatorInput, SimulatorOutput};
use crate::db::AppDatabase;

fn get_glm5_client(db: &AppDatabase) -> Result<GLM5Client, String> {
    let key = db
        .get_setting("glm5_api_key")?
        .ok_or_else(|| "GLM-5 API key not configured. Set it in Settings.".to_string())?;
    Ok(GLM5Client::new(&key))
}

fn get_groq_client(db: &AppDatabase) -> Result<GroqClient, String> {
    let key = db
        .get_setting("groq_api_key")?
        .ok_or_else(|| "Groq API key not configured. Set it in Settings.".to_string())?;
    Ok(GroqClient::new(&key))
}

#[tauri::command]
pub async fn run_compiler(db: State<'_, AppDatabase>, input: CompilerInput) -> Result<CompilerOutput, String> {
    let client = get_glm5_client(&db)?;
    compiler::run_compiler(&client, &input).await
}

#[tauri::command]
pub async fn run_simulator(db: State<'_, AppDatabase>, input: SimulatorInput) -> Result<SimulatorOutput, String> {
    let client = get_glm5_client(&db)?;
    simulator::run_simulator(&client, &input).await
}

#[tauri::command]
pub async fn ai_chat(
    app: AppHandle,
    db: State<'_, AppDatabase>,
    construct_id: Option<String>,
    message: String,
) -> Result<String, String> {
    let client = get_glm5_client(&db)?;

    db.add_chat_message(construct_id.as_deref(), "user", &message)?;

    let history = db.get_chat_history(construct_id.as_deref())?;
    let messages: Vec<ChatMsg> = history
        .iter()
        .map(|m| ChatMsg {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();

    let construct_ctx = if let Some(ref cid) = construct_id {
        db.get_construct(cid).ok().map(|c| {
            format!(
                "Name: {}\nTopology: {}\nSequence length: {} bp\nParts: [loaded in editor]",
                c.name,
                c.topology,
                c.sequence.len()
            )
        })
    } else {
        None
    };

    let response = crate::ai::chat::chat_with_context(&client, messages, construct_ctx.as_deref()).await?;

    db.add_chat_message(construct_id.as_deref(), "assistant", &response)?;

    let _ = app.emit("ai:response", &response);

    Ok(response)
}

#[tauri::command]
pub async fn ai_chat_stream(
    app: AppHandle,
    db: State<'_, AppDatabase>,
    construct_id: Option<String>,
    message: String,
) -> Result<String, String> {
    use futures_lite::StreamExt;

    let client = get_glm5_client(&db)?;

    db.add_chat_message(construct_id.as_deref(), "user", &message)?;

    let history = db.get_chat_history(construct_id.as_deref())?;
    let system = crate::ai::chat::build_system_prompt(None);
    let mut messages = vec![system];
    for m in &history {
        messages.push(ChatMsg {
            role: m.role.clone(),
            content: m.content.clone(),
        });
    }

    let response = client.chat_stream(messages, 0.7).await?;
    let mut stream = response.bytes_stream();
    let mut full_response = String::new();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| e.to_string())?;
        let text = String::from_utf8_lossy(&chunk);

        for line in text.lines() {
            if let Some(data) = line.strip_prefix("data: ") {
                if data.trim() == "[DONE]" {
                    break;
                }
                if let Ok(parsed) = serde_json::from_str::<StreamChunk>(data) {
                    for choice in &parsed.choices {
                        if let Some(ref delta) = choice.delta {
                            if let Some(ref content) = delta.content {
                                full_response.push_str(content);
                                let _ = app.emit("ai:chunk", content.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    let _ = app.emit("ai:done", true);
    db.add_chat_message(construct_id.as_deref(), "assistant", &full_response)?;

    Ok(full_response)
}

#[tauri::command]
pub async fn get_chat_history(
    db: State<'_, AppDatabase>,
    construct_id: Option<String>,
) -> Result<Vec<crate::db::chat::ChatMessage>, String> {
    db.get_chat_history(construct_id.as_deref())
}

#[tauri::command]
pub async fn clear_chat_history(
    db: State<'_, AppDatabase>,
    construct_id: Option<String>,
) -> Result<(), String> {
    db.clear_chat_history(construct_id.as_deref())
}

#[tauri::command]
pub async fn suggest_parts(
    db: State<'_, AppDatabase>,
    current_parts: Vec<String>,
) -> Result<Vec<PartSuggestion>, String> {
    let client = get_groq_client(&db)?;
    let all_parts = db.list_parts(None)?;
    let library_names: Vec<String> = all_parts
        .iter()
        .map(|p| format!("{} ({})", p.name, p.part_type))
        .collect();

    part_match::suggest_parts(&client, &current_parts, &library_names).await
}
