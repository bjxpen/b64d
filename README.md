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

## Multi-Codec Text Detection

Not all text files are saved in standard ASCII/UTF-8. In Windows, if you copy-paste a base64 string into Notepad and hit save, it may default to **UTF-16LE (Unicode)**, **UTF-16BE**, or **UTF-8 with BOM**. 

In standard base64 decoders, trying to decode a UTF-16 text file of base64 characters fails immediately because of hidden null bytes (`0x00`) and BOM markers. 

`b64d` fixes this. It includes a built-in text codec sensor that automatically detects:
1. **UTF-8 with BOM**
2. **UTF-16LE (with or without BOM)**
3. **UTF-16BE (with or without BOM)**
4. **Standard 8-bit ASCII & ANSI Code Pages** (such as Windows-1252, GBK/Chinese, Shift-JIS/Japanese, and EUC-KR/Korean)

If it detects the file is encoded in a text codec (even without an extension), it automatically translates the file's codec to a readable Base64 payload first, decodes it, and writes the output file. No manual format conversions are required!

---

## Heavy Multiline & Line Wrap Support (CRLF & LF)

`b64d` is extremely resilient with multiline base64 text files. 

If your file has Base64 segments split across several lines, has line breaks (`\n` or `\r\n`), or uses different wrapping widths (such as standard 64 or 76 character wraps), `b64d` handles it with ease:
* It reads the entire text file and automatically filters out any whitespaces, space padding, line feeds, and carriage returns.
* It merges the multi-line segments back into a single unbroken Base64 string for lossless binary translation.

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

## Command Line (CLI)

The only arguments are file paths. The executable is designed for drag-and-drop than becoming one of the million CLI decoders out there.

### Decode one or more files directly
Pass one or multiple file paths as arguments:
```bash
# Single file
b64d encoded.txt

# Multiple files at once
b64d file1.b64 image.png.txt data.pem
