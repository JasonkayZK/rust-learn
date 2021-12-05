.PHONY: fmt clippy clean build pack all ci

all: clean fmt clippy pack

ci: fmt clippy

fmt:
	cargo fmt --all --

clippy:
	cargo clippy --  -D warnings

clean:
	rm -rf ./target

build:
	cargo build

pack:
	cargo build --release
