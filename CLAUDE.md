# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust project named `solar_economy` using the 2024 edition of Rust. The project is in early development with a minimal structure.

## Build and Development Commands

- **Build the project**: `cargo build`
- **Run the project**: `cargo run`
- **Build for release**: `cargo build --release`
- **Run tests**: `cargo test`
- **Run a specific test**: `cargo test <test_name>`
- **Check code without building**: `cargo check`
- **Format code**: `cargo fmt`
- **Run linter**: `cargo clippy`
- **Clean build artifacts**: `cargo clean`

## Project Structure

Currently the project has a minimal structure:
- `src/main.rs`: Entry point containing the main function
- `Cargo.toml`: Project manifest with no dependencies yet
- Build artifacts are excluded via `.gitignore` (target directory)

## Architecture Notes

This is a new project with basic scaffolding. The codebase currently consists of a single main.rs file with a hello world implementation.
