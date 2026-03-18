use super::glm5::{ChatMsg, GLM5Client};

pub fn build_system_prompt(construct_context: Option<&str>) -> ChatMsg {
    let base = "You are Kernel AI, the assistant in Plasmageddon — a desktop CAD for synthetic biology. \
        You help with genetic construct design, part selection, codon optimization, cloning strategies, \
        and general molecular biology questions. Be concise, accurate, and practical.";

    let content = if let Some(ctx) = construct_context {
        format!("{base}\n\nCurrent construct context:\n{ctx}")
    } else {
        base.to_string()
    };

    ChatMsg {
        role: "system".to_string(),
        content,
    }
}

pub async fn chat_with_context(
    client: &GLM5Client,
    messages: Vec<ChatMsg>,
    construct_context: Option<&str>,
) -> Result<String, String> {
    let system = build_system_prompt(construct_context);
    let mut full_messages = vec![system];
    full_messages.extend(messages);
    client.chat(full_messages, 0.7).await
}
