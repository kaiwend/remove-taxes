pub mod calculator;
pub mod cli;
pub mod display;
pub mod utils;

// Re-export main functionality for testing
pub use calculator::process_numbers;
pub use cli::{parse_arguments, print_usage};
pub use display::{create_clipboard_content, display_results};
pub use utils::copy_to_clipboard;