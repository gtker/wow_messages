#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod allowed_class_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod bag_family_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod bonding_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod class_vanilla_tbc;
pub use crate::manual::shared::*;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod gender_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod inventory_type_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod item_class_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod item_damage_type_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod item_quality_vanilla_tbc;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod item_socket_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod item_spells_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod item_stat_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod language_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "vanilla"))]
pub mod language_vanilla_vanilla;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod page_text_material_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc"))]
pub mod power_vanilla_tbc;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod pvp_rank_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod sheathe_type_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod spell_school_vanilla_vanilla_tbc_wrath;
#[cfg(any(feature = "tbc", feature = "wrath"))]
pub mod spell_trigger_type_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod vector2d_vanilla_tbc_wrath;
#[cfg(any(feature = "vanilla", feature = "tbc", feature = "wrath"))]
pub mod vector3d_vanilla_tbc_wrath;
