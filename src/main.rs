use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let in_console = is_running_in_console();

    if args.len() < 2 {
        if in_console {
            println!("================ b64d (Base64 Decoder) ================");
            println!("Usage:");
            println!("  1. Drag and drop Base64-encoded file(s) onto this executable.");
            println!("  2. Or run from CLI: b64d <file1> [file2] ...");
            println!();
            print!("Or enter a file path to decode manually: ");
            let _ = io::stdout().flush();

            let mut input_path = String::new();
            if io::stdin().read_line(&mut input_path).is_ok() {
                let trimmed_path = input_path.trim().trim_matches('"').trim_matches('\'');
                if trimmed_path.is_empty() {
                    println!("No file specified. Exiting.");
                    return;
                }

                let path = Path::new(trimmed_path);
                if !path.exists() {
                    let err_msg = format!("File '{}' does not exist.", trimmed_path);
                    report_error("File Not Found", &err_msg, true);
                    return;
                }

                process_file(path, true);
            }
            wait_for_enter();
        } else {
            // Run from GUI/double-click with no arguments.
            // Prompt the user with a friendly error/instructions dialog.
            report_error(
                "b64d - Usage Instructions",
                "Please drag and drop Base64-encoded file(s) onto this executable to decode them.",
                false,
            );
        }
    } else {
        // Process all arguments passed (drag-and-dropped files or CLI args)
        let mut has_errors = false;
        for arg in &args[1..] {
            let path = Path::new(arg);
            if path.exists() {
                if let Err(e) = process_file_or_error(path) {
                    has_errors = true;
                    let err_msg = format!("Failed to decode file {:?}:\n{}", path, e);
                    report_error("Decoding Error", &err_msg, in_console);
                }
            } else {
                has_errors = true;
                let err_msg = format!("File '{:?}' does not exist.", path);
                report_error("File Not Found", &err_msg, in_console);
            }
        }

        if in_console {
            println!("\nDone!");
            wait_for_enter();
        } else if has_errors {
            // Keep window open or let them know errors occurred if we're not in console
        }
    }
}

fn wait_for_enter() {
    print!("Press Enter to continue...");
    let _ = io::stdout().flush();
    let mut temp = String::new();
    let _ = io::stdin().read_line(&mut temp);
}

/// Helper to report error either via stdout/stderr (if in console) or via GUI dialog (if not)
fn report_error(title: &str, message: &str, in_console: bool) {
    if in_console {
        eprintln!("Error [{}]: {}", title, message);
    } else {
        show_gui_error(title, message);
    }
}

/// Process file and print message directly (for console mode)
fn process_file(path: &Path, in_console: bool) {
    match process_file_or_error(path) {
        Ok(saved_path) => {
            if in_console {
                println!("Success! Decoded into: {:?}", saved_path);
            }
        }
        Err(e) => {
            let err_msg = format!("Failed to decode file {:?}:\n{}", path, e);
            report_error("Decoding Error", &err_msg, in_console);
        }
    }
}

/// Process file and return Result
fn process_file_or_error(path: &Path) -> Result<std::path::PathBuf, String> {
    // Read raw bytes from the file
    let raw_bytes = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;

    if raw_bytes.is_empty() {
        return Err("File is empty.".to_string());
    }

    // Extract and clean the base64 content
    let b64_str = extract_b64_content(&raw_bytes);
    if b64_str.is_empty() {
        return Err("No valid Base64 characters found in file.".to_string());
    }

    // Decode the base64 content
    let decoded_bytes = decode_b64(&b64_str)?;

    // Determine the output path
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("decoded");
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let mut index = 0;
    let target_path = loop {
        let suffix = if index == 0 {
            "-decoded".to_string()
        } else {
            format!("-decoded({})", index)
        };

        let mut new_filename = format!("{}{}", file_stem, suffix);
        if !extension.is_empty() {
            new_filename.push('.');
            new_filename.push_str(extension);
        }

        let candidate = parent.join(&new_filename);
        if !candidate.exists() {
            break candidate;
        }
        index += 1;
    };

    // Write decoded bytes to the target path
    fs::write(&target_path, &decoded_bytes)
        .map_err(|e| format!("Failed to write decoded file: {}", e))?;

    Ok(target_path)
}

/// Extract base64 content from raw bytes, handling PEM blocks and data URLs
fn extract_b64_content(raw_bytes: &[u8]) -> String {
    let content_str = String::from_utf8_lossy(raw_bytes);
    let mut captured_lines = Vec::new();
    let mut in_block = false;
    let mut has_block = false;

    for line in content_str.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("-----BEGIN") {
            in_block = true;
            has_block = true;
            continue;
        }
        if trimmed.starts_with("-----END") {
            in_block = false;
            continue;
        }
        if has_block {
            if in_block {
                captured_lines.push(trimmed);
            }
        } else {
            captured_lines.push(trimmed);
        }
    }

    let joined = captured_lines.join("");

    // Handle data URL prefix if any (e.g. "data:text/plain;base64,SGVsbG8=")
    let mut start_idx = 0;
    let joined_bytes = joined.as_bytes();
    if joined_bytes.starts_with(b"data:") {
        if let Some(pos) = joined_bytes.windows(8).position(|w| w == b";base64,") {
            start_idx = pos + 8;
        } else if let Some(pos) = joined_bytes.iter().position(|&b| b == b',') {
            start_idx = pos + 1;
        }
    }

    // Filter characters to keep only valid base64 chars
    let mut filtered = String::new();
    for &b in &joined_bytes[start_idx..] {
        let c = b as char;
        if c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '-' || c == '_' || c == '=' {
            filtered.push(c);
        }
    }
    filtered
}

/// High-performance streaming Base64 decoder with pre-allocated vector
fn decode_b64(input: &str) -> Result<Vec<u8>, String> {
    let input_bytes = input.as_bytes();
    let mut decoded = Vec::with_capacity((input_bytes.len() * 3) / 4 + 4);

    let mut buffer = 0u32;
    let mut bits_collected = 0;
    let mut padding_count = 0;

    for &b in input_bytes {
        let c = b as char;
        if c.is_whitespace() {
            continue;
        }
        if c == '=' {
            padding_count += 1;
            continue;
        }
        if padding_count > 0 {
            return Err("Invalid base64: characters found after padding '='".to_string());
        }

        let val = match b {
            b'A'..=b'Z' => (b - b'A') as u32,
            b'a'..=b'z' => (b - b'a') as u32 + 26,
            b'0'..=b'9' => (b - b'0') as u32 + 52,
            b'+' | b'-' => 62,
            b'/' | b'_' => 63,
            _ => return Err(format!("Invalid character '{}' in base64", c)),
        };

        buffer = (buffer << 6) | val;
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            let byte = ((buffer >> bits_collected) & 0xFF) as u8;
            decoded.push(byte);
            buffer &= (1 << bits_collected) - 1;
        }
    }

    Ok(decoded)
}

/// Detect if the program is running inside an active console/CLI session
#[cfg(unix)]
fn is_running_in_console() -> bool {
    extern "C" {
        fn isatty(fd: i32) -> i32;
    }
    unsafe { isatty(1) != 0 }
}

/// Detect if the program is running inside an active console/CLI session
#[cfg(target_os = "windows")]
fn is_running_in_console() -> bool {
    extern "system" {
        fn GetConsoleProcessList(lpdwProcessList: *mut u32, dwProcessCount: u32) -> u32;
    }
    let mut process_list = [0u32; 2];
    let count = unsafe { GetConsoleProcessList(process_list.as_mut_ptr(), 2) };
    count > 1
}

/// Detect if the program is running inside an active console/CLI session
#[cfg(not(any(unix, target_os = "windows")))]
fn is_running_in_console() -> bool {
    // Default fallback
    true
}

/// Show GUI error dialog box on Linux
#[cfg(target_os = "linux")]
fn show_gui_error(title: &str, message: &str) {
    // Attempt to use zenity (standard on GNOME/Ubuntu)
    let zenity_status = std::process::Command::new("zenity")
        .arg("--error")
        .arg(format!("--title={}", title))
        .arg(format!("--text={}", message))
        .status();

    if zenity_status.is_err() || !zenity_status.unwrap().success() {
        // Fallback to kdialog (standard on KDE)
        let kdialog_status = std::process::Command::new("kdialog")
            .arg("--error")
            .arg(message)
            .arg("--title")
            .arg(title)
            .status();

        if kdialog_status.is_err() || !kdialog_status.unwrap().success() {
            // Fallback to xmessage (traditional X11)
            let _ = std::process::Command::new("xmessage")
                .arg("-center")
                .arg(format!("{}: {}", title, message))
                .status();
        }
    }
}

/// Show GUI error dialog box on macOS
#[cfg(target_os = "macos")]
fn show_gui_error(title: &str, message: &str) {
    // Standard system dialog via AppleScript
    let script = format!("display alert \"{}\" message \"{}\" as critical", title, message);
    let _ = std::process::Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .status();
}

/// Show GUI error dialog box on Windows
#[cfg(target_os = "windows")]
fn show_gui_error(title: &str, message: &str) {
    extern "system" {
        fn MessageBoxW(
            hWnd: *mut std::ffi::c_void,
            lpText: *const u16,
            lpCaption: *const u16,
            uType: u32,
        ) -> i32;
    }

    let mut wide_title: Vec<u16> = title.encode_utf16().collect();
    wide_title.push(0);
    let mut wide_msg: Vec<u16> = message.encode_utf16().collect();
    wide_msg.push(0);

    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            wide_msg.as_ptr(),
            wide_title.as_ptr(),
            0x00000010, // MB_ICONERROR
        );
    }
}

/// Fallback for other operating systems
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn show_gui_error(title: &str, message: &str) {
    eprintln!("Error [{}]: {}", title, message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_b64_basic() {
        assert_eq!(decode_b64("SGVsbG8=").unwrap(), b"Hello");
        assert_eq!(decode_b64("SGVsbG8gd29ybGQ=").unwrap(), b"Hello world");
    }

    #[test]
    fn test_decode_b64_unpadded() {
        assert_eq!(decode_b64("SGVsbG8").unwrap(), b"Hello");
        assert_eq!(decode_b64("SGVsbG8gd29ybGQ").unwrap(), b"Hello world");
    }

    #[test]
    fn test_decode_b64_whitespace() {
        assert_eq!(decode_b64("SGVs bG8=\n").unwrap(), b"Hello");
    }

    #[test]
    fn test_decode_b64_url_safe() {
        assert_eq!(decode_b64("+/8=").unwrap(), vec![251, 255]);
        assert_eq!(decode_b64("-_8=").unwrap(), vec![251, 255]);
    }

    #[test]
    fn test_extract_b64_content_data_url() {
        let raw = b"data:text/plain;base64,SGVsbG8gd29ybGQ=";
        assert_eq!(extract_b64_content(raw), "SGVsbG8gd29ybGQ=");

        let raw_simple = b"data:,SGVsbG8=";
        assert_eq!(extract_b64_content(raw_simple), "SGVsbG8=");
    }

    #[test]
    fn test_extract_b64_content_pem() {
        let raw = b"
-----BEGIN CERTIFICATE-----
SGVsbG8=
-----END CERTIFICATE-----
";
        assert_eq!(extract_b64_content(raw), "SGVsbG8=");
    }

    #[test]
    fn test_process_file_duplicate() {
        let temp_dir = Path::new("test_dup_dir_new");
        let _ = fs::create_dir_all(temp_dir);

        let input_file = temp_dir.join("input.txt");
        fs::write(&input_file, b"SGVsbG8gd29ybGQ=").unwrap();

        // 1st run
        let out1 = process_file_or_error(&input_file).unwrap();
        assert_eq!(out1, temp_dir.join("input-decoded.txt"));
        assert_eq!(fs::read_to_string(&out1).unwrap(), "Hello world");

        // 2nd run
        let out2 = process_file_or_error(&input_file).unwrap();
        assert_eq!(out2, temp_dir.join("input-decoded(1).txt"));
        assert_eq!(fs::read_to_string(&out2).unwrap(), "Hello world");

        // 3rd run
        let out3 = process_file_or_error(&input_file).unwrap();
        assert_eq!(out3, temp_dir.join("input-decoded(2).txt"));
        assert_eq!(fs::read_to_string(&out3).unwrap(), "Hello world");

        // Clean up
        let _ = fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_is_running_in_console() {
        // Simple sanity check that calling it does not crash
        let _ = is_running_in_console();
    }
}
