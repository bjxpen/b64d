# b64d — Robust, High-Performance Drag-and-Drop Base64 Decoder

`b64d` is a zero-dependency, lightning-fast cross-platform Base64 decoder written in Rust. It is specially designed with a smart dual execution model that detects whether it is running in a traditional console or as a graphical shell process. 

When files are dragged and dropped onto the executable icon (or double-clicked) in Windows, macOS, or Linux, any decryption or file parsing errors trigger a **beautiful native OS graphical dialog box** rather than instantly flashing the console window and disappearing.

---

## Key Features

- 🖱️ **Seamless Drag & Drop**: Drag one or multiple Base64-encoded files directly onto the executable icon. It immediately decodes and outputs files alongside the originals.
- 📁 **Smart Duplicate Suffixing**: Outputs files into `<basename>-decoded.<ext>`. If a file already exists, it dynamically resolved duplicates by appending incrementing indexes like `<basename>-decoded(1).<ext>`.
- 🚀 **Extreme Byte-Level Performance**: Custom hand-crafted Base64 decoder with pre-allocated vectors and zero-allocation cleansing filters for peak throughput and minimum memory footprints.
- 🛡️ **Extremely Robust Formats**:
  - Automatically handles wrapped lines with whitespace, newlines, and tabs.
  - Automatically strips standard HTML / web **Data URL scheme headers** (e.g. `data:image/png;base64,...` or `data:text/plain;base64,...`).
  - Supports extracting only raw content stored inside standard **PEM block wrappers** (e.g. `-----BEGIN CERTIFICATE-----` ... `-----END CERTIFICATE-----`).
  - Gracefully decodes both standard MIME and **URL-Safe** base64 (`-` and `_` instead of `+` and `/`), with or without padding.
- 🖥️ **Smart CLI & GUI Dual-Execution**:
  - **Console Mode**: Interactive shell prompt letting you input file paths manually.
  - **GUI/Desktop Mode**: Auto-detects TTY state. Pops up a native dialogue on Windows (Win32 `MessageBoxW`), macOS (`osascript` AppleScript), and Linux (`zenity` / `kdialog` / `xmessage` cascade) if instructions or error notifications need to be displayed.

---

## Architecture Design (SOLID & DRY)

The codebase has been refactored into modular, strictly decoupled components following the best clean coding standards:

- **`src/decoder.rs`**: Hand-crafted base64-to-byte decoding algorithm with zero heap-reallocations during filtering.
- **`src/extractor.rs`**: Safe, zero-allocation byte windows for isolating data payload inside Data URLs or PEM container blocks.
- **`src/path_resolver.rs`**: Path resolution layer encapsulating filename manipulations and file existence validations.
- **`src/platform/`**: Abstracts operating system FFI routines (native console checks and native warning/info box systems) behind a unified, compile-time selected interface.
- **`src/app.rs`**: High-level orchestrator supervising input and output operations, error logging, and console interaction.

---

## Installation & Compilation

Ensure you have [Rust and Cargo](https://rustup.rs/) installed.

### 1. Build and Run Tests
Run the fully-featured unit and integration test suite to verify implementation correctness:
```bash
cargo test
```

### 2. Build for Local Release
Build the highly optimized release binary:
```bash
cargo build --release
```
The compiled release executable will be available at:
- **Linux / macOS**: `target/release/b64d`
- **Windows**: `target/release/b64d.exe`

### 3. Cross-Compiling for Windows from Linux
To produce a static, single-executable `.exe` for Windows systems while developing on Linux:
```bash
# Add the Windows GNU target
rustup target add x86_64-pc-windows-gnu

# Compile the release binary for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

---

## Usage Instructions

### Method A: Drag & Drop (GUI Mode)
1. Select one or more `.txt`, `.b64`, or PEM files containing base64 data.
2. Drag and drop them directly onto the `b64d` executable icon.
3. The files will decode silently and save into the same folder as the input. If any error occurs, a GUI message box pop-up will notify you of the exact root cause.

### Method B: Interactive Terminal (Console Mode)
Simply double-click the executable to launch an interactive terminal menu:
```text
================ b64d (Base64 Decoder) ================
Usage:
  1. Drag and drop Base64-encoded file(s) onto this executable.
  2. Or run from CLI: b64d <file1> [file2] ...

Or enter a file path to decode manually: C:\Users\Name\Desktop\encoded.txt
Success! Decoded into: "C:\\Users\\Name\\Desktop\\encoded-decoded.txt"
Press Enter to continue...
```

### Method C: Command-Line Interface (CLI Mode)
Integrate `b64d` inside your terminal scripts or automated pipelines:
```bash
# Decode a single file
b64d raw_data.txt

# Decode multiple files in one run
b64d cert.pem logo.b64 payload.txt
```

---

## License

This project is licensed under the terms of the MIT License (see `LICENSE` for details).
