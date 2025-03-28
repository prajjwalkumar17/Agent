
.PHONY: build inlama run install


run: ./cmd/inlama/main.go inlama
	go run ./cmd/inlama

inlama: ./streams.go ./requests.go ./go.mod


build:
	mkdir -p bin
	go build -o bin/inlama ./cmd/inlama

install: build
	sudo cp bin/inlama /usr/local/bin/inlama

update:
	git pull origin HEAD

