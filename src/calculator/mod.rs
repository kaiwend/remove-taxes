use crate::cli::NumberInput;

pub struct CalculationResult {
    pub with_vat: f64,
    pub without_vat: f64,
    pub uses_comma: bool,
}

pub fn calculate_without_vat(amount: f64, vat_rate: f64) -> f64 {
    amount / (1.0 + vat_rate / 100.0)
}

pub fn process_numbers(numbers: &[NumberInput], vat_rate: f64) -> Vec<CalculationResult> {
    numbers
        .iter()
        .map(|input| {
            let without_vat = calculate_without_vat(input.value, vat_rate);
            CalculationResult {
                with_vat: input.value,
                without_vat,
                uses_comma: input.uses_comma,
            }
        })
        .collect()
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
    fn test_calculate_without_vat_100_percent() {
        let result = calculate_without_vat(200.0, 100.0);
        assert!((result - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_without_vat_negative_amount() {
        let result = calculate_without_vat(-119.0, 19.0);
        assert!((result - (-100.0)).abs() < 0.01);
    }

    #[test]
    fn test_calculate_without_vat_negative_rate() {
        let result = calculate_without_vat(95.0, -5.0);
        assert!((result - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_without_vat_very_small_amount() {
        let result = calculate_without_vat(0.0119, 19.0);
        assert!((result - 0.01).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_without_vat_very_large_amount() {
        let result = calculate_without_vat(1_000_000_000.0, 19.0);
        assert!((result - 840_336_134.45).abs() < 0.1);
    }

    #[test]
    fn test_calculate_without_vat_fractional_rate() {
        let result = calculate_without_vat(107.5, 7.5);
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
    fn test_process_numbers_empty() {
        let numbers = vec![];
        let results = process_numbers(&numbers, 19.0);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_process_numbers_single() {
        let numbers = vec![NumberInput {
            value: 119.0,
            uses_comma: false,
        }];
        let results = process_numbers(&numbers, 19.0);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].with_vat, 119.0);
        assert!((results[0].without_vat - 100.0).abs() < 0.01);
        assert!(!results[0].uses_comma);
    }

    #[test]
    fn test_process_numbers_mixed_formats() {
        let numbers = vec![
            NumberInput {
                value: 119.0,
                uses_comma: true,
            },
            NumberInput {
                value: 238.0,
                uses_comma: false,
            },
        ];
        let results = process_numbers(&numbers, 19.0);
        assert_eq!(results.len(), 2);
        assert!(results[0].uses_comma);
        assert!(!results[1].uses_comma);
    }

    #[test]
    fn test_calculation_result_structure() {
        let result = CalculationResult {
            with_vat: 119.0,
            without_vat: 100.0,
            uses_comma: true,
        };
        assert_eq!(result.with_vat, 119.0);
        assert_eq!(result.without_vat, 100.0);
        assert!(result.uses_comma);
    }
}

