# Beacon Generator

This project is a Rust-based tool for generating beacon data files from a given API endpoint. The generated files contain beacon information grouped by buildings and are used for location-based services.

## Features

- Fetches beacon data from a specified API endpoint.
- Parses the JSON response and groups beacons by buildings.
- Generates Rust source files with beacon data.
- Supports feature flags for conditional compilation of beacon data.

## Requirements

- Rust
- Cargo

## Usage

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. Build the project:
    ```sh
    cargo build
    ```

3. Run the tool to generate beacon data:
    ```sh
    cargo run --package xtask generate --uri <API-URL> --output <OUTPUT-PATH>
    ```

    - `--uri`: The API endpoint to fetch beacon data from.
