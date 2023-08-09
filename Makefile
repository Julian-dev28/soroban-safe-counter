default: build

all: test

test: build
	cargo test

build:
	soroban contract build
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

build-logs:
	soroban contract build --profile release-with-logs

fmt:
	cargo fmt --all

clean:
	cargo clean
