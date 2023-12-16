#![feature(return_position_impl_trait_in_trait)]

pub mod matcher;
#[cfg(feature = "minimal")]
pub mod minimal;
pub mod pinyin;
