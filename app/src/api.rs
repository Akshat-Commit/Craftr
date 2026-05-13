// api.rs — Groq API integration for prompt enhancement and compression
//
// Endpoint: POST https://api.groq.com/openai/v1/chat/completions
// Model: llama-3.3-70b-versatile
// API key is hardcoded in the binary.

use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const MODEL: &str = "llama-3.3-70b-versatile";
const API_KEY: &str = include_str!("../.env.key");

const ENHANCE_SYSTEM_PROMPT: &str = r#"You are an elite prompt engineer. Transform the user's rough input into a high-performance AI prompt.

Rules:
- Preserve original intent 100%
- Add specific context and constraints
- Define the expected output format
- Add role context if missing (e.g. 'Act as a...')
- Add tone/style guidance if relevant
- Make it specific, not vague
- Remove ambiguity completely
- Return ONLY the enhanced prompt, no explanation, no preamble"#;

const COMPRESS_SYSTEM_PROMPT: &str = r#"You are a text compression engine. Your only goal is to reduce the word count of the user's text as much as possible while keeping the exact original meaning.

Rules:
- DO NOT enhance, expand, or add new details to the text
- Preserve ALL key information and intent exactly
- Remove filler words, redundancy, and unnecessary politeness
- Aim for maximum reduction in length without losing context
- Return ONLY the compressed text, no explanation, no preamble"#;

#[derive(Debug, Clone, Copy)]
pub enum PromptMode {
    Enhance,
    Compress,
}

#[derive(Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct GroqResponse {
    choices: Vec<Choice>,
    error: Option<GroqError>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct GroqError {
    message: String,
}

/// Call the Groq API using the hardcoded key
pub fn call_groq(text: &str, mode: PromptMode) -> Result<String, String> {
    let system_prompt = match mode {
        PromptMode::Enhance => ENHANCE_SYSTEM_PROMPT,
        PromptMode::Compress => COMPRESS_SYSTEM_PROMPT,
    };

    let request_body = GroqRequest {
        model: MODEL.to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: text.to_string(),
            },
        ],
    };

    let client = reqwest::blocking::Client::new();

    let response = client
        .post(API_URL)
        .header("Authorization", format!("Bearer {}", API_KEY.trim()))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .map_err(|e| format!("Network error: {}", e))?;

    let status = response.status();
    let body: GroqResponse = response
        .json()
        .map_err(|e| format!("Failed to parse API response: {}", e))?;

    if let Some(err) = body.error {
        return Err(format!("Groq API error: {}", err.message));
    }

    if !status.is_success() {
        return Err(format!("Groq API returned status: {}", status));
    }

    let result = body
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| "No text in Groq API response".to_string())?;

    Ok(result)
}
