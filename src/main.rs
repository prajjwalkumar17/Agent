mod cli;
mod config;
mod handler;
mod requests;
mod streams;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    // Parse command line arguments
    let config = config::load_config()?;
    
    // Determine whether to use oneshot or streaming mode
    if config.stream {
        handler::stream_handler(&config).await?;
    } else {
        handler::oneshot_handler(&config).await?;
    }
    
    Ok(())
}
