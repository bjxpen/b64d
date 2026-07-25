/// Unix-specific implementation of platform APIs.

pub fn is_running_in_console() -> bool {
    extern "C" {
        fn isatty(fd: i32) -> i32;
    }
    unsafe { isatty(1) != 0 }
}

pub fn show_gui_error(title: &str, message: &str) {
    #[cfg(target_os = "macos")]
    {
        let script = format!("display alert \"{}\" message \"{}\" as critical", title, message);
        let _ = std::process::Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .status();
    }

    #[cfg(target_os = "linux")]
    {
        // Try zenity first
        let zenity_status = std::process::Command::new("zenity")
            .arg("--error")
            .arg(format!("--title={}", title))
            .arg(format!("--text={}", message))
            .status();

        if zenity_status.is_err() || !zenity_status.unwrap().success() {
            // Fallback to kdialog
            let kdialog_status = std::process::Command::new("kdialog")
                .arg("--error")
                .arg(message)
                .arg("--title")
                .arg(title)
                .status();

            if kdialog_status.is_err() || !kdialog_status.unwrap().success() {
                // Fallback to xmessage
                let _ = std::process::Command::new("xmessage")
                    .arg("-center")
                    .arg(format!("{}: {}", title, message))
                    .status();
            }
        }
    }
}
