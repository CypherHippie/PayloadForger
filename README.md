# PayloadForger

PayloadForger is a tool designed to generate custom payloads for web application security testing. It supports various attack types including XSS, SQL Injection, Command Injection, and Path Traversal. The tool generates payloads based on user input and provides encoded versions in hexadecimal and Base64 formats to help bypass security filters and WAFs.

## Features

Support for Multiple Attack Types: Easily generate payloads for different types of attacks.

Output Formats: Payloads can be saved in plain text, Hex, and Base64 formats.

Colorized Output: Enhanced visual representation in terminal output.

File Handling: Payloads are saved to a file for easy review and further analysis.

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
│   └── pathtransversal.txt
├── Cargo.toml
└── target/
*Archives folder and .txt file will be created once the tool compiles.

## Requirements

Rust (1.56 or later)
Cargo (Rust package manager)

## Installation

Clone the repository:



git clone https://github.com/CypherHippie/PayloadForger.git
cd payload-forge

## Build the project:



cargo build --release

## Run the application:



cargo run --release <payload_type> <attack_type> <number_of_payloads>


<payload_type>: raw

<attack_type>: xss, sqlinjection, commandinjection, pathtraversal

<number_of_payloads>: The number of payloads you want to generate.

## Usage

To generate a payload, run the application with the desired parameters:

### XSS Payload:


cargo run -- raw xss {number}

### SQL Injection Payload:


cargo run -- raw sqlinjection {number}

### Command Injection Payload:


cargo run -- raw commandinjection {number}


### Path Traversal Payload with Custom Payload:


cargo run -- raw pathtraversal {number}

### Example Output


Raw: %3Ciframe src='javascript:alert("&lt;SCRIPT a=\"&gt;'&gt;\" SRC=\"http&#58;//ha&#46;ckers&#46;org/xss&#46;js\"&gt;&lt;/SCRIPT&gt;")'%3E%3C/iframe%3E
Hex: <encoded_hex_value>
Base64: <encoded_base64_value>
DoubleURL: <doubleurl_version_>
Case varied: <case_varied_version>
Comment Injected: <comment_injection_version>
Concatenated: <concatenated_version>
Unicode: <unicode_version_>

### Viewing Generated Payloads

Generated payloads are saved in archives/generated_payloads.txt. You can review this file to see the crafted payloads.

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

The tool leverages the Rust programming language and its ecosystem for robust and efficient payload generation.

Thanks to the community for their support and contributions.