use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Match {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) is_pattern_partial: bool,
}

impl Match {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    pub fn is_pattern_partial(&self) -> bool {
        self.is_pattern_partial
    }
}

#[derive(Clone, Copy)]
pub(crate) struct SubMatch {
    pub len: usize,
    pub is_pattern_partial: bool,
}

impl SubMatch {
    pub fn new(len: usize, is_pattern_partial: bool) -> Self {
        Self {
            len,
            is_pattern_partial,
        }
    }
}
