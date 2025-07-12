use std::ops::Range;

use crate::Sealed;

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

    /// Mainly used for bytes to char units conversion.
    pub fn div(self, rhs: usize) -> Match {
        Match {
            start: self.start / rhs,
            end: self.end / rhs,
            is_pattern_partial: self.is_pattern_partial,
        }
    }
}

pub trait OptionMatchExt: Sealed + Into<Option<Match>> + Sized {
    /// Mainly used for bytes to char units conversion.
    fn div(self, rhs: usize) -> Option<Match> {
        self.into().map(|m| m.div(rhs))
    }
}

impl Sealed for Option<Match> {}
impl OptionMatchExt for Option<Match> {}

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
