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

pub struct Field {
    pub name: &'static str,
    pub value: Value,
}

impl Field {
    pub const fn new(name: &'static str, value: Value) -> Self {
        Self { name, value }
    }
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

impl Value {
    pub const fn should_import(&self) -> bool {
        match self {
            Value::String(_) | Value::Uint(_) | Value::Int(_) | Value::Float(_) => false,

            Value::VanillaItemClassAndSubClass(_)
            | Value::TbcItemClassAndSubClass(_)
            | Value::WrathItemClassAndSubClass(_)
            | Value::VanillaTbcItemQuality(_)
            | Value::WrathItemQuality(_)
            | Value::InventoryType(_)
            | Value::VanillaTbcAllowedClass(_)
            | Value::WrathAllowedClass(_)
            | Value::VanillaAllowedRace(_)
            | Value::TbcAllowedRace(_)
            | Value::WrathAllowedRace(_)
            | Value::SpellSchool(_)
            | Value::VanillaSpellTriggerType(_)
            | Value::TbcWrathSpellTriggerType(_)
            | Value::Bonding(_) => true,
        }
    }

    pub const fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "&'static str",
            Value::Int(_) => "i32",
            Value::Uint(_) => "u32",
            Value::Float(_) => "f32",
            Value::VanillaItemClassAndSubClass(_)
            | Value::TbcItemClassAndSubClass(_)
            | Value::WrathItemClassAndSubClass(_) => "ItemClassAndSubClass",
            Value::VanillaTbcItemQuality(_) | Value::WrathItemQuality(_) => "ItemQuality",
            Value::InventoryType(_) => "InventoryType",
            Value::VanillaTbcAllowedClass(_) | Value::WrathAllowedClass(_) => "AllowedClass",
            Value::VanillaAllowedRace(_)
            | Value::TbcAllowedRace(_)
            | Value::WrathAllowedRace(_) => "AllowedRace",
            Value::SpellSchool(_) => "SpellSchool",
            Value::VanillaSpellTriggerType(_) | Value::TbcWrathSpellTriggerType(_) => {
                "SpellTriggerType"
            }
            Value::Bonding(_) => "Bonding",
        }
    }

    pub fn to_string(&self) -> String {
        fn float_format(v: f32) -> String {
            let s = format!("{v}");
            if s.contains('.') {
                s
            } else {
                format!("{s}.0")
            }
        }

        match self {
            Value::String(v) => format!("\"{}\"", v.replace('"', "\\\"")),
            Value::Int(v) => (*v).to_string(),
            Value::Uint(v) => (*v).to_string(),
            Value::Float(v) => float_format(*v),
            Value::VanillaItemClassAndSubClass(v) => format!("ItemClassAndSubClass::{:?}", v),
            Value::TbcItemClassAndSubClass(v) => format!("ItemClassAndSubClass::{:?}", v),
            Value::WrathItemClassAndSubClass(v) => format!("ItemClassAndSubClass::{:?}", v),
            Value::VanillaTbcItemQuality(v) => format!("ItemQuality::{:?}", v),
            Value::WrathItemQuality(v) => format!("ItemQuality::{:?}", v),
            Value::InventoryType(v) => format!("InventoryType::{:?}", v),
            Value::VanillaTbcAllowedClass(v) => format!("AllowedClass::new({})", v.as_int()),
            Value::WrathAllowedClass(v) => format!("AllowedClass::new({})", v.as_int()),
            Value::VanillaAllowedRace(v) => format!("AllowedRace::new({})", v.as_int()),
            Value::TbcAllowedRace(v) => format!("AllowedRace::new({})", v.as_int()),
            Value::WrathAllowedRace(v) => format!("AllowedRace::new({})", v.as_int()),
            Value::SpellSchool(v) => format!("SpellSchool::{:?}", v),
            Value::VanillaSpellTriggerType(v) => format!("SpellTriggerType::{:?}", v),
            Value::TbcWrathSpellTriggerType(v) => format!("SpellTriggerType::{:?}", v),
            Value::Bonding(v) => format!("Bonding::{:?}", v),
        }
    }
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
