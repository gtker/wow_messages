use crate::base_printer::Expansion;
use hashbrown::HashMap;
use ordered_float::OrderedFloat;
use rusqlite::Connection;
use std::cmp::Ordering;
use std::collections::BTreeSet;

pub mod tbc;
pub mod vanilla;
pub mod wrath;

use crate::base_printer::write::items::GenericThing;

use crate::float_format;
use wow_world_base::shared as shared_base;
use wow_world_base::tbc as tbc_base;
use wow_world_base::vanilla as vanilla_base;
use wow_world_base::wrath as wrath_base;

#[derive(Debug, Clone)]
pub enum FieldOptimization {
    None,
    ConstantValue(Value),
    Baseline(Value, Vec<(BTreeSet<u32>, Value)>),
}

const OPTIMIZATION_BASELINE_DEVIATIONS: usize = 100;

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
    field_optimizations: Vec<FieldOptimization>,
    type_optimizations: Vec<Option<IntegerSize>>,
}

impl Optimizations {
    pub fn optimization(&self, index: usize) -> &FieldOptimization {
        &self.field_optimizations[index]
    }

    pub fn is_non_native_type(&self, field_index: usize) -> bool {
        self.integer_size(field_index).is_some()
    }

    pub fn integer_size(&self, index: usize) -> Option<IntegerSize> {
        self.type_optimizations[index]
    }

    pub fn native_integer_type_cast(&self, index: usize) -> Option<&'static str> {
        if let Some(g) = self.integer_size(index) {
            Some(match g {
                IntegerSize::U16 | IntegerSize::U8 => "u32",
                IntegerSize::I16 | IntegerSize::I8 => "i32",
                _ => return None,
            })
        } else {
            None
        }
    }

    pub fn type_name(&self, field: &Field, field_index: usize) -> &'static str {
        if let Some(t) = self.integer_size(field_index) {
            t.string_value()
        } else {
            field.value.type_name()
        }
    }
}

impl Optimizations {
    pub fn new(items: &[GenericThing], fields: &[Field]) -> Self {
        let mut field_optimizations = Vec::with_capacity(fields.len());
        let mut type_optimizations = Vec::with_capacity(fields.len());

        for (field_index, field) in fields.iter().enumerate() {
            let fields = items
                .iter()
                .map(|item| (item.entry, &item.fields[field_index].value));

            let mut unsigned_max = 0;
            let mut signed_min = 0;
            let mut signed_max = 0;

            let mut different_values: HashMap<Value, BTreeSet<u32>> = HashMap::new();
            for (item_id, field) in fields {
                if let Some(v) = field.i64_value() {
                    if v > signed_max {
                        signed_max = v;
                    }
                    if v < signed_min {
                        signed_min = v;
                    }
                } else if let Some(v) = field.u64_value() {
                    if v > unsigned_max {
                        unsigned_max = v;
                    }
                }

                if let Some(v) = different_values.get_mut(field) {
                    v.insert(item_id);
                } else {
                    let mut v = BTreeSet::new();
                    v.insert(item_id);
                    different_values.insert(field.clone(), v);
                }
            }

            if field.value.i64_value().is_some() {
                type_optimizations.push(Some(IntegerSize::from_signed(signed_min, signed_max)));
            } else if field.value.u64_value().is_some() {
                type_optimizations.push(Some(IntegerSize::from_unsigned(unsigned_max)));
            } else {
                type_optimizations.push(None);
            }

            let optimization = if different_values.len() == 1 {
                FieldOptimization::ConstantValue(field.value.clone())
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
                        if s.0 == baseline.0 {
                            continue;
                        }

                        v.push((s.1.clone(), s.0.clone()));
                    }

                    v.sort_by(|a, b| a.0.cmp(&b.0));

                    FieldOptimization::Baseline(baseline.0.clone(), v)
                } else {
                    FieldOptimization::None
                }
            };

            field_optimizations.push(optimization);
        }

        Self {
            field_optimizations,
            type_optimizations,
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Array {
    pub variable_name: &'static str,
    pub type_name: &'static str,
    pub instances: ArrayInstances,
    pub import_only: bool,
}

impl Array {
    pub const fn new(
        variable_name: &'static str,
        type_name: &'static str,
        import_only: bool,
        instances: ArrayInstances,
    ) -> Self {
        Self {
            variable_name,
            type_name,
            instances,
            import_only,
        }
    }

    pub fn field_info(&self) -> &ArrayFieldInfo {
        &self.instances.field_info
    }

    pub fn array_length(&self) -> usize {
        self.instances.array_length
    }

    pub fn is_default(&self) -> bool {
        self.instances.instances().iter().all(|a| a.is_default())
    }
}

impl Eq for Array {}

#[allow(clippy::derive_ord_xor_partial_ord)] // f32 can not derive Ord
impl Ord for Array {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ArrayFieldInfo {
    fields: Vec<ArrayField>,
}

impl ArrayFieldInfo {
    pub fn new(fields: Vec<ArrayField>) -> Self {
        Self { fields }
    }

    pub fn fields(&self) -> &[ArrayField] {
        &self.fields
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ArrayInstances {
    instances: Vec<ArrayInstance>,
    field_info: ArrayFieldInfo,
    array_length: usize,
}

impl ArrayInstances {
    pub fn new(instances: Vec<ArrayInstance>) -> Self {
        let field_info = ArrayFieldInfo::new(instances[0].fields.to_vec());
        let array_length = instances.len();

        let instances = instances.into_iter().filter(|a| !a.is_default()).collect();

        Self {
            instances,
            field_info,
            array_length,
        }
    }

    pub fn instances(&self) -> &[ArrayInstance] {
        &self.instances
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ArrayInstance {
    is_default: bool,
    fields: Vec<ArrayField>,
}

impl ArrayInstance {
    pub fn new(is_default: bool, fields: Vec<ArrayField>) -> Self {
        Self { is_default, fields }
    }

    pub fn default_values(fields: Vec<ArrayField>) -> Self {
        let is_default = fields.iter().all(|a| a.is_default());
        Self { is_default, fields }
    }

    pub fn fields(&self) -> &[ArrayField] {
        &self.fields
    }

    pub fn is_default(&self) -> bool {
        self.is_default
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Value {
    String(String),
    Int(i32),
    Int64(i64),
    Uint(u32),
    Uint64(u64),
    Float(OrderedFloat<f32>),

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

    SpellSchool(shared_base::spell_school_vanilla_tbc_wrath::SpellSchool),

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

    VanillaAuraMod(vanilla_base::AuraMod),
    TbcAuraMod(tbc_base::AuraMod),
    WrathAuraMod(wrath_base::AuraMod),

    Gold(shared_base::gold_vanilla_tbc_wrath::Gold),
}

impl Value {
    pub fn float(v: f32) -> Self {
        Self::Float(OrderedFloat(v))
    }

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

    pub fn const_value(&self) -> Self {
        match self {
            Value::VanillaTbcAllowedClass(v) => Self::Uint(v.as_int()),
            Value::WrathAllowedClass(v) => Self::Uint(v.as_int()),
            Value::VanillaAllowedRace(v) => Self::Uint(v.as_int()),
            Value::TbcAllowedRace(v) => Self::Uint(v.as_int()),
            Value::WrathAllowedRace(v) => Self::Uint(v.as_int()),
            Value::VanillaAttributes(v) => Self::Uint(v.as_int()),
            Value::VanillaAttributesEx1(v) => Self::Uint(v.as_int()),
            Value::VanillaAttributesEx2(v) => Self::Uint(v.as_int()),
            Value::VanillaAttributesEx3(v) => Self::Uint(v.as_int()),
            Value::VanillaAttributesEx4(v) => Self::Uint(v.as_int()),
            Value::VanillaItemFlag(v) => Self::Uint(v.as_int()),
            Value::TbcItemFlag(v) => Self::Uint(v.as_int()),
            Value::WrathItemFlag(v) => Self::Uint(v.as_int()),
            Value::WrathItemFlag2(v) => Self::Uint(v.as_int()),
            Value::TbcWrathBagFamily(v) => Self::Uint(v.as_int()),
            Value::Gold(v) => Self::Uint(v.as_int()),
            _ => self.clone(),
        }
    }

    pub fn constructor_type_name(&self) -> &'static str {
        self.const_value().type_name()
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
            Value::VanillaAuraMod(_) | Value::TbcAuraMod(_) | Value::WrathAuraMod(_) => "AuraMod",
            Value::Gold(_) => "Gold",
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
        match self {
            Value::String(v) => format!("\"{}\"", v.replace('"', "\\\"")),
            Value::Int(v) => (*v).to_string(),
            Value::Int64(v) => (*v).to_string(),
            Value::Uint(v) => (*v).to_string(),
            Value::Uint64(v) => (*v).to_string(),
            Value::Float(v) => float_format(v.into_inner()),
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
            Value::Gold(v) => v.as_int().to_string(),
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
            Value::VanillaAuraMod(v) => format!("AuraMod::{v:?}"),
            Value::TbcAuraMod(v) => format!("AuraMod::{v:?}"),
            Value::WrathAuraMod(v) => format!("AuraMod::{v:?}"),
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
            | Value::Gold(_)
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
            Value::Float(_) => Value::float(0.0),
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
            Value::SpellSchool(_) => {
                Value::SpellSchool(shared_base::spell_school_vanilla_tbc_wrath::SpellSchool::Normal)
            }
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
            Value::VanillaAuraMod(_) => Value::VanillaAuraMod(Default::default()),
            Value::TbcAuraMod(_) => Value::TbcAuraMod(Default::default()),
            Value::WrathAuraMod(_) => Value::WrathAuraMod(Default::default()),
            Value::Gold(_) => Value::Gold(Default::default()),
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

pub fn process_extra_flags(entry: u32, mut extra_flags: i32, name: &str) -> i32 {
    const UNOBTAINABLE_FLAG: i32 = 0x04;
    let unobtainable_flag_is_set = extra_flags & UNOBTAINABLE_FLAG != 0;

    let name_ends_with_deprecated = name.ends_with("DEPRECATED") || name.ends_with("DEP");
    let name_ends_with_test = name.ends_with(" Test") || name.ends_with("(Test)");

    let name_starts_with_old = name.starts_with("OLD") || name.starts_with("(OLD)");
    let name_starts_with_monster = name.starts_with("Monster - ");
    let name_starts_with_test = name.starts_with("TEST ");
    let name_starts_with_deprecated = name.starts_with("Deprecated");

    let name_contains_ph = name.contains("[PH]");

    let martin_thunder_or_martin_fury = entry == 17 || entry == 192;

    let glaive_of_the_defender = entry == 23051;

    let warglaives_of_azzinoth = entry == 18582 || entry == 18583 || entry == 18584;

    let unobtainable = unobtainable_flag_is_set
        || name_ends_with_deprecated
        || name_starts_with_old
        || name_ends_with_test
        || name_starts_with_monster
        || name_starts_with_test
        || name_starts_with_deprecated
        || name_contains_ph
        || martin_thunder_or_martin_fury
        || glaive_of_the_defender
        || warglaives_of_azzinoth;

    if unobtainable {
        extra_flags |= UNOBTAINABLE_FLAG;
    }

    extra_flags
}
