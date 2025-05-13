# Rust Password Generator

A secure password generator written in Rust that allows users to create random passwords with customizable options.

## Features

- Configurable password length (8-128 characters)
- Options to include/exclude:
  - Uppercase letters
  - Numbers
  - Special characters
- Always includes lowercase letters for readability
- Uses Rust's secure random number generator

## Requirements

- Rust and Cargo installed on your system

## Installation

1. Clone the repository:
```bash
git clone https://github.com/YOUR_USERNAME/password-generator.git
cd password-generator
```

2. Build the project:
```bash
cargo build --release
```

## Usage

Run the program:
```bash
cargo run
```

The program will prompt you for:
- Password length (defaults to 12 if no input is provided)
- Whether to include uppercase letters (default: yes)
- Whether to include numbers (default: yes)
- Whether to include special characters (default: yes)

## License

MIT License 