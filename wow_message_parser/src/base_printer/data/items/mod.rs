use crate::base_printer::Expansion;
use rusqlite::Connection;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hasher;

pub mod tbc;
pub mod vanilla;
pub mod wrath;

use crate::base_printer::write::items::GenericThing;
use wow_world_base::shared as shared_base;
use wow_world_base::tbc as tbc_base;
use wow_world_base::vanilla as vanilla_base;
use wow_world_base::wrath as wrath_base;

#[derive(Debug, Clone)]
pub enum FieldOptimization {
    None,
    ConstantValue(Value),
    Baseline(Value, Vec<(Vec<u32>, Value)>),
}

const OPTIMIZATION_BASELINE_DEVIATIONS: usize = 50;

impl FieldOptimization {
    pub const fn skip_field(&self) -> bool {
        !matches!(self, Self::None)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum IntegerSize {
    U64,
    U32,
    U16,
    U8,
    I64,
    I32,
    I16,
    I8,
}

impl IntegerSize {
    pub fn string_value(&self) -> &'static str {
        match self {
            IntegerSize::U64 => "u64",
            IntegerSize::U32 => "u32",
            IntegerSize::U16 => "u16",
            IntegerSize::U8 => "u8",
            IntegerSize::I64 => "i64",
            IntegerSize::I32 => "i32",
            IntegerSize::I16 => "i16",
            IntegerSize::I8 => "i8",
        }
    }

    pub fn from_unsigned(upper: u64) -> Self {
        if upper <= u8::MAX.into() {
            Self::U8
        } else if upper <= u16::MAX.into() {
            Self::U16
        } else if upper <= u32::MAX.into() {
            Self::U32
        } else {
            Self::U64
        }
    }

    pub fn from_signed(lower: i64, upper: i64) -> Self {
        if lower >= i8::MIN.into() && upper <= i8::MAX.into() {
            Self::I8
        } else if lower >= i16::MIN.into() && upper <= i16::MAX.into() {
            Self::I16
        } else if lower >= i32::MIN.into() && upper <= i32::MAX.into() {
            Self::I32
        } else {
            Self::I64
        }
    }
}

pub struct Optimizations {
    field_optimizations: HashMap<String, FieldOptimization>,
    type_optimizations: HashMap<String, IntegerSize>,
}

impl Optimizations {
    pub fn optimization(&self, field_name: &str) -> FieldOptimization {
        self.field_optimizations
            .iter()
            .find(|(field, _)| field.as_str() == field_name)
            .unwrap()
            .1
            .clone()
    }

    pub fn is_non_native_type(&self, field: &Field) -> bool {
        self.integer_size(field).is_some()
    }

    pub fn integer_size(&self, field: &Field) -> Option<IntegerSize> {
        self.type_optimizations.get(field.name).cloned()
    }

    pub fn native_integer_type_cast(&self, field: &Field) -> Option<&'static str> {
        if let Some(g) = self.integer_size(field) {
            Some(match g {
                IntegerSize::U16 | IntegerSize::U8 => "u32",
                IntegerSize::I16 | IntegerSize::I8 => "i32",
                _ => return None,
            })
        } else {
            None
        }
    }

    pub fn type_name(&self, field: &Field) -> &'static str {
        if let Some(t) = self.integer_size(field) {
            t.string_value()
        } else {
            field.value.type_name()
        }
    }
}

impl Optimizations {
    pub fn new(items: &[GenericThing]) -> Self {
        let mut field_optimizations = HashMap::new();
        let mut type_optimizations = HashMap::new();

        let field_names = items[0].fields.iter().map(|field| field.name);

        for field_name in field_names {
            let value = &items[0]
                .fields
                .iter()
                .find(|field| field.name == field_name)
                .unwrap()
                .value;
            let fields = items.iter().map(|item| {
                (
                    item.entry,
                    &item
                        .fields
                        .iter()
                        .find(|field| field.name == field_name)
                        .unwrap()
                        .value,
                )
            });

            let mut unsigned_max = 0;
            let mut signed_min = 0;
            let mut signed_max = 0;

            let mut different_values: BTreeMap<Value, Vec<u32>> = BTreeMap::new();
            for field in fields {
                if let Some(v) = field.1.i64_value() {
                    if v > signed_max {
                        signed_max = v;
                    }
                    if v < signed_min {
                        signed_min = v;
                    }
                }

                if let Some(v) = field.1.u64_value() {
                    if v > unsigned_max {
                        unsigned_max = v;
                    }
                }

                if let Some(v) = different_values.get_mut(field.1) {
                    v.push(field.0);
                } else {
                    let v = vec![field.0];
                    different_values.insert(field.1.clone(), v);
                }
            }

            if value.i64_value().is_some() {
                type_optimizations.insert(
                    field_name.to_string(),
                    IntegerSize::from_signed(signed_min, signed_max),
                );
            } else if value.u64_value().is_some() {
                type_optimizations.insert(
                    field_name.to_string(),
                    IntegerSize::from_unsigned(unsigned_max),
                );
            }

            let optimization = if different_values.len() == 1 {
                FieldOptimization::ConstantValue(value.clone())
            } else {
                let mut baseline = different_values.iter().next().unwrap();
                for s in &different_values {
                    if s.1.len() > baseline.1.len() {
                        baseline = s;
                    }
                }

                let total = different_values.iter().fold(0_usize, |a, b| a + b.1.len());
                let deviations = total - baseline.1.len();

                if deviations <= OPTIMIZATION_BASELINE_DEVIATIONS {
                    let mut v = Vec::new();

                    for s in &different_values {
                        if s == baseline {
                            continue;
                        }

                        v.push((s.1.clone(), s.0.clone()));
                    }

                    FieldOptimization::Baseline(baseline.0.clone(), v)
                } else {
                    FieldOptimization::None
                }
            };

            field_optimizations.insert(field_name.to_string(), optimization);
        }

        Self {
            field_optimizations,
            type_optimizations,
        }
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

    pub fn integer_size(&self) -> Option<IntegerSize> {
        Some(match self.value {
            Value::Int(_) => IntegerSize::I32,
            Value::Int64(_) => IntegerSize::I64,
            Value::Uint(_) => IntegerSize::U32,
            Value::Uint64(_) => IntegerSize::U64,
            _ => return None,
        })
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

    VanillaPageTextMaterial(vanilla_base::PageTextMaterial),
    TbcWrathPageTextMaterial(shared_base::page_text_material_tbc_wrath::PageTextMaterial),

    VanillaLanguage(vanilla_base::Language),
    TbcWrathLanguage(shared_base::language_tbc_wrath::Language),

    VanillaItemSet(vanilla_base::ItemSet),
    TbcItemSet(tbc_base::ItemSet),
    WrathItemSet(wrath_base::ItemSet),

    VanillaFaction(vanilla_base::Faction),
    TbcFaction(tbc_base::Faction),
    WrathFaction(wrath_base::Faction),
}

impl std::hash::Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::String(v) => v.hash(state),
            Value::Int(v) => v.hash(state),
            Value::Int64(v) => v.hash(state),
            Value::Uint(v) => v.hash(state),
            Value::Uint64(v) => v.hash(state),
            Value::Float(v) => u32::from_le_bytes(v.to_le_bytes()).hash(state),
            Value::VanillaItemClassAndSubClass(v) => v.hash(state),
            Value::TbcItemClassAndSubClass(v) => v.hash(state),
            Value::WrathItemClassAndSubClass(v) => v.hash(state),
            Value::VanillaTbcItemQuality(v) => v.hash(state),
            Value::WrathItemQuality(v) => v.hash(state),
            Value::InventoryType(v) => v.hash(state),
            Value::VanillaTbcAllowedClass(v) => v.hash(state),
            Value::WrathAllowedClass(v) => v.hash(state),
            Value::VanillaAllowedRace(v) => v.hash(state),
            Value::TbcAllowedRace(v) => v.hash(state),
            Value::WrathAllowedRace(v) => v.hash(state),
            Value::SpellSchool(v) => v.hash(state),
            Value::VanillaSpellTriggerType(v) => v.hash(state),
            Value::TbcWrathSpellTriggerType(v) => v.hash(state),
            Value::Bonding(v) => v.hash(state),
            Value::VanillaSkill(v) => v.hash(state),
            Value::TbcSkill(v) => v.hash(state),
            Value::WrathSkill(v) => v.hash(state),
            Value::VanillaMap(v) => v.hash(state),
            Value::TbcMap(v) => v.hash(state),
            Value::WrathMap(v) => v.hash(state),
            Value::VanillaArea(v) => v.hash(state),
            Value::TbcArea(v) => v.hash(state),
            Value::WrathArea(v) => v.hash(state),
            Value::VanillaAttributes(v) => v.hash(state),
            Value::VanillaAttributesEx1(v) => v.hash(state),
            Value::VanillaAttributesEx2(v) => v.hash(state),
            Value::VanillaAttributesEx3(v) => v.hash(state),
            Value::VanillaAttributesEx4(v) => v.hash(state),
            Value::VanillaItemFlag(v) => v.hash(state),
            Value::TbcItemFlag(v) => v.hash(state),
            Value::WrathItemFlag(v) => v.hash(state),
            Value::WrathItemFlag2(v) => v.hash(state),
            Value::PvpRank(v) => v.hash(state),
            Value::SheatheType(v) => v.hash(state),
            Value::VanillaBagFamily(v) => v.hash(state),
            Value::TbcWrathBagFamily(v) => v.hash(state),
            Value::VanillaPageTextMaterial(v) => v.hash(state),
            Value::TbcWrathPageTextMaterial(v) => v.hash(state),
            Value::VanillaLanguage(v) => v.hash(state),
            Value::TbcWrathLanguage(v) => v.hash(state),
            Value::VanillaItemSet(v) => v.hash(state),
            Value::TbcItemSet(v) => v.hash(state),
            Value::WrathItemSet(v) => v.hash(state),
            Value::VanillaFaction(v) => v.hash(state),
            Value::TbcFaction(v) => v.hash(state),
            Value::WrathFaction(v) => v.hash(state),
        }
    }
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

    pub fn u64_value(&self) -> Option<u64> {
        match self {
            Value::Uint(v) => Some((*v).into()),
            Value::Uint64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn i64_value(&self) -> Option<i64> {
        match self {
            Value::Int(v) => Some((*v).into()),
            Value::Int64(v) => Some(*v),
            _ => None,
        }
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
            Value::VanillaPageTextMaterial(_) | Value::TbcWrathPageTextMaterial(_) => {
                "PageTextMaterial"
            }
            Value::VanillaLanguage(_) | Value::TbcWrathLanguage(_) => "Language",

            Value::VanillaItemSet(_) | Value::TbcItemSet(_) | Value::WrathItemSet(_) => "ItemSet",
            Value::VanillaFaction(_) | Value::TbcFaction(_) | Value::WrathFaction(_) => "Faction",
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
            Value::VanillaPageTextMaterial(v) => {
                format!("PageTextMaterial::{v:?}")
            }
            Value::TbcWrathPageTextMaterial(v) => {
                format!("PageTextMaterial::{v:?}")
            }
            Value::VanillaLanguage(v) => format!("Language::{v:?}"),
            Value::TbcWrathLanguage(v) => format!("Language::{v:?}"),
            Value::VanillaItemSet(v) => format!("ItemSet::{v:?}"),
            Value::TbcItemSet(v) => format!("ItemSet::{v:?}"),
            Value::WrathItemSet(v) => format!("ItemSet::{v:?}"),

            Value::VanillaFaction(v) => format!("Faction::{v:?}"),
            Value::TbcFaction(v) => format!("Faction::{v:?}"),
            Value::WrathFaction(v) => format!("Faction::{v:?}"),
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
            Value::VanillaPageTextMaterial(_) => {
                Value::VanillaPageTextMaterial(vanilla_base::PageTextMaterial::None)
            }
            Value::TbcWrathPageTextMaterial(_) => Value::TbcWrathPageTextMaterial(
                shared_base::page_text_material_tbc_wrath::PageTextMaterial::None,
            ),
            Value::VanillaLanguage(_) => Value::VanillaLanguage(vanilla_base::Language::Universal),
            Value::TbcWrathLanguage(_) => {
                Value::TbcWrathLanguage(shared_base::language_tbc_wrath::Language::Universal)
            }
            Value::VanillaItemSet(_) => Value::VanillaItemSet(Default::default()),
            Value::TbcItemSet(_) => Value::TbcItemSet(Default::default()),
            Value::WrathItemSet(_) => Value::WrathItemSet(Default::default()),

            Value::VanillaFaction(_) => Value::VanillaFaction(Default::default()),
            Value::TbcFaction(_) => Value::TbcFaction(Default::default()),
            Value::WrathFaction(_) => Value::WrathFaction(Default::default()),
        }
    }
}

fn i32_to_u32(v: i32) -> u32 {
    u32::from_le_bytes(v.to_le_bytes())
}

pub(crate) fn get_items(
    conn: &Connection,
    expansion: Expansion,
) -> (Vec<GenericThing>, Optimizations) {
    match expansion {
        Expansion::Vanilla => vanilla::vanilla(conn),
        Expansion::BurningCrusade => tbc::tbc(conn),
        Expansion::WrathOfTheLichKing => wrath::wrath(conn),
    }
}
