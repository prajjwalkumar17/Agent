package main

import (
	"github.com/NishantJoshi00/inlama"
)

func main() {
	config := inlama.Init()

	switch config.Stream {
	case false:
		inlama.OneshotHandler(config)
	case true:
    inlama.StreamHandler(config)
	}

}
