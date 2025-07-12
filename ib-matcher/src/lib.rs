//! A multilingual and fast string matcher, supports 拼音匹配 (Chinese pinyin match) and ローマ字検索 (Japanese romaji match).
//!
//! ## Usage
//! ```
//! //! cargo add ib-matcher --features pinyin,romaji
//! use ib_matcher::{
//!     matcher::{IbMatcher, PinyinMatchConfig, RomajiMatchConfig},
//!     pinyin::PinyinNotation,
//! };
//!
//! let matcher = IbMatcher::builder("pysousuoeve")
//!     .pinyin(PinyinMatchConfig::notations(
//!         PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter,
//!     ))
//!     .build();
//! assert!(matcher.is_match("拼音搜索Everything"));
//!
//! let matcher = IbMatcher::builder("konosuba")
//!     .romaji(RomajiMatchConfig::default())
//!     .is_pattern_partial(true)
//!     .build();
//! assert!(matcher.is_match("この素晴らしい世界に祝福を"));
//! ```
//!
//! ## Features
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = "doc", doc = document_features::document_features!())]

pub mod matcher;
#[cfg(feature = "minimal")]
pub mod minimal;
#[cfg(feature = "pinyin")]
pub mod pinyin;
pub mod unicode;

#[cfg(feature = "romaji")]
pub use ib_romaji as romaji;
