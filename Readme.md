# Log Localizer

`log-localizer` is a simple command-line tool written in Rust that helps you localize logs or log files by displaying which files and lines have generated the given logs.

## Features

- Localize logs from a single file or provided as an input.
- Display the file names and line numbers where the logs were generated.
- Easy to use and efficient in processing large log files.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo installed on your system.

## Installation

1. Clone the repository:

```bash
git clone https://github.com/alejandromc23/log-localizer.git
```

2. Change into the project directory:

```bash
cd log-localizer
```

3. Build the project:

```bash
cargo build --release
```

4. Add the binary to your `PATH` or copy it to a directory in your `PATH`. The binary will be located in `target/release/log-localizer`

## Usage

To localize logs from log file:

```bash
log-localizer -f /path/to/your/logfile.log 
```

To localize logs from the command line:

```bash
log-localizer "your log data here"
```

This will output the file names and line numbers where the given logs were generated.

## License

This project is licensed under the MIT License. See [LICENSE](https://github.com/alejandromc23/log-localizer/blob/master/LICENSE) for more information.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests
