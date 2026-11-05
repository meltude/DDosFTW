.PHONY: all build run clean

all: build

build:
	cargo build --release
	mkdir -p bin
	cp target/release/DDosFTW bin/DDosFTW

run:
	cargo run

clean:
	rm -rf bin/ target/
