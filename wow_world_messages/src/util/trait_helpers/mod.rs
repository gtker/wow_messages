#[cfg(feature = "vanilla")]
mod vanilla;
#[cfg(feature = "vanilla")]
pub(crate) use vanilla::*;

#[cfg(feature = "tbc")]
mod tbc;
#[cfg(feature = "tbc")]
pub(crate) use tbc::*;

#[cfg(feature = "wrath")]
mod wrath;
#[cfg(feature = "wrath")]
pub(crate) use wrath::*;

#[cfg(any(feature = "vanilla", feature = "tbc"))]
mod vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub(crate) use vanilla_tbc::*;

#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub(crate) const CLIENT_HEADER_LENGTH: u16 = 6;
