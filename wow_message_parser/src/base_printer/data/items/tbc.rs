use crate::base_printer::data::get_fields;
use crate::base_printer::data::items::vanilla::assertions;
use crate::base_printer::data::items::{
    process_extra_flags, Array, ArrayField, ArrayInstance, ArrayInstances, Field, Optimizations,
    Value,
};
use crate::base_printer::read_csv_file;
use crate::base_printer::write::items::GenericThing;
use serde::Deserialize;
use std::path::Path;
use wow_world_base::tbc::Skill;

#[derive(Deserialize)]
struct TbcItem {
    pub entry: u32,
    pub name: String,
    pub class: i32,
    pub subclass: i32,
    #[serde(rename = "displayid")]
    pub display_id: u32,
    #[serde(rename = "Quality")]
    pub quality: u32,
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "BuyCount")]
    pub buy_count: i32,
    #[serde(rename = "BuyPrice")]
    pub buy_price: u32,
    #[serde(rename = "SellPrice")]
    pub sell_price: u32,
    #[serde(rename = "InventoryType")]
    pub inventory_type: u32,
    #[serde(rename = "AllowableClass")]
    pub allowed_class: i32,
    #[serde(rename = "AllowableRace")]
    pub allowed_race: i32,
    #[serde(rename = "ItemLevel")]
    pub item_level: i32,
    #[serde(rename = "RequiredLevel")]
    pub required_level: i32,
    #[serde(rename = "RequiredSkill")]
    pub required_skill: u32,
    #[serde(rename = "RequiredSkillRank")]
    pub required_skill_rank: i32,
    #[serde(rename = "requiredspell")]
    pub required_spell: i32,
    #[serde(rename = "requiredhonorrank")]
    pub required_honor_rank: u32,
    #[serde(rename = "RequiredCityRank")]
    pub required_city_rank: i32,
    #[serde(rename = "RequiredReputationFaction")]
    pub required_faction: u32,
    #[serde(rename = "RequiredReputationRank")]
    pub required_reputation_rank: i32,
    #[serde(rename = "maxcount")]
    pub max_count: i32,
    pub stackable: i32,
    #[serde(rename = "ContainerSlots")]
    pub container_slots: i32,

    pub stat_type1: i32,
    pub stat_value1: i32,
    pub stat_type2: i32,
    pub stat_value2: i32,
    pub stat_type3: i32,
    pub stat_value3: i32,
    pub stat_type4: i32,
    pub stat_value4: i32,
    pub stat_type5: i32,
    pub stat_value5: i32,
    pub stat_type6: i32,
    pub stat_value6: i32,
    pub stat_type7: i32,
    pub stat_value7: i32,
    pub stat_type8: i32,
    pub stat_value8: i32,
    pub stat_type9: i32,
    pub stat_value9: i32,
    pub stat_type10: i32,
    pub stat_value10: i32,

    pub dmg_min1: f32,
    pub dmg_max1: f32,
    pub dmg_type1: u32,
    pub dmg_min2: f32,
    pub dmg_max2: f32,
    pub dmg_type2: u32,
    pub dmg_min3: f32,
    pub dmg_max3: f32,
    pub dmg_type3: u32,
    pub dmg_min4: f32,
    pub dmg_max4: f32,
    pub dmg_type4: u32,
    pub dmg_min5: f32,
    pub dmg_max5: f32,
    pub dmg_type5: u32,
    pub armor: i32,
    pub holy_res: i32,
    pub fire_res: i32,
    pub nature_res: i32,
    pub frost_res: i32,
    pub shadow_res: i32,
    pub arcane_res: i32,
    pub delay: i32,
    pub ammo_type: i32,
    #[serde(rename = "RangedModRange")]
    pub ranged_mod_range: f32,
    #[serde(rename = "spellid_1")]
    pub spell_id_1: i32,
    #[serde(rename = "spelltrigger_1")]
    pub spell_trigger_1: u32,
    #[serde(rename = "spellcharges_1")]
    pub spell_charges_1: i32,
    #[serde(rename = "spellppmRate_1")]
    pub spell_ppm_rate_1: f32,
    #[serde(rename = "spellcooldown_1")]
    pub spell_cooldown_1: i32,
    #[serde(rename = "spellcategory_1")]
    pub spell_category_1: i32,
    #[serde(rename = "spellcategorycooldown_1")]
    pub spell_category_cooldown_1: i32,
    #[serde(rename = "spellid_2")]
    pub spell_id_2: i32,
    #[serde(rename = "spelltrigger_2")]
    pub spell_trigger_2: u32,
    #[serde(rename = "spellcharges_2")]
    pub spell_charges_2: i32,
    #[serde(rename = "spellppmRate_2")]
    pub spell_ppm_rate_2: f32,
    #[serde(rename = "spellcooldown_2")]
    pub spell_cooldown_2: i32,
    #[serde(rename = "spellcategory_2")]
    pub spell_category_2: i32,
    #[serde(rename = "spellcategorycooldown_2")]
    pub spell_category_cooldown_2: i32,
    #[serde(rename = "spellid_3")]
    pub spell_id_3: i32,
    #[serde(rename = "spelltrigger_3")]
    pub spell_trigger_3: u32,
    #[serde(rename = "spellcharges_3")]
    pub spell_charges_3: i32,
    #[serde(rename = "spellppmRate_3")]
    pub spell_ppm_rate_3: f32,
    #[serde(rename = "spellcooldown_3")]
    pub spell_cooldown_3: i32,
    #[serde(rename = "spellcategory_3")]
    pub spell_category_3: i32,
    #[serde(rename = "spellcategorycooldown_3")]
    pub spell_category_cooldown_3: i32,
    #[serde(rename = "spellid_4")]
    pub spell_id_4: i32,
    #[serde(rename = "spelltrigger_4")]
    pub spell_trigger_4: u32,
    #[serde(rename = "spellcharges_4")]
    pub spell_charges_4: i32,
    #[serde(rename = "spellppmRate_4")]
    pub spell_ppm_rate_4: f32,
    #[serde(rename = "spellcooldown_4")]
    pub spell_cooldown_4: i32,
    #[serde(rename = "spellcategory_4")]
    pub spell_category_4: i32,
    #[serde(rename = "spellcategorycooldown_4")]
    pub spell_category_cooldown_4: i32,
    #[serde(rename = "spellid_5")]
    pub spell_id_5: i32,
    #[serde(rename = "spelltrigger_5")]
    pub spell_trigger_5: u32,
    #[serde(rename = "spellcharges_5")]
    pub spell_charges_5: i32,
    #[serde(rename = "spellppmRate_5")]
    pub spell_ppm_rate_5: f32,
    #[serde(rename = "spellcooldown_5")]
    pub spell_cooldown_5: i32,
    #[serde(rename = "spellcategory_5")]
    pub spell_category_5: i32,
    #[serde(rename = "spellcategorycooldown_5")]
    pub spell_category_cooldown_5: i32,
    pub bonding: u32,
    pub description: String,
    #[serde(rename = "PageText")]
    pub page_text: i32,
    #[serde(rename = "LanguageID")]
    pub language: u32,
    #[serde(rename = "PageMaterial")]
    pub page_text_material: u32,
    #[serde(rename = "startquest")]
    pub start_quest: i32,
    #[serde(rename = "lockid")]
    pub lock_id: i32,
    #[serde(rename = "Material")]
    pub material: i32,
    #[serde(rename = "sheath")]
    pub sheathe_type: u32,
    #[serde(rename = "RandomProperty")]
    pub random_property: i32,
    pub block: i32,
    #[serde(rename = "itemset")]
    pub item_set: u32,
    #[serde(rename = "MaxDurability")]
    pub max_durability: i32,
    pub area: u32,
    #[serde(rename = "Map")]
    pub map: u32,
    #[serde(rename = "BagFamily")]
    pub bag_family: u32,

    #[allow(unused)] // This is used internally in cmangos, but has no utility outside of it
    #[serde(rename = "ScriptName")]
    pub script_name: String,

    #[serde(rename = "DisenchantID")]
    pub disenchant_id: i32,
    #[serde(rename = "FoodType")]
    pub food_type: i32,
    #[serde(rename = "minMoneyLoot")]
    pub min_money_loot: i32,
    #[serde(rename = "maxMoneyLoot")]
    pub max_money_loot: i32,
    #[serde(rename = "Duration")]
    pub duration: i32,
    #[serde(rename = "ExtraFlags")]
    pub extra_flags: i32,

    #[serde(rename = "unk0")]
    pub sound_override_sub_class: i32,
    #[serde(rename = "RandomSuffix")]
    pub random_suffix: i32,
    #[serde(rename = "TotemCategory")]
    pub totem_category: i32,
    #[serde(rename = "socketColor_1")]
    pub socket_color_1: u32,
    #[serde(rename = "socketContent_1")]
    pub socket_content_1: u32,
    #[serde(rename = "socketColor_2")]
    pub socket_color_2: u32,
    #[serde(rename = "socketContent_2")]
    pub socket_content_2: u32,
    #[serde(rename = "socketColor_3")]
    pub socket_color_3: u32,
    #[serde(rename = "socketContent_3")]
    pub socket_content_3: u32,
    #[serde(rename = "socketBonus")]
    pub socket_bonus: i32,
    #[serde(rename = "GemProperties")]
    pub gem_properties: i32,
    #[serde(rename = "RequiredDisenchantSkill")]
    pub required_disenchant_skill: i32,
    #[serde(rename = "ArmorDamageModifier")]
    pub armor_damage_modifier: f32,
}

pub fn tbc(dir: &Path) -> (Vec<GenericThing>, Optimizations) {
    let items = read_csv_file::<TbcItem>(dir, "items");

    let items: Vec<_> = items
        .into_iter()
        .map(|row| {
            let (required_skill, required_skill_rank) = {
                let (skill, required_skill_level) = if row.required_skill == 242 {
                    // Cmangos weirdly uses a non existent skill
                    (0, 0)
                } else {
                    (row.required_skill, row.required_skill_rank)
                };
                (Skill::try_from(skill).unwrap(), required_skill_level)
            };

            let fields = vec![
                Field::new("entry", Value::Uint(row.entry)),
                Field::new(
                    "class_and_sub_class",
                    Value::TbcItemClassAndSubClass(
                        wow_world_base::tbc::ItemClassAndSubClass::try_from(
                            (row.subclass as u64) << 32 | row.class as u64,
                        )
                        .unwrap(),
                    ),
                ),
                Field::new(
                    "sound_override_sub_class",
                    Value::Int(row.sound_override_sub_class),
                ),
                Field::new("name", Value::String(row.name.clone())),
                Field::new("display_id", Value::Uint(row.display_id)),
                Field::new(
                    "quality",
                    Value::VanillaTbcItemQuality(row.quality.try_into().unwrap()),
                ),
                Field::new("flags", Value::TbcItemFlag(row.flags.try_into().unwrap())),
                Field::new("buy_count", Value::Int(row.buy_count)),
                Field::new("buy_price", Value::Gold(row.buy_price.try_into().unwrap())),
                Field::new(
                    "sell_price",
                    Value::Gold(row.sell_price.try_into().unwrap()),
                ),
                Field::new(
                    "inventory_type",
                    Value::InventoryType(row.inventory_type.try_into().unwrap()),
                ),
                Field::new(
                    "allowed_class",
                    Value::VanillaTbcAllowedClass(row.allowed_class.try_into().unwrap()),
                ),
                Field::new(
                    "allowed_race",
                    Value::TbcAllowedRace(row.allowed_race.try_into().unwrap()),
                ),
                Field::new("item_level", Value::Int(row.item_level)),
                Field::new("required_level", Value::Int(row.required_level)),
                Field::new("required_skill", Value::TbcSkill(required_skill)),
                Field::new("required_skill_rank", Value::Int(required_skill_rank)),
                Field::new("required_spell", Value::Int(row.required_spell)),
                Field::new(
                    "required_honor_rank",
                    Value::PvpRank(row.required_honor_rank.try_into().unwrap()),
                ),
                Field::new("required_city_rank", Value::Int(row.required_city_rank)),
                Field::new(
                    "required_faction",
                    Value::TbcFaction(row.required_faction.try_into().unwrap()),
                ),
                Field::new(
                    "required_reputation_rank",
                    Value::Int(row.required_reputation_rank),
                ),
                Field::new("max_count", Value::Int(row.max_count)),
                Field::new("stackable", Value::Int(row.stackable)),
                Field::new("container_slots", Value::Int(row.container_slots)),
                Field::new("armor", Value::Int(row.armor)),
                Field::new("holy_res", Value::Int(row.holy_res)),
                Field::new("fire_res", Value::Int(row.fire_res)),
                Field::new("nature_res", Value::Int(row.nature_res)),
                Field::new("frost_res", Value::Int(row.frost_res)),
                Field::new("shadow_res", Value::Int(row.shadow_res)),
                Field::new("arcane_res", Value::Int(row.arcane_res)),
                Field::new("delay", Value::Int(row.delay)),
                Field::new("ammo_type", Value::Int(row.ammo_type)),
                Field::new("ranged_mod_range", Value::float(row.ranged_mod_range)),
                Field::new("bonding", Value::Bonding(row.bonding.try_into().unwrap())),
                Field::new("description", Value::String(row.description)),
                Field::new("page_text", Value::Int(row.page_text)),
                Field::new(
                    "language",
                    Value::TbcWrathLanguage(row.language.try_into().unwrap()),
                ),
                Field::new(
                    "page_text_material",
                    Value::TbcWrathPageTextMaterial(row.page_text_material.try_into().unwrap()),
                ),
                Field::new("start_quest", Value::Int(row.start_quest)),
                Field::new("lock_id", Value::Int(row.lock_id)),
                Field::new("material", Value::Int(row.material)),
                Field::new(
                    "sheathe_type",
                    Value::SheatheType(row.sheathe_type.try_into().unwrap()),
                ),
                Field::new("random_property", Value::Int(row.random_property)),
                Field::new("random_suffix", Value::Int(row.random_suffix)),
                Field::new("block", Value::Int(row.block)),
                Field::new(
                    "item_set",
                    Value::TbcItemSet(row.item_set.try_into().unwrap()),
                ),
                Field::new("max_durability", Value::Int(row.max_durability)),
                Field::new("area", Value::TbcArea(row.area.try_into().unwrap())),
                Field::new("map", Value::TbcMap(row.map.try_into().unwrap())),
                Field::new(
                    "bag_family",
                    Value::TbcWrathBagFamily(row.bag_family.try_into().unwrap()),
                ),
                Field::new("totem_category", Value::Int(row.totem_category)),
                Field::new("socket_bonus", Value::Int(row.socket_bonus)),
                Field::new("gem_properties", Value::Int(row.gem_properties)),
                Field::new(
                    "required_disenchant_skill",
                    Value::Int(row.required_disenchant_skill),
                ),
                Field::new(
                    "armor_damage_modifier",
                    Value::float(row.armor_damage_modifier),
                ),
                Field::new("disenchant_id", Value::Int(row.disenchant_id)),
                Field::new("food_type", Value::Int(row.food_type)),
                Field::new("min_money_loot", Value::Int(row.min_money_loot)),
                Field::new("max_money_loot", Value::Int(row.max_money_loot)),
                Field::new("duration", Value::Int(row.duration)),
                Field::new(
                    "extra_flags",
                    Value::Int(process_extra_flags(row.entry, row.extra_flags, &row.name)),
                ),
            ];

            let arrays = vec![
                Array::new(
                    "sockets",
                    "ItemSocket",
                    true,
                    ArrayInstances::new(vec![
                        ArrayInstance::default_values(vec![
                            ArrayField::new(
                                "color",
                                "socket_color_1",
                                Value::Uint(row.socket_color_1),
                            ),
                            ArrayField::new(
                                "content",
                                "socket_content_1",
                                Value::Uint(row.socket_content_1),
                            ),
                        ]),
                        ArrayInstance::default_values(vec![
                            ArrayField::new(
                                "color",
                                "socket_color_2",
                                Value::Uint(row.socket_color_2),
                            ),
                            ArrayField::new(
                                "content",
                                "socket_content_2",
                                Value::Uint(row.socket_content_2),
                            ),
                        ]),
                        ArrayInstance::default_values(vec![
                            ArrayField::new(
                                "color",
                                "socket_color_3",
                                Value::Uint(row.socket_color_3),
                            ),
                            ArrayField::new(
                                "content",
                                "socket_content_3",
                                Value::Uint(row.socket_content_3),
                            ),
                        ]),
                    ]),
                ),
                Array::new(
                    "damages",
                    "ItemDamageType",
                    true,
                    ArrayInstances::new(vec![
                        ArrayInstance::new(
                            row.dmg_min1 == 0.0 && row.dmg_max1 == 0.0,
                            vec![
                                ArrayField::new(
                                    "damage_minimum",
                                    "dmg_min1",
                                    Value::float(row.dmg_min1),
                                ),
                                ArrayField::new(
                                    "damage_maximum",
                                    "dmg_max1",
                                    Value::float(row.dmg_max1),
                                ),
                                ArrayField::new(
                                    "school",
                                    "dmg_type1",
                                    Value::SpellSchool(row.dmg_type1.try_into().unwrap()),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.dmg_min2 == 0.0 && row.dmg_max2 == 0.0,
                            vec![
                                ArrayField::new(
                                    "damage_minimum",
                                    "dmg_min2",
                                    Value::float(row.dmg_min2),
                                ),
                                ArrayField::new(
                                    "damage_maximum",
                                    "dmg_max2",
                                    Value::float(row.dmg_max2),
                                ),
                                ArrayField::new(
                                    "school",
                                    "dmg_type2",
                                    Value::SpellSchool(row.dmg_type2.try_into().unwrap()),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.dmg_min3 == 0.0 && row.dmg_max3 == 0.0,
                            vec![
                                ArrayField::new(
                                    "damage_minimum",
                                    "dmg_min3",
                                    Value::float(row.dmg_min3),
                                ),
                                ArrayField::new(
                                    "damage_maximum",
                                    "dmg_max3",
                                    Value::float(row.dmg_max3),
                                ),
                                ArrayField::new(
                                    "school",
                                    "dmg_type3",
                                    Value::SpellSchool(row.dmg_type3.try_into().unwrap()),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.dmg_min4 == 0.0 && row.dmg_max4 == 0.0,
                            vec![
                                ArrayField::new(
                                    "damage_minimum",
                                    "dmg_min4",
                                    Value::float(row.dmg_min4),
                                ),
                                ArrayField::new(
                                    "damage_maximum",
                                    "dmg_max4",
                                    Value::float(row.dmg_max4),
                                ),
                                ArrayField::new(
                                    "school",
                                    "dmg_type4",
                                    Value::SpellSchool(row.dmg_type4.try_into().unwrap()),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.dmg_min5 == 0.0 && row.dmg_max5 == 0.0,
                            vec![
                                ArrayField::new(
                                    "damage_minimum",
                                    "dmg_min5",
                                    Value::float(row.dmg_min5),
                                ),
                                ArrayField::new(
                                    "damage_maximum",
                                    "dmg_max5",
                                    Value::float(row.dmg_max5),
                                ),
                                ArrayField::new(
                                    "school",
                                    "dmg_type5",
                                    Value::SpellSchool(row.dmg_type5.try_into().unwrap()),
                                ),
                            ],
                        ),
                    ]),
                ),
                Array::new(
                    "stats",
                    "ItemStat",
                    true,
                    ArrayInstances::new(vec![
                        ArrayInstance::new(
                            row.stat_value1 == 0,
                            vec![
                                ArrayField::new(
                                    "stat_type",
                                    "stat_type1",
                                    Value::Uint(row.stat_type1.try_into().unwrap()),
                                ),
                                ArrayField::new(
                                    "value",
                                    "stat_value1",
                                    Value::Int(row.stat_value1),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.stat_value2 == 0,
                            vec![
                                ArrayField::new(
                                    "stat_type",
                                    "stat_type2",
                                    Value::Uint(row.stat_type2.try_into().unwrap()),
                                ),
                                ArrayField::new(
                                    "value",
                                    "stat_value2",
                                    Value::Int(row.stat_value2),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.stat_value3 == 0,
                            vec![
                                ArrayField::new(
                                    "stat_type",
                                    "stat_type3",
                                    Value::Uint(row.stat_type3.try_into().unwrap()),
                                ),
                                ArrayField::new(
                                    "value",
                                    "stat_value3",
                                    Value::Int(row.stat_value3),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.stat_value4 == 0,
                            vec![
                                ArrayField::new(
                                    "stat_type",
                                    "stat_type4",
                                    Value::Uint(row.stat_type4.try_into().unwrap()),
                                ),
                                ArrayField::new(
                                    "value",
                                    "stat_value4",
                                    Value::Int(row.stat_value4),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.stat_value5 == 0,
                            vec![
                                ArrayField::new(
                                    "stat_type",
                                    "stat_type5",
                                    Value::Uint(row.stat_type5.try_into().unwrap()),
                                ),
                                ArrayField::new(
                                    "value",
                                    "stat_value5",
                                    Value::Int(row.stat_value5),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.stat_value6 == 0,
                            vec![
                                ArrayField::new(
                                    "stat_type",
                                    "stat_type6",
                                    Value::Uint(row.stat_type6.try_into().unwrap()),
                                ),
                                ArrayField::new(
                                    "value",
                                    "stat_value6",
                                    Value::Int(row.stat_value6),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.stat_value7 == 0,
                            vec![
                                ArrayField::new(
                                    "stat_type",
                                    "stat_type7",
                                    Value::Uint(row.stat_type7.try_into().unwrap()),
                                ),
                                ArrayField::new(
                                    "value",
                                    "stat_value7",
                                    Value::Int(row.stat_value7),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.stat_value8 == 0,
                            vec![
                                ArrayField::new(
                                    "stat_type",
                                    "stat_type8",
                                    Value::Uint(row.stat_type8.try_into().unwrap()),
                                ),
                                ArrayField::new(
                                    "value",
                                    "stat_value8",
                                    Value::Int(row.stat_value8),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.stat_value9 == 0,
                            vec![
                                ArrayField::new(
                                    "stat_type",
                                    "stat_type9",
                                    Value::Uint(row.stat_type9.try_into().unwrap()),
                                ),
                                ArrayField::new(
                                    "value",
                                    "stat_value9",
                                    Value::Int(row.stat_value9),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.stat_value10 == 0,
                            vec![
                                ArrayField::new(
                                    "stat_type",
                                    "stat_type10",
                                    Value::Uint(row.stat_type10.try_into().unwrap()),
                                ),
                                ArrayField::new(
                                    "value",
                                    "stat_value10",
                                    Value::Int(row.stat_value10),
                                ),
                            ],
                        ),
                    ]),
                ),
                Array::new(
                    "spells",
                    "Spells",
                    false,
                    ArrayInstances::new(vec![
                        ArrayInstance::new(
                            row.spell_id_1 == 0,
                            vec![
                                ArrayField::new("spell", "spell_id_1", Value::Int(row.spell_id_1)),
                                ArrayField::new(
                                    "spell_trigger",
                                    "spell_trigger_1",
                                    Value::TbcWrathSpellTriggerType(
                                        row.spell_trigger_1.try_into().unwrap(),
                                    ),
                                ),
                                ArrayField::new(
                                    "spell_charges",
                                    "spell_charges_1",
                                    Value::Int(row.spell_charges_1),
                                ),
                                ArrayField::new(
                                    "spell_ppm_rate",
                                    "spell_ppm_rate_1",
                                    Value::float(row.spell_ppm_rate_1),
                                ),
                                ArrayField::new(
                                    "spell_cooldown",
                                    "spell_cooldown_1",
                                    Value::Int(row.spell_cooldown_1),
                                ),
                                ArrayField::new(
                                    "spell_category",
                                    "spell_category_1",
                                    Value::Int(row.spell_category_1),
                                ),
                                ArrayField::new(
                                    "spell_category_cooldown",
                                    "spell_category_cooldown_1",
                                    Value::Int(row.spell_category_cooldown_1),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.spell_id_2 == 0,
                            vec![
                                ArrayField::new("spell", "spell_id_2", Value::Int(row.spell_id_2)),
                                ArrayField::new(
                                    "spell_trigger",
                                    "spell_trigger_2",
                                    Value::TbcWrathSpellTriggerType(
                                        row.spell_trigger_2.try_into().unwrap(),
                                    ),
                                ),
                                ArrayField::new(
                                    "spell_charges",
                                    "spell_charges_2",
                                    Value::Int(row.spell_charges_2),
                                ),
                                ArrayField::new(
                                    "spell_ppm_rate",
                                    "spell_ppm_rate_2",
                                    Value::float(row.spell_ppm_rate_2),
                                ),
                                ArrayField::new(
                                    "spell_cooldown",
                                    "spell_cooldown_2",
                                    Value::Int(row.spell_cooldown_2),
                                ),
                                ArrayField::new(
                                    "spell_category",
                                    "spell_category_2",
                                    Value::Int(row.spell_category_2),
                                ),
                                ArrayField::new(
                                    "spell_category_cooldown",
                                    "spell_category_cooldown_2",
                                    Value::Int(row.spell_category_cooldown_2),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.spell_id_3 == 0,
                            vec![
                                ArrayField::new("spell", "spell_id_3", Value::Int(row.spell_id_3)),
                                ArrayField::new(
                                    "spell_trigger",
                                    "spell_trigger_3",
                                    Value::TbcWrathSpellTriggerType(
                                        row.spell_trigger_3.try_into().unwrap(),
                                    ),
                                ),
                                ArrayField::new(
                                    "spell_charges",
                                    "spell_charges_3",
                                    Value::Int(row.spell_charges_3),
                                ),
                                ArrayField::new(
                                    "spell_ppm_rate",
                                    "spell_ppm_rate_3",
                                    Value::float(row.spell_ppm_rate_3),
                                ),
                                ArrayField::new(
                                    "spell_cooldown",
                                    "spell_cooldown_3",
                                    Value::Int(row.spell_cooldown_3),
                                ),
                                ArrayField::new(
                                    "spell_category",
                                    "spell_category_3",
                                    Value::Int(row.spell_category_3),
                                ),
                                ArrayField::new(
                                    "spell_category_cooldown",
                                    "spell_category_cooldown_3",
                                    Value::Int(row.spell_category_cooldown_3),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.spell_id_4 == 0,
                            vec![
                                ArrayField::new("spell", "spell_id_4", Value::Int(row.spell_id_4)),
                                ArrayField::new(
                                    "spell_trigger",
                                    "spell_trigger_4",
                                    Value::TbcWrathSpellTriggerType(
                                        row.spell_trigger_4.try_into().unwrap(),
                                    ),
                                ),
                                ArrayField::new(
                                    "spell_charges",
                                    "spell_charges_4",
                                    Value::Int(row.spell_charges_4),
                                ),
                                ArrayField::new(
                                    "spell_ppm_rate",
                                    "spell_ppm_rate_4",
                                    Value::float(row.spell_ppm_rate_4),
                                ),
                                ArrayField::new(
                                    "spell_cooldown",
                                    "spell_cooldown_4",
                                    Value::Int(row.spell_cooldown_4),
                                ),
                                ArrayField::new(
                                    "spell_category",
                                    "spell_category_4",
                                    Value::Int(row.spell_category_4),
                                ),
                                ArrayField::new(
                                    "spell_category_cooldown",
                                    "spell_category_cooldown_4",
                                    Value::Int(row.spell_category_cooldown_4),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.spell_id_5 == 0,
                            vec![
                                ArrayField::new("spell", "spell_id_5", Value::Int(row.spell_id_5)),
                                ArrayField::new(
                                    "spell_trigger",
                                    "spell_trigger_5",
                                    Value::TbcWrathSpellTriggerType(
                                        row.spell_trigger_5.try_into().unwrap(),
                                    ),
                                ),
                                ArrayField::new(
                                    "spell_charges",
                                    "spell_charges_5",
                                    Value::Int(row.spell_charges_5),
                                ),
                                ArrayField::new(
                                    "spell_ppm_rate",
                                    "spell_ppm_rate_5",
                                    Value::float(row.spell_ppm_rate_5),
                                ),
                                ArrayField::new(
                                    "spell_cooldown",
                                    "spell_cooldown_5",
                                    Value::Int(row.spell_cooldown_5),
                                ),
                                ArrayField::new(
                                    "spell_category",
                                    "spell_category_5",
                                    Value::Int(row.spell_category_5),
                                ),
                                ArrayField::new(
                                    "spell_category_cooldown",
                                    "spell_category_cooldown_5",
                                    Value::Int(row.spell_category_cooldown_5),
                                ),
                            ],
                        ),
                    ]),
                ),
            ];

            GenericThing::new(row.entry, row.extra_flags, row.name, fields, arrays)
        })
        .collect();

    assertions(&items);
    let opt = Optimizations::new(&items, get_fields(&items));
    (items, opt)
}
