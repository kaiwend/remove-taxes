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
    fn test_calculate_multiple_values() {
        let values = [119.0, 238.0, 357.0];
        let expected = [100.0, 200.0, 300.0];

        for (value, expected) in values.iter().zip(expected.iter()) {
            let result = calculate_without_vat(*value, 19.0);
            assert!((result - expected).abs() < 0.01);
        }
    }
}

