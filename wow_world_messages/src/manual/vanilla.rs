use crate::vanilla::{
    Area, ItemDamageType, ItemSpells, ItemStat, SMSG_ITEM_QUERY_SINGLE_RESPONSE_found,
    SMSG_ITEM_NAME_QUERY_RESPONSE, SMSG_ITEM_QUERY_SINGLE_RESPONSE,
};
use std::convert::TryFrom;
use wow_world_base::vanilla::{Bonding, Item, Map, Skill};

/// Convert an [`Item`] to a [`SMSG_ITEM_QUERY_SINGLE_RESPONSE`].
///
/// The message is just a tedious listing of [`Item`] fields with no
/// potential deviations so it has been upstreamed.
pub fn item_to_query_response(item: &Item) -> SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    item.into()
}

/// Convert an [`Item`] to a [`SMSG_ITEM_NAME_QUERY_RESPONSE`].
///
/// The message is just a listing of [`Item`] fields with no
/// potential deviations so it has been upstreamed.
pub fn item_to_name_query_response(item: &Item) -> SMSG_ITEM_NAME_QUERY_RESPONSE {
    item.into()
}

impl From<&Item> for SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    fn from(v: &Item) -> Self {
        Self {
            item: v.entry as u32,
            found: Some(SMSG_ITEM_QUERY_SINGLE_RESPONSE_found {
                class_and_sub_class: v.class_and_sub_class,
                name1: v.name.to_string(),
                name2: "".to_string(),
                name3: "".to_string(),
                name4: "".to_string(),
                item_display_info: v.displayid as u32,
                quality: v.quality,
                flags: v.flags as u32,
                buy_price: v.buy_price as u32,
                sell_price: v.sell_price as u32,
                inventory_type: v.inventory_type,
                allowed_class: v.allowed_class,
                allowed_race: v.allowed_race,
                item_level: v.item_level as u32,
                required_level: v.required_level as u32,
                required_skill: Skill::try_from(v.required_skill as u16).unwrap(),
                required_skill_rank: v.required_skill_rank as u32,
                required_spell: v.required_spell as u32,
                required_honor_rank: v.required_honor_rank as u32,
                required_city_rank: v.required_city_rank as u32,
                required_reputation_faction: v.required_reputation_faction as u32,
                required_reputation_rank: v.required_reputation_rank as u32,
                max_count: v.max_count as u32,
                stackable: v.stackable as u32,
                container_slots: v.container_slots as u32,
                stats: [
                    ItemStat {
                        item_stat_type: v.stat_type1 as u32,
                        item_stat_value: v.stat_value1,
                    },
                    ItemStat {
                        item_stat_type: v.stat_type2 as u32,
                        item_stat_value: v.stat_value2,
                    },
                    ItemStat {
                        item_stat_type: v.stat_type3 as u32,
                        item_stat_value: v.stat_value3,
                    },
                    ItemStat {
                        item_stat_type: v.stat_type4 as u32,
                        item_stat_value: v.stat_value4,
                    },
                    ItemStat {
                        item_stat_type: v.stat_type5 as u32,
                        item_stat_value: v.stat_value5,
                    },
                    ItemStat {
                        item_stat_type: v.stat_type6 as u32,
                        item_stat_value: v.stat_value6,
                    },
                    ItemStat {
                        item_stat_type: v.stat_type7 as u32,
                        item_stat_value: v.stat_value7,
                    },
                    ItemStat {
                        item_stat_type: 0,
                        item_stat_value: 0,
                    },
                    ItemStat {
                        item_stat_type: 0,
                        item_stat_value: 0,
                    },
                    ItemStat {
                        item_stat_type: 0,
                        item_stat_value: 0,
                    },
                ],
                damages: [
                    ItemDamageType {
                        damage_minimum: v.dmg_min1,
                        damage_maximum: v.dmg_max1,
                        damage_type: v.dmg_type1 as u32,
                    },
                    ItemDamageType {
                        damage_minimum: v.dmg_min2,
                        damage_maximum: v.dmg_max2,
                        damage_type: v.dmg_type2 as u32,
                    },
                    ItemDamageType {
                        damage_minimum: v.dmg_min3,
                        damage_maximum: v.dmg_max3,
                        damage_type: v.dmg_type3 as u32,
                    },
                    ItemDamageType {
                        damage_minimum: v.dmg_min4,
                        damage_maximum: v.dmg_max4,
                        damage_type: v.dmg_type4 as u32,
                    },
                    ItemDamageType {
                        damage_minimum: v.dmg_min5,
                        damage_maximum: v.dmg_max5,
                        damage_type: v.dmg_type5 as u32,
                    },
                ],
                armor: v.armor,
                holy_resistance: v.holy_res,
                fire_resistance: v.fire_res,
                nature_resistance: v.nature_res,
                frost_resistance: v.frost_res,
                shadow_resistance: v.shadow_res,
                arcane_resistance: v.arcane_res,
                delay: v.delay as u32,
                ammo_type: v.ammo_type as u32,
                ranged_range_modification: v.ranged_mod_range,
                spells: [
                    ItemSpells {
                        spell: v.spell_id_1 as u32,
                        spell_trigger: v.spell_trigger_1 as u32,
                        spell_charges: v.spell_charges_1,
                        spell_cooldown: v.spell_cooldown_1,
                        spell_category: v.spell_category_1 as u32,
                        spell_category_cooldown: v.spell_category_cooldown_1,
                    },
                    ItemSpells {
                        spell: v.spell_id_2 as u32,
                        spell_trigger: v.spell_trigger_2 as u32,
                        spell_charges: v.spell_charges_2,
                        spell_cooldown: v.spell_cooldown_2,
                        spell_category: v.spell_category_2 as u32,
                        spell_category_cooldown: v.spell_category_cooldown_2,
                    },
                    ItemSpells {
                        spell: v.spell_id_3 as u32,
                        spell_trigger: v.spell_trigger_3 as u32,
                        spell_charges: v.spell_charges_3,
                        spell_cooldown: v.spell_cooldown_3,
                        spell_category: v.spell_category_3 as u32,
                        spell_category_cooldown: v.spell_category_cooldown_3,
                    },
                    ItemSpells {
                        spell: v.spell_id_4 as u32,
                        spell_trigger: v.spell_trigger_4 as u32,
                        spell_charges: v.spell_charges_4,
                        spell_cooldown: v.spell_cooldown_4,
                        spell_category: v.spell_category_4 as u32,
                        spell_category_cooldown: v.spell_category_cooldown_4,
                    },
                    ItemSpells {
                        spell: v.spell_id_5 as u32,
                        spell_trigger: v.spell_trigger_5 as u32,
                        spell_charges: v.spell_charges_5,
                        spell_cooldown: v.spell_cooldown_5,
                        spell_category: v.spell_category_5 as u32,
                        spell_category_cooldown: v.spell_category_cooldown_5,
                    },
                ],
                bonding: Bonding::try_from(v.bonding as u8).unwrap(),
                description: v.description.to_string(),
                page_text: v.page_text as u32,
                language_id: v.language_id as u32,
                page_material: v.page_material as u32,
                start_quest: v.start_quest as u32,
                lock_id: v.lock_id as u32,
                material: v.material as u32,
                sheath: v.sheath as u32,
                random_property: v.random_property as u32,
                block: v.block as u32,
                item_set: v.itemset as u32,
                max_durability: v.max_durability as u32,
                area: Area::try_from(v.area as u32).unwrap(),
                map: Map::try_from(v.map as u32).unwrap(),
                bag_family: v.bag_family as u32,
            }),
        }
    }
}

impl From<Item> for SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    fn from(v: Item) -> Self {
        (&v).into()
    }
}

impl From<&Item> for SMSG_ITEM_NAME_QUERY_RESPONSE {
    fn from(v: &Item) -> Self {
        Self {
            item: v.entry as u32,
            item_name: v.name.to_string(),
        }
    }
}

impl From<Item> for SMSG_ITEM_NAME_QUERY_RESPONSE {
    fn from(v: Item) -> Self {
        (&v).into()
    }
}
