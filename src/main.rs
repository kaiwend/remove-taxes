use std::env;

mod calculator;
mod cli;
mod display;
mod utils;

use calculator::process_numbers;
use cli::{parse_arguments, print_usage};
use display::{create_clipboard_content, display_results};
use utils::copy_to_clipboard;

fn run(args: Vec<String>) -> Result<(), String> {
    let program_name = args[0].clone();

    let parsed_args = match parse_arguments(args) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("Error: {error}");
            print_usage(&program_name);
            return Err(error);
        }
    };

    let results = process_numbers(&parsed_args.numbers, parsed_args.vat_rate);
    display_results(&results, parsed_args.vat_rate);
    let clipboard_content = create_clipboard_content(&results);
    copy_to_clipboard(&clipboard_content);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if run(args).is_err() {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_with_valid_args() {
        let args = vec!["program".to_string(), "119".to_string()];
        let result = run(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_no_args() {
        let args = vec!["program".to_string()];
        let result = run(args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No arguments provided");
    }

    #[test]
    fn test_run_with_invalid_numbers() {
        let args = vec!["program".to_string(), "abc".to_string()];
        let result = run(args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No valid numbers provided");
    }

    #[test]
    fn test_run_with_rate_missing_value() {
        let args = vec![
            "program".to_string(),
            "100".to_string(),
            "--rate".to_string(),
        ];
        let result = run(args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "--rate requires a value");
    }
}
