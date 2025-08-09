use std::env;

mod calculator;
mod cli;
mod display;
mod utils;

use calculator::process_numbers;
use cli::{parse_arguments, print_usage};
use display::{create_clipboard_content, display_results};
use utils::copy_to_clipboard;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let parsed_args = match parse_arguments(args) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("Error: {error}");
            print_usage(&program_name);
            std::process::exit(1);
        }
    };

    let results = process_numbers(&parsed_args.numbers, parsed_args.vat_rate);

    display_results(&results, parsed_args.vat_rate);

    let clipboard_content = create_clipboard_content(&results);
    copy_to_clipboard(&clipboard_content);
}

