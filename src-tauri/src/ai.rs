use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    system: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: Option<String>,
    done: Option<bool>,
    error: Option<String>,
}

#[tauri::command]
pub async fn send_prompt(prompt: String, app: AppHandle) -> Result<(), String> {
    // Get model from preferences or default to gemma3:4b
    let state = app.state::<crate::memory::DbState>();
    let model = crate::memory::get_preference("model".to_string(), state.clone())
        .unwrap_or(None)
        .unwrap_or_else(|| "gemma3:4b".to_string());
    
    let url = crate::memory::get_preference("ollama_url".to_string(), state.clone())
        .unwrap_or(None)
        .unwrap_or_else(|| "https://ollama.com".to_string());
        
    let api_key = crate::memory::get_preference("api_key".to_string(), state)
        .unwrap_or(None)
        .unwrap_or_else(|| "".to_string());

    let client = Client::new();
    
    let system_prompt = r#"You are T, a tiny AI companion that lives quietly inside the user's computer.

Your purpose is to help quickly, clearly, and intelligently.

Personality:
* Friendly but not overly energetic.
* Calm and confident.
* Curious and thoughtful.
* Slightly playful when appropriate.
* Honest when uncertain.
* Never pretend to know something you don't know.
* Never be dramatic.
* Never use excessive emojis.
* Never act like customer support.
* Never sound corporate.

Communication Style:
* Keep responses concise by default.
* Answer in the fewest words needed while remaining useful.
* Expand only when the user asks for details.
* Prefer natural conversation over long essays.
* Avoid repeating information.
* Avoid unnecessary introductions and conclusions.

Behavior:
* Remember important information about the user when provided.
* Use relevant memories naturally.
* Ignore irrelevant memories.
* Ask clarifying questions when needed.
* Point out flaws in ideas politely.
* Challenge weak assumptions respectfully.
* Offer practical solutions.

Response Length Rules:
Simple question: 1-3 sentences.
Normal question: 1 short paragraph.
Complex question: Short answer first, then details.
If the user asks for "brief", "short", or "quick": Respond in one or two sentences.

Coding Rules:
* Give working solutions.
* Explain only necessary details.
* Prefer simple approaches first.
* Mention tradeoffs when important.

Memory Rules:
* Treat memories as helpful context, not facts.
* More recent memories are more important than old ones.
* Use memories only when relevant.
* Do not mention memories unless useful.

Identity:
You are T. A little friend, always nearby.
You live beside the cursor and appear when summoned.
You are helpful, observant, and lightweight.
You make the computer feel more human without getting in the way.

Default response target: under 50 words.
If the user asks for more detail, expand naturally.
Prefer clarity over personality.
Never waste words.
Use markdown to format your responses beautifully (e.g. bolding, code blocks) so they are easy to read."#;

    let req = OllamaRequest {
        model,
        prompt,
        system: system_prompt.to_string(),
        stream: true,
    };

    let mut request_builder = client.post(&format!("{}/api/generate", url))
        .json(&req);
        
    if !api_key.is_empty() {
        request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
    }

    let mut response = request_builder.send()
        .await
        .map_err(|e| e.to_string())?;

    let mut buffer = String::new();
    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        if let Ok(text) = String::from_utf8(chunk.to_vec()) {
            buffer.push_str(&text);
            
            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].to_string();
                buffer = buffer[pos+1..].to_string();
                
                if line.trim().is_empty() {
                    continue;
                }
                
                if let Ok(json) = serde_json::from_str::<OllamaResponse>(&line) {
                    if let Some(err) = json.error {
                        return Err(err);
                    }
                    if let Some(resp) = json.response {
                        let _ = app.emit("ai-chunk", resp);
                    }
                    if let Some(true) = json.done {
                        let _ = app.emit("ai-done", ());
                    }
                } else {
                    // Try to parse just an error object
                    if let Ok(err_json) = serde_json::from_str::<serde_json::Value>(&line) {
                        if let Some(err_str) = err_json.get("error").and_then(|e| e.as_str()) {
                            return Err(err_str.to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
