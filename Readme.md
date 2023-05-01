# Multigrep

`multigrep` is a simple command-line tool written in Rust that helps you localize queries by displaying which files and lines have generated the given queries.

## Features

- Localize queries from a single file or provided as an input.
- Display the file names and line numbers where the queries were generated.
- Easy to use and efficient in processing large files.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo installed on your system.

## Installation

1. Clone the repository:

```bash
git clone https://github.com/alejandromc23/multigrep.git
```

2. Change into the project directory:

```bash
cd multigrep
```

3. Build the project:

```bash
cargo build --release
```

4. Add the binary to your `PATH` or copy it to a directory in your `PATH`. The binary will be located in `target/release/multigrep`

## Usage

To localize queries from file:

```bash
multigrep -f /path/to/your/file 
```

To localize queries from the command line:

```bash
multigrep "query 1" "query 2"
```

This will output the file names and line numbers where the given queries were generated.

## License

This project is licensed under the MIT License. See [LICENSE](https://github.com/alejandromc23/multigrep/blob/master/LICENSE) for more information.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests
