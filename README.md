
# b64d (Base64 Decoder)

A tiny, fast, zero-dependency command-line and drag-and-drop Base64 decoder written in Rust.

Drop a base64-encoded file onto the executable, and it immediately decodes it right next to the original file. No flashing command windows, no complex setups. If there's an error and you're not running it from a terminal, it pops up a clean native system dialog to let you know what went wrong.

---

## Why use b64d?

- **Zero Setup / Single Binary**: Just download the binary and use it. No runtimes, no dependencies.
- **Smart GUI & CLI Detection**:
  - Run it from a terminal (CLI)? It works like a standard command line tool.
  - Double-click or drag-and-drop a file? It runs quietly and only pops up a native dialogue if something goes wrong so you can actually read the error.
- **Tolerates messy inputs**: Easily handles wrapped lines, PEM headers (`-----BEGIN CERTIFICATE-----`), and browser Data URLs (`data:image/png;base64,...`).
- **Supports standard and URL-safe Base64**: Decodes both safely.

---

## Automatic Output Naming

When you pass a file to `b64d` (by dragging it onto the executable or passing it in the terminal), the program automatically determines the output file path based on your input:

1. **Location**: The decoded file is always saved in the **same directory** as the original file.
2. **Naming**:
   - Original: `photo.png` -> Decoded: `photo-decoded.png`
   - Original: `data.txt` -> Decoded: `data-decoded.txt`
   - Original: `archive` (no extension) -> Decoded: `archive-decoded`
3. **Automatic Duplicate Prevention**:
   If the decoded file already exists, it will not overwrite it. Instead, it automatically appends an incrementing index:
   - 1st run: `photo-decoded.png`
   - 2nd run: `photo-decoded(1).png`
   - 3rd run: `photo-decoded(2).png`
   ... and so on.

---

## Command Line (CLI) Support

`b64d` has full CLI support and works beautifully inside terminal scripts or manual workflows.

### 1. Decode one or more files directly
Pass one or multiple file paths as arguments:
```bash
# Single file
b64d encoded.txt

# Multiple files at once
b64d file1.b64 image.png.txt data.pem
```

### 2. Interactive CLI Prompt
If you run `b64d` inside a terminal without passing any arguments, it starts an interactive CLI menu:
```text
================ b64d (Base64 Decoder) ================
Usage:
  1. Drag and drop Base64-encoded file(s) onto this executable.
  2. Or run from CLI: b64d <file1> [file2] ...

Or enter a file path to decode manually: /home/user/encoded.txt
Success! Decoded into: "/home/user/encoded-decoded.txt"
Press Enter to continue...
```

---

## Quick Build & Run

If you want to compile `b64d` from source:

```bash
# Run tests
cargo test

# Build optimized binary
cargo build --release
```
The compiled binary will be placed at `target/release/b64d` (or `b64d.exe` on Windows).
