use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use crate::decoder::Base64Decoder;
use crate::extractor::Base64Extractor;
use crate::path_resolver::PathResolver;
use crate::platform::Platform;

pub struct App {
    in_console: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            in_console: Platform::is_running_in_console(),
        }
    }

    /// Orchestrates application execution based on CLI/drag-and-drop arguments.
    pub fn run(&self, args: &[String]) {
        if args.len() < 2 {
            if self.in_console {
                self.run_interactive_cli();
            } else {
                Platform::show_gui_error(
                    "b64d - Usage Instructions",
                    "Please drag and drop Base64-encoded file(s) onto this executable to decode them.",
                );
            }
        } else {
            self.process_arguments(&args[1..]);
        }
    }

    fn run_interactive_cli(&self) {
        println!("================ b64d (Base64 Decoder) ================");
        println!("Usage:");
        println!("  1. Drag and drop Base64-encoded file(s) onto this executable.");
        println!("  2. Or run from CLI: b64d <file1> [file2] ...");
        println!();
        print!("Or enter a file path to decode manually: ");
        let _ = io::stdout().flush();

        let mut input_path = String::new();
        if io::stdin().read_line(&mut input_path).is_ok() {
            let trimmed = input_path.trim().trim_matches('"').trim_matches('\'');
            if trimmed.is_empty() {
                println!("No file specified. Exiting.");
                return;
            }

            let path = Path::new(trimmed);
            if !path.exists() {
                self.report_error("File Not Found", &format!("File '{}' does not exist.", trimmed));
                self.wait_for_enter();
                return;
            }

            self.process_file(path);
        }
        self.wait_for_enter();
    }

    fn process_arguments(&self, paths: &[String]) {
        for arg in paths {
            let path = Path::new(arg);
            if path.exists() {
                if let Err(e) = self.decode_file(path) {
                    self.report_error("Decoding Error", &format!("Failed to decode file {:?}:\n{}", path, e));
                }
            } else {
                self.report_error("File Not Found", &format!("File '{:?}' does not exist.", path));
            }
        }

        if self.in_console {
            println!("\nDone!");
            self.wait_for_enter();
        }
    }

    fn process_file(&self, path: &Path) {
        match self.decode_file(path) {
            Ok(saved) => {
                if self.in_console {
                    println!("Success! Decoded into: {:?}", saved);
                }
            }
            Err(e) => {
                self.report_error("Decoding Error", &format!("Failed to decode file {:?}:\n{}", path, e));
            }
        }
    }

    /// Handles file read, cleansing, base64 decoding, path resolution, and saving.
    pub fn decode_file(&self, path: &Path) -> Result<PathBuf, String> {
        let raw_bytes = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;
        if raw_bytes.is_empty() {
            return Err("File is empty.".to_string());
        }

        let b64_str = Base64Extractor::extract(&raw_bytes);
        if b64_str.is_empty() {
            return Err("No valid Base64 characters found in file.".to_string());
        }

        let decoded_bytes = Base64Decoder::decode(&b64_str)?;
        let target_path = PathResolver::generate_unique_path(path);

        fs::write(&target_path, &decoded_bytes)
            .map_err(|e| format!("Failed to write decoded file: {}", e))?;

        Ok(target_path)
    }

    fn report_error(&self, title: &str, message: &str) {
        if self.in_console {
            eprintln!("Error [{}]: {}", title, message);
        } else {
            Platform::show_gui_error(title, message);
        }
    }

    fn wait_for_enter(&self) {
        print!("Press Enter to continue...");
        let _ = io::stdout().flush();
        let mut temp = String::new();
        let _ = io::stdin().read_line(&mut temp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_decode_duplicate() {
        let temp_dir = Path::new("test_app_dup_dir");
        let _ = fs::create_dir_all(temp_dir);

        let input_file = temp_dir.join("payload.txt");
        fs::write(&input_file, b"SGVsbG8gd29ybGQ=").unwrap();

        let app = App { in_console: true };

        let res1 = app.decode_file(&input_file).unwrap();
        assert_eq!(res1, temp_dir.join("payload-decoded.txt"));
        assert_eq!(fs::read_to_string(&res1).unwrap(), "Hello world");

        let res2 = app.decode_file(&input_file).unwrap();
        assert_eq!(res2, temp_dir.join("payload-decoded(1).txt"));
        assert_eq!(fs::read_to_string(&res2).unwrap(), "Hello world");

        let _ = fs::remove_dir_all(temp_dir);
    }
}