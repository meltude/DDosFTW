.PHONY: all build run clean

all: build

build:
	go build -o bin/main src/main.go

run:
	go run src/main.go

clean:
	rm -rf bin/
