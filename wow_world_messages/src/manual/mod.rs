#[cfg(feature = "vanilla")]
pub(crate) mod vanilla;

#[cfg(feature = "tbc")]
pub(crate) mod tbc;

#[cfg(feature = "wrath")]
pub(crate) mod wrath;

pub(crate) mod shared;
