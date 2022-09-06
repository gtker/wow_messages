pub(crate) mod guid;

#[cfg(feature = "vanilla")]
pub(crate) mod vanilla;

#[cfg(feature = "wrath")]
pub(crate) mod wrath;

#[cfg(feature = "tbc")]
pub(crate) mod tbc;
pub(crate) mod update_mask_common;
