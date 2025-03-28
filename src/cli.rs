use clap::{Parser, Subcommand};
use clap_complete::{Shell, generate, Generator};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Stream input to model
    #[arg(short = 'f', long)]
    pub stream: bool,
    
    /// System prompt for model
    #[arg(short, long, default_value = "Generate a one line summary of the following text.")]
    pub prompt: String,
    
    /// Buffer time for streaming input (in seconds)
    #[arg(short = 'b', long, default_value = "1")]
    pub buffer_time: u64,
    
    /// URL for model
    #[arg(short, long, default_value = "http://localhost:11434")]
    pub url: String,
    
    /// Model to use
    #[arg(short, long, default_value = "llama3.2")]
    pub model: String,
    
    /// Enable debug output
    #[arg(short, long)]
    pub debug: bool,
    
    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Generate shell completions
    Completion {
        /// Shell type
        shell: Shell,
    },
}

pub fn generate_completions(shell: Shell) {
    use clap::CommandFactory;
    
    fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
        generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
    }
    
    let mut cmd = Cli::command();
    print_completions(shell, &mut cmd);
}
