pub(crate) mod guid;

#[cfg(feature = "vanilla")]
pub(crate) mod vanilla;
#[cfg(feature = "vanilla")]
pub use vanilla::*;

#[cfg(feature = "wrath")]
pub(crate) mod wrath;

#[cfg(feature = "tbc")]
pub(crate) mod tbc;
