#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod allowed_class_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod class_vanilla_tbc;
pub use crate::manual::shared::*;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod gender_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod inventory_type_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod item_class_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod power_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod vector2d_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod vector3d_vanilla_tbc_wrath;
