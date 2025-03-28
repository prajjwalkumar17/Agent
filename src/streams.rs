use anyhow::Result;
use std::io::{self, BufRead, BufReader};
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;

pub async fn stream_read_stdin(tx: mpsc::Sender<String>) -> Result<()> {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    
    for line in reader.lines() {
        match line {
            Ok(text) => {
                tx.send(text).await?;
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Error reading from stdin: {}", e));
            }
        }
    }
    
    Ok(())
}

pub async fn oneshot_read_stdin() -> Result<Vec<String>> {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut buffer = Vec::new();
    
    for line in reader.lines() {
        match line {
            Ok(text) => {
                buffer.push(text);
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Error reading from stdin: {}", e));
            }
        }
    }
    
    Ok(buffer)
}
