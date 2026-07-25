/// Windows-specific implementation of platform APIs.

pub fn is_running_in_console() -> bool {
    extern "system" {
        fn GetConsoleProcessList(lpdwProcessList: *mut u32, dwProcessCount: u32) -> u32;
    }
    let mut process_list = [0u32; 2];
    let count = unsafe { GetConsoleProcessList(process_list.as_mut_ptr(), 2) };
    count > 1
}

pub fn show_gui_error(title: &str, message: &str) {
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
