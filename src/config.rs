use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs, io};

use crate::cli::{Cli, Commands};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub stream: bool,
    pub prompt: String,
    pub buffer_time: u64,
    pub url: String,
    pub model: String,
    pub presets: Vec<String>,
    pub debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            stream: false,
            prompt: "Generate a one line summary of the following text.".to_string(),
            buffer_time: 1,
            url: "http://localhost:11434".to_string(),
            model: "llama3.2".to_string(), // Updated to match your installed model
            presets: vec!["Generate a one line summary of the following text.".to_string()],
            debug: false,
        }
    }
}

pub fn load_config() -> Result<Config> {
    // Try to load from config file first
    let mut config = load_config_file().unwrap_or_default();
    
    // Parse CLI args
    let cli = Cli::parse();
    
    // Handle subcommands if present
    if let Some(Commands::Completion { shell }) = cli.command {
        crate::cli::generate_completions(shell);
        std::process::exit(0);
    }
    
    // Override with CLI args
    config.stream = cli.stream;
    
    // Only override if explicitly provided
    if cli.prompt != "Generate a one line summary of the following text." {
        config.prompt = cli.prompt;
    }
    
    if cli.buffer_time != 1 {
        config.buffer_time = cli.buffer_time;
    }
    
    if cli.url != "http://localhost:11434" {
        config.url = cli.url;
    }
    
    if cli.model != "llama3" {
        config.model = cli.model;
    }
    
    config.debug = cli.debug;
    
    if config.debug {
        eprintln!("Configured with model: {}", config.model);
        eprintln!("Server URL: {}", config.url);
    }
    
    Ok(config)
}

fn load_config_file() -> Result<Config> {
    // Check environment variable for config file path
    if let Ok(config_path) = env::var("CONFIG_FILE") {
        let expanded_path = shellexpand::tilde(&config_path);
        let path = Path::new(expanded_path.as_ref());
        
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            let config: Config = toml::from_str(&contents)?;
            return Ok(config);
        }
    }
    
    // Check default locations
    let home_dir = dirs::home_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find home directory")
    })?;
    
    let config_paths = [
        home_dir.join(".config/inlama/config.toml"),
        home_dir.join(".inlama.toml"),
    ];
    
    for path in &config_paths {
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            let config: Config = toml::from_str(&contents)?;
            return Ok(config);
        }
    }
    
    // No config file found, return error
    Err(anyhow::anyhow!("No config file found"))
}
