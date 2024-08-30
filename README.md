# cloc-rs

A Rust implementation of the cloc (Count Lines of Code) utility.

## Description

cloc-rs is a command-line tool that counts the number of lines of code in a given directory or file. It can filter by file extension and recursively search through directories.

## Features

- Count lines of code in files and directories
- Filter by file extension
- Recursive directory traversal
- Simple and fast implementation in Rust

## Installation

To install cloc-rs, you need to have Rust and Cargo installed on your system. Then, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/AdolfoVillalobos/cloc-rs.git
   cd cloc-rs
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The binary will be available at `target/release/cloc-rs`

Alternatively, you can install it directly using:
```
cargo install --path .
```

## Usage

To count the lines of code in a directory, run:
```
cloc-rs <path> [extension]

- `PATH`: The directory or file to analyze (default: current directory)
- `EXTENSION`: Optional file extension to filter by (e.g., "rs" for Rust files)
```

## Examples

Count lines in the current directory:

```
cloc-rs .
```

Count lines in the current directory, filtering by Rust files:

```
cloc-rs . rs
```


## Development

To run the tests, run:
```
cargo test
```


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the original [cloc](https://github.com/AlDanial/cloc) tool
- Built with [Rust](https://www.rust-lang.org/) and [walkdir](https://crates.io/crates/walkdir)