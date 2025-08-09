use crate::calculator::CalculationResult;

pub fn format_number(value: f64, use_comma: bool) -> String {
    let formatted = format!("{value:.2}");
    if use_comma {
        formatted.replace('.', ",")
    } else {
        formatted
    }
}

pub fn print_table_header(vat_rate: f64) {
    println!("\nVAT Rate: {vat_rate}%");
    println!("{:-<50}", "");
    println!("{:<20} | {:<20}", "With VAT", "Without VAT");
    println!("{:-<50}", "");
}

pub fn print_table_footer() {
    println!("{:-<50}", "");
}

pub fn print_table_row(with_vat: f64, without_vat: f64, use_comma: bool) {
    let with_vat_str = format_number(with_vat, use_comma);
    let without_vat_str = format_number(without_vat, use_comma);
    println!("{with_vat_str:<20} | {without_vat_str:<20}");
}

pub fn display_results(results: &[CalculationResult], vat_rate: f64) {
    print_table_header(vat_rate);

    for result in results {
        print_table_row(result.with_vat, result.without_vat, result.uses_comma);
    }

    print_table_footer();
}

pub fn create_clipboard_content(results: &[CalculationResult]) -> String {
    results
        .iter()
        .map(|result| format_number(result.without_vat, result.uses_comma))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculator::CalculationResult;

    #[test]
    fn test_format_number_with_comma() {
        assert_eq!(format_number(123.456, true), "123,46");
    }

    #[test]
    fn test_format_number_with_dot() {
        assert_eq!(format_number(123.456, false), "123.46");
    }

    #[test]
    fn test_format_number_zero() {
        assert_eq!(format_number(0.0, false), "0.00");
        assert_eq!(format_number(0.0, true), "0,00");
    }

    #[test]
    fn test_format_number_negative() {
        assert_eq!(format_number(-123.456, false), "-123.46");
        assert_eq!(format_number(-123.456, true), "-123,46");
    }

    #[test]
    fn test_format_number_very_large() {
        assert_eq!(format_number(999999999.999, false), "1000000000.00");
        assert_eq!(format_number(999999999.999, true), "1000000000,00");
    }

    #[test]
    fn test_format_number_very_small() {
        assert_eq!(format_number(0.001, false), "0.00");
        assert_eq!(format_number(0.009, false), "0.01");
    }

    #[test]
    fn test_format_number_rounding() {
        assert_eq!(format_number(123.454, false), "123.45");
        assert_eq!(format_number(123.455, false), "123.45");
        assert_eq!(format_number(123.456, false), "123.46");
    }

    #[test]
    fn test_create_clipboard_content_empty() {
        let results = vec![];
        assert_eq!(create_clipboard_content(&results), "");
    }

    #[test]
    fn test_create_clipboard_content_single() {
        let results = vec![CalculationResult {
            with_vat: 119.0,
            without_vat: 100.0,
            uses_comma: false,
        }];
        assert_eq!(create_clipboard_content(&results), "100.00");
    }

    #[test]
    fn test_create_clipboard_content_multiple() {
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
        assert_eq!(create_clipboard_content(&results), "100.00\n200,00");
    }

    #[test]
    fn test_create_clipboard_content_mixed_formats() {
        let results = vec![
            CalculationResult {
                with_vat: 119.50,
                without_vat: 100.42,
                uses_comma: true,
            },
            CalculationResult {
                with_vat: 238.00,
                without_vat: 200.00,
                uses_comma: false,
            },
            CalculationResult {
                with_vat: 357.00,
                without_vat: 300.00,
                uses_comma: true,
            },
        ];
        assert_eq!(create_clipboard_content(&results), "100,42\n200.00\n300,00");
    }

    #[test]
    fn test_display_results() {
        // This test captures stdout to verify display_results works
        let results = vec![CalculationResult {
            with_vat: 119.0,
            without_vat: 100.0,
            uses_comma: false,
        }];

        // We can't easily capture stdout in tests, but we can ensure the function runs without panic
        display_results(&results, 19.0);
        // If this doesn't panic, the test passes
    }

    #[test]
    fn test_print_table_row_coverage() {
        // Test to ensure print_table_row is covered
        print_table_row(119.0, 100.0, false);
        print_table_row(119.0, 100.0, true);
    }
}
