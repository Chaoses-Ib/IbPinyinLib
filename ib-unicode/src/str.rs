use crate::Sealed;

/// Polyfill for unstable [`#![feature(round_char_boundary)]`](https://github.com/rust-lang/rust/issues/93743)
pub trait RoundCharBoundaryExt: Sealed {
    /// Finds the closest `x` not exceeding `index` where [`is_char_boundary(x)`] is `true`.
    ///
    /// This method can help you truncate a string so that it's still valid UTF-8, but doesn't
    /// exceed a given number of bytes. Note that this is done purely at the character level
    /// and can still visually split graphemes, even though the underlying characters aren't
    /// split. For example, the emoji 🧑‍🔬 (scientist) could be split so that the string only
    /// includes 🧑 (person) instead.
    ///
    /// [`is_char_boundary(x)`]: str::is_char_boundary
    ///
    /// # Examples
    ///
    /// ```
    /// use ib_unicode::str::RoundCharBoundaryExt;
    ///
    /// let s = "❤️🧡💛💚💙💜";
    /// assert_eq!(s.len(), 26);
    /// assert!(!s.is_char_boundary(13));
    ///
    /// let closest = s.floor_char_boundary_ib(13);
    /// assert_eq!(closest, 10);
    /// assert_eq!(&s[..closest], "❤️🧡");
    /// ```
    fn floor_char_boundary_ib(&self, index: usize) -> usize;

    /// Finds the closest `x` not below `index` where [`is_char_boundary(x)`] is `true`.
    ///
    /// If `index` is greater than the length of the string, this returns the length of the string.
    ///
    /// This method is the natural complement to [`floor_char_boundary`]. See that method
    /// for more details.
    ///
    /// [`floor_char_boundary`]: str::floor_char_boundary
    /// [`is_char_boundary(x)`]: str::is_char_boundary
    ///
    /// # Examples
    ///
    /// ```
    /// use ib_unicode::str::RoundCharBoundaryExt;
    ///
    /// let s = "❤️🧡💛💚💙💜";
    /// assert_eq!(s.len(), 26);
    /// assert!(!s.is_char_boundary(13));
    ///
    /// let closest = s.ceil_char_boundary_ib(13);
    /// assert_eq!(closest, 14);
    /// assert_eq!(&s[..closest], "❤️🧡💛");
    /// ```
    fn ceil_char_boundary_ib(&self, index: usize) -> usize;
}

impl RoundCharBoundaryExt for str {
    #[inline]
    fn floor_char_boundary_ib(&self, index: usize) -> usize {
        if index >= self.len() {
            self.len()
        } else {
            let lower_bound = index.saturating_sub(3);
            let new_index = self.as_bytes()[lower_bound..=index].iter().rposition(|&b| {
                // b.is_utf8_char_boundary()
                (b as i8) >= -0x40
            });

            // SAFETY: we know that the character boundary will be within four bytes
            unsafe { lower_bound + new_index.unwrap_unchecked() }
        }
    }

    #[inline]
    fn ceil_char_boundary_ib(&self, index: usize) -> usize {
        if index > self.len() {
            self.len()
        } else {
            let upper_bound = Ord::min(index + 4, self.len());
            self.as_bytes()[index..upper_bound]
                .iter()
                .position(|&b| {
                    // b.is_utf8_char_boundary()
                    (b as i8) >= -0x40
                })
                .map_or(upper_bound, |pos| pos + index)
        }
    }
}
