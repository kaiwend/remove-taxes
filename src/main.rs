use arboard::Clipboard;
use std::env;

fn calculate_without_vat(amount: f64, vat_rate: f64) -> f64 {
    amount / (1.0 + vat_rate / 100.0)
}

struct NumberInput {
    value: f64,
    uses_comma: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} <number1> [number2 ...] [--rate <percentage>]",
            args[0]
        );
        eprintln!("Environment variable: DEFAULT_VAT_RATE (default: 19)");
        std::process::exit(1);
    }

    let default_rate = env::var("DEFAULT_VAT_RATE")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(19.0);

    let mut vat_rate = default_rate;
    let mut numbers = Vec::new();
    let mut i = 1;

    while i < args.len() {
        if args[i] == "--rate" {
            if i + 1 < args.len() {
                let rate_str = args[i + 1].replace(',', ".");
                vat_rate = rate_str.parse().unwrap_or(default_rate);
                i += 2;
            } else {
                eprintln!("Error: --rate requires a value");
                std::process::exit(1);
            }
        } else {
            let original = args[i].clone();
            let uses_comma = original.contains(',');
            let num_str = original.replace(',', ".");
            if let Ok(num) = num_str.parse::<f64>() {
                numbers.push(NumberInput {
                    value: num,
                    uses_comma,
                });
            }
            i += 1;
        }
    }

    if numbers.is_empty() {
        eprintln!("Error: No valid numbers provided");
        std::process::exit(1);
    }

    let mut clipboard_text = Vec::new();
    let mut results = Vec::new();

    for input in &numbers {
        let without_vat = calculate_without_vat(input.value, vat_rate);
        results.push((input, without_vat));

        let formatted = if input.uses_comma {
            format!("{without_vat:.2}").replace('.', ",")
        } else {
            format!("{without_vat:.2}")
        };
        clipboard_text.push(formatted);
    }

    println!("\nVAT Rate: {vat_rate}%");
    println!("{:-<50}", "");
    println!("{:<20} | {:<20}", "With VAT", "Without VAT");
    println!("{:-<50}", "");

    for (input, without_vat) in &results {
        let with_vat_str = if input.uses_comma {
            format!("{:.2}", input.value).replace('.', ",")
        } else {
            format!("{:.2}", input.value)
        };

        let without_vat_str = if input.uses_comma {
            format!("{without_vat:.2}").replace('.', ",")
        } else {
            format!("{without_vat:.2}")
        };

        println!("{with_vat_str:<20} | {without_vat_str:<20}");
    }
    println!("{:-<50}", "");

    let clipboard_content = clipboard_text.join("\n");
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(&clipboard_content) {
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
}

