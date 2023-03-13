pub(crate) mod guid;

#[cfg(feature = "vanilla")]
pub(crate) mod vanilla;

#[cfg(feature = "wrath")]
pub(crate) mod wrath;

#[cfg(feature = "tbc")]
pub(crate) mod tbc;
pub(crate) mod update_mask_common;

#[cfg(any(feature = "tbc", feature = "wrath"))]
pub(crate) mod named_guid;

pub(crate) mod datetime;
pub(crate) mod shared;
