use arboard::Clipboard;

pub fn copy_to_clipboard(content: &str) {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(content) {
                eprintln!("Warning: Could not copy to clipboard: {e}");
            } else {
                println!("\n✓ Results copied to clipboard (without VAT values)");
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not access clipboard: {e}");
        }
    }
}

