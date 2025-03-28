package inlama

import (
	"bytes"
	"encoding/json"
	"net/http"
)

/*
 * Currently, Olamma's API contract is used to communicate with the LLM model.
 */

type OllamaRequest struct {
	Model   string `json:"model"`
	Prompt  string `json:"prompt"`
	System  string `json:"system"`
	Context *[]int `json:"context"`
	Stream  bool   `json:"stream"`
}

type OllamaResponse struct {
	Done     bool   `json:"done"`
	Response string `json:"response"`
	Context  []int  `json:"context"`
}

func GenerateFirstRequest(body string, config Cli) OllamaRequest {
	return OllamaRequest{
		Model:   config.Model,
		Prompt:  body,
		System:  config.SystemPrompt,
		Context: nil,
		Stream:  true,
	}
}

func GenerateRequest(body string, config Cli, context []int) OllamaRequest {
	return OllamaRequest{
		Model:   config.Model,
		Prompt:  body,
		System:  config.SystemPrompt,
		Context: &context,
		Stream:  true,
	}
}

func SendRequest(request OllamaRequest, config Cli, chunks chan string) ([]int, error) {
	// create a post request and handle a stream of responses

	requestBody, err := json.Marshal(request)
	requestUrl := config.Url + "/api/generate"

	if err != nil {
		return nil, err
	}

	client := &http.Client{}
	req, err := http.NewRequest("POST", requestUrl, bytes.NewBuffer(requestBody))

	if err != nil {
		return nil, err
	}

	req.Header.Set("Content-Type", "application/json")

	resp, err := client.Do(req)

	if err != nil {
		return nil, err
	}

	defer resp.Body.Close()

	decoder := json.NewDecoder(resp.Body)

	var finalContext OllamaResponse

	for {
		var item OllamaResponse

		if err := decoder.Decode(&item); err != nil {
			if err.Error() == "EOF" {
				break
			}
		}

		chunks <- item.Response

		if item.Done {
			finalContext = item
			break
		}
	}

	close(chunks)
	return finalContext.Context, nil

}
