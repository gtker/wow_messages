#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
mod base;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub use base::*;

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
mod shared;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) use shared::*;

#[cfg(feature = "wrath")]
mod wrath;
#[cfg(feature = "wrath")]
pub(crate) use wrath::*;
