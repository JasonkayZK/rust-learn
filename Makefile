.PHONY: fmt clippy clean build pack

ci: fmt clippy

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy -- -D warnings

clean:
	rm -rf ./target

build:
	cargo build

pack:
	cargo build --
