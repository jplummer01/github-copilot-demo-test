# HotDog! 🌭 - Random Dog Image Viewer

A simple and fun web application that displays random dog images fetched from the Dog CEO API. Built with Rust and the Dioxus framework.

## Features

- 🐕 **Random Dog Images**: Fetches and displays random dog photos from the Dog CEO API
- 🔄 **Skip Function**: Click the "skip" button to load a new random dog image
- 🎨 **Clean UI**: Simple, responsive web interface with custom styling
- ⚡ **Fast Performance**: Built with Rust for optimal performance

## Technology Stack

- **Language**: Rust
- **Frontend Framework**: Dioxus (Rust-based UI framework similar to React)
- **HTTP Client**: reqwest for API calls
- **API**: [Dog CEO API](https://dog.ceo/api/breeds/image/random) for random dog images
- **Build Tool**: Cargo & Dioxus CLI

## Prerequisites

Before running this application, ensure you have:

- [Rust](https://rustup.rs/) (latest stable version)
- [Dioxus CLI](https://github.com/DioxusLabs/dioxus) - Install with: `cargo install dioxus-cli`

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd reckitt-test
```

2. Install dependencies:
```bash
cargo build
```

## Development


### Running the Application

#### Method 1: Using Dioxus CLI (Recommended for Development)

Run the following command in the root of your project to start the development server:

```bash
dx serve
```

This will start a local development server (typically at `http://localhost:8080`) with hot-reload functionality.

#### Platform-Specific Development

To run for a different platform, use the `--platform` flag:

```bash
# Web platform (default)
dx serve --platform web

# Desktop application
dx serve --platform desktop

# Mobile platform  
dx serve --platform mobile
```

#### Method 2: Using Cargo

Alternatively, you can build and run using standard Cargo commands:

```bash
# Development build
cargo run

# Release build (optimized)
cargo build --release
cargo run --release
```

### Building for Production

To create an optimized build for deployment:

```bash
# Build for web deployment
dx build --release

# Or use cargo for a standard binary
cargo build --release
```

The built application will be available in the `dist/` directory (for web builds) or `target/release/` directory (for binary builds).

## Usage

1. Open the application in your web browser
2. Enjoy viewing random dog images!
3. Click the "skip" button to load a new random dog picture
4. Each image is fetched fresh from the Dog CEO API

## Project Structure

```
├── src/
│   └── main.rs          # Main application logic
├── assets/
│   ├── main.css         # Application styles
│   ├── favicon.ico      # Website icon
│   └── header.svg       # Header graphics
├── Cargo.toml           # Rust dependencies and metadata
├── Dioxus.toml          # Dioxus framework configuration
└── README.md            # This file
```

