# Makefile for Rust Projects

# Default goal
.DEFAULT_GOAL := help

# Help
help:
	@printf "\n\033[0;36mAvailable Makefile Commands:\033[0m\n"
	@printf "  \033[0;32mmake build     \033[0m- Build the Rust project\n"
	@printf "  \033[0;32mmake run       \033[0m- Run the Rust project\n"
	@printf "  \033[0;32mmake test      \033[0m- Run tests\n"
	@printf "  \033[0;32mmake clean     \033[0m- Clean build artifacts\n"
	@printf "  \033[0;32mmake fmt       \033[0m- Format code with rustfmt\n"
	@printf "  \033[0;32mmake clippy    \033[0m- Lint code with Clippy\n"
	@printf "  \033[0;32mmake doc       \033[0m- Generate documentation\n"

# Build the project
build:
	cargo build

# Run the project
run:
	cargo run

# Run tests
test:
	cargo nextest run

# Clean the project
clean:
	cargo clean

# Format code
fmt:
	cargo fmt

# Lint code
clippy:
	cargo clippy

# Generate documentation
doc:
	cargo doc --open

.PHONY: help build run test clean fmt clippy doc
