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
