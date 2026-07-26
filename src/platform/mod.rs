/// Abstraction for platform-specific capabilities.
pub struct Platform;

impl Platform {
    /// Detects if the current process is running in an interactive CLI/console.
    /// On Windows, it attempts to attach to the parent console process.
    pub fn is_running_in_console() -> bool {
        #[cfg(target_os = "windows")]
        {
            Self::try_attach_console()
        }

        #[cfg(unix)]
        {
            extern "C" {
                fn isatty(fd: i32) -> i32;
            }
            unsafe { isatty(1) != 0 }
        }

        #[cfg(not(any(target_os = "windows", unix)))]
        {
            true
        }
    }

    /// Attempts to attach to the parent console process on Windows.
    /// If successful, redirects standard handles so standard print macros print to the parent console.
    #[cfg(target_os = "windows")]
    fn try_attach_console() -> bool {
        #[link(name = "kernel32")]
        extern "system" {
            fn AttachConsole(dwProcessId: u32) -> i32;
            fn GetStdHandle(nStdHandle: u32) -> *mut std::ffi::c_void;
            fn SetStdHandle(nStdHandle: u32, hHandle: *mut std::ffi::c_void) -> i32;
        }

        // ATTACH_PARENT_PROCESS is -1 (0xFFFFFFFF)
        if unsafe { AttachConsole(0xFFFFFFFF) != 0 } {
            unsafe {
                let h_out = GetStdHandle(0xFFFFFFF5); // STD_OUTPUT_HANDLE
                let h_err = GetStdHandle(0xFFFFFFF4); // STD_ERROR_HANDLE
                let h_in = GetStdHandle(0xFFFFFFF6);  // STD_INPUT_HANDLE

                SetStdHandle(0xFFFFFFF5, h_out);
                SetStdHandle(0xFFFFFFF4, h_err);
                SetStdHandle(0xFFFFFFF6, h_in);
            }
            true
        } else {
            false
        }
    }

    /// Shows a native GUI error dialog box.
    pub fn show_gui_error(title: &str, message: &str) {
        #[cfg(target_os = "windows")]
        {
            #[link(name = "user32")]
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
                MessageBoxW(std::ptr::null_mut(), wide_msg.as_ptr(), wide_title.as_ptr(), 0x10);
            }
        }

        #[cfg(target_os = "macos")]
        {
            let script = format!("display alert \"{}\" message \"{}\" as critical", title, message);
            let _ = std::process::Command::new("osascript").arg("-e").arg(&script).status();
        }

        #[cfg(target_os = "linux")]
        {
            if std::process::Command::new("zenity")
                .arg("--error")
                .arg(format!("--title={}", title))
                .arg(format!("--text={}", message))
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
            {
                return;
            }

            if std::process::Command::new("kdialog")
                .arg("--error")
                .arg(message)
                .arg("--title")
                .arg(title)
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
            {
                return;
            }

            let _ = std::process::Command::new("xmessage")
                .arg("-center")
                .arg(format!("{}: {}", title, message))
                .status();
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            eprintln!("Error [{}]: {}", title, message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_sanity() {
        let _ = Platform::is_running_in_console();
    }
}