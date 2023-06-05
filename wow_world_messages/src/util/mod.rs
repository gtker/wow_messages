#[cfg(feature = "tokio")]
mod tokio_impl;
#[cfg(feature = "tokio")]
pub use tokio_impl::*;

#[cfg(feature = "async-std")]
mod async_std_impl;
#[cfg(feature = "async-std")]
pub use async_std_impl::*;

mod functions;
pub use functions::*;

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
mod trait_helpers;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) use trait_helpers::*;

#[cfg(feature = "print-testcase")]
pub(crate) fn write_bytes(
    s: &mut String,
    bytes: &mut impl Iterator<Item = u8>,
    size: usize,
    name: &str,
    prefix: &str,
) {
    use std::fmt::Write;
    let mut bytes = bytes.take(size);

    write!(s, "{prefix}{:#04X}, ", bytes.next().unwrap()).unwrap();

    for b in bytes {
        write!(s, "{b:#04X}, ").unwrap();
    }
    writeln!(s, "/* {name} */").unwrap();
}
