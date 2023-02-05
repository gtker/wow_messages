use crate::base_printer::Expansion;
use rusqlite::Connection;
use std::cmp::Ordering;

pub mod tbc;
pub mod vanilla;
pub mod wrath;

use wow_world_base::shared as shared_base;
use wow_world_base::tbc as tbc_base;
use wow_world_base::vanilla as vanilla_base;
use wow_world_base::wrath as wrath_base;

pub struct GenericItem {
    pub entry: u32,
    pub extra_flags: i32,
    pub name: String,
    pub fields: Vec<Field>,
    pub arrays: Vec<Array>,
}

pub struct Array {
    pub variable_name: &'static str,
    pub type_name: &'static str,
    pub instances: Vec<Vec<ArrayField>>,
    pub import_only: bool,
}

impl Array {
    pub const fn new(
        variable_name: &'static str,
        type_name: &'static str,
        import_only: bool,
        instances: Vec<Vec<ArrayField>>,
    ) -> Self {
        Self {
            variable_name,
            type_name,
            instances,
            import_only,
        }
    }

    pub fn is_default(&self) -> bool {
        self.instances
            .iter()
            .all(|a| a.iter().all(|a| a.is_default()))
    }
}

pub struct ArrayField {
    pub name: &'static str,
    pub variable_name: &'static str,
    pub value: Value,
}

impl ArrayField {
    pub const fn new(name: &'static str, variable_name: &'static str, value: Value) -> Self {
        Self {
            name,
            variable_name,
            value,
        }
    }

    pub fn is_default(&self) -> bool {
        self.value.is_default()
    }
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Value {
    String(String),
    Int(i32),
    Int64(i64),
    Uint(u32),
    Uint64(u64),
    Float(f32),

    VanillaItemClassAndSubClass(vanilla_base::ItemClassAndSubClass),
    TbcItemClassAndSubClass(tbc_base::ItemClassAndSubClass),
    WrathItemClassAndSubClass(wrath_base::ItemClassAndSubClass),

    VanillaTbcItemQuality(shared_base::item_quality_vanilla_tbc::ItemQuality),
    WrathItemQuality(wrath_base::ItemQuality),

    InventoryType(shared_base::inventory_type_vanilla_tbc_wrath::InventoryType),

    VanillaTbcAllowedClass(shared_base::allowed_class_vanilla_tbc::AllowedClass),
    WrathAllowedClass(wrath_base::AllowedClass),

    VanillaAllowedRace(vanilla_base::AllowedRace),
    TbcAllowedRace(tbc_base::AllowedRace),
    WrathAllowedRace(wrath_base::AllowedRace),

    SpellSchool(shared_base::spell_school_vanilla_vanilla_tbc_wrath::SpellSchool),

    VanillaSpellTriggerType(vanilla_base::SpellTriggerType),
    TbcWrathSpellTriggerType(shared_base::spell_trigger_type_tbc_wrath::SpellTriggerType),

    Bonding(shared_base::bonding_vanilla_tbc_wrath::Bonding),

    VanillaSkill(vanilla_base::Skill),
    TbcSkill(tbc_base::Skill),
    WrathSkill(wrath_base::Skill),

    VanillaMap(vanilla_base::Map),
    TbcMap(tbc_base::Map),
    WrathMap(wrath_base::Map),

    VanillaArea(vanilla_base::Area),
    TbcArea(tbc_base::Area),
    WrathArea(wrath_base::Area),

    VanillaAttributes(vanilla_base::Attributes),
    VanillaAttributesEx1(vanilla_base::AttributesEx1),
    VanillaAttributesEx2(vanilla_base::AttributesEx2),
    VanillaAttributesEx3(vanilla_base::AttributesEx3),
    VanillaAttributesEx4(vanilla_base::AttributesEx4),

    VanillaItemFlag(vanilla_base::ItemFlag),
    TbcItemFlag(tbc_base::ItemFlag),
    WrathItemFlag(wrath_base::ItemFlag),
    WrathItemFlag2(wrath_base::ItemFlag2),

    PvpRank(shared_base::pvp_rank_vanilla_tbc_wrath::PvpRank),

    SheatheType(shared_base::sheathe_type_vanilla_tbc_wrath::SheatheType),

    VanillaBagFamily(vanilla_base::BagFamily),
    TbcWrathBagFamily(shared_base::bag_family_tbc_wrath::BagFamily),
}

impl Eq for Value {}

#[allow(clippy::derive_ord_xor_partial_ord)] // f32 can not derive Ord
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Value {
    pub const fn should_import(&self) -> bool {
        !matches!(
            self,
            Value::String(_)
                | Value::Uint(_)
                | Value::Int(_)
                | Value::Int64(_)
                | Value::Uint64(_)
                | Value::Float(_)
        )
    }

    pub const fn constructor_type_name(&self) -> &'static str {
        match self {
            Value::VanillaTbcAllowedClass(_)
            | Value::WrathAllowedClass(_)
            | Value::VanillaAllowedRace(_)
            | Value::TbcAllowedRace(_)
            | Value::WrathAllowedRace(_)
            | Value::VanillaAttributes(_)
            | Value::VanillaAttributesEx1(_)
            | Value::VanillaAttributesEx2(_)
            | Value::VanillaAttributesEx3(_)
            | Value::VanillaAttributesEx4(_)
            | Value::VanillaItemFlag(_)
            | Value::TbcItemFlag(_)
            | Value::WrathItemFlag(_)
            | Value::WrathItemFlag2(_)
            | Value::TbcWrathBagFamily(_) => "u32",
            _ => self.type_name(),
        }
    }

    pub const fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "&'static str",
            Value::Int(_) => "i32",
            Value::Int64(_) => "i64",
            Value::Uint(_) => "u32",
            Value::Uint64(_) => "u64",
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
            Value::VanillaSkill(_) | Value::TbcSkill(_) | Value::WrathSkill(_) => "Skill",
            Value::VanillaMap(_) | Value::TbcMap(_) | Value::WrathMap(_) => "Map",
            Value::VanillaArea(_) | Value::TbcArea(_) | Value::WrathArea(_) => "Area",
            Value::VanillaAttributes(_) => "Attributes",
            Value::VanillaAttributesEx1(_) => "AttributesEx1",
            Value::VanillaAttributesEx2(_) => "AttributesEx2",
            Value::VanillaAttributesEx3(_) => "AttributesEx3",
            Value::VanillaAttributesEx4(_) => "AttributesEx4",
            Value::VanillaItemFlag(_) | Value::TbcItemFlag(_) | Value::WrathItemFlag(_) => {
                "ItemFlag"
            }
            Value::WrathItemFlag2(_) => "ItemFlag2",
            Value::PvpRank(_) => "PvpRank",
            Value::SheatheType(_) => "SheatheType",
            Value::VanillaBagFamily(_) | Value::TbcWrathBagFamily(_) => "BagFamily",
        }
    }

    pub const fn import_name(&self) -> Option<&'static str> {
        if self.should_import() {
            Some(self.type_name())
        } else {
            None
        }
    }

    pub fn const_name(&self) -> &'static str {
        match self {
            Value::String(_) => "A",
            Value::Int(_) => "B",
            Value::Int64(_) => "C",
            Value::Uint(_) => "D",
            Value::Uint64(_) => "E",
            Value::Float(_) => "F",

            Value::VanillaItemClassAndSubClass(_) => "G",
            Value::TbcItemClassAndSubClass(_) => "G",
            Value::WrathItemClassAndSubClass(_) => "G",

            Value::VanillaTbcItemQuality(_) => "H",
            Value::WrathItemQuality(_) => "H",

            Value::InventoryType(_) => "I",

            Value::VanillaTbcAllowedClass(_) => "J",
            Value::WrathAllowedClass(_) => "J",

            Value::VanillaAllowedRace(_) => "K",
            Value::TbcAllowedRace(_) => "K",
            Value::WrathAllowedRace(_) => "K",

            Value::SpellSchool(_) => "L",

            Value::VanillaSpellTriggerType(_) => "M",
            Value::TbcWrathSpellTriggerType(_) => "M",

            Value::Bonding(_) => "N",

            Value::VanillaSkill(_) => "O",
            Value::TbcSkill(_) => "O",
            Value::WrathSkill(_) => "O",

            Value::VanillaMap(_) => "P",
            Value::TbcMap(_) => "P",
            Value::WrathMap(_) => "P",

            Value::VanillaArea(_) => "Q",
            Value::TbcArea(_) => "Q",
            Value::WrathArea(_) => "Q",

            Value::VanillaAttributes(_) => "R",
            Value::VanillaAttributesEx1(_) => "S",
            Value::VanillaAttributesEx2(_) => "T",
            Value::VanillaAttributesEx3(_) => "U",
            Value::VanillaAttributesEx4(_) => "V",

            Value::VanillaItemFlag(_) | Value::TbcItemFlag(_) | Value::WrathItemFlag(_) => "X",

            Value::WrathItemFlag2(_) => "Y",

            Value::PvpRank(_) => "Z",

            Value::SheatheType(_) => "AA",
            Value::VanillaBagFamily(_) | Value::TbcWrathBagFamily(_) => "AB",
        }
    }

    pub fn is_default(&self) -> bool {
        &self.default_value() == self
    }

    pub fn to_string_value(&self) -> String {
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
            Value::Int64(v) => (*v).to_string(),
            Value::Uint(v) => (*v).to_string(),
            Value::Uint64(v) => (*v).to_string(),
            Value::Float(v) => float_format(*v),
            Value::VanillaItemClassAndSubClass(v) => {
                format!("ItemClassAndSubClass::{v:?}")
            }
            Value::TbcItemClassAndSubClass(v) => {
                format!("ItemClassAndSubClass::{v:?}")
            }
            Value::WrathItemClassAndSubClass(v) => format!("ItemClassAndSubClass::{v:?}"),
            Value::VanillaTbcItemQuality(v) => {
                format!("ItemQuality::{v:?}")
            }
            Value::WrathItemQuality(v) => {
                format!("ItemQuality::{v:?}")
            }
            Value::InventoryType(v) => format!("InventoryType::{v:?}"),
            Value::VanillaTbcAllowedClass(v) => v.as_int().to_string(),
            Value::WrathAllowedClass(v) => v.as_int().to_string(),
            Value::VanillaAllowedRace(v) => v.as_int().to_string(),
            Value::TbcAllowedRace(v) => v.as_int().to_string(),
            Value::WrathAllowedRace(v) => v.as_int().to_string(),
            Value::SpellSchool(v) => format!("SpellSchool::{v:?}"),
            Value::VanillaSpellTriggerType(v) => {
                format!("SpellTriggerType::{v:?}")
            }
            Value::TbcWrathSpellTriggerType(v) => {
                format!("SpellTriggerType::{v:?}")
            }
            Value::Bonding(v) => format!("Bonding::{v:?}"),
            Value::VanillaSkill(v) => {
                format!("Skill::{v:?}")
            }
            Value::TbcSkill(v) => {
                format!("Skill::{v:?}")
            }
            Value::WrathSkill(v) => {
                format!("Skill::{v:?}")
            }
            Value::VanillaMap(v) => {
                format!("Map::{v:?}")
            }
            Value::TbcMap(v) => {
                format!("Map::{v:?}")
            }
            Value::WrathMap(v) => format!("Map::{v:?}"),
            Value::VanillaArea(v) => {
                format!("Area::{v:?}")
            }
            Value::TbcArea(v) => {
                format!("Area::{v:?}")
            }
            Value::WrathArea(v) => {
                format!("Area::{v:?}")
            }
            Value::VanillaAttributes(v) => v.as_int().to_string(),
            Value::VanillaAttributesEx1(v) => v.as_int().to_string(),
            Value::VanillaAttributesEx2(v) => v.as_int().to_string(),
            Value::VanillaAttributesEx3(v) => v.as_int().to_string(),
            Value::VanillaAttributesEx4(v) => v.as_int().to_string(),
            Value::VanillaItemFlag(v) => v.as_int().to_string(),
            Value::TbcItemFlag(v) => v.as_int().to_string(),
            Value::WrathItemFlag(v) => v.as_int().to_string(),
            Value::WrathItemFlag2(v) => v.as_int().to_string(),
            Value::PvpRank(v) => format!("PvpRank::{v:?}"),
            Value::SheatheType(v) => format!("SheatheType::{v:?}"),
            Value::VanillaBagFamily(v) => format!("BagFamily::{v:?}"),
            Value::TbcWrathBagFamily(v) => v.as_int().to_string(),
        }
    }

    pub fn definition_has_extra(&self) -> Option<String> {
        match self {
            Value::VanillaTbcAllowedClass(_)
            | Value::WrathAllowedClass(_)
            | Value::VanillaAllowedRace(_)
            | Value::TbcAllowedRace(_)
            | Value::WrathAllowedRace(_)
            | Value::VanillaAttributes(_)
            | Value::VanillaAttributesEx1(_)
            | Value::VanillaAttributesEx2(_)
            | Value::VanillaAttributesEx3(_)
            | Value::VanillaAttributesEx4(_)
            | Value::VanillaItemFlag(_)
            | Value::TbcItemFlag(_)
            | Value::WrathItemFlag(_)
            | Value::WrathItemFlag2(_)
            | Value::TbcWrathBagFamily(_) => Some(format!("{}::new", self.type_name())),
            _ => None,
        }
    }

    pub fn default_value(&self) -> Self {
        match self {
            Value::String(_) => Value::String("".to_string()),
            Value::Int(_) => Value::Int(0),
            Value::Int64(_) => Value::Int64(0),
            Value::Uint(_) => Value::Uint(0),
            Value::Uint64(_) => Value::Uint64(0),
            Value::Float(_) => Value::Float(0.0),
            Value::VanillaAllowedRace(_) => {
                Value::VanillaAllowedRace(vanilla_base::AllowedRace::empty())
            }
            Value::VanillaArea(_) => Value::VanillaArea(vanilla_base::Area::None),
            Value::VanillaItemClassAndSubClass(_) => {
                Value::VanillaItemClassAndSubClass(vanilla_base::ItemClassAndSubClass::Consumable)
            }
            Value::VanillaMap(_) => Value::VanillaMap(vanilla_base::Map::EasternKingdoms),
            Value::VanillaSkill(_) => Value::VanillaSkill(vanilla_base::Skill::None),
            Value::VanillaSpellTriggerType(_) => {
                Value::VanillaSpellTriggerType(vanilla_base::SpellTriggerType::OnUse)
            }
            Value::VanillaTbcAllowedClass(_) => Value::VanillaTbcAllowedClass(
                shared_base::allowed_class_vanilla_tbc::AllowedClass::empty(),
            ),
            Value::VanillaTbcItemQuality(_) => Value::VanillaTbcItemQuality(
                shared_base::item_quality_vanilla_tbc::ItemQuality::Poor,
            ),
            Value::TbcItemClassAndSubClass(_) => {
                Value::TbcItemClassAndSubClass(tbc_base::ItemClassAndSubClass::Consumable)
            }
            Value::WrathItemClassAndSubClass(_) => {
                Value::WrathItemClassAndSubClass(wrath_base::ItemClassAndSubClass::Consumable)
            }
            Value::WrathItemQuality(_) => Value::WrathItemQuality(wrath_base::ItemQuality::Poor),
            Value::InventoryType(_) => Value::InventoryType(
                shared_base::inventory_type_vanilla_tbc_wrath::InventoryType::NonEquip,
            ),
            Value::WrathAllowedClass(_) => {
                Value::WrathAllowedClass(wrath_base::AllowedClass::empty())
            }
            Value::TbcAllowedRace(_) => Value::TbcAllowedRace(tbc_base::AllowedRace::empty()),
            Value::WrathAllowedRace(_) => Value::WrathAllowedRace(wrath_base::AllowedRace::empty()),
            Value::SpellSchool(_) => Value::SpellSchool(
                shared_base::spell_school_vanilla_vanilla_tbc_wrath::SpellSchool::Normal,
            ),
            Value::TbcWrathSpellTriggerType(_) => Value::TbcWrathSpellTriggerType(
                shared_base::spell_trigger_type_tbc_wrath::SpellTriggerType::OnUse,
            ),
            Value::Bonding(_) => {
                Value::Bonding(shared_base::bonding_vanilla_tbc_wrath::Bonding::NoBind)
            }
            Value::TbcSkill(_) => Value::TbcSkill(tbc_base::Skill::None),
            Value::WrathSkill(_) => Value::WrathSkill(wrath_base::Skill::None),
            Value::TbcMap(_) => Value::TbcMap(tbc_base::Map::EasternKingdoms),
            Value::WrathMap(_) => Value::WrathMap(wrath_base::Map::EasternKingdoms),
            Value::TbcArea(_) => Value::TbcArea(tbc_base::Area::None),
            Value::WrathArea(_) => Value::WrathArea(wrath_base::Area::None),
            Value::VanillaAttributes(_) => {
                Value::VanillaAttributes(vanilla_base::Attributes::empty())
            }
            Value::VanillaAttributesEx1(_) => {
                Value::VanillaAttributesEx1(vanilla_base::AttributesEx1::empty())
            }
            Value::VanillaAttributesEx2(_) => {
                Value::VanillaAttributesEx2(vanilla_base::AttributesEx2::empty())
            }
            Value::VanillaAttributesEx3(_) => {
                Value::VanillaAttributesEx3(vanilla_base::AttributesEx3::empty())
            }
            Value::VanillaAttributesEx4(_) => {
                Value::VanillaAttributesEx4(vanilla_base::AttributesEx4::empty())
            }
            Value::VanillaItemFlag(_) => Value::VanillaItemFlag(vanilla_base::ItemFlag::empty()),
            Value::TbcItemFlag(_) => Value::TbcItemFlag(tbc_base::ItemFlag::empty()),
            Value::WrathItemFlag(_) => Value::WrathItemFlag(wrath_base::ItemFlag::empty()),
            Value::WrathItemFlag2(_) => Value::WrathItemFlag2(wrath_base::ItemFlag2::empty()),
            Value::PvpRank(_) => {
                Value::PvpRank(shared_base::pvp_rank_vanilla_tbc_wrath::PvpRank::NoRank)
            }
            Value::SheatheType(_) => {
                Value::SheatheType(shared_base::sheathe_type_vanilla_tbc_wrath::SheatheType::None)
            }
            Value::VanillaBagFamily(_) => Value::VanillaBagFamily(vanilla_base::BagFamily::None),
            Value::TbcWrathBagFamily(_) => {
                Value::TbcWrathBagFamily(shared_base::bag_family_tbc_wrath::BagFamily::empty())
            }
        }
    }
}

fn i32_to_u32(v: i32) -> u32 {
    u32::from_le_bytes(v.to_le_bytes())
}

pub(crate) fn get_items(conn: &Connection, expansion: Expansion) -> Vec<GenericItem> {
    match expansion {
        Expansion::Vanilla => vanilla::vanilla(conn),
        Expansion::BurningCrusade => tbc::tbc(conn),
        Expansion::WrathOfTheLichKing => wrath::wrath(conn),
    }
}
