use std::io::{Read, Write};

use crate::shared::gold_vanilla_tbc_wrath::Gold;
use crate::shared::level_vanilla_tbc_wrath::Level;
use crate::vanilla::{
    AllowedClass, AllowedRace, Area, BagFamily, Bonding, Faction, InventoryType, 
    ItemClassAndSubClass, ItemDamageType, ItemFlag, ItemQuality, ItemSet, ItemSpells, 
    ItemStat, Language, Map, PageTextMaterial, SheatheType, Skill,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:172`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L172):
/// ```text
/// smsg SMSG_ITEM_QUERY_SINGLE_RESPONSE = 0x0058 {
///     Item item;
///     optional found {
///         ItemClassAndSubClass class_and_sub_class;
///         CString name1;
///         CString name2;
///         CString name3;
///         CString name4;
///         u32 display_id;
///         (u32)ItemQuality quality;
///         ItemFlag flags;
///         Gold buy_price;
///         Gold sell_price;
///         (u32)InventoryType inventory_type;
///         AllowedClass allowed_class;
///         AllowedRace allowed_race;
///         Level32 item_level;
///         Level32 required_level;
///         (u32)Skill required_skill;
///         u32 required_skill_rank;
///         Spell required_spell;
///         u32 required_honor_rank;
///         u32 required_city_rank;
///         (u32)Faction required_faction;
///         u32 required_faction_rank;
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
///         (u32)Bonding bonding;
///         CString description;
///         u32 page_text;
///         Language language;
///         (u32)PageTextMaterial page_text_material;
///         u32 start_quest;
///         u32 lock_id;
///         u32 material;
///         (u32)SheatheType sheathe_type;
///         u32 random_property;
///         u32 block;
///         (u32)ItemSet item_set;
///         u32 max_durability;
///         Area area;
///         Map map;
///         (u32)BagFamily bag_family;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    pub item: u32,
    pub found: Option<SMSG_ITEM_QUERY_SINGLE_RESPONSE_found>,
}

impl crate::private::Sealed for SMSG_ITEM_QUERY_SINGLE_RESPONSE {}
impl SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=1732).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // optional found
        let current_size = {
            4 // item: Item
        };
        let found = if current_size < body_size as usize {
            // class_and_sub_class: ItemClassAndSubClass
            let class_and_sub_class = crate::util::read_u64_le(&mut r)?.try_into()?;

            // name1: CString
            let name1 = {
                let name1 = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(name1)?
            };

            // name2: CString
            let name2 = {
                let name2 = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(name2)?
            };

            // name3: CString
            let name3 = {
                let name3 = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(name3)?
            };

            // name4: CString
            let name4 = {
                let name4 = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(name4)?
            };

            // display_id: u32
            let display_id = crate::util::read_u32_le(&mut r)?;

            // quality: ItemQuality
            let quality = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

            // flags: ItemFlag
            let flags = ItemFlag::new(crate::util::read_u32_le(&mut r)?);

            // buy_price: Gold
            let buy_price = Gold::new(crate::util::read_u32_le(&mut r)?);

            // sell_price: Gold
            let sell_price = Gold::new(crate::util::read_u32_le(&mut r)?);

            // inventory_type: InventoryType
            let inventory_type = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

            // allowed_class: AllowedClass
            let allowed_class = AllowedClass::new(crate::util::read_u32_le(&mut r)?);

            // allowed_race: AllowedRace
            let allowed_race = AllowedRace::new(crate::util::read_u32_le(&mut r)?);

            // item_level: Level32
            let item_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

            // required_level: Level32
            let required_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

            // required_skill: Skill
            let required_skill = (crate::util::read_u32_le(&mut r)? as u16).try_into()?;

            // required_skill_rank: u32
            let required_skill_rank = crate::util::read_u32_le(&mut r)?;

            // required_spell: Spell
            let required_spell = crate::util::read_u32_le(&mut r)?;

            // required_honor_rank: u32
            let required_honor_rank = crate::util::read_u32_le(&mut r)?;

            // required_city_rank: u32
            let required_city_rank = crate::util::read_u32_le(&mut r)?;

            // required_faction: Faction
            let required_faction = (crate::util::read_u32_le(&mut r)? as u16).try_into()?;

            // required_faction_rank: u32
            let required_faction_rank = crate::util::read_u32_le(&mut r)?;

            // max_count: u32
            let max_count = crate::util::read_u32_le(&mut r)?;

            // stackable: u32
            let stackable = crate::util::read_u32_le(&mut r)?;

            // container_slots: u32
            let container_slots = crate::util::read_u32_le(&mut r)?;

            // stats: ItemStat[10]
            let stats = {
                let mut stats = [ItemStat::default(); 10];
                for i in stats.iter_mut() {
                    *i = crate::util::vanilla_itemstat_read(&mut r)?;
                }
                stats
            };

            // damages: ItemDamageType[5]
            let damages = {
                let mut damages = [ItemDamageType::default(); 5];
                for i in damages.iter_mut() {
                    *i = crate::util::vanilla_tbc_wrath_itemdamagetype_read(&mut r)?;
                }
                damages
            };

            // armor: i32
            let armor = crate::util::read_i32_le(&mut r)?;

            // holy_resistance: i32
            let holy_resistance = crate::util::read_i32_le(&mut r)?;

            // fire_resistance: i32
            let fire_resistance = crate::util::read_i32_le(&mut r)?;

            // nature_resistance: i32
            let nature_resistance = crate::util::read_i32_le(&mut r)?;

            // frost_resistance: i32
            let frost_resistance = crate::util::read_i32_le(&mut r)?;

            // shadow_resistance: i32
            let shadow_resistance = crate::util::read_i32_le(&mut r)?;

            // arcane_resistance: i32
            let arcane_resistance = crate::util::read_i32_le(&mut r)?;

            // delay: u32
            let delay = crate::util::read_u32_le(&mut r)?;

            // ammo_type: u32
            let ammo_type = crate::util::read_u32_le(&mut r)?;

            // ranged_range_modification: f32
            let ranged_range_modification = crate::util::read_f32_le(&mut r)?;

            // spells: ItemSpells[5]
            let spells = {
                let mut spells = [ItemSpells::default(); 5];
                for i in spells.iter_mut() {
                    *i = crate::util::vanilla_itemspells_read(&mut r)?;
                }
                spells
            };

            // bonding: Bonding
            let bonding = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

            // description: CString
            let description = {
                let description = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(description)?
            };

            // page_text: u32
            let page_text = crate::util::read_u32_le(&mut r)?;

            // language: Language
            let language = crate::util::read_u32_le(&mut r)?.try_into()?;

            // page_text_material: PageTextMaterial
            let page_text_material = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

            // start_quest: u32
            let start_quest = crate::util::read_u32_le(&mut r)?;

            // lock_id: u32
            let lock_id = crate::util::read_u32_le(&mut r)?;

            // material: u32
            let material = crate::util::read_u32_le(&mut r)?;

            // sheathe_type: SheatheType
            let sheathe_type = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

            // random_property: u32
            let random_property = crate::util::read_u32_le(&mut r)?;

            // block: u32
            let block = crate::util::read_u32_le(&mut r)?;

            // item_set: ItemSet
            let item_set = (crate::util::read_u32_le(&mut r)? as u16).try_into()?;

            // max_durability: u32
            let max_durability = crate::util::read_u32_le(&mut r)?;

            // area: Area
            let area = crate::util::read_u32_le(&mut r)?.try_into()?;

            // map: Map
            let map = crate::util::read_u32_le(&mut r)?.try_into()?;

            // bag_family: BagFamily
            let bag_family = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

            Some(SMSG_ITEM_QUERY_SINGLE_RESPONSE_found {
                class_and_sub_class,
                name1,
                name2,
                name3,
                name4,
                display_id,
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
                required_faction,
                required_faction_rank,
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
                language,
                page_text_material,
                start_quest,
                lock_id,
                material,
                sheathe_type,
                random_property,
                block,
                item_set,
                max_durability,
                area,
                map,
                bag_family,
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

impl crate::Message for SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    const OPCODE: u32 = 0x0058;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ITEM_QUERY_SINGLE_RESPONSE"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // optional found
        if let Some(v) = &self.found {
            // class_and_sub_class: ItemClassAndSubClass
            w.write_all(&(v.class_and_sub_class.as_int().to_le_bytes()))?;

            // name1: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name1.as_bytes().iter().next_back(), Some(&0_u8), "String `name1` must not be null-terminated.");
            w.write_all(v.name1.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name2: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name2.as_bytes().iter().next_back(), Some(&0_u8), "String `name2` must not be null-terminated.");
            w.write_all(v.name2.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name3: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name3.as_bytes().iter().next_back(), Some(&0_u8), "String `name3` must not be null-terminated.");
            w.write_all(v.name3.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name4: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name4.as_bytes().iter().next_back(), Some(&0_u8), "String `name4` must not be null-terminated.");
            w.write_all(v.name4.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // display_id: u32
            w.write_all(&v.display_id.to_le_bytes())?;

            // quality: ItemQuality
            w.write_all(&u32::from(v.quality.as_int()).to_le_bytes())?;

            // flags: ItemFlag
            w.write_all(&(v.flags.as_int().to_le_bytes()))?;

            // buy_price: Gold
            w.write_all((v.buy_price.as_int()).to_le_bytes().as_slice())?;

            // sell_price: Gold
            w.write_all((v.sell_price.as_int()).to_le_bytes().as_slice())?;

            // inventory_type: InventoryType
            w.write_all(&u32::from(v.inventory_type.as_int()).to_le_bytes())?;

            // allowed_class: AllowedClass
            w.write_all(&(v.allowed_class.as_int().to_le_bytes()))?;

            // allowed_race: AllowedRace
            w.write_all(&(v.allowed_race.as_int().to_le_bytes()))?;

            // item_level: Level32
            w.write_all(&u32::from(v.item_level.as_int()).to_le_bytes())?;

            // required_level: Level32
            w.write_all(&u32::from(v.required_level.as_int()).to_le_bytes())?;

            // required_skill: Skill
            w.write_all(&u32::from(v.required_skill.as_int()).to_le_bytes())?;

            // required_skill_rank: u32
            w.write_all(&v.required_skill_rank.to_le_bytes())?;

            // required_spell: Spell
            w.write_all(&v.required_spell.to_le_bytes())?;

            // required_honor_rank: u32
            w.write_all(&v.required_honor_rank.to_le_bytes())?;

            // required_city_rank: u32
            w.write_all(&v.required_city_rank.to_le_bytes())?;

            // required_faction: Faction
            w.write_all(&u32::from(v.required_faction.as_int()).to_le_bytes())?;

            // required_faction_rank: u32
            w.write_all(&v.required_faction_rank.to_le_bytes())?;

            // max_count: u32
            w.write_all(&v.max_count.to_le_bytes())?;

            // stackable: u32
            w.write_all(&v.stackable.to_le_bytes())?;

            // container_slots: u32
            w.write_all(&v.container_slots.to_le_bytes())?;

            // stats: ItemStat[10]
            for i in v.stats.iter() {
                crate::util::vanilla_itemstat_write_into_vec(i, &mut w)?;
            }

            // damages: ItemDamageType[5]
            for i in v.damages.iter() {
                crate::util::vanilla_tbc_wrath_itemdamagetype_write_into_vec(i, &mut w)?;
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
                crate::util::vanilla_itemspells_write_into_vec(i, &mut w)?;
            }

            // bonding: Bonding
            w.write_all(&u32::from(v.bonding.as_int()).to_le_bytes())?;

            // description: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.description.as_bytes().iter().next_back(), Some(&0_u8), "String `description` must not be null-terminated.");
            w.write_all(v.description.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // page_text: u32
            w.write_all(&v.page_text.to_le_bytes())?;

            // language: Language
            w.write_all(&(v.language.as_int().to_le_bytes()))?;

            // page_text_material: PageTextMaterial
            w.write_all(&u32::from(v.page_text_material.as_int()).to_le_bytes())?;

            // start_quest: u32
            w.write_all(&v.start_quest.to_le_bytes())?;

            // lock_id: u32
            w.write_all(&v.lock_id.to_le_bytes())?;

            // material: u32
            w.write_all(&v.material.to_le_bytes())?;

            // sheathe_type: SheatheType
            w.write_all(&u32::from(v.sheathe_type.as_int()).to_le_bytes())?;

            // random_property: u32
            w.write_all(&v.random_property.to_le_bytes())?;

            // block: u32
            w.write_all(&v.block.to_le_bytes())?;

            // item_set: ItemSet
            w.write_all(&u32::from(v.item_set.as_int()).to_le_bytes())?;

            // max_durability: u32
            w.write_all(&v.max_durability.to_le_bytes())?;

            // area: Area
            w.write_all(&(v.area.as_int().to_le_bytes()))?;

            // map: Map
            w.write_all(&(v.map.as_int().to_le_bytes()))?;

            // bag_family: BagFamily
            w.write_all(&u32::from(v.bag_family.as_int()).to_le_bytes())?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(88, "SMSG_ITEM_QUERY_SINGLE_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ITEM_QUERY_SINGLE_RESPONSE {}

impl SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // item: Item
        + if let Some(found) = &self.found {
            8 // class_and_sub_class: ItemClassAndSubClass
            + found.name1.len() + 1 // name1: CString
            + found.name2.len() + 1 // name2: CString
            + found.name3.len() + 1 // name3: CString
            + found.name4.len() + 1 // name4: CString
            + 4 // display_id: u32
            + 4 // quality: ItemQuality
            + 4 // flags: ItemFlag
            + 4 // buy_price: Gold
            + 4 // sell_price: Gold
            + 4 // inventory_type: InventoryType
            + 4 // allowed_class: AllowedClass
            + 4 // allowed_race: AllowedRace
            + 4 // item_level: Level32
            + 4 // required_level: Level32
            + 4 // required_skill: Skill
            + 4 // required_skill_rank: u32
            + 4 // required_spell: Spell
            + 4 // required_honor_rank: u32
            + 4 // required_city_rank: u32
            + 4 // required_faction: Faction
            + 4 // required_faction_rank: u32
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
            + 4 // bonding: Bonding
            + found.description.len() + 1 // description: CString
            + 4 // page_text: u32
            + 4 // language: Language
            + 4 // page_text_material: PageTextMaterial
            + 4 // start_quest: u32
            + 4 // lock_id: u32
            + 4 // material: u32
            + 4 // sheathe_type: SheatheType
            + 4 // random_property: u32
            + 4 // block: u32
            + 4 // item_set: ItemSet
            + 4 // max_durability: u32
            + 4 // area: Area
            + 4 // map: Map
            + 4 // bag_family: BagFamily
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_ITEM_QUERY_SINGLE_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 482] = [ 0x01, 0xE0, 0x58, 0x00, 0x3E, 0x1C, 0x00, 0x00, 0x02,
         0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x53, 0x6D, 0x69, 0x74, 0x65,
         0x27, 0x73, 0x20, 0x4D, 0x69, 0x67, 0x68, 0x74, 0x79, 0x20, 0x48, 0x61,
         0x6D, 0x6D, 0x65, 0x72, 0x00, 0x00, 0x00, 0x00, 0x9A, 0x4C, 0x00, 0x00,
         0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9B, 0x3C, 0x00, 0x00,
         0x1F, 0x0C, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0xDF, 0x05, 0x00, 0x00,
         0xFF, 0x01, 0x00, 0x00, 0x17, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
         0x0B, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
         0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x5C, 0x42, 0x00, 0x00, 0xA6, 0x42, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0xAC, 0x0D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ITEM_QUERY_SINGLE_RESPONSE {
        SMSG_ITEM_QUERY_SINGLE_RESPONSE {
            item: 0x1C3E,
            found: Some(SMSG_ITEM_QUERY_SINGLE_RESPONSE_found {
                class_and_sub_class: ItemClassAndSubClass::TwoHandedMace,
                name1: String::from("Smite's Mighty Hammer"),
                name2: String::from(""),
                name3: String::from(""),
                name4: String::from(""),
                display_id: 0x4C9A,
                quality: ItemQuality::Rare,
                flags: ItemFlag::empty()
                    ,
                buy_price: Gold::try_from(0x3C9B).unwrap(),
                sell_price: Gold::try_from(0xC1F).unwrap(),
                inventory_type: InventoryType::TwoHandedWeapon,
                allowed_class: AllowedClass::empty()
                    .set_warrior()
                    .set_paladin()
                    .set_hunter()
                    .set_rogue()
                    .set_priest()
                    .set_shaman()
                    .set_mage()
                    .set_warlock()
                    .set_druid()
                    ,
                allowed_race: AllowedRace::empty()
                    .set_human()
                    .set_orc()
                    .set_dwarf()
                    .set_night_elf()
                    .set_undead()
                    .set_tauren()
                    .set_gnome()
                    .set_troll()
                    .set_goblin()
                    ,
                item_level: Level::try_from(0x17).unwrap(),
                required_level: Level::try_from(0x12).unwrap(),
                required_skill: Skill::None,
                required_skill_rank: 0x0,
                required_spell: 0x0,
                required_honor_rank: 0x0,
                required_city_rank: 0x0,
                required_faction: Faction::None,
                required_faction_rank: 0x0,
                max_count: 0x0,
                stackable: 0x1,
                container_slots: 0x0,
                stats: [
                    ItemStat {
                        stat_type: ItemStatType::Mana,
                        value: 0x0,
                    },
                    ItemStat {
                        stat_type: ItemStatType::Health,
                        value: 0x0,
                    },
                    ItemStat {
                        stat_type: ItemStatType::Strength,
                        value: 0xB,
                    },
                    ItemStat {
                        stat_type: ItemStatType::Agility,
                        value: 0x4,
                    },
                    ItemStat {
                        stat_type: ItemStatType::Stamina,
                        value: 0x0,
                    },
                    ItemStat {
                        stat_type: ItemStatType::Intellect,
                        value: 0x0,
                    },
                    ItemStat {
                        stat_type: ItemStatType::Spirit,
                        value: 0x0,
                    },
                    ItemStat {
                        stat_type: ItemStatType::Mana,
                        value: 0x0,
                    },
                    ItemStat {
                        stat_type: ItemStatType::Mana,
                        value: 0x0,
                    },
                    ItemStat {
                        stat_type: ItemStatType::Mana,
                        value: 0x0,
                    },
                ],
                damages: [
                    ItemDamageType {
                        damage_minimum: 55_f32,
                        damage_maximum: 83_f32,
                        school: SpellSchool::Normal,
                    },
                    ItemDamageType {
                        damage_minimum: 0_f32,
                        damage_maximum: 0_f32,
                        school: SpellSchool::Normal,
                    },
                    ItemDamageType {
                        damage_minimum: 0_f32,
                        damage_maximum: 0_f32,
                        school: SpellSchool::Normal,
                    },
                    ItemDamageType {
                        damage_minimum: 0_f32,
                        damage_maximum: 0_f32,
                        school: SpellSchool::Normal,
                    },
                    ItemDamageType {
                        damage_minimum: 0_f32,
                        damage_maximum: 0_f32,
                        school: SpellSchool::Normal,
                    },
                ],
                armor: 0x0,
                holy_resistance: 0x0,
                fire_resistance: 0x0,
                nature_resistance: 0x0,
                frost_resistance: 0x0,
                shadow_resistance: 0x0,
                arcane_resistance: 0x0,
                delay: 0xDAC,
                ammo_type: 0x0,
                ranged_range_modification: 0_f32,
                spells: [
                    ItemSpells {
                        spell: 0x0,
                        spell_trigger: SpellTriggerType::OnUse,
                        spell_charges: 0x0,
                        spell_cooldown: 0x0,
                        spell_category: 0x0,
                        spell_category_cooldown: 0x0,
                    },
                    ItemSpells {
                        spell: 0x0,
                        spell_trigger: SpellTriggerType::OnUse,
                        spell_charges: 0x0,
                        spell_cooldown: 0x0,
                        spell_category: 0x0,
                        spell_category_cooldown: 0x0,
                    },
                    ItemSpells {
                        spell: 0x0,
                        spell_trigger: SpellTriggerType::OnUse,
                        spell_charges: 0x0,
                        spell_cooldown: 0x0,
                        spell_category: 0x0,
                        spell_category_cooldown: 0x0,
                    },
                    ItemSpells {
                        spell: 0x0,
                        spell_trigger: SpellTriggerType::OnUse,
                        spell_charges: 0x0,
                        spell_cooldown: 0x0,
                        spell_category: 0x0,
                        spell_category_cooldown: 0x0,
                    },
                    ItemSpells {
                        spell: 0x0,
                        spell_trigger: SpellTriggerType::OnUse,
                        spell_charges: 0x0,
                        spell_cooldown: 0x0,
                        spell_category: 0x0,
                        spell_category_cooldown: 0x0,
                    },
                ],
                bonding: Bonding::PickUp,
                description: String::from(""),
                page_text: 0x0,
                language: Language::Universal,
                page_text_material: PageTextMaterial::None,
                start_quest: 0x0,
                lock_id: 0x0,
                material: 0x2,
                sheathe_type: SheatheType::MainHand,
                random_property: 0x0,
                block: 0x0,
                item_set: ItemSet::None,
                max_durability: 0x50,
                area: Area::None,
                map: Map::EasternKingdoms,
                bag_family: BagFamily::None,
            })
        }

    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm` line 239.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_item_query_single_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ITEM_QUERY_SINGLE_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ITEM_QUERY_SINGLE_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm` line 239.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_item_query_single_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ITEM_QUERY_SINGLE_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ITEM_QUERY_SINGLE_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm` line 239.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_item_query_single_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ITEM_QUERY_SINGLE_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ITEM_QUERY_SINGLE_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_ITEM_QUERY_SINGLE_RESPONSE_found {
    pub class_and_sub_class: ItemClassAndSubClass,
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub name4: String,
    pub display_id: u32,
    pub quality: ItemQuality,
    pub flags: ItemFlag,
    pub buy_price: Gold,
    pub sell_price: Gold,
    pub inventory_type: InventoryType,
    pub allowed_class: AllowedClass,
    pub allowed_race: AllowedRace,
    pub item_level: Level,
    pub required_level: Level,
    pub required_skill: Skill,
    pub required_skill_rank: u32,
    pub required_spell: u32,
    pub required_honor_rank: u32,
    pub required_city_rank: u32,
    pub required_faction: Faction,
    pub required_faction_rank: u32,
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
    pub bonding: Bonding,
    pub description: String,
    pub page_text: u32,
    pub language: Language,
    pub page_text_material: PageTextMaterial,
    pub start_quest: u32,
    pub lock_id: u32,
    pub material: u32,
    pub sheathe_type: SheatheType,
    pub random_property: u32,
    pub block: u32,
    pub item_set: ItemSet,
    pub max_durability: u32,
    pub area: Area,
    pub map: Map,
    pub bag_family: BagFamily,
}

