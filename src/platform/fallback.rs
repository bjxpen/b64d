/// Fallback implementation of platform APIs.

pub fn is_running_in_console() -> bool {
    true
}

pub fn show_gui_error(title: &str, message: &str) {
    eprintln!("Error [{}]: {}", title, message);
}
