# Display Rust tool versions
rust-version:
	@echo "Rust command-line utility versions:"
	@rustc --version            # Rust compiler
	@cargo --version            # Rust package manager
	@rustfmt --version          # Rust code formatter
	@rustup --version           # Rust toolchain manager
	@clippy-driver --version    # Rust linter

# Install Rust toolchain
install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Build project in release mode
release:
	cd rust_cli_binary && cargo build --release

# Format code using rustfmt
format:
	cd rust_cli_binary && cargo fmt --quiet

# Run clippy linter
lint:
	cd rust_cli_binary && cargo clippy --quiet

# Run the project
run:
	cd rust_cli_binary && cargo run

# Run tests
test:
	cd rust_cli_binary && cargo test --quiet
