# remove-tax

A simple CLI tool to calculate prices without VAT (German MwSt). Automatically copies results to clipboard.

## Features

- Remove VAT from prices (default: 19% German MwSt)
- Support for custom VAT rates via `--rate` flag or `DEFAULT_VAT_RATE` environment variable
- Accept both comma and dot as decimal separators
- Preserve input number format in output
- Automatically copy results to clipboard
- Display results in a formatted table

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/remove-tax.git
cd remove-tax

# Install to /usr/local/bin (requires sudo)
./install.sh

# Or install to custom directory
./install.sh ~/bin
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

## Requirements

- Rust 1.85 or later
- Cargo

## License

MIT