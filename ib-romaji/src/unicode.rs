/// TODO: https://github.com/rust-lang/rust/issues/93743
#[inline]
pub fn floor_char_boundary(s: &str, index: usize) -> usize {
    if index >= s.len() {
        s.len()
    } else {
        let lower_bound = index.saturating_sub(3);
        let new_index = s.as_bytes()[lower_bound..=index].iter().rposition(|&b| {
            // b.is_utf8_char_boundary()
            (b as i8) >= -0x40
        });

        // SAFETY: we know that the character boundary will be within four bytes
        unsafe { lower_bound + new_index.unwrap_unchecked() }
    }
}
