

# protocheck

Proto vs JSON Validator

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**protocheck** is a command line tool designed to validate JSON data against a Protobuf schema (`.proto` file). It ensures that the JSON adheres to the structure defined in the `.proto` file, checking for additional fields, missing fields, and potential type mismatches.

---

## Table of Contents

1. [Overview](#overview)
2. [Features](#features)
3. [Installation](#installation)
4. [Usage](#usage)
5. [Dependencies](#dependencies)
6. [Example](#example)
7. [Validation Errors](#validation-errors)
8. [Contributing](#contributing)
9. [License](#license)

---

## Overview

This tool allows you to validate JSON data dynamically against a Protobuf schema at runtime. It uses the `protobuf` and `protobuf-parse` crates to parse `.proto` files and extract their structure. The tool then compares the JSON data with the schema to identify any discrepancies.

Key features include:
- Detecting additional fields in the JSON that are not defined in the `.proto` file.
- Identifying missing fields in the JSON that are required by the `.proto` file.
- Ignoring missing fields if specified via a CLI flag.

---

## Features

- **Runtime Validation**: Validate JSON data against `.proto` schemas without precompiling descriptors.
- **Flexible Input**: Accept JSON input from a file or standard input (`stdin`).
- **Customizable Behavior**: Use the `--ignore-missing` flag to ignore missing fields in the JSON.
- **Error Reporting**: Provides detailed error messages for invalid JSON data.

---

## Installation

### Prebuilt Binaries
You can download the latest release from the [Releases](https://github.com/xeonn/protocheck/releases) page.

**Note:** Due to changes in our release pipeline, macOS binaries are not provided. We currently support:
- Linux (AMD64)
- Windows (AMD64)

### Build from Source
To build from source, ensure you have Rust installed:
```sh
cargo build --release
```

## Usage

### Command-Line Arguments

```bash
protocheck [OPTIONS]
```

#### Options

| Flag              | Description                                                                 |
|-------------------|-----------------------------------------------------------------------------|
| `-p, --proto FILE` | Path to the `.proto` file defining the schema.                              |
| `-i, --include DIR`| Path to directory containing other `.proto` files referenced by the schema.                              |
| `-j, --json FILE`  | Path to the JSON file to validate. Use `-` to read from standard input.     |
| `-g, --ignore-missing` | Ignore missing fields in the JSON that are defined in the `.proto` file.    |

### Example Commands

1. Validate a JSON file against a `.proto` file:
   ```bash
   protocheck --proto example.proto --include proto_inc --json input.json
   ```

2. Validate JSON from standard input:
   ```bash
   echo '{"txnid": "abc123", "amount": 100}' | protocheck --proto example.proto --include proto_inc --json -
   ```

3. Ignore missing fields in the JSON:
   ```bash
   protocheck --proto example.proto --include proto_inc --json input.json --ignore-missing
   ```

---

## Dependencies

This project relies on the following Rust crates:

- [`clap`](https://crates.io/crates/clap): For parsing command-line arguments.
- [`protobuf`](https://crates.io/crates/protobuf): For working with Protobuf schemas.
- [`protobuf-parse`](https://crates.io/crates/protobuf-parse): For parsing `.proto` files at runtime.
- [`serde_json`](https://crates.io/crates/serde_json): For parsing and manipulating JSON data.
- [`tokio`](https://crates.io/crates/tokio): For asynchronous file I/O operations.

Add these dependencies to your `Cargo.toml` file:

```toml
[dependencies]
clap = "4.0"
protobuf = "3.7.1"
protobuf-parse = "3.7.1"
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

---

## Example

### `.proto` File (`example.proto`)

```proto
syntax = "proto3";

message Transaction {
    string txnid = 1;
    int32 amount = 2;
}
```

### JSON Input (`input.json`)

```json
{
    "txnid": "abc123",
    "amount": 100
}
```

### Output

```plaintext
JSON input: input.json
Read from file
JSON: {"txnid":"abc123","amount":100}
JSON is valid against the Protobuf schema.
```

If the JSON contains errors, they will be reported as follows:

```plaintext
Validation Error: ValidationError { field: "unknown_field", error_type: AdditionalField }
Validation Error: ValidationError { field: "amount", error_type: MissingField }
```

---

## Validation Errors

The tool reports the following types of validation errors:

- **AdditionalField**: A field exists in the JSON but is not defined in the `.proto` file.
- **MissingField**: A field required by the `.proto` file is missing in the JSON.
- **WrongDataType**: (Future enhancement) A field has an incorrect data type.
- **MissingArrayField**: (Future enhancement) An array field is missing or malformed.

---

## Contributing

We welcome contributions! To contribute to this project:

1. Fork the repository.
2. Create a new branch for your feature or bug fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Commit your changes:
   ```bash
   git commit -m "Add your commit message here"
   ```
4. Push your branch:
   ```bash
   git push origin feature/your-feature-name
   ```
5. Open a pull request.

Please ensure your code adheres to Rust's best practices and includes appropriate tests.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- Thanks to the Rust community for providing excellent tools like `protobuf`, `serde_json`, and `clap`.
- Special thanks to contributors who help improve this project.

---

Feel free to customize this `README.md` further based on your project's specific needs. Let me know if you need additional sections or modifications! 😊