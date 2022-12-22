use std::convert::{TryFrom, TryInto};
use crate::world::tbc::ItemDamageType;
use crate::world::tbc::ItemSocket;
use crate::world::tbc::ItemSpells;
use crate::world::tbc::ItemStat;
use crate::world::tbc::Area;
use crate::world::tbc::InventoryType;
use crate::world::tbc::ItemClass;
use crate::world::tbc::ItemQuality;
use crate::world::tbc::Map;
use crate::world::tbc::Skill;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:121`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L121):
/// ```text
/// smsg SMSG_ITEM_QUERY_SINGLE_RESPONSE = 0x0058 {
///     u32 item;
///     optional found {
///         (u32)ItemClass item_class;
///         u32 item_sub_class;
///         u32 sound_override_sub_class;
///         CString name1;
///         CString name2;
///         CString name3;
///         CString name4;
///         u32 item_display_info;
///         (u32)ItemQuality quality;
///         u32 flags;
///         u32 buy_price;
///         u32 sell_price;
///         (u32)InventoryType inventory_type;
///         u32 allowed_class;
///         u32 allowed_race;
///         u32 item_level;
///         u32 required_level;
///         (u32)Skill required_skill;
///         u32 required_skill_rank;
///         u32 required_spell;
///         u32 required_honor_rank;
///         u32 required_city_rank;
///         u32 required_reputation_faction;
///         u32 required_reputation_rank;
///         u32 max_count;
///         u32 stackable;
///         u32 container_slots;
///         ItemStat[10] stats;
///         ItemDamageType[5] damages;
///         i32 armor;
///         i32 holy_resistance;
///         i32 fire_resistance;
///         i32 nature_resistance;
///         i32 frost_resistance;
///         i32 shadow_resistance;
///         i32 arcane_resistance;
///         u32 delay;
///         u32 ammo_type;
///         f32 ranged_range_modification;
///         ItemSpells[5] spells;
///         u32 bonding;
///         CString description;
///         u32 page_text;
///         u32 language_id;
///         u32 page_material;
///         u32 start_quest;
///         u32 lock_id;
///         u32 material;
///         u32 sheath;
///         u32 random_property;
///         u32 block;
///         u32 item_set;
///         u32 max_durability;
///         Area area;
///         Map map;
///         u32 bag_family;
///         u32 totem_category;
///         ItemSocket[3] sockets;
///         u32 socket_bonus;
///         u32 gem_properties;
///         u32 required_disenchant_skill;
///         f32 armor_damage_modifier;
///         u32 duration_in_seconds;
///     }
/// }
/// ```
pub struct SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    pub item: u32,
    pub found: Option<SMSG_ITEM_QUERY_SINGLE_RESPONSE_found>,
}

impl crate::Message for SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    const OPCODE: u32 = 0x0058;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // optional found
        if let Some(v) = &self.found {
            // item_class: ItemClass
            w.write_all(&(v.item_class.as_int() as u32).to_le_bytes())?;

            // item_sub_class: u32
            w.write_all(&v.item_sub_class.to_le_bytes())?;

            // sound_override_sub_class: u32
            w.write_all(&v.sound_override_sub_class.to_le_bytes())?;

            // name1: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name1.as_bytes().iter().rev().next(), Some(&0_u8), "String `name1` must not be null-terminated.");
            w.write_all(v.name1.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name2: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name2.as_bytes().iter().rev().next(), Some(&0_u8), "String `name2` must not be null-terminated.");
            w.write_all(v.name2.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name3: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name3.as_bytes().iter().rev().next(), Some(&0_u8), "String `name3` must not be null-terminated.");
            w.write_all(v.name3.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name4: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name4.as_bytes().iter().rev().next(), Some(&0_u8), "String `name4` must not be null-terminated.");
            w.write_all(v.name4.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // item_display_info: u32
            w.write_all(&v.item_display_info.to_le_bytes())?;

            // quality: ItemQuality
            w.write_all(&(v.quality.as_int() as u32).to_le_bytes())?;

            // flags: u32
            w.write_all(&v.flags.to_le_bytes())?;

            // buy_price: u32
            w.write_all(&v.buy_price.to_le_bytes())?;

            // sell_price: u32
            w.write_all(&v.sell_price.to_le_bytes())?;

            // inventory_type: InventoryType
            w.write_all(&(v.inventory_type.as_int() as u32).to_le_bytes())?;

            // allowed_class: u32
            w.write_all(&v.allowed_class.to_le_bytes())?;

            // allowed_race: u32
            w.write_all(&v.allowed_race.to_le_bytes())?;

            // item_level: u32
            w.write_all(&v.item_level.to_le_bytes())?;

            // required_level: u32
            w.write_all(&v.required_level.to_le_bytes())?;

            // required_skill: Skill
            w.write_all(&(v.required_skill.as_int() as u32).to_le_bytes())?;

            // required_skill_rank: u32
            w.write_all(&v.required_skill_rank.to_le_bytes())?;

            // required_spell: u32
            w.write_all(&v.required_spell.to_le_bytes())?;

            // required_honor_rank: u32
            w.write_all(&v.required_honor_rank.to_le_bytes())?;

            // required_city_rank: u32
            w.write_all(&v.required_city_rank.to_le_bytes())?;

            // required_reputation_faction: u32
            w.write_all(&v.required_reputation_faction.to_le_bytes())?;

            // required_reputation_rank: u32
            w.write_all(&v.required_reputation_rank.to_le_bytes())?;

            // max_count: u32
            w.write_all(&v.max_count.to_le_bytes())?;

            // stackable: u32
            w.write_all(&v.stackable.to_le_bytes())?;

            // container_slots: u32
            w.write_all(&v.container_slots.to_le_bytes())?;

            // stats: ItemStat[10]
            for i in v.stats.iter() {
                i.write_into_vec(w)?;
            }

            // damages: ItemDamageType[5]
            for i in v.damages.iter() {
                i.write_into_vec(w)?;
            }

            // armor: i32
            w.write_all(&v.armor.to_le_bytes())?;

            // holy_resistance: i32
            w.write_all(&v.holy_resistance.to_le_bytes())?;

            // fire_resistance: i32
            w.write_all(&v.fire_resistance.to_le_bytes())?;

            // nature_resistance: i32
            w.write_all(&v.nature_resistance.to_le_bytes())?;

            // frost_resistance: i32
            w.write_all(&v.frost_resistance.to_le_bytes())?;

            // shadow_resistance: i32
            w.write_all(&v.shadow_resistance.to_le_bytes())?;

            // arcane_resistance: i32
            w.write_all(&v.arcane_resistance.to_le_bytes())?;

            // delay: u32
            w.write_all(&v.delay.to_le_bytes())?;

            // ammo_type: u32
            w.write_all(&v.ammo_type.to_le_bytes())?;

            // ranged_range_modification: f32
            w.write_all(&v.ranged_range_modification.to_le_bytes())?;

            // spells: ItemSpells[5]
            for i in v.spells.iter() {
                i.write_into_vec(w)?;
            }

            // bonding: u32
            w.write_all(&v.bonding.to_le_bytes())?;

            // description: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.description.as_bytes().iter().rev().next(), Some(&0_u8), "String `description` must not be null-terminated.");
            w.write_all(v.description.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // page_text: u32
            w.write_all(&v.page_text.to_le_bytes())?;

            // language_id: u32
            w.write_all(&v.language_id.to_le_bytes())?;

            // page_material: u32
            w.write_all(&v.page_material.to_le_bytes())?;

            // start_quest: u32
            w.write_all(&v.start_quest.to_le_bytes())?;

            // lock_id: u32
            w.write_all(&v.lock_id.to_le_bytes())?;

            // material: u32
            w.write_all(&v.material.to_le_bytes())?;

            // sheath: u32
            w.write_all(&v.sheath.to_le_bytes())?;

            // random_property: u32
            w.write_all(&v.random_property.to_le_bytes())?;

            // block: u32
            w.write_all(&v.block.to_le_bytes())?;

            // item_set: u32
            w.write_all(&v.item_set.to_le_bytes())?;

            // max_durability: u32
            w.write_all(&v.max_durability.to_le_bytes())?;

            // area: Area
            w.write_all(&(v.area.as_int() as u32).to_le_bytes())?;

            // map: Map
            w.write_all(&(v.map.as_int() as u32).to_le_bytes())?;

            // bag_family: u32
            w.write_all(&v.bag_family.to_le_bytes())?;

            // totem_category: u32
            w.write_all(&v.totem_category.to_le_bytes())?;

            // sockets: ItemSocket[3]
            for i in v.sockets.iter() {
                i.write_into_vec(w)?;
            }

            // socket_bonus: u32
            w.write_all(&v.socket_bonus.to_le_bytes())?;

            // gem_properties: u32
            w.write_all(&v.gem_properties.to_le_bytes())?;

            // required_disenchant_skill: u32
            w.write_all(&v.required_disenchant_skill.to_le_bytes())?;

            // armor_damage_modifier: f32
            w.write_all(&v.armor_damage_modifier.to_le_bytes())?;

            // duration_in_seconds: u32
            w.write_all(&v.duration_in_seconds.to_le_bytes())?;

        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=1784).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0058, size: body_size as u32 });
        }

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // optional found
        let current_size = {
            4 // item: u32
        };
        let found = if current_size < body_size as usize {
            // item_class: ItemClass
            let item_class: ItemClass = (crate::util::read_u32_le(r)? as u8).try_into()?;

            // item_sub_class: u32
            let item_sub_class = crate::util::read_u32_le(r)?;

            // sound_override_sub_class: u32
            let sound_override_sub_class = crate::util::read_u32_le(r)?;

            // name1: CString
            let name1 = crate::util::read_c_string_to_vec(r)?;
            let name1 = String::from_utf8(name1)?;

            // name2: CString
            let name2 = crate::util::read_c_string_to_vec(r)?;
            let name2 = String::from_utf8(name2)?;

            // name3: CString
            let name3 = crate::util::read_c_string_to_vec(r)?;
            let name3 = String::from_utf8(name3)?;

            // name4: CString
            let name4 = crate::util::read_c_string_to_vec(r)?;
            let name4 = String::from_utf8(name4)?;

            // item_display_info: u32
            let item_display_info = crate::util::read_u32_le(r)?;

            // quality: ItemQuality
            let quality: ItemQuality = (crate::util::read_u32_le(r)? as u8).try_into()?;

            // flags: u32
            let flags = crate::util::read_u32_le(r)?;

            // buy_price: u32
            let buy_price = crate::util::read_u32_le(r)?;

            // sell_price: u32
            let sell_price = crate::util::read_u32_le(r)?;

            // inventory_type: InventoryType
            let inventory_type: InventoryType = (crate::util::read_u32_le(r)? as u8).try_into()?;

            // allowed_class: u32
            let allowed_class = crate::util::read_u32_le(r)?;

            // allowed_race: u32
            let allowed_race = crate::util::read_u32_le(r)?;

            // item_level: u32
            let item_level = crate::util::read_u32_le(r)?;

            // required_level: u32
            let required_level = crate::util::read_u32_le(r)?;

            // required_skill: Skill
            let required_skill: Skill = (crate::util::read_u32_le(r)? as u16).try_into()?;

            // required_skill_rank: u32
            let required_skill_rank = crate::util::read_u32_le(r)?;

            // required_spell: u32
            let required_spell = crate::util::read_u32_le(r)?;

            // required_honor_rank: u32
            let required_honor_rank = crate::util::read_u32_le(r)?;

            // required_city_rank: u32
            let required_city_rank = crate::util::read_u32_le(r)?;

            // required_reputation_faction: u32
            let required_reputation_faction = crate::util::read_u32_le(r)?;

            // required_reputation_rank: u32
            let required_reputation_rank = crate::util::read_u32_le(r)?;

            // max_count: u32
            let max_count = crate::util::read_u32_le(r)?;

            // stackable: u32
            let stackable = crate::util::read_u32_le(r)?;

            // container_slots: u32
            let container_slots = crate::util::read_u32_le(r)?;

            // stats: ItemStat[10]
            let mut stats = [ItemStat::default(); 10];
            for i in stats.iter_mut() {
                *i = ItemStat::read(r)?;
            }

            // damages: ItemDamageType[5]
            let mut damages = [ItemDamageType::default(); 5];
            for i in damages.iter_mut() {
                *i = ItemDamageType::read(r)?;
            }

            // armor: i32
            let armor = crate::util::read_i32_le(r)?;

            // holy_resistance: i32
            let holy_resistance = crate::util::read_i32_le(r)?;

            // fire_resistance: i32
            let fire_resistance = crate::util::read_i32_le(r)?;

            // nature_resistance: i32
            let nature_resistance = crate::util::read_i32_le(r)?;

            // frost_resistance: i32
            let frost_resistance = crate::util::read_i32_le(r)?;

            // shadow_resistance: i32
            let shadow_resistance = crate::util::read_i32_le(r)?;

            // arcane_resistance: i32
            let arcane_resistance = crate::util::read_i32_le(r)?;

            // delay: u32
            let delay = crate::util::read_u32_le(r)?;

            // ammo_type: u32
            let ammo_type = crate::util::read_u32_le(r)?;

            // ranged_range_modification: f32
            let ranged_range_modification = crate::util::read_f32_le(r)?;
            // spells: ItemSpells[5]
            let mut spells = [ItemSpells::default(); 5];
            for i in spells.iter_mut() {
                *i = ItemSpells::read(r)?;
            }

            // bonding: u32
            let bonding = crate::util::read_u32_le(r)?;

            // description: CString
            let description = crate::util::read_c_string_to_vec(r)?;
            let description = String::from_utf8(description)?;

            // page_text: u32
            let page_text = crate::util::read_u32_le(r)?;

            // language_id: u32
            let language_id = crate::util::read_u32_le(r)?;

            // page_material: u32
            let page_material = crate::util::read_u32_le(r)?;

            // start_quest: u32
            let start_quest = crate::util::read_u32_le(r)?;

            // lock_id: u32
            let lock_id = crate::util::read_u32_le(r)?;

            // material: u32
            let material = crate::util::read_u32_le(r)?;

            // sheath: u32
            let sheath = crate::util::read_u32_le(r)?;

            // random_property: u32
            let random_property = crate::util::read_u32_le(r)?;

            // block: u32
            let block = crate::util::read_u32_le(r)?;

            // item_set: u32
            let item_set = crate::util::read_u32_le(r)?;

            // max_durability: u32
            let max_durability = crate::util::read_u32_le(r)?;

            // area: Area
            let area: Area = crate::util::read_u32_le(r)?.try_into()?;

            // map: Map
            let map: Map = crate::util::read_u32_le(r)?.try_into()?;

            // bag_family: u32
            let bag_family = crate::util::read_u32_le(r)?;

            // totem_category: u32
            let totem_category = crate::util::read_u32_le(r)?;

            // sockets: ItemSocket[3]
            let mut sockets = [ItemSocket::default(); 3];
            for i in sockets.iter_mut() {
                *i = ItemSocket::read(r)?;
            }

            // socket_bonus: u32
            let socket_bonus = crate::util::read_u32_le(r)?;

            // gem_properties: u32
            let gem_properties = crate::util::read_u32_le(r)?;

            // required_disenchant_skill: u32
            let required_disenchant_skill = crate::util::read_u32_le(r)?;

            // armor_damage_modifier: f32
            let armor_damage_modifier = crate::util::read_f32_le(r)?;
            // duration_in_seconds: u32
            let duration_in_seconds = crate::util::read_u32_le(r)?;

            Some(SMSG_ITEM_QUERY_SINGLE_RESPONSE_found {
                item_class,
                item_sub_class,
                sound_override_sub_class,
                name1,
                name2,
                name3,
                name4,
                item_display_info,
                quality,
                flags,
                buy_price,
                sell_price,
                inventory_type,
                allowed_class,
                allowed_race,
                item_level,
                required_level,
                required_skill,
                required_skill_rank,
                required_spell,
                required_honor_rank,
                required_city_rank,
                required_reputation_faction,
                required_reputation_rank,
                max_count,
                stackable,
                container_slots,
                stats,
                damages,
                armor,
                holy_resistance,
                fire_resistance,
                nature_resistance,
                frost_resistance,
                shadow_resistance,
                arcane_resistance,
                delay,
                ammo_type,
                ranged_range_modification,
                spells,
                bonding,
                description,
                page_text,
                language_id,
                page_material,
                start_quest,
                lock_id,
                material,
                sheath,
                random_property,
                block,
                item_set,
                max_durability,
                area,
                map,
                bag_family,
                totem_category,
                sockets,
                socket_bonus,
                gem_properties,
                required_disenchant_skill,
                armor_damage_modifier,
                duration_in_seconds,
            })
        } else {
            None
        };

        Ok(Self {
            item,
            found,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_ITEM_QUERY_SINGLE_RESPONSE {}

impl SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // item: u32
        + if let Some(found) = &self.found {
            4 // item_class: ItemClass
            + 4 // item_sub_class: u32
            + 4 // sound_override_sub_class: u32
            + found.name1.len() + 1 // name1: CString
            + found.name2.len() + 1 // name2: CString
            + found.name3.len() + 1 // name3: CString
            + found.name4.len() + 1 // name4: CString
            + 4 // item_display_info: u32
            + 4 // quality: ItemQuality
            + 4 // flags: u32
            + 4 // buy_price: u32
            + 4 // sell_price: u32
            + 4 // inventory_type: InventoryType
            + 4 // allowed_class: u32
            + 4 // allowed_race: u32
            + 4 // item_level: u32
            + 4 // required_level: u32
            + 4 // required_skill: Skill
            + 4 // required_skill_rank: u32
            + 4 // required_spell: u32
            + 4 // required_honor_rank: u32
            + 4 // required_city_rank: u32
            + 4 // required_reputation_faction: u32
            + 4 // required_reputation_rank: u32
            + 4 // max_count: u32
            + 4 // stackable: u32
            + 4 // container_slots: u32
            + 10 * 8 // stats: ItemStat[10]
            + 5 * 12 // damages: ItemDamageType[5]
            + 4 // armor: i32
            + 4 // holy_resistance: i32
            + 4 // fire_resistance: i32
            + 4 // nature_resistance: i32
            + 4 // frost_resistance: i32
            + 4 // shadow_resistance: i32
            + 4 // arcane_resistance: i32
            + 4 // delay: u32
            + 4 // ammo_type: u32
            + 4 // ranged_range_modification: f32
            + 5 * 24 // spells: ItemSpells[5]
            + 4 // bonding: u32
            + found.description.len() + 1 // description: CString
            + 4 // page_text: u32
            + 4 // language_id: u32
            + 4 // page_material: u32
            + 4 // start_quest: u32
            + 4 // lock_id: u32
            + 4 // material: u32
            + 4 // sheath: u32
            + 4 // random_property: u32
            + 4 // block: u32
            + 4 // item_set: u32
            + 4 // max_durability: u32
            + 4 // area: Area
            + 4 // map: Map
            + 4 // bag_family: u32
            + 4 // totem_category: u32
            + 3 * 8 // sockets: ItemSocket[3]
            + 4 // socket_bonus: u32
            + 4 // gem_properties: u32
            + 4 // required_disenchant_skill: u32
            + 4 // armor_damage_modifier: f32
            + 4 // duration_in_seconds: u32
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SMSG_ITEM_QUERY_SINGLE_RESPONSE_found {
    pub item_class: ItemClass,
    pub item_sub_class: u32,
    pub sound_override_sub_class: u32,
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub name4: String,
    pub item_display_info: u32,
    pub quality: ItemQuality,
    pub flags: u32,
    pub buy_price: u32,
    pub sell_price: u32,
    pub inventory_type: InventoryType,
    pub allowed_class: u32,
    pub allowed_race: u32,
    pub item_level: u32,
    pub required_level: u32,
    pub required_skill: Skill,
    pub required_skill_rank: u32,
    pub required_spell: u32,
    pub required_honor_rank: u32,
    pub required_city_rank: u32,
    pub required_reputation_faction: u32,
    pub required_reputation_rank: u32,
    pub max_count: u32,
    pub stackable: u32,
    pub container_slots: u32,
    pub stats: [ItemStat; 10],
    pub damages: [ItemDamageType; 5],
    pub armor: i32,
    pub holy_resistance: i32,
    pub fire_resistance: i32,
    pub nature_resistance: i32,
    pub frost_resistance: i32,
    pub shadow_resistance: i32,
    pub arcane_resistance: i32,
    pub delay: u32,
    pub ammo_type: u32,
    pub ranged_range_modification: f32,
    pub spells: [ItemSpells; 5],
    pub bonding: u32,
    pub description: String,
    pub page_text: u32,
    pub language_id: u32,
    pub page_material: u32,
    pub start_quest: u32,
    pub lock_id: u32,
    pub material: u32,
    pub sheath: u32,
    pub random_property: u32,
    pub block: u32,
    pub item_set: u32,
    pub max_durability: u32,
    pub area: Area,
    pub map: Map,
    pub bag_family: u32,
    pub totem_category: u32,
    pub sockets: [ItemSocket; 3],
    pub socket_bonus: u32,
    pub gem_properties: u32,
    pub required_disenchant_skill: u32,
    pub armor_damage_modifier: f32,
    pub duration_in_seconds: u32,
}

impl SMSG_ITEM_QUERY_SINGLE_RESPONSE_found {
    pub(crate) fn size(&self) -> usize {
        4 // item_class: ItemClass
        + 4 // item_sub_class: u32
        + 4 // sound_override_sub_class: u32
        + self.name1.len() + 1 // name1: CString
        + self.name2.len() + 1 // name2: CString
        + self.name3.len() + 1 // name3: CString
        + self.name4.len() + 1 // name4: CString
        + 4 // item_display_info: u32
        + 4 // quality: ItemQuality
        + 4 // flags: u32
        + 4 // buy_price: u32
        + 4 // sell_price: u32
        + 4 // inventory_type: InventoryType
        + 4 // allowed_class: u32
        + 4 // allowed_race: u32
        + 4 // item_level: u32
        + 4 // required_level: u32
        + 4 // required_skill: Skill
        + 4 // required_skill_rank: u32
        + 4 // required_spell: u32
        + 4 // required_honor_rank: u32
        + 4 // required_city_rank: u32
        + 4 // required_reputation_faction: u32
        + 4 // required_reputation_rank: u32
        + 4 // max_count: u32
        + 4 // stackable: u32
        + 4 // container_slots: u32
        + 10 * 8 // stats: ItemStat[10]
        + 5 * 12 // damages: ItemDamageType[5]
        + 4 // armor: i32
        + 4 // holy_resistance: i32
        + 4 // fire_resistance: i32
        + 4 // nature_resistance: i32
        + 4 // frost_resistance: i32
        + 4 // shadow_resistance: i32
        + 4 // arcane_resistance: i32
        + 4 // delay: u32
        + 4 // ammo_type: u32
        + 4 // ranged_range_modification: f32
        + 5 * 24 // spells: ItemSpells[5]
        + 4 // bonding: u32
        + self.description.len() + 1 // description: CString
        + 4 // page_text: u32
        + 4 // language_id: u32
        + 4 // page_material: u32
        + 4 // start_quest: u32
        + 4 // lock_id: u32
        + 4 // material: u32
        + 4 // sheath: u32
        + 4 // random_property: u32
        + 4 // block: u32
        + 4 // item_set: u32
        + 4 // max_durability: u32
        + 4 // area: Area
        + 4 // map: Map
        + 4 // bag_family: u32
        + 4 // totem_category: u32
        + 3 * 8 // sockets: ItemSocket[3]
        + 4 // socket_bonus: u32
        + 4 // gem_properties: u32
        + 4 // required_disenchant_skill: u32
        + 4 // armor_damage_modifier: f32
        + 4 // duration_in_seconds: u32
    }

}

