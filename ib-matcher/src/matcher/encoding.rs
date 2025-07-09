/// ## Performance
/// Although multiple encodings are supported, UTF-8 (`str`) is most optimized.
///
/// TODO: Extended ASCII code pages
/// TODO: Index/SliceIndex
pub trait EncodedStr: Sealed {
    const ELEMENT_LEN_BYTE: usize;

    fn is_ascii(&self) -> bool;
    fn as_bytes(&self) -> &[u8];

    fn char_index_strs(&self) -> impl Iterator<Item = (usize, char, &Self)>;
    fn char_len_next_strs(&self) -> impl Iterator<Item = (char, usize, &Self)>;
    fn chars_count(&self) -> usize {
        self.char_index_strs().count()
    }
}

mod private {
    pub trait Sealed {}
}
use private::Sealed;

impl Sealed for str {}
#[cfg(feature = "encoding")]
impl Sealed for widestring::U16Str {}
#[cfg(feature = "encoding")]
impl Sealed for widestring::U32Str {}

impl EncodedStr for str {
    const ELEMENT_LEN_BYTE: usize = core::mem::size_of::<u8>();

    fn is_ascii(&self) -> bool {
        self.is_ascii()
    }

    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn char_index_strs(&self) -> impl Iterator<Item = (usize, char, &Self)> {
        self.char_indices().map(|(i, c)| (i, c, &self[i..]))
    }

    fn char_len_next_strs(&self) -> impl Iterator<Item = (char, usize, &Self)> {
        self.char_indices().map(|(i, c)| {
            let len = c.len_utf8();
            (c, len, &self[i + len..])
        })
    }

    fn chars_count(&self) -> usize {
        self.chars().count()
    }
}

#[cfg(feature = "encoding")]
impl EncodedStr for widestring::U16Str {
    const ELEMENT_LEN_BYTE: usize = core::mem::size_of::<u16>();

    fn is_ascii(&self) -> bool {
        // TODO: Since this may not be optimized with SIMD, should we use `is_in_range` instead?
        self.chars_lossy().all(|c| c.is_ascii())
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.as_ptr() as *const u8,
                self.len() * core::mem::size_of::<u16>(),
            )
        }
    }

    fn char_index_strs(&self) -> impl Iterator<Item = (usize, char, &Self)> {
        self.char_indices_lossy().map(|(i, c)| (i, c, &self[i..]))
    }

    fn char_len_next_strs(&self) -> impl Iterator<Item = (char, usize, &Self)> {
        self.char_indices_lossy().map(|(i, c)| {
            let len = c.len_utf16();
            (c, len, &self[i + len..])
        })
    }
}

#[cfg(feature = "encoding")]
impl EncodedStr for widestring::U32Str {
    const ELEMENT_LEN_BYTE: usize = core::mem::size_of::<u32>();

    fn is_ascii(&self) -> bool {
        // TODO: Since this may not be optimized with SIMD, should we use `is_in_range` instead?
        self.chars_lossy().all(|c| c.is_ascii())
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.as_ptr() as *const u8,
                self.len() * core::mem::size_of::<u32>(),
            )
        }
    }

    fn char_index_strs(&self) -> impl Iterator<Item = (usize, char, &Self)> {
        self.char_indices_lossy().map(|(i, c)| (i, c, &self[i..]))
    }

    fn char_len_next_strs(&self) -> impl Iterator<Item = (char, usize, &Self)> {
        self.char_indices_lossy()
            .map(|(i, c)| (c, 1, &self[i + 1..]))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(feature = "encoding")]
    #[test]
    fn u16_is_ascii() {
        use widestring::u16str;

        assert!(u16str!("").is_ascii());
        assert!(u16str!("abc").is_ascii());
        assert!(u16str!("协作").is_ascii() == false);
    }

    #[cfg(feature = "encoding")]
    #[test]
    fn u32_is_ascii() {
        use widestring::u32str;

        assert!(u32str!("").is_ascii());
        assert!(u32str!("abc").is_ascii());
        assert!(u32str!("协作").is_ascii() == false);
    }
}
