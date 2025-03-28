use anyhow::Result;
use std::io::{self, Write};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

use crate::config::Config;
use crate::requests::{generate_first_request, generate_request, send_request};
use crate::streams::{oneshot_read_stdin, stream_read_stdin};

pub async fn oneshot_handler(config: &Config) -> Result<()> {
    // Print debug info if enabled
    if config.debug {
        eprintln!("Starting oneshot handler");
        eprintln!("Using model: {}", config.model);
        eprintln!("Using URL: {}", config.url);
    }
    
    // Read all input from stdin
    let input = oneshot_read_stdin().await?;
    if config.debug {
        eprintln!("Read {} lines from stdin", input.len());
    }
    let joined_input = input.join("\n");
    
    // Set up channel for response chunks
    let (tx, mut rx) = mpsc::channel::<String>(100);
    
    // Clone necessary data for the async task
    let model = config.model.clone();
    let prompt = config.prompt.clone();
    let url = config.url.clone();
    let debug = config.debug;
    let config_clone = Config {
        model,
        prompt,
        url,
        buffer_time: config.buffer_time,
        stream: config.stream,
        presets: config.presets.clone(),
        debug,
    };
    
    // Create and send request - move the joined_input into the task
    let input_clone = joined_input.clone();
    if config.debug {
        eprintln!("Sending request to LLM...");
    }
    let sender_task = tokio::spawn(async move {
        let request = generate_first_request(&input_clone, &config_clone);
        send_request(request, &config_clone, tx).await
    });
    
    // Output handler
    let mut stdout = io::stdout();
    let mut output = String::new();
    if config.debug {
        eprintln!("Waiting for response chunks...");
    }
    
    while let Some(chunk) = rx.recv().await {
        if config.debug {
            eprintln!("Received chunk of length: {}", chunk.len());
        }
        output.push_str(&chunk);
        stdout.write_all(chunk.as_bytes())?;
        stdout.flush()?;
    }
    
    // Make sure we have a final newline
    if !output.is_empty() && !output.ends_with('\n') {
        stdout.write_all(b"\n")?;
    }
    
    // Check for errors from sender task
    if config.debug {
        eprintln!("Waiting for sender task to complete...");
    }
    match sender_task.await {
        Ok(Ok(_)) => {
            if config.debug {
                eprintln!("Request completed successfully");
            }
        },
        Ok(Err(e)) => eprintln!("Request failed: {}", e),
        Err(e) => eprintln!("Sender task failed: {}", e),
    }
    
    if config.debug {
        eprintln!("Handler completed");
    }
    Ok(())
}

pub async fn stream_handler(config: &Config) -> Result<()> {
    // Print debug info if enabled
    if config.debug {
        eprintln!("Starting stream handler");
        eprintln!("Using model: {}", config.model);
        eprintln!("Using URL: {}", config.url);
    }
    
    // Set up channel for input stream from stdin
    let (stdin_tx, mut stdin_rx) = mpsc::channel::<String>(100);
    
    // Spawn task to read from stdin
    tokio::spawn(async move {
        if let Err(e) = stream_read_stdin(stdin_tx).await {
            eprintln!("Error reading stdin: {}", e);
        }
    });
    
    let mut full_body = Vec::new();
    let mut context: Option<Vec<i32>> = None;
    let timeout = Duration::from_secs(config.buffer_time);
    let mut stdout = io::stdout();
    
    // Clone config for use in the loop
    let model = config.model.clone();
    let prompt = config.prompt.clone();
    let url = config.url.clone();
    let debug = config.debug;
    let config_clone = Config {
        model,
        prompt,
        url,
        buffer_time: config.buffer_time,
        stream: config.stream,
        presets: config.presets.clone(),
        debug,
    };
    
    if config.debug {
        eprintln!("Entering main processing loop...");
    }
    loop {
        tokio::select! {
            Some(line) = stdin_rx.recv() => {
                if config.debug {
                    eprintln!("Received input line: {}", line);
                }
                full_body.push(line);
            }
            _ = time::sleep(timeout) => {
                if full_body.is_empty() {
                    if config.debug {
                        eprintln!("No input received, continuing...");
                    }
                    continue;
                }
                
                let joined_input = full_body.join("\n");
                if config.debug {
                    eprintln!("Processing {} lines of input", full_body.len());
                }
                full_body.clear();
                
                // Set up channel for response chunks
                let (tx, mut rx) = mpsc::channel::<String>(100);
                
                // Create a complete clone of all data needed for the task
                let task_config = config_clone.clone();
                let task_input = joined_input.clone();
                let task_context = context.clone();
                
                // Create and send the request
                if config.debug {
                    eprintln!("Sending request to LLM...");
                }
                let sender_task = tokio::spawn(async move {
                    match task_context {
                        Some(ctx) => {
                            if task_config.debug {
                                eprintln!("Using existing context of length {}", ctx.len());
                            }
                            let ctx_clone = ctx.clone();
                            let request = generate_request(&task_input, &task_config, &ctx_clone);
                            send_request(request, &task_config, tx).await
                        },
                        None => {
                            if task_config.debug {
                                eprintln!("No existing context, starting new conversation");
                            }
                            let request = generate_first_request(&task_input, &task_config);
                            send_request(request, &task_config, tx).await
                        }
                    }
                });
                
                // Output response chunks
                let mut output = String::new();
                if config.debug {
                    eprintln!("Waiting for response chunks...");
                }
                while let Some(chunk) = rx.recv().await {
                    if config.debug {
                        eprintln!("Received chunk of length: {}", chunk.len());
                    }
                    output.push_str(&chunk);
                    stdout.write_all(chunk.as_bytes())?;
                    stdout.flush()?;
                }
                
                // Make sure we have a final newline
                if !output.is_empty() && !output.ends_with('\n') {
                    stdout.write_all(b"\n")?;
                    stdout.flush()?;
                }
                
                // Update context for next request
                if config.debug {
                    eprintln!("Waiting for sender task to complete...");
                }
                match sender_task.await {
                    Ok(Ok(ctx)) => {
                        if config.debug {
                            eprintln!("Request completed with context of length {}", ctx.len());
                        }
                        context = Some(ctx);
                    },
                    Ok(Err(e)) => {
                        eprintln!("Error sending request: {}", e);
                        return Err(anyhow::anyhow!("Error sending request: {}", e));
                    }
                    Err(e) => {
                        eprintln!("Task error: {}", e);
                        return Err(anyhow::anyhow!("Task error: {}", e));
                    }
                }
            }
        }
    }
}
