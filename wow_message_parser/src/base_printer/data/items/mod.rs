use crate::base_printer::Expansion;
use rusqlite::Connection;
use tbc::TbcItem;
use vanilla::VanillaItem;
use wrath::WrathItem;

pub mod tbc;
pub mod vanilla;
pub mod wrath;

pub enum Items {
    Vanilla(Vec<VanillaItem>),
    BurningCrusade(Vec<TbcItem>),
    Wrath(Vec<WrathItem>),
}

pub enum Value {
    String(String),
    Int(i32),
    Uint(u32),
    Float(f32),

    VanillaItemClassAndSubClass(wow_world_base::vanilla::ItemClassAndSubClass),
    TbcItemClassAndSubClass(wow_world_base::tbc::ItemClassAndSubClass),
    WrathItemClassAndSubClass(wow_world_base::wrath::ItemClassAndSubClass),

    VanillaTbcItemQuality(wow_world_base::shared::item_quality_vanilla_tbc::ItemQuality),
    WrathItemQuality(wow_world_base::wrath::ItemQuality),

    InventoryType(wow_world_base::shared::inventory_type_vanilla_tbc_wrath::InventoryType),

    VanillaTbcAllowedClass(wow_world_base::shared::allowed_class_vanilla_tbc::AllowedClass),
    WrathAllowedClass(wow_world_base::wrath::AllowedClass),

    VanillaAllowedRace(wow_world_base::vanilla::AllowedRace),
    TbcAllowedRace(wow_world_base::tbc::AllowedRace),
    WrathAllowedRace(wow_world_base::wrath::AllowedRace),

    SpellSchool(wow_world_base::shared::spell_school_vanilla_vanilla_tbc_wrath::SpellSchool),

    VanillaSpellTriggerType(wow_world_base::vanilla::SpellTriggerType),
    TbcWrathSpellTriggerType(
        wow_world_base::shared::spell_trigger_type_tbc_wrath::SpellTriggerType,
    ),

    Bonding(wow_world_base::shared::bonding_vanilla_tbc_wrath::Bonding),
}

fn i32_to_u32(v: i32) -> u32 {
    u32::from_le_bytes(v.to_le_bytes())
}

pub(crate) fn get_items(conn: &Connection, expansion: Expansion) -> Items {
    match expansion {
        Expansion::Vanilla => Items::Vanilla(vanilla::vanilla(conn)),
        Expansion::BurningCrusade => Items::BurningCrusade(tbc::tbc(conn)),
        Expansion::WrathOfTheLichKing => Items::Wrath(wrath::wrath(conn)),
    }
}
