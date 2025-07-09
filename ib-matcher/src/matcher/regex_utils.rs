use core::fmt::Write;

/// https://github.com/rust-lang/regex/issues/451
pub fn escape_bytes(bytes: &[u8]) -> String {
    let mut pattern = String::with_capacity(bytes.len() * 4);
    for byte in bytes {
        write!(pattern, "\\x{:02X}", byte).unwrap();
    }
    return pattern;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_bytes() {
        assert_eq!(
            escape_bytes(b"pysseve"),
            "\\x70\\x79\\x73\\x73\\x65\\x76\\x65"
        );
    }
}
