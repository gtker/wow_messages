#![forbid(unsafe_code)]
#![warn(
    clippy::approx_constant,
    clippy::bool_to_int_with_if,
    clippy::complexity,
    clippy::correctness,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_markdown,
    clippy::format_in_format_args,
    clippy::uninlined_format_args,
    clippy::enum_variant_names,
    clippy::large_enum_variant,
    clippy::needless_borrow,
    clippy::perf,
    clippy::single_match,
    clippy::style,
    clippy::unseparated_literal_suffix,
    clippy::upper_case_acronyms,
    dead_code,
    non_camel_case_types,
    unused
)]
#![allow(clippy::too_many_arguments)]

mod base_printer;
mod path_utils;
mod writer;
mod file_utils;
mod ordered_float;

fn main() {
    base_printer::print_base();
}
pub(crate) fn float_format(v: f32) -> String {
    let s = format!("{v}");
    if s.contains('.') {
        s
    } else {
        format!("{s}.0")
    }
}
