/// TODO: Extended ASCII code pages
/// TODO: Index/SliceIndex
pub trait EncodedStr {
    const ELEMENT_LEN_BYTE: usize;

    fn is_ascii(&self) -> bool;
    fn as_bytes(&self) -> &[u8];

    fn char_index_strs(&self) -> impl Iterator<Item = (usize, char, &Self)>;
    fn char_len_next_strs(&self) -> impl Iterator<Item = (char, usize, &Self)>;
}

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
}

#[cfg(feature = "encoding")]
impl EncodedStr for widestring::U16Str {
    const ELEMENT_LEN_BYTE: usize = core::mem::size_of::<u16>();

    fn is_ascii(&self) -> bool {
        self.as_bytes().is_ascii()
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
        self.as_bytes().is_ascii()
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
