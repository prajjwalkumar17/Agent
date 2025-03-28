package inlama

import (
	"bufio"
	"fmt"
	"os"
)

func StreamReadStdin(dataChan chan string) error {
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		dataChan <- scanner.Text()
	}

	close(dataChan)

	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "Error reading from stdin:")
		fmt.Fprintln(os.Stderr, err)
		return err
	}

	return nil
}

func OneshotReadStdin() ([]string, error) {
	scanner := bufio.NewScanner(os.Stdin)
	var buffer []string

	for scanner.Scan() {
		buffer = append(buffer, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "Error reading from stdin:")
    fmt.Fprintln(os.Stderr, err)
		return buffer, err
	}

	return buffer, nil
}
