pub(crate) mod shared;
pub mod top_level;

#[cfg(feature = "tbc")]
pub(crate) mod tbc;

#[cfg(feature = "wrath")]
pub(crate) mod wrath;

#[cfg(feature = "vanilla")]
pub(crate) mod vanilla;
