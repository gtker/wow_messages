#![forbid(unsafe_code)]
#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::missing_panics_doc,
    clippy::style,
    clippy::missing_const_for_fn,
    clippy::doc_markdown,
    clippy::unseparated_literal_suffix
)]
#![allow(
    unused,
    non_camel_case_types,
    non_snake_case,
    clippy::missing_errors_doc,
    clippy::upper_case_acronyms,
    clippy::missing_const_for_fn,
    clippy::approx_constant
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub(crate) mod errors;
#[cfg(feature = "extended")]
pub(crate) mod extended;
pub(crate) mod inner;
pub(crate) mod manual;
pub(crate) mod util;

pub use errors::*;
#[cfg(feature = "extended")]
pub use extended::top_level::*;
pub use inner::*;
