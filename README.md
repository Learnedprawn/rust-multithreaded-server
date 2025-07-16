# Multithreaded Web Server

A simple HTTP server built from scratch in Rust, following the exercises from [The Rust Programming Language](https://doc.rust-lang.org/book/) book.

## Features

- **HTTP/1.1 support** - Handles basic GET requests
- **Multithreaded** - Uses a thread pool to handle concurrent connections
- **Graceful shutdown** - Properly closes connections and cleans up resources
- **Static file serving** - Serves HTML files from the local directory

## Quick Start

```bash
# Clone and run
git clone <https://github.com/Learnedprawn/rust-multithreaded-server.git>
cd rust-multithreaded-server
cargo run

# Server starts on http://127.0.0.1:3000
```

## What's Inside

- `src/main.rs` - Server setup and request handling
- `src/lib.rs` - Thread pool implementation
- `hello.html` - Sample HTML file
- `404.html` - Error page

## Learning Goals

This project demonstrates:
- TCP socket programming
- HTTP protocol basics
- Thread pool patterns
- Rust ownership and concurrency
- Error handling and resource management

Built while working through Chapter 21 of the Rust Book! 🦀
