use anyhow::Result;
use futures_util::StreamExt;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::config::Config;

#[derive(Debug, Serialize)]
pub struct OllamaRequest<'a> {
    pub model: &'a str,
    pub prompt: &'a str,
    pub system: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<&'a Vec<i32>>,
    pub stream: bool,
}

// Updated to match the actual response format
#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub model: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub response: String,
    pub done: bool,
    #[serde(default)]
    pub context: Vec<i32>,
    #[serde(rename = "done_reason", default)]
    pub done_reason: Option<String>,
    #[serde(rename = "total_duration", default)]
    pub total_duration: Option<u64>,
    #[serde(rename = "load_duration", default)]
    pub load_duration: Option<u64>,
    #[serde(rename = "prompt_eval_count", default)]
    pub prompt_eval_count: Option<u64>,
    #[serde(rename = "prompt_eval_duration", default)]
    pub prompt_eval_duration: Option<u64>,
    #[serde(rename = "eval_count", default)]
    pub eval_count: Option<u64>,
    #[serde(rename = "eval_duration", default)]
    pub eval_duration: Option<u64>,
}

pub async fn send_request(
    request: OllamaRequest<'_>,
    config: &Config,
    tx: mpsc::Sender<String>,
) -> Result<Vec<i32>> {
    let client = Client::new();
    let request_url = format!("{}/api/generate", config.url);
    
    // Only print debug info if debug mode is enabled
    if config.debug {
        eprintln!("Making request to: {}", request_url);
        eprintln!("Model: {}", request.model);
        eprintln!("System prompt: {}", request.system);
        eprintln!("User prompt: {}", request.prompt);
        
        // Print the serialized request for debugging
        let request_json = serde_json::to_string(&request).unwrap_or_default();
        eprintln!("Request JSON: {}", request_json);
    }
    
    let response = match client
        .post(&request_url)
        .json(&request)
        .send()
        .await {
            Ok(r) => {
                if config.debug {
                    eprintln!("Request sent successfully, status: {:?}", r.status());
                }
                r
            },
            Err(e) => {
                eprintln!("Error sending request: {}", e);
                return Err(anyhow::anyhow!("Error sending request: {}", e));
            }
        };
    
    process_stream_response(response, tx, config.debug).await
}

async fn process_stream_response(
    response: Response,
    tx: mpsc::Sender<String>,
    debug: bool,
) -> Result<Vec<i32>> {
    let mut stream = response.bytes_stream();
    let mut buffer = Vec::new();
    let mut context = Vec::new();
    let mut response_count = 0;
    
    if debug {
        eprintln!("Processing response stream...");
    }
    
    while let Some(item) = stream.next().await {
        let chunk = match item {
            Ok(c) => {
                if debug {
                    eprintln!("Received chunk of size: {}", c.len());
                }
                c
            },
            Err(e) => {
                eprintln!("Error receiving chunk: {}", e);
                return Err(anyhow::anyhow!("Error receiving chunk: {}", e));
            }
        };
        
        buffer.extend_from_slice(&chunk);
        
        // Process complete JSON objects from the buffer
        let mut start = 0;
        for i in 0..buffer.len() {
            if buffer[i] == b'\n' {
                if let Ok(text) = std::str::from_utf8(&buffer[start..i]) {
                    if debug {
                        eprintln!("Processing JSON: {}", text);
                    }
                    
                    match serde_json::from_str::<OllamaResponse>(text) {
                        Ok(response) => {
                            response_count += 1;
                            if debug {
                                eprintln!("Response {}: {} chars, done: {}", 
                                         response_count, response.response.len(), response.done);
                            }
                            
                            // Send response to output channel
                            tx.send(response.response).await?;
                            
                            if response.done {
                                if debug {
                                    eprintln!("Final response received, context length: {}", response.context.len());
                                }
                                context = response.context;
                            }
                        },
                        Err(e) => {
                            if debug {
                                eprintln!("Error parsing response JSON: {}", e);
                            }
                        }
                    }
                } else if debug {
                    eprintln!("Invalid UTF-8 in response");
                }
                start = i + 1;
            }
        }
        
        // Keep the remaining partial data
        if start < buffer.len() {
            buffer = buffer[start..].to_vec();
            if debug {
                eprintln!("{} bytes remaining in buffer", buffer.len());
            }
        } else {
            buffer.clear();
            if debug {
                eprintln!("Buffer cleared");
            }
        }
    }
    
    if debug {
        eprintln!("Response stream ended, processed {} responses", response_count);
    }
    Ok(context)
}

pub fn generate_first_request<'a>(
    body: &'a str,
    config: &'a Config,
) -> OllamaRequest<'a> {
    OllamaRequest {
        model: &config.model,
        prompt: body,
        system: &config.prompt,
        context: None,
        stream: true,
    }
}

pub fn generate_request<'a>(
    body: &'a str,
    config: &'a Config,
    context: &'a Vec<i32>,
) -> OllamaRequest<'a> {
    OllamaRequest {
        model: &config.model,
        prompt: body,
        system: &config.prompt,
        context: Some(context),
        stream: true,
    }
}
