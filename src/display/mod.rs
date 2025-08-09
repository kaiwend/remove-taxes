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

    #[test]
    fn test_format_number_with_comma() {
        assert_eq!(format_number(123.456, true), "123,46");
    }

    #[test]
    fn test_format_number_with_dot() {
        assert_eq!(format_number(123.456, false), "123.46");
    }
}

