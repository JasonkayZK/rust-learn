use std::io;

use arboard::Clipboard;
use clipboard_master::{CallbackResult, ClipboardHandler, Master};

struct ClipboardManager {
    clipboard: Clipboard,
}

impl ClipboardHandler for ClipboardManager {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!("Clipboard change happened!");

        match self.clipboard.get_text() {
            Ok(text) => {
                println!("Clipboard text was: {}", text);
            }
            Err(err) => {
                println!("[Error] Read clipboard err: {}", err);
                match self.clipboard.get_image() {
                    Ok(image) => {
                        println!("Clipboard image bytes length was: {}", image.bytes.len());
                    }
                    Err(_) => {
                        println!("[Error-2] Read clipboard err: {}", err);
                    }
                }
            }
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, err: io::Error) -> CallbackResult {
        eprintln!("Error: {}", err);
        CallbackResult::Next
    }
}

fn main() {
    let _ = Master::new(ClipboardManager {
        clipboard: Clipboard::new().unwrap(),
    }).run();
}

// fn main() {
// let mut clipboard = Clipboard::new().unwrap();

// let the_string = "Hello, world!";
// clipboard.set_text(the_string).unwrap();
// println!("But now the clipboard text should be: \"{}\"", the_string);
// }
