# inLama-rs

A powerful CLI tool written in Rust that seamlessly integrates Large Language Models (LLMs) into your command-line workflow, enabling intelligent text processing through simple Unix-style pipes.

## Description

inLama bridges the gap between traditional command-line tools and modern AI capabilities by:

- Allowing direct piping of text into LLMs for processing
- Supporting both one-shot and continuous streaming modes
- Providing customizable system prompts for specific use cases
- Integrating smoothly with existing shell tools and workflows

Perfect for developers, system administrators, and anyone who needs to analyze, summarize, or process text data directly from the command line.

## Installation

### Prerequisites

- Rust and Cargo (1.70 or higher recommended)
- Running Ollama server (default: http://localhost:11434)

### From Cargo

```bash
cargo install inlama
```

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/inlama-rs.git
cd inlama-rs

# Build and install
cargo install --path .
```

## Usage

### Basic Commands

```bash
# Process a file
cat logs.txt | inlama

# Stream processing (continuous monitoring)
tail -f /var/log/syslog | inlama -f

# Custom system prompt
echo "Hello World" | inlama -p "Translate this text to French"

# Use a different model
cat article.txt | inlama -m gpt4
```

### Shell Completion

Set up shell completion for enhanced usability:

```bash
# Bash
inlama completion bash > ~/.bash_completion.d/inlama

# Zsh
inlama completion zsh > ~/.zfunc/_inlama

# Fish
inlama completion fish > ~/.config/fish/completions/inlama.fish
```

### Configuration Options

- `-f, --stream`: Enable streaming mode for continuous input
- `-p, --prompt`: Set custom system prompt
- `-b, --buffer-time`: Set buffer time for streaming (seconds)
- `-u, --url`: Set custom Ollama server URL
- `-m, --model`: Specify LLM model to use

### Configuration File

Create a configuration file at one of these locations:
- `~/.config/inlama/config.toml`
- `~/.inlama.toml`

Or specify a custom location using the environment variable:
```bash
CONFIG_FILE=/path/to/config.toml inlama
```

Example configuration:

```toml
stream = false
prompt = "Generate a one line summary of the following text."
buffer_time = 1
url = "http://localhost:11434"
model = "llama3"
presets = [
  "Generate a one line summary of the following text.",
  "Translate this text to French."
]
```

## Features

- **Unix-Style Piping**: Seamlessly integrates with standard Unix pipes and filters
- **Streaming Support**: Real-time processing of continuous data streams
- **Flexible Configuration**:
  - Custom system prompts for specialized tasks
  - Configurable buffer times for streaming
  - Support for different LLM models
- **Smart Context Management**: Maintains conversation context in streaming mode
- **Shell Integration**:
  - Comprehensive shell completion support
  - Compatible with bash, zsh, and fish shells
- **Configuration File Support**: TOML-based configuration for persistent settings

## Contributing Guidelines

1. **Issue First**: Create or find an issue before starting work
2. **Issue Tags**: Use descriptive tags:
   - [BUG] for bug reports
   - [FEATURE] for feature requests
   - [DOCS] for documentation improvements
3. **Testing**: Ensure your changes don't break existing functionality
4. **Code Style**: Follow Rust standard formatting guidelines with `cargo fmt`
5. **Documentation**: Update relevant documentation for any changes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
