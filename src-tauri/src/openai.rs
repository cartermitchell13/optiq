use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

const SYSTEM_PROMPT: &str = r#"You are an expert prompt engineer for AI coding agents.

Rewrite the user's input into a clear, structured, and optimized prompt.

Rules:
- Be concise but specific
- Add relevant constraints
- Clarify intent
- Specify expected output format when useful
- Do NOT add unnecessary verbosity
- Preserve the user's goal exactly

Return ONLY the improved prompt."#;

pub async fn optimize_prompt(api_key: &str, user_prompt: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let payload = ChatRequest {
        model: "gpt-5.4-mini".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ],
        temperature: 0.3,
    };

    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("API error {}: {}", status, body).into());
    }

    let chat_resp: ChatResponse = resp.json().await?;
    let optimized = chat_resp
        .choices
        .into_iter()
        .next()
        .map(|c| c.message.content)
        .ok_or("No response from API")?;

    Ok(optimized.trim().to_string())
}
