/// Abstraction for platform-specific capabilities.
pub struct Platform;

impl Platform {
    /// Detects if the current process is running in an interactive CLI/console.
    pub fn is_running_in_console() -> bool {
        sys::is_running_in_console()
    }

    /// Shows a native GUI error dialog box.
    pub fn show_gui_error(title: &str, message: &str) {
        sys::show_gui_error(title, message);
    }
}

// Compile-time platform selection
#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod sys;

#[cfg(unix)]
#[path = "unix.rs"]
mod sys;

#[cfg(not(any(target_os = "windows", unix)))]
#[path = "fallback.rs"]
mod sys;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_sanity() {
        // Sanity check that we can call this on any platform without panicking
        let _ = Platform::is_running_in_console();
    }
}
