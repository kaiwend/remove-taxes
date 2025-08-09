use std::process::Command;

#[test]
fn test_basic_calculation() {
    let output = Command::new("cargo")
        .args(&["run", "--", "119"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("VAT Rate: 19%"));
    assert!(stdout.contains("119.00"));
    assert!(stdout.contains("100.00"));
}

#[test]
fn test_multiple_numbers() {
    let output = Command::new("cargo")
        .args(&["run", "--", "119", "238", "357"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("100.00"));
    assert!(stdout.contains("200.00"));
    assert!(stdout.contains("300.00"));
}

#[test]
fn test_custom_vat_rate() {
    let output = Command::new("cargo")
        .args(&["run", "--", "107", "--rate", "7"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("VAT Rate: 7%"));
    assert!(stdout.contains("100.00"));
}

#[test]
fn test_comma_decimal_separator() {
    let output = Command::new("cargo")
        .args(&["run", "--", "119,50"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("119,50"));
    assert!(stdout.contains("100,42"));
}

#[test]
fn test_mixed_decimal_separators() {
    let output = Command::new("cargo")
        .args(&["run", "--", "119,50", "238.00"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("119,50"));
    assert!(stdout.contains("100,42"));
    assert!(stdout.contains("238.00"));
    assert!(stdout.contains("200.00"));
}

#[test]
fn test_no_arguments_shows_error() {
    let output = Command::new("cargo")
        .args(&["run", "--"])
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error:"));
    assert!(stderr.contains("Usage:"));
    assert!(!output.status.success());
}

#[test]
fn test_invalid_number_ignored() {
    let output = Command::new("cargo")
        .args(&["run", "--", "119", "invalid", "238"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("119.00"));
    assert!(stdout.contains("238.00"));
    assert!(!stdout.contains("invalid"));
}

#[test]
fn test_rate_without_value_shows_error() {
    let output = Command::new("cargo")
        .args(&["run", "--", "119", "--rate"])
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--rate requires a value"));
    assert!(!output.status.success());
}

#[test]
fn test_env_var_default_rate() {
    let output = Command::new("cargo")
        .env("DEFAULT_VAT_RATE", "7")
        .args(&["run", "--", "107"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("VAT Rate: 7%"));
    assert!(stdout.contains("100.00"));
}

#[test]
fn test_rate_flag_overrides_env_var() {
    let output = Command::new("cargo")
        .env("DEFAULT_VAT_RATE", "7")
        .args(&["run", "--", "119", "--rate", "19"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("VAT Rate: 19%"));
    assert!(stdout.contains("100.00"));
}