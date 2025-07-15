# TerminalAI: A Local Code Interpreter in Rust

## Description
TerminalAI is a local, privacy-focused alternative to OpenAI's Code Interpreter. It allows users to interact with their local environment using natural language, executing code based on those interactions. Built with Rust for performance, safety, and efficiency, TerminalAI ensures reliable and secure execution of tasks directly on your machine.

Rust was chosen as the primary language for this project due to its:
- **Memory Safety**: Rust's ownership model eliminates common errors like null pointer dereferences and data races, ensuring safe and reliable execution.
- **Performance**: Rust's zero-cost abstractions and native compilation provide speed comparable to C and C++, making it ideal for resource-intensive tasks.
- **Concurrency**: Rust's built-in support for concurrency through async/await and threads enables efficient handling of multiple tasks, such as managing user input and executing code.
- **CLI Support**: Rust has excellent libraries like `clap` for building robust command-line interfaces, making it a natural choice for terminal-based applications.

## Features
- Runs entirely locally on your machine
- Full access to local environment, including internet and file system
- Integrates with local language models for natural language understanding
- Supports execution of code in various programming languages (planned)
- Safe execution environment with sandboxing (planned)
- Advanced utilities and commands (see below)

## Usage Guide

### File and Directory Operations
- `list files in current directory` — List files in the current directory.
- `show current directory` — Show the current working directory.
- `change directory to <path>` — Change the working directory.
- `show file <filename>` — Display the contents of a file.
- `write to file <filename>: <content>` — Overwrite a file with content.
- `delete file <filename>` — Delete a file.
- `search "<pattern>" in <filename or directory>` — Search for a regex pattern in a file or directory.
- `summarize file <filename>` — Show a summary (first 5 lines) of a file.
- `count lines in <filename>` / `count words in <filename>` / `count chars in <filename>` — File statistics.
- `diff <file1> <file2>` — Show a unified diff between two files.
- `rename files in <directory> matching "<pattern>" to "<replacement>"` — Batch rename files using regex.
- `show disk usage` — Show disk usage for the current directory.
- `watch <file or directory>` — Monitor for changes (blocks until interrupted).

### System and Utility Commands
- `show system info` — Show OS and architecture.
- `show date and time` — Show the current date and time.
- `show top processes` — List top CPU-consuming processes.
- `help` — List all available commands.
- `schedule "<command>" at <HH:MM>` — Schedule a command to run at a specific time.

### Code and Math
- `calculate <expression>` — Evaluate a math expression.
- `run code <language>: <code>` — Run code in Python, JavaScript (Node), or Bash.
- `plot a sine wave` — Generate and save a sine wave plot.

### Internet and External
- `download <url> to <filename>` — Download a file from the internet.
- `show weather in <city>` — Show current weather for a city.

### Media and Conversion
- `resize image <file> to <width>x<height>` — Resize an image.
- `convert image <file> to <format>` — Convert image format (png, jpg, bmp, gif).
- `play audio <file>` — Play an audio file (afplay/aplay).
- `convert audio <file> to <format>` — Convert audio using ffmpeg.
- `speak "<text>"` — Text-to-speech (say/espeak).

### Clipboard
- `copy "<text>" to clipboard` — Copy text to clipboard.
- `paste from clipboard` — Paste text from clipboard.

### Advanced
- `explain "<shell command>"` — Explain a shell command (stub).
- `run "<natural language instruction>"` — Run a shell command from natural language.
- `generate password <length>` — Generate a random password.
- `extract <archive> to <directory>` — Extract .zip or .tar.gz/.tgz archives.

## Installation
To build from source, clone the repository and run:
```sh
cargo build --release
```
The binary will be located at `target/release/terminalai`.

## Usage
Run `terminalai` in your terminal to start the interpreter. Type your natural language commands, and the interpreter will execute the corresponding code.

Example:
```
> list files in current directory
main.rs
Cargo.toml
README.md

> calculate 2 + 2
4

> plot a sine wave
[plot displayed]
```

## Contributing
Contributions are welcome! Please fork the repository and submit pull requests. Guidelines for contributing can be found in the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments
- [llm](https://github.com/rustformers/llm) for LLM inference in Rust
- [mistral.rs](https://github.com/ParisNeo/mistral.rs) for quantized LLM models
- [Open Interpreter](https://github.com/OpenInterpreter/open-interpreter) for inspiration
