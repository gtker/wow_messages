#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod monster_move_spline_vanilla_tbc_wrath;
#[cfg(feature = "tbc")]
pub(crate) mod tbc;
#[cfg(feature = "vanilla")]
pub(crate) mod vanilla;
#[cfg(feature = "wrath")]
pub(crate) mod wrath;
