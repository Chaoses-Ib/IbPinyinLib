use core::fmt::Write;

/// https://github.com/rust-lang/regex/issues/451
pub fn escape_bytes(bytes: &[u8]) -> String {
    let mut pattern = String::with_capacity(bytes.len() * 4);
    for byte in bytes {
        write!(pattern, "\\x{:02X}", byte).unwrap();
    }
    return pattern;
}
