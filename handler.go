package inlama

import (
	"fmt"
	"io"
	"os"
	"strings"
	"sync"
	"time"
)

func IoHandler(response chan string, stream io.Writer, wg *sync.WaitGroup) {

	wg.Add(1)
	go func() {
		defer wg.Done()
		for {
			resp, more := <-response
			if !more {
				break
			}
			fmt.Fprint(stream, resp)
		}
	}()

}

// # Finality Handlers
//

func OneshotHandler(config Cli) {
	var wg sync.WaitGroup
	input, err := OneshotReadStdin()

	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	joinedInput := strings.Join(input, "\n")

	response := make(chan string)

	request := GenerateFirstRequest(joinedInput, config)

	go IoHandler(response, os.Stdout, &wg)

	_, err = SendRequest(request, config, response)

	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}

	wg.Wait()
}

func StreamHandler(config Cli) {
	var wg sync.WaitGroup

	bodyStream := make(chan string)
	timeout := time.Duration(config.BufferTime) * time.Second

	go func() {
		err := StreamReadStdin(bodyStream)

		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
	}()

	var fullBody []string
	var context []int = nil

	for {
		select {
		case body, more := <-bodyStream:

			if !more {
				return
			} else {
				fullBody = append(fullBody, body)
			}
		case <-time.After(timeout):

			joinedInput := strings.Join(fullBody, "\n")
			fullBody = nil

			response := make(chan string)

			go IoHandler(response, os.Stdout, &wg)

			var request OllamaRequest

			if joinedInput == "" {
				continue
			}

			if context == nil {
				request = GenerateFirstRequest(joinedInput, config)
			} else {
				request = GenerateRequest(joinedInput, config, context)
			}

			var err error

			context, err = SendRequest(request, config, response)

			if err != nil {
				fmt.Fprintln(os.Stderr, err)
				os.Exit(1)
			}

			wg.Wait()
		}
	}
}
