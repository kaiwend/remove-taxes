use arboard::Clipboard;
use std::env;

fn calculate_without_vat(amount: f64, vat_rate: f64) -> f64 {
    amount / (1.0 + vat_rate / 100.0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <number1> [number2 ...] [--rate <percentage>]", args[0]);
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
            let num_str = args[i].replace(',', ".");
            if let Ok(num) = num_str.parse::<f64>() {
                numbers.push(num);
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
    
    for num in &numbers {
        let without_vat = calculate_without_vat(*num, vat_rate);
        results.push((*num, without_vat));
        clipboard_text.push(format!("{:.2}", without_vat));
    }
    
    println!("\nVAT Rate: {}%", vat_rate);
    println!("{:-<50}", "");
    println!("{:<20} | {:<20}", "With VAT", "Without VAT");
    println!("{:-<50}", "");
    
    for (with_vat, without_vat) in &results {
        println!("{:<20.2} | {:<20.2}", with_vat, without_vat);
    }
    println!("{:-<50}", "");
    
    let clipboard_content = clipboard_text.join("\n");
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(&clipboard_content) {
                eprintln!("Warning: Could not copy to clipboard: {}", e);
            } else {
                println!("\n✓ Results copied to clipboard (without VAT values)");
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not access clipboard: {}", e);
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
        let values = vec![119.0, 238.0, 357.0];
        let expected = vec![100.0, 200.0, 300.0];
        
        for (value, expected) in values.iter().zip(expected.iter()) {
            let result = calculate_without_vat(*value, 19.0);
            assert!((result - expected).abs() < 0.01);
        }
    }
}