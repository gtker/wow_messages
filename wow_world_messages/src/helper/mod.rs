#[cfg(feature = "vanilla")]
pub(crate) mod vanilla;

#[cfg(feature = "wrath")]
pub(crate) mod wrath;

#[cfg(feature = "tbc")]
pub(crate) mod tbc;

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) mod update_mask_common;
