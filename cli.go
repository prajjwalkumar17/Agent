package inlama

import (
	"fmt"
	"os"

	"github.com/BurntSushi/toml"
	"github.com/spf13/cobra"
)

type Cli struct {
	Stream       bool     // Stream input to model
	SystemPrompt string   // System prompt for model
	BufferTime   int      // Buffer time for streaming input (in seconds)
	Url          string   // Url for model
	Model        string   // Model to use
	Presets      []string // Presets for system prompts
}

func Init() Cli {
	var cli Cli

	var rootCmd = &cobra.Command{
		Use:   "inlama",
		Short: "Inlama is a CLI tool for using Stdin with LLM models",
		Long:  "Inlama allows you to pass stdin to LLMs to generate statistics, reports and more... \nThe behaviour can be tweaked with flags allowing for streaming input as well as configuring different models and system prompts for precise control over the output.",

		Run: func(cmd *cobra.Command, args []string) {
			// ..
		},
	}

	var presets = []string{
		"Generate a one line summary of the following text.",
	}

	var defaults = Cli{
		Stream:       false,
		SystemPrompt: presets[0],
		BufferTime:   1,
		Url:          "http://localhost:11434",
		Model:        "llama3",
		Presets:      presets,
	}

	// Check if a config file is used

	configFile := os.Getenv("CONFIG_FILE")

	_, err := os.Stat(configFile)

	if err == nil {
		_, err = toml.DecodeFile(configFile, &defaults)

		if err != nil {
			fmt.Fprintln(os.Stderr, "Error reading config file")
			fmt.Fprintln(os.Stderr, err)
			fmt.Fprintln(os.Stderr)
			os.Exit(1)
		}
	}

	selectedPreset := make([]bool, len(defaults.Presets))

	for i := range selectedPreset {
		selectedPreset[i] = false
	}

	var completion string = ""

	rootCmd.Flags().BoolVarP(&cli.Stream, "follow", "f", defaults.Stream, "Stream input to model")
	rootCmd.Flags().StringVarP(&cli.SystemPrompt, "prompt", "p", defaults.SystemPrompt, "System prompt for model")
	rootCmd.Flags().IntVarP(&cli.BufferTime, "buffer-time", "b", defaults.BufferTime, "Buffer time for streaming input (in seconds)")
	rootCmd.Flags().StringVarP(&cli.Url, "url", "u", defaults.Url, "Url for model")
	rootCmd.Flags().StringVarP(&cli.Model, "model", "m", defaults.Model, "Model to use")
	rootCmd.Flags().StringVar(&completion, "completion", "", "Generate shell completion script")

	for i, preset := range defaults.Presets {
		rootCmd.Flags().BoolVar(&selectedPreset[i], fmt.Sprintf("p%d", i), false, fmt.Sprintf("Use the preset: %s", preset))
	}

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	for i, preset := range selectedPreset {
		if preset {
			cli.SystemPrompt = defaults.Presets[i]
			break
		}
	}

	if rootCmd.Flags().Changed("help") {
		os.Exit(0)
	}

	switch completion {
	case "bash":
		rootCmd.Root().GenBashCompletion(os.Stdout)
		os.Exit(0)
	case "zsh":
		rootCmd.Root().GenZshCompletion(os.Stdout)
		os.Exit(0)
	case "fish":
		rootCmd.Root().GenFishCompletion(os.Stdout, true)
		os.Exit(0)
	case "":
		// ..
	default:
		fmt.Fprintf(os.Stderr, "Unknown completion type %q\n", completion)
		os.Exit(1)
	}

	return cli
}
