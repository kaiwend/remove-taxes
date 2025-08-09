use arboard::Clipboard;
use std::env;

fn calculate_without_vat(amount: f64, vat_rate: f64) -> f64 {
    amount / (1.0 + vat_rate / 100.0)
}

struct NumberInput {
    value: f64,
    uses_comma: bool,
}

struct ParsedArgs {
    numbers: Vec<NumberInput>,
    vat_rate: f64,
}

fn print_usage(program_name: &str) {
    eprintln!("Usage: {program_name} <number1> [number2 ...] [--rate <percentage>]");
    eprintln!("Environment variable: DEFAULT_VAT_RATE (default: 19)");
}

fn get_default_vat_rate() -> f64 {
    env::var("DEFAULT_VAT_RATE")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(19.0)
}

fn parse_number(input: &str) -> Option<NumberInput> {
    let uses_comma = input.contains(',');
    let normalized = input.replace(',', ".");

    normalized
        .parse::<f64>()
        .ok()
        .map(|value| NumberInput { value, uses_comma })
}

fn parse_vat_rate(input: &str) -> f64 {
    input
        .replace(',', ".")
        .parse()
        .unwrap_or_else(|_| get_default_vat_rate())
}

fn parse_arguments(args: Vec<String>) -> Result<ParsedArgs, String> {
    if args.len() < 2 {
        return Err("No arguments provided".to_string());
    }

    let mut vat_rate = get_default_vat_rate();
    let mut numbers = Vec::new();
    let mut i = 1;

    while i < args.len() {
        if args[i] == "--rate" {
            if i + 1 < args.len() {
                vat_rate = parse_vat_rate(&args[i + 1]);
                i += 2;
            } else {
                return Err("--rate requires a value".to_string());
            }
        } else if let Some(number) = parse_number(&args[i]) {
            numbers.push(number);
            i += 1;
        } else {
            i += 1;
        }
    }

    if numbers.is_empty() {
        return Err("No valid numbers provided".to_string());
    }

    Ok(ParsedArgs { numbers, vat_rate })
}

fn format_number(value: f64, use_comma: bool) -> String {
    let formatted = format!("{value:.2}");
    if use_comma {
        formatted.replace('.', ",")
    } else {
        formatted
    }
}

fn print_table_header(vat_rate: f64) {
    println!("\nVAT Rate: {vat_rate}%");
    println!("{:-<50}", "");
    println!("{:<20} | {:<20}", "With VAT", "Without VAT");
    println!("{:-<50}", "");
}

fn print_table_footer() {
    println!("{:-<50}", "");
}

fn print_table_row(with_vat: f64, without_vat: f64, use_comma: bool) {
    let with_vat_str = format_number(with_vat, use_comma);
    let without_vat_str = format_number(without_vat, use_comma);
    println!("{with_vat_str:<20} | {without_vat_str:<20}");
}

fn copy_to_clipboard(content: &str) {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(content) {
                eprintln!("Warning: Could not copy to clipboard: {e}");
            } else {
                println!("\n✓ Results copied to clipboard (without VAT values)");
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not access clipboard: {e}");
        }
    }
}

fn process_numbers(numbers: &[NumberInput], vat_rate: f64) -> Vec<(f64, f64, bool)> {
    numbers
        .iter()
        .map(|input| {
            let without_vat = calculate_without_vat(input.value, vat_rate);
            (input.value, without_vat, input.uses_comma)
        })
        .collect()
}

fn create_clipboard_content(results: &[(f64, f64, bool)]) -> String {
    results
        .iter()
        .map(|(_, without_vat, use_comma)| format_number(*without_vat, *use_comma))
        .collect::<Vec<_>>()
        .join("\n")
}

fn display_results(results: &[(f64, f64, bool)], vat_rate: f64) {
    print_table_header(vat_rate);

    for (with_vat, without_vat, use_comma) in results {
        print_table_row(*with_vat, *without_vat, *use_comma);
    }

    print_table_footer();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let parsed_args = match parse_arguments(args) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("Error: {error}");
            print_usage(&program_name);
            std::process::exit(1);
        }
    };

    let results = process_numbers(&parsed_args.numbers, parsed_args.vat_rate);

    display_results(&results, parsed_args.vat_rate);

    let clipboard_content = create_clipboard_content(&results);
    copy_to_clipboard(&clipboard_content);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_without_vat_19_percent() {
        let result = calculate_without_vat(119.0, 19.0);
        assert!((result - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_without_vat_7_percent() {
        let result = calculate_without_vat(107.0, 7.0);
        assert!((result - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_without_vat_zero_percent() {
        let result = calculate_without_vat(100.0, 0.0);
        assert!((result - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_multiple_values() {
        let values = [119.0, 238.0, 357.0];
        let expected = [100.0, 200.0, 300.0];

        for (value, expected) in values.iter().zip(expected.iter()) {
            let result = calculate_without_vat(*value, 19.0);
            assert!((result - expected).abs() < 0.01);
        }
    }

    #[test]
    fn test_parse_number_with_dot() {
        let input = parse_number("123.45").unwrap();
        assert_eq!(input.value, 123.45);
        assert!(!input.uses_comma);
    }

    #[test]
    fn test_parse_number_with_comma() {
        let input = parse_number("123,45").unwrap();
        assert_eq!(input.value, 123.45);
        assert!(input.uses_comma);
    }

    #[test]
    fn test_format_number_with_comma() {
        assert_eq!(format_number(123.456, true), "123,46");
    }

    #[test]
    fn test_format_number_with_dot() {
        assert_eq!(format_number(123.456, false), "123.46");
    }

    #[test]
    fn test_parse_vat_rate_with_comma() {
        assert_eq!(parse_vat_rate("7,5"), 7.5);
    }

    #[test]
    fn test_parse_vat_rate_with_dot() {
        assert_eq!(parse_vat_rate("7.5"), 7.5);
    }
}

