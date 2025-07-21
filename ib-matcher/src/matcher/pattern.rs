#![cfg_attr(not(feature = "syntax"), allow(dead_code))]
use crate::matcher::encoding::EncodedStr;

#[derive(Debug)]
pub struct Pattern<'a, HaystackStr>
where
    HaystackStr: EncodedStr + ?Sized,
{
    pub(crate) pattern: &'a HaystackStr,
    pub(crate) lang_only: Option<LangOnly>,
}

impl<'a, HaystackStr> From<&'a HaystackStr> for Pattern<'a, HaystackStr>
where
    HaystackStr: EncodedStr + ?Sized,
{
    fn from(value: &'a HaystackStr) -> Self {
        Self {
            pattern: value,
            lang_only: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LangOnly {
    English,
    Pinyin,
    Romaji,
}
