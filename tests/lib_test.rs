use remove_tax::calculator::{CalculationResult, process_numbers};
use remove_tax::cli::{parse_arguments, print_usage};
use remove_tax::display::{create_clipboard_content, display_results};
use remove_tax::utils::copy_to_clipboard;

#[test]
fn test_integration_parse_error_flow() {
    // Test the error flow that happens in main
    let args = vec!["program".to_string()];
    let result = parse_arguments(args);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "No arguments provided");
}

#[test]
fn test_integration_full_flow() {
    // Test successful flow through all modules
    let args = vec!["program".to_string(), "119".to_string()];

    let parsed = parse_arguments(args).unwrap();
    assert_eq!(parsed.vat_rate, 19.0);
    assert_eq!(parsed.numbers.len(), 1);

    let results = process_numbers(&parsed.numbers, parsed.vat_rate);
    assert_eq!(results.len(), 1);

    let clipboard_content = create_clipboard_content(&results);
    assert_eq!(clipboard_content, "100.00");

    // These functions print to stdout, just ensure they don't panic
    display_results(&results, parsed.vat_rate);
    copy_to_clipboard(&clipboard_content);
}

#[test]
fn test_print_usage_coverage() {
    // Test print_usage function - it prints to stderr
    print_usage("test-program");
    // If it doesn't panic, test passes
}

#[test]
fn test_copy_to_clipboard_coverage() {
    // Test clipboard function with various inputs
    copy_to_clipboard("");
    copy_to_clipboard("test content");
    copy_to_clipboard("100.00\n200.00");
}

#[test]
fn test_display_results_empty() {
    let results: Vec<CalculationResult> = vec![];
    display_results(&results, 19.0);
}

#[test]
fn test_display_results_multiple() {
    let results = vec![
        CalculationResult {
            with_vat: 119.0,
            without_vat: 100.0,
            uses_comma: false,
        },
        CalculationResult {
            with_vat: 238.0,
            without_vat: 200.0,
            uses_comma: true,
        },
    ];
    display_results(&results, 19.0);
}
