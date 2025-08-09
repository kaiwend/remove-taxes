use std::env;

pub struct NumberInput {
    pub value: f64,
    pub uses_comma: bool,
}

pub struct ParsedArgs {
    pub numbers: Vec<NumberInput>,
    pub vat_rate: f64,
}

pub fn print_usage(program_name: &str) {
    eprintln!("Usage: {program_name} <number1> [number2 ...] [--rate <percentage>]");
    eprintln!("Environment variable: DEFAULT_VAT_RATE (default: 19)");
}

pub fn get_default_vat_rate() -> f64 {
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

pub fn parse_arguments(args: Vec<String>) -> Result<ParsedArgs, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_parse_vat_rate_with_comma() {
        assert_eq!(parse_vat_rate("7,5"), 7.5);
    }

    #[test]
    fn test_parse_vat_rate_with_dot() {
        assert_eq!(parse_vat_rate("7.5"), 7.5);
    }
}

