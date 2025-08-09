# remove-tax

[![CI](https://github.com/yourusername/remove-tax/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/remove-tax/actions/workflows/ci.yml)

A simple CLI tool to calculate prices without VAT (German MwSt). Automatically copies results to clipboard.

## Features

- Remove VAT from prices (default: 19% German MwSt)
- Support for custom VAT rates via `--rate` flag or `DEFAULT_VAT_RATE` environment variable
- Accept both comma and dot as decimal separators
- Preserve input number format in output
- Automatically copy results to clipboard
- Display results in a formatted table
- Cross-platform support (Linux, Windows, macOS)

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/remove-tax.git
cd remove-tax

# Build and install to /usr/local/bin (requires sudo)
./install.sh

# Or install to custom directory
./install.sh ~/bin
```

### Manual Build

```bash
# Build release version
cargo build --release

# Binary will be at ./target/release/remove-tax
```

## Usage

```bash
# Remove 19% VAT (default)
remove-tax 119 238 357

# Use comma as decimal separator
remove-tax 119,50 238,00

# Custom VAT rate
remove-tax 100 200 --rate 7

# Set default VAT rate via environment variable
DEFAULT_VAT_RATE=7 remove-tax 107 214

# Mix number formats (comma and dot)
remove-tax 119,50 238.00 --rate 7,5
```

## Example Output

```
VAT Rate: 19%
--------------------------------------------------
With VAT             | Without VAT         
--------------------------------------------------
119.00               | 100.00              
238.00               | 200.00              
--------------------------------------------------

✓ Results copied to clipboard (without VAT values)
```

## Development

### Project Structure

```
src/
├── main.rs          # Application entry point
├── lib.rs           # Library exports
├── cli/             # Command-line argument parsing
├── calculator/      # VAT calculation logic
├── display/         # Output formatting and display
└── utils/           # Clipboard utilities
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run directly
cargo run -- 119 238
```

### Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test --verbose

# Run specific test module
cargo test cli::tests

# Check code coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Stdout
```

Current test coverage: **94.94%** (75/79 lines)

### Code Quality

```bash
# Format code
cargo fmt

# Run clippy linter
cargo clippy

# Check formatting
cargo fmt -- --check
```

## CI/CD

The project uses GitHub Actions for continuous integration:

- **Multi-platform testing**: Linux, Windows, macOS
- **Automated checks**: Build, test, format, clippy
- **Runs on**: Push to main branch and pull requests

## Requirements

- Rust 1.85 or later
- Cargo

## License

MIT