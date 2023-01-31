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

impl Items {
    pub fn to_generic(&self) -> (Vec<GenericItem>, Expansion) {
        match self {
            Items::Vanilla(v) => (
                v.iter()
                    .map(|a| {
                        let (fields, arrays) = a.values();
                        GenericItem {
                            entry: a.entry,
                            extra_flags: a.extra_flags,
                            name: a.name.clone(),
                            fields,
                            arrays,
                        }
                    })
                    .collect(),
                Expansion::Vanilla,
            ),
            Items::BurningCrusade(v) => (
                v.iter()
                    .map(|a| {
                        let (fields, arrays) = a.values();
                        GenericItem {
                            entry: a.entry,
                            extra_flags: a.extra_flags,
                            name: a.name.clone(),
                            fields,
                            arrays,
                        }
                    })
                    .collect(),
                Expansion::BurningCrusade,
            ),
            Items::Wrath(v) => (
                v.iter()
                    .map(|a| {
                        let (fields, arrays) = a.values();
                        GenericItem {
                            entry: a.entry,
                            extra_flags: a.extra_flags,
                            name: a.name.clone(),
                            fields,
                            arrays,
                        }
                    })
                    .collect(),
                Expansion::WrathOfTheLichKing,
            ),
        }
    }
}

pub struct GenericItem {
    pub entry: u32,
    pub extra_flags: i32,
    pub name: String,
    pub fields: Vec<Field>,
    pub arrays: Vec<Array>,
}

impl GenericItem {
    pub fn all_arrays_are_default(&self) -> bool {
        self.arrays.iter().all(|a| {
            a.instances
                .iter()
                .all(|a| a.iter().all(|a| a.value.is_default()))
        })
    }
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

    VanillaSkill(wow_world_base::vanilla::Skill),
    TbcSkill(wow_world_base::tbc::Skill),
    WrathSkill(wow_world_base::wrath::Skill),

    VanillaMap(wow_world_base::vanilla::Map),
    TbcMap(wow_world_base::tbc::Map),
    WrathMap(wow_world_base::wrath::Map),

    VanillaArea(wow_world_base::vanilla::Area),
    TbcArea(wow_world_base::tbc::Area),
    WrathArea(wow_world_base::wrath::Area),
}

impl Value {
    pub const fn should_import(&self) -> bool {
        match self {
            Value::String(_) | Value::Uint(_) | Value::Int(_) | Value::Float(_) => false,

            _ => true,
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
            Value::VanillaSkill(_) | Value::TbcSkill(_) | Value::WrathSkill(_) => "Skill",
            Value::VanillaMap(_) | Value::TbcMap(_) | Value::WrathMap(_) => "Map",
            Value::VanillaArea(_) | Value::TbcArea(_) | Value::WrathArea(_) => "Area",
        }
    }

    pub const fn import_name(&self) -> Option<&'static str> {
        if self.should_import() {
            Some(self.type_name())
        } else {
            None
        }
    }

    pub fn is_default(&self) -> bool {
        &self.default_value() == self
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
            Value::VanillaItemClassAndSubClass(v) => {
                format!("ItemClassAndSubClass::{:?}", v)
            }
            Value::TbcItemClassAndSubClass(v) => {
                format!("ItemClassAndSubClass::{:?}", v)
            }
            Value::WrathItemClassAndSubClass(v) => format!("ItemClassAndSubClass::{:?}", v),
            Value::VanillaTbcItemQuality(v) => {
                format!("ItemQuality::{:?}", v)
            }
            Value::WrathItemQuality(v) => {
                format!("ItemQuality::{:?}", v)
            }
            Value::InventoryType(v) => format!("InventoryType::{:?}", v),
            Value::VanillaTbcAllowedClass(v) => {
                format!("AllowedClass::new({})", v.as_int())
            }
            Value::WrathAllowedClass(v) => {
                format!("AllowedClass::new({})", v.as_int())
            }
            Value::VanillaAllowedRace(v) => {
                format!("AllowedRace::new({})", v.as_int())
            }
            Value::TbcAllowedRace(v) => {
                format!("AllowedRace::new({})", v.as_int())
            }
            Value::WrathAllowedRace(v) => format!("AllowedRace::new({})", v.as_int()),
            Value::SpellSchool(v) => format!("SpellSchool::{:?}", v),
            Value::VanillaSpellTriggerType(v) => {
                format!("SpellTriggerType::{:?}", v)
            }
            Value::TbcWrathSpellTriggerType(v) => {
                format!("SpellTriggerType::{:?}", v)
            }
            Value::Bonding(v) => format!("Bonding::{:?}", v),
            Value::VanillaSkill(v) => {
                format!("Skill::{:?}", v)
            }
            Value::TbcSkill(v) => {
                format!("Skill::{:?}", v)
            }
            Value::WrathSkill(v) => {
                format!("Skill::{:?}", v)
            }
            Value::VanillaMap(v) => {
                format!("Map::{:?}", v)
            }
            Value::TbcMap(v) => {
                format!("Map::{:?}", v)
            }
            Value::WrathMap(v) => format!("Map::{:?}", v),
            Value::VanillaArea(v) => {
                format!("Area::{:?}", v)
            }
            Value::TbcArea(v) => {
                format!("Area::{:?}", v)
            }
            Value::WrathArea(v) => {
                format!("Area::{:?}", v)
            }
        }
    }

    pub fn default_value(&self) -> Self {
        match self {
            Value::String(_) => Value::String("".to_string()),
            Value::Int(_) => Value::Int(0),
            Value::Uint(_) => Value::Uint(0),
            Value::Float(_) => Value::Float(0.0),
            Value::VanillaAllowedRace(_) => {
                Value::VanillaAllowedRace(wow_world_base::vanilla::AllowedRace::empty())
            }
            Value::VanillaArea(_) => Value::VanillaArea(wow_world_base::vanilla::Area::None),
            Value::VanillaItemClassAndSubClass(_) => Value::VanillaItemClassAndSubClass(
                wow_world_base::vanilla::ItemClassAndSubClass::Consumable,
            ),
            Value::VanillaMap(_) => {
                Value::VanillaMap(wow_world_base::vanilla::Map::EasternKingdoms)
            }
            Value::VanillaSkill(_) => Value::VanillaSkill(wow_world_base::vanilla::Skill::None),
            Value::VanillaSpellTriggerType(_) => {
                Value::VanillaSpellTriggerType(wow_world_base::vanilla::SpellTriggerType::OnUse)
            }
            Value::VanillaTbcAllowedClass(_) => Value::VanillaTbcAllowedClass(
                wow_world_base::shared::allowed_class_vanilla_tbc::AllowedClass::empty(),
            ),
            Value::VanillaTbcItemQuality(_) => Value::VanillaTbcItemQuality(
                wow_world_base::shared::item_quality_vanilla_tbc::ItemQuality::Poor,
            ),
            Value::TbcItemClassAndSubClass(_) => Value::TbcItemClassAndSubClass(
                wow_world_base::tbc::ItemClassAndSubClass::Consumable,
            ),
            Value::WrathItemClassAndSubClass(_) => Value::WrathItemClassAndSubClass(
                wow_world_base::wrath::ItemClassAndSubClass::Consumable,
            ),
            Value::WrathItemQuality(_) => {
                Value::WrathItemQuality(wow_world_base::wrath::ItemQuality::Poor)
            }
            Value::InventoryType(_) => Value::InventoryType(
                wow_world_base::shared::inventory_type_vanilla_tbc_wrath::InventoryType::NonEquip,
            ),
            Value::WrathAllowedClass(_) => {
                Value::WrathAllowedClass(wow_world_base::wrath::AllowedClass::empty())
            }
            Value::TbcAllowedRace(_) => {
                Value::TbcAllowedRace(wow_world_base::tbc::AllowedRace::empty())
            }
            Value::WrathAllowedRace(_) => {
                Value::WrathAllowedRace(wow_world_base::wrath::AllowedRace::empty())
            }
            Value::SpellSchool(_) => Value::SpellSchool(
                wow_world_base::shared::spell_school_vanilla_vanilla_tbc_wrath::SpellSchool::Normal,
            ),
            Value::TbcWrathSpellTriggerType(_) => Value::TbcWrathSpellTriggerType(
                wow_world_base::shared::spell_trigger_type_tbc_wrath::SpellTriggerType::OnUse,
            ),
            Value::Bonding(_) => {
                Value::Bonding(wow_world_base::shared::bonding_vanilla_tbc_wrath::Bonding::NoBind)
            }
            Value::TbcSkill(_) => Value::TbcSkill(wow_world_base::tbc::Skill::None),
            Value::WrathSkill(_) => Value::WrathSkill(wow_world_base::wrath::Skill::None),
            Value::TbcMap(_) => Value::TbcMap(wow_world_base::tbc::Map::EasternKingdoms),
            Value::WrathMap(_) => Value::WrathMap(wow_world_base::wrath::Map::EasternKingdoms),
            Value::TbcArea(_) => Value::TbcArea(wow_world_base::tbc::Area::None),
            Value::WrathArea(_) => Value::WrathArea(wow_world_base::wrath::Area::None),
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
