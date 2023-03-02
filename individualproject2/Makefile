rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format-check:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet


build-release:
	cargo build --release
	
run:
	cargo run

build:
	docker build -t spotifyrecommdention .

run-docker:
	docker run -it --rm -p 8000:8000 spotifyrecommdention

run-pulled-image:
	docker pull chen/spotifyrecommdention:latest
	docker run -it --rm -p 8000:8000 chen/spotifyrecommdention:latest

all: format lint test run
