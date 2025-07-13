//! ## Performance
//! With default `release` profile, using `Input` instead of `&HaystackStr` is 3~5% slower (without using Bon), while with `lto = "fat"` and `codegen-units = 1` using `Input` is 3~5% faster, well...
use bon::Builder;

use crate::matcher::encoding::EncodedStr;

#[derive(Builder, Clone)]
pub struct Input<'h, HaystackStr = str>
where
    HaystackStr: EncodedStr + ?Sized,
{
    #[builder(start_fn)]
    pub(crate) haystack: &'h HaystackStr,
    // #[builder(default = haystack.is_ascii())]
    // pub(crate) is_ascii: bool,
}

impl<'h, HaystackStr> From<&'h HaystackStr> for Input<'h, HaystackStr>
where
    HaystackStr: EncodedStr + ?Sized,
{
    #[inline]
    fn from(haystack: &'h HaystackStr) -> Self {
        // Input::builder(haystack).build()
        Input { haystack }
    }
}
