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
    non_camel_case_types,
    non_snake_case,
    clippy::missing_errors_doc,
    clippy::upper_case_acronyms,
    clippy::approx_constant,
    clippy::derive_partial_eq_without_eq
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate core;

pub mod errors;
pub mod helper;
mod manual;
mod traits;
pub(crate) mod util;
#[allow(
    unused,
    clippy::complexity,
    clippy::bool_to_int_with_if,
    dead_code,
    clippy::single_match,
    clippy::large_enum_variant,
    clippy::enum_variant_names,
    clippy::missing_const_for_fn,
    clippy::needless_borrow
)]
mod world;

pub use traits::*;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub use world::*;

pub use helper::datetime::DateTime;
pub use helper::datetime::Weekday;
pub use helper::guid::Guid;

pub const DEFAULT_PORT: u16 = 8085;
