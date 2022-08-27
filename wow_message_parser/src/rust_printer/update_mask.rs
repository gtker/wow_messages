use crate::file_utils::{overwrite_if_not_same_contents, UPDATE_MASK_LOCATION};
use crate::parser::types::tags::Tag;
use crate::rust_printer::Writer;
use crate::Tags;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::Path;

pub fn print_update_mask_docs() {
    const UPDATE_MASK_FILE: &str = "wowm_language/src/spec/update-mask.md";
    const LOOKUP_TABLE: &str = "## Lookup Table\n";
    let contents = read_to_string(UPDATE_MASK_FILE).unwrap();

    let update_types = [
        ("object", UpdateMaskType::Object),
        ("item", UpdateMaskType::Item),
        ("container", UpdateMaskType::Container),
        ("unit", UpdateMaskType::Unit),
        ("player", UpdateMaskType::Player),
        ("gameobject", UpdateMaskType::GameObject),
        ("dynamicobject", UpdateMaskType::DynamicObject),
        ("corpse", UpdateMaskType::Corpse),
    ];

    let (s, _) = contents.split_once(LOOKUP_TABLE).unwrap();
    let mut s = s.to_string();
    s.push_str(LOOKUP_TABLE);
    s.push_str("Taken from [vmangos](https://github.com/vmangos/core/blob/4b2a5173b0ca4917dfe91aa7b87d84232fd7203c/src/game/Objects/UpdateFields_1_12_1.cpp#L5) with some modifications.\n\n");

    for (name, u) in update_types {
        writeln!(s, "Fields that all {}s have:\n", name).unwrap();

        s.push_str("| Name | Offset | Size | Type |\n");
        s.push_str("|------|--------|------|------|\n");
        for field in FIELDS {
            if field.object_ty == u {
                let ty = match field.ty {
                    UfType::Guid => "GUID",
                    UfType::Int => "INT",
                    UfType::Float => "FLOAT",
                    UfType::BytesWithNames(_, _, _, _)
                    | UfType::Bytes
                    | UfType::BytesWithTypes(_, _, _, _) => "BYTES",
                    UfType::TwoShort => "TWO_SHORT",
                };

                writeln!(
                    s,
                    "|`{}_{}`| 0x{:04x?} | {} | {} |",
                    name.to_uppercase(),
                    field.name(),
                    field.offset,
                    field.size,
                    ty
                )
                .unwrap();

                if field.offset == 0 && field.ty == UfType::Guid {
                    writeln!(
                        s,
                        "|`{}_TYPE`| 0x{:04x?} | {} | INT |",
                        name.to_uppercase(),
                        2,
                        1,
                    )
                    .unwrap();
                }
            }
        }

        s.push_str("\n\n");
    }

    overwrite_if_not_same_contents(&s, Path::new(UPDATE_MASK_FILE));
}

pub fn print_update_mask() {
    print_update_mask_docs();

    let update_types = [
        (
            "UpdateItem",
            vec![UpdateMaskType::Object, UpdateMaskType::Item],
        ),
        (
            "UpdateContainer",
            vec![
                UpdateMaskType::Object,
                UpdateMaskType::Item,
                UpdateMaskType::Container,
            ],
        ),
        (
            "UpdateUnit",
            vec![UpdateMaskType::Object, UpdateMaskType::Unit],
        ),
        (
            "UpdatePlayer",
            vec![
                UpdateMaskType::Object,
                UpdateMaskType::Unit,
                UpdateMaskType::Player,
            ],
        ),
        (
            "UpdateGameObject",
            vec![UpdateMaskType::Object, UpdateMaskType::GameObject],
        ),
        (
            "UpdateDynamicObject",
            vec![UpdateMaskType::Object, UpdateMaskType::DynamicObject],
        ),
        (
            "UpdateCorpse",
            vec![UpdateMaskType::Object, UpdateMaskType::Corpse],
        ),
    ];

    let mut s = Writer::new("");
    s.wln("use crate::Guid;");
    for m in FIELDS {
        if let UfType::BytesWithTypes(a, b, c, d) = m.ty() {
            s.wln(format!(
                // The UpdateMask implementation realistically only works with 1.12
                "use crate::vanilla::{{{}, {}, {}, {}}};",
                a, b, c, d
            ));

            let mut tags = Tags::new();
            tags.push(Tag::new("versions", "1.12"));
        }
    }

    s.wln("use crate::helper::update_mask::{UpdateContainer, UpdateCorpse, UpdateDynamicObject, UpdateGameObject, UpdateItem, UpdateMask, UpdatePlayer, UpdateUnit};");
    s.newline();

    for (ty, types) in update_types {
        s.bodyn(format!("impl {}", ty), |s| {
            for m in FIELDS.iter().filter(|a| types.contains(&a.object_ty)) {
                print_functions(s, m);
            }
        });
    }

    overwrite_if_not_same_contents(s.inner(), Path::new(UPDATE_MASK_LOCATION));
}

fn print_functions(s: &mut Writer, m: &MemberType) {
    s.open_curly(format!(
        "pub fn set_{}_{}(mut self, {}) -> Self",
        m.object_ty,
        m.name,
        m.ty.parameter_str(),
    ));

    s.wln(format!("self.header_set({});", m.offset));
    match m.ty {
        UfType::Guid => {
            s.wln(format!("self.header_set({});", m.offset + 1));

            s.wln(format!(
                "self.values.insert({}, v.guid() as u32);",
                m.offset
            ));
            s.wln(format!(
                "self.values.insert({}, (v.guid() >> 32) as u32);",
                m.offset + 1
            ));
        }
        _ => {
            let value = match m.ty {
                UfType::Int => "v as u32".to_string(),
                UfType::Float => "u32::from_le_bytes(v.to_le_bytes())".to_string(),
                UfType::Bytes => "u32::from_le_bytes([a, b, c, d])".to_string(),
                UfType::TwoShort => "v".to_string(),
                UfType::BytesWithTypes(_, _, _, _) => {
                    let mut tags = Tags::new();
                    tags.push(Tag::new("versions", "1.12"));

                    let get_name = |variable_name| -> String {
                        format!("{name}.as_int()", name = variable_name)
                    };

                    let a = get_name("a");
                    let b = get_name("b");
                    let c = get_name("c");
                    let d = get_name("d");

                    format!(
                        "u32::from_le_bytes([{a}, {b}, {c}, {d}])",
                        a = a,
                        b = b,
                        c = c,
                        d = d
                    )
                }
                UfType::BytesWithNames(a, b, c, d) => {
                    format!("u32::from_le_bytes([{}, {}, {}, {}])", a, b, c, d)
                }
                _ => unreachable!(),
            };

            s.wln(format!("self.values.insert({}, {});", m.offset, value));
        }
    }

    s.wln("self");
    s.closing_curly_newline(); // pub fn set_
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum UpdateMaskType {
    Object,
    Item,
    Unit,
    Player,
    Container,
    GameObject,
    DynamicObject,
    Corpse,
}

impl Display for UpdateMaskType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UpdateMaskType::Object => "object",
            UpdateMaskType::Item => "item",
            UpdateMaskType::Unit => "unit",
            UpdateMaskType::Player => "player",
            UpdateMaskType::Container => "container",
            UpdateMaskType::GameObject => "gameobject",
            UpdateMaskType::DynamicObject => "dynamicobject",
            UpdateMaskType::Corpse => "corpse",
        })
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum UfType {
    Guid,
    Int,
    Float,
    Bytes,
    BytesWithNames(&'static str, &'static str, &'static str, &'static str),
    BytesWithTypes(&'static str, &'static str, &'static str, &'static str),
    TwoShort,
}

const GUID_TYPE: &str = "Guid";
const INT_TYPE: &str = "i32";
const FLOAT_TYPE: &str = "f32";
const TWO_SHORT_TYPE: &str = "u32";

impl UfType {
    pub fn parameter_str(&self) -> String {
        format!(
            "v: {}",
            match self {
                UfType::Guid => GUID_TYPE,
                UfType::Int => INT_TYPE,
                UfType::Float => FLOAT_TYPE,
                UfType::TwoShort => TWO_SHORT_TYPE,
                UfType::Bytes => {
                    return "a: u8, b: u8, c: u8, d: u8".to_string();
                }
                UfType::BytesWithTypes(a, b, c, d) => {
                    return format!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
                }
                UfType::BytesWithNames(a, b, c, d) => {
                    return format!("{}: u8, {}: u8, {}: u8, {}: u8", a, b, c, d);
                }
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct MemberType {
    object_ty: UpdateMaskType,
    name: &'static str,
    offset: i32,
    size: i32,
    ty: UfType,
}

impl MemberType {
    const fn new(ty: UpdateMaskType, s: &'static str, offset: i32, size: i32, uf: UfType) -> Self {
        Self {
            object_ty: ty,
            name: s,
            offset,
            size,
            ty: uf,
        }
    }
    pub fn object_ty(&self) -> UpdateMaskType {
        self.object_ty
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn ty(&self) -> UfType {
        self.ty
    }
}

pub const FIELDS: [MemberType; 296] = [
    MemberType::new(UpdateMaskType::Object, "GUID", 0x0, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Object, "ENTRY", 0x3, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Object, "SCALE_X", 0x4, 1, UfType::Float),
    MemberType::new(UpdateMaskType::Item, "OWNER", 0x6, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Item, "CONTAINED", 0x8, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Item, "CREATOR", 0xA, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Item, "GIFTCREATOR", 0xC, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Item, "STACK_COUNT", 0xE, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Item, "DURATION", 0xF, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Item, "SPELL_CHARGES", 0x10, 5, UfType::Int),
    MemberType::new(UpdateMaskType::Item, "FLAGS", 0x15, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Item, "ENCHANTMENT", 0x16, 21, UfType::Int),
    MemberType::new(UpdateMaskType::Item, "PROPERTY_SEED", 0x2B, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Item,
        "RANDOM_PROPERTIES_ID",
        0x2C,
        1,
        UfType::Int,
    ),
    MemberType::new(UpdateMaskType::Item, "ITEM_TEXT_ID", 0x2D, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Item, "DURABILITY", 0x2E, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Item, "MAXDURABILITY", 0x2F, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Container, "NUM_SLOTS", 0x30, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Container, "SLOT_1", 0x32, 72, UfType::Guid),
    MemberType::new(UpdateMaskType::Unit, "CHARM", 0x6, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Unit, "SUMMON", 0x8, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Unit, "CHARMEDBY", 0xA, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Unit, "SUMMONEDBY", 0xC, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Unit, "CREATEDBY", 0xE, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Unit, "TARGET", 0x10, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Unit, "PERSUADED", 0x12, 2, UfType::Guid),
    MemberType::new(
        UpdateMaskType::Unit,
        "CHANNEL_OBJECT",
        0x14,
        2,
        UfType::Guid,
    ),
    MemberType::new(UpdateMaskType::Unit, "HEALTH", 0x16, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "POWER1", 0x17, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "POWER2", 0x18, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "POWER3", 0x19, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "POWER4", 0x1A, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "POWER5", 0x1B, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "MAXHEALTH", 0x1C, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "MAXPOWER1", 0x1D, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "MAXPOWER2", 0x1E, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "MAXPOWER3", 0x1F, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "MAXPOWER4", 0x20, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "MAXPOWER5", 0x21, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "LEVEL", 0x22, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Unit,
        "FACTIONTEMPLATE",
        0x23,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "BYTES_0",
        0x24,
        1,
        UfType::BytesWithTypes("Race", "Class", "Gender", "Power"),
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "VIRTUAL_ITEM_SLOT_DISPLAY",
        0x25,
        3,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "VIRTUAL_ITEM_INFO",
        0x28,
        6,
        UfType::Bytes,
    ),
    MemberType::new(UpdateMaskType::Unit, "FLAGS", 0x2E, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "AURA", 0x2F, 48, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "AURAFLAGS", 0x5F, 6, UfType::Bytes),
    MemberType::new(UpdateMaskType::Unit, "AURALEVELS", 0x65, 12, UfType::Bytes),
    MemberType::new(
        UpdateMaskType::Unit,
        "AURAAPPLICATIONS",
        0x71,
        12,
        UfType::Bytes,
    ),
    MemberType::new(UpdateMaskType::Unit, "AURASTATE", 0x7D, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "BASEATTACKTIME", 0x7E, 2, UfType::Int),
    MemberType::new(
        UpdateMaskType::Unit,
        "RANGEDATTACKTIME",
        0x80,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "BOUNDINGRADIUS",
        0x81,
        1,
        UfType::Float,
    ),
    MemberType::new(UpdateMaskType::Unit, "COMBATREACH", 0x82, 1, UfType::Float),
    MemberType::new(UpdateMaskType::Unit, "DISPLAYID", 0x83, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Unit,
        "NATIVEDISPLAYID",
        0x84,
        1,
        UfType::Int,
    ),
    MemberType::new(UpdateMaskType::Unit, "MOUNTDISPLAYID", 0x85, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "MINDAMAGE", 0x86, 1, UfType::Float),
    MemberType::new(UpdateMaskType::Unit, "MAXDAMAGE", 0x87, 1, UfType::Float),
    MemberType::new(
        UpdateMaskType::Unit,
        "MINOFFHANDDAMAGE",
        0x88,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "MAXOFFHANDDAMAGE",
        0x89,
        1,
        UfType::Float,
    ),
    MemberType::new(UpdateMaskType::Unit, "BYTES_1", 0x8A, 1, UfType::Bytes),
    MemberType::new(UpdateMaskType::Unit, "PETNUMBER", 0x8B, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Unit,
        "PET_NAME_TIMESTAMP",
        0x8C,
        1,
        UfType::Int,
    ),
    MemberType::new(UpdateMaskType::Unit, "PETEXPERIENCE", 0x8D, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Unit,
        "PETNEXTLEVELEXP",
        0x8E,
        1,
        UfType::Int,
    ),
    MemberType::new(UpdateMaskType::Unit, "DYNAMIC_FLAGS", 0x8F, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "CHANNEL_SPELL", 0x90, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Unit,
        "MOD_CAST_SPEED",
        0x91,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "CREATED_BY_SPELL",
        0x92,
        1,
        UfType::Int,
    ),
    MemberType::new(UpdateMaskType::Unit, "NPC_FLAGS", 0x93, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "NPC_EMOTESTATE", 0x94, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Unit,
        "TRAINING_POINTS",
        0x95,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(UpdateMaskType::Unit, "STRENGTH", 0x96, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "AGILITY", 0x97, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "STAMINA", 0x98, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "INTELLECT", 0x99, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "SPIRIT", 0x9A, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Unit,
        "NORMAL_RESISTANCE",
        0x9B,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "HOLY_RESISTANCE",
        0x9C,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "FIRE_RESISTANCE",
        0x9D,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "NATURE_RESISTANCE",
        0x9E,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "FROST_RESISTANCE",
        0x9F,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "SHADOW_RESISTANCE",
        0xA0,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "ARCANE_RESISTANCE",
        0xA1,
        1,
        UfType::Int,
    ),
    MemberType::new(UpdateMaskType::Unit, "BASE_MANA", 0xA2, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Unit, "BASE_HEALTH", 0xA3, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Unit,
        "BYTES_2",
        0xA4,
        1,
        UfType::BytesWithNames("facial_hair", "unknown", "bank_bag_slots", "rested_state"),
    ),
    MemberType::new(UpdateMaskType::Unit, "ATTACK_POWER", 0xA5, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Unit,
        "ATTACK_POWER_MODS",
        0xA6,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "ATTACK_POWER_MULTIPLIER",
        0xA7,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "RANGED_ATTACK_POWER",
        0xA8,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "RANGED_ATTACK_POWER_MODS",
        0xA9,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "RANGED_ATTACK_POWER_MULTIPLIER",
        0xAA,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "MINRANGEDDAMAGE",
        0xAB,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "MAXRANGEDDAMAGE",
        0xAC,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "POWER_COST_MODIFIER",
        0xAD,
        7,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Unit,
        "POWER_COST_MULTIPLIER",
        0xB4,
        7,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "DUEL_ARBITER",
        0xBC,
        2,
        UfType::Guid,
    ),
    MemberType::new(UpdateMaskType::Player, "FLAGS", 0xBE, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Player, "GUILDID", 0xBF, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Player, "GUILDRANK", 0xC0, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Player, "FEATURES", 0xC1, 1, UfType::Bytes),
    MemberType::new(UpdateMaskType::Player, "BYTES_2", 0xC2, 1, UfType::Bytes),
    MemberType::new(UpdateMaskType::Player, "BYTES_3", 0xC3, 1, UfType::Bytes),
    MemberType::new(UpdateMaskType::Player, "DUEL_TEAM", 0xC4, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Player,
        "GUILD_TIMESTAMP",
        0xC5,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_1_1",
        0xC6,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_1_2",
        0xC7,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_2_1",
        0xC9,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_2_2",
        0xCA,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_3_1",
        0xCC,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_3_2",
        0xCD,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_4_1",
        0xCF,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_4_2",
        0xD0,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_5_1",
        0xD2,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_5_2",
        0xD3,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_6_1",
        0xD5,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_6_2",
        0xD6,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_7_1",
        0xD8,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_7_2",
        0xD9,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_8_1",
        0xDB,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_8_2",
        0xDC,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_9_1",
        0xDE,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_9_2",
        0xDF,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_10_1",
        0xE1,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_10_2",
        0xE2,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_11_1",
        0xE4,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_11_2",
        0xE5,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_12_1",
        0xE7,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_12_2",
        0xE8,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_13_1",
        0xEA,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_13_2",
        0xEB,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_14_1",
        0xED,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_14_2",
        0xEE,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_15_1",
        0xF0,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_15_2",
        0xF1,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_16_1",
        0xF3,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_16_2",
        0xF4,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_17_1",
        0xF6,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_17_2",
        0xF7,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_18_1",
        0xF9,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_18_2",
        0xFA,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_19_1",
        0xFC,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_19_2",
        0xFD,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_20_1",
        0xFF,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "QUEST_LOG_20_2",
        0x100,
        2,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_1_CREATOR",
        0x102,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_1_0",
        0x104,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_1_PROPERTIES",
        0x10C,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_2_CREATOR",
        0x10E,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_2_0",
        0x110,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_2_PROPERTIES",
        0x118,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_3_CREATOR",
        0x11A,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_3_0",
        0x11C,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_3_PROPERTIES",
        0x124,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_4_CREATOR",
        0x126,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_4_0",
        0x128,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_4_PROPERTIES",
        0x130,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_5_CREATOR",
        0x132,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_5_0",
        0x134,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_5_PROPERTIES",
        0x13C,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_6_CREATOR",
        0x13E,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_6_0",
        0x140,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_6_PROPERTIES",
        0x148,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_7_CREATOR",
        0x14A,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_7_0",
        0x14C,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_7_PROPERTIES",
        0x154,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_8_CREATOR",
        0x156,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_8_0",
        0x158,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_8_PROPERTIES",
        0x160,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_9_CREATOR",
        0x162,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_9_0",
        0x164,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_9_PROPERTIES",
        0x16C,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_10_CREATOR",
        0x16E,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_10_0",
        0x170,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_10_PROPERTIES",
        0x178,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_11_CREATOR",
        0x17A,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_11_0",
        0x17C,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_11_PROPERTIES",
        0x184,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_12_CREATOR",
        0x186,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_12_0",
        0x188,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_12_PROPERTIES",
        0x190,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_13_CREATOR",
        0x192,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_13_0",
        0x194,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_13_PROPERTIES",
        0x19C,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_14_CREATOR",
        0x19E,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_14_0",
        0x1A0,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_14_PROPERTIES",
        0x1A8,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_15_CREATOR",
        0x1AA,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_15_0",
        0x1AC,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_15_PROPERTIES",
        0x1B4,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_16_CREATOR",
        0x1B6,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_16_0",
        0x1B8,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_16_PROPERTIES",
        0x1C0,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_17_CREATOR",
        0x1C2,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_17_0",
        0x1C4,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_17_PROPERTIES",
        0x1CC,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_18_CREATOR",
        0x1CE,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_18_0",
        0x1D0,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_18_PROPERTIES",
        0x1D8,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_19_CREATOR",
        0x1DA,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_19_0",
        0x1DC,
        8,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "VISIBLE_ITEM_19_PROPERTIES",
        0x1E4,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_INV_SLOT_HEAD",
        0x1E6,
        46,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_PACK_SLOT_1",
        0x214,
        32,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_BANK_SLOT_1",
        0x234,
        48,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_BANKBAG_SLOT_1",
        0x264,
        12,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_VENDORBUYBACK_SLOT_1",
        0x270,
        24,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_KEYRING_SLOT_1",
        0x288,
        64,
        UfType::Guid,
    ),
    MemberType::new(UpdateMaskType::Player, "FARSIGHT", 0x2C8, 2, UfType::Guid),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_COMBO_TARGET",
        0x2CA,
        2,
        UfType::Guid,
    ),
    MemberType::new(UpdateMaskType::Player, "XP", 0x2CC, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Player,
        "NEXT_LEVEL_XP",
        0x2CD,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "SKILL_INFO_1_1",
        0x2CE,
        384,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "CHARACTER_POINTS1",
        0x44E,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "CHARACTER_POINTS2",
        0x44F,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "TRACK_CREATURES",
        0x450,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "TRACK_RESOURCES",
        0x451,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "BLOCK_PERCENTAGE",
        0x452,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "DODGE_PERCENTAGE",
        0x453,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "PARRY_PERCENTAGE",
        0x454,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "CRIT_PERCENTAGE",
        0x455,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "RANGED_CRIT_PERCENTAGE",
        0x456,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "EXPLORED_ZONES_1",
        0x457,
        64,
        UfType::Bytes,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "REST_STATE_EXPERIENCE",
        0x497,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_COINAGE",
        0x498,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_POSSTAT0",
        0x499,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_POSSTAT1",
        0x49A,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_POSSTAT2",
        0x49B,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_POSSTAT3",
        0x49C,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_POSSTAT4",
        0x49D,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_NEGSTAT0",
        0x49E,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_NEGSTAT1",
        0x49F,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_NEGSTAT2",
        0x4A0,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_NEGSTAT3",
        0x4A1,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_NEGSTAT4",
        0x4A2,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_RESISTANCEBUFFMODSPOSITIVE",
        0x4A3,
        7,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_RESISTANCEBUFFMODSNEGATIVE",
        0x4AA,
        7,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_MOD_DAMAGE_DONE_POS",
        0x4B1,
        7,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_MOD_DAMAGE_DONE_NEG",
        0x4B8,
        7,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_MOD_DAMAGE_DONE_PCT",
        0x4BF,
        7,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_BYTES",
        0x4C6,
        1,
        UfType::Bytes,
    ),
    MemberType::new(UpdateMaskType::Player, "AMMO_ID", 0x4C7, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Player,
        "SELF_RES_SPELL",
        0x4C8,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_PVP_MEDALS",
        0x4C9,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_BUYBACK_PRICE_1",
        0x4CA,
        12,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_BUYBACK_TIMESTAMP_1",
        0x4D6,
        12,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_SESSION_KILLS",
        0x4E2,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_YESTERDAY_KILLS",
        0x4E3,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_LAST_WEEK_KILLS",
        0x4E4,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_THIS_WEEK_KILLS",
        0x4E5,
        1,
        UfType::TwoShort,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_THIS_WEEK_CONTRIBUTION",
        0x4E6,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_LIFETIME_HONORBALE_KILLS",
        0x4E7,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_LIFETIME_DISHONORBALE_KILLS",
        0x4E8,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_YESTERDAY_CONTRIBUTION",
        0x4E9,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_LAST_WEEK_CONTRIBUTION",
        0x4EA,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_LAST_WEEK_RANK",
        0x4EB,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_BYTES2",
        0x4EC,
        1,
        UfType::Bytes,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_WATCHED_FACTION_INDEX",
        0x4ED,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::Player,
        "FIELD_COMBAT_RATING_1",
        0x4EE,
        20,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::GameObject,
        "CREATED_BY",
        0x6,
        2,
        UfType::Guid,
    ),
    MemberType::new(UpdateMaskType::GameObject, "DISPLAYID", 0x8, 1, UfType::Int),
    MemberType::new(UpdateMaskType::GameObject, "FLAGS", 0x9, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::GameObject,
        "ROTATION",
        0xA,
        4,
        UfType::Float,
    ),
    MemberType::new(UpdateMaskType::GameObject, "STATE", 0xE, 1, UfType::Int),
    MemberType::new(UpdateMaskType::GameObject, "POS_X", 0xF, 1, UfType::Float),
    MemberType::new(UpdateMaskType::GameObject, "POS_Y", 0x10, 1, UfType::Float),
    MemberType::new(UpdateMaskType::GameObject, "POS_Z", 0x11, 1, UfType::Float),
    MemberType::new(UpdateMaskType::GameObject, "FACING", 0x12, 1, UfType::Float),
    MemberType::new(
        UpdateMaskType::GameObject,
        "DYN_FLAGS",
        0x13,
        1,
        UfType::Int,
    ),
    MemberType::new(UpdateMaskType::GameObject, "FACTION", 0x14, 1, UfType::Int),
    MemberType::new(UpdateMaskType::GameObject, "TYPE_ID", 0x15, 1, UfType::Int),
    MemberType::new(UpdateMaskType::GameObject, "LEVEL", 0x16, 1, UfType::Int),
    MemberType::new(UpdateMaskType::GameObject, "ARTKIT", 0x17, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::GameObject,
        "ANIMPROGRESS",
        0x18,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::DynamicObject,
        "CASTER",
        0x6,
        2,
        UfType::Guid,
    ),
    MemberType::new(
        UpdateMaskType::DynamicObject,
        "BYTES",
        0x8,
        1,
        UfType::Bytes,
    ),
    MemberType::new(
        UpdateMaskType::DynamicObject,
        "SPELLID",
        0x9,
        1,
        UfType::Int,
    ),
    MemberType::new(
        UpdateMaskType::DynamicObject,
        "RADIUS",
        0xA,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::DynamicObject,
        "POS_X",
        0xB,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::DynamicObject,
        "POS_Y",
        0xC,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::DynamicObject,
        "POS_Z",
        0xD,
        1,
        UfType::Float,
    ),
    MemberType::new(
        UpdateMaskType::DynamicObject,
        "FACING",
        0xE,
        1,
        UfType::Float,
    ),
    MemberType::new(UpdateMaskType::Corpse, "OWNER", 0x6, 2, UfType::Guid),
    MemberType::new(UpdateMaskType::Corpse, "FACING", 0x8, 1, UfType::Float),
    MemberType::new(UpdateMaskType::Corpse, "POS_X", 0x9, 1, UfType::Float),
    MemberType::new(UpdateMaskType::Corpse, "POS_Y", 0xA, 1, UfType::Float),
    MemberType::new(UpdateMaskType::Corpse, "POS_Z", 0xB, 1, UfType::Float),
    MemberType::new(UpdateMaskType::Corpse, "DISPLAY_ID", 0xC, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Corpse, "ITEM", 0xD, 19, UfType::Int),
    MemberType::new(UpdateMaskType::Corpse, "BYTES_1", 0x20, 1, UfType::Bytes),
    MemberType::new(UpdateMaskType::Corpse, "BYTES_2", 0x21, 1, UfType::Bytes),
    MemberType::new(UpdateMaskType::Corpse, "GUILD", 0x22, 1, UfType::Int),
    MemberType::new(UpdateMaskType::Corpse, "FLAGS", 0x23, 1, UfType::Int),
    MemberType::new(
        UpdateMaskType::Corpse,
        "DYNAMIC_FLAGS",
        0x24,
        1,
        UfType::Int,
    ),
];
