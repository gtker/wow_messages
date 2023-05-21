use std::io::{Read, Write};

use crate::vanilla::{
    AllowedClass, AllowedRace, Area, BagFamily, Bonding, Faction, InventoryType, 
    ItemClassAndSubClass, ItemDamageType, ItemFlag, ItemQuality, ItemSet, ItemSpells, 
    ItemStat, ItemStatType, Language, Map, PageTextMaterial, SheatheType, Skill, 
    SpellSchool, SpellTriggerType,
};
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:185`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L185):
/// ```text
/// smsg SMSG_ITEM_QUERY_SINGLE_RESPONSE = 0x0058 {
///     u32 item;
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
///         u32 required_spell;
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
pub struct SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    pub item: u32,
    pub found: Option<SMSG_ITEM_QUERY_SINGLE_RESPONSE_found>,
}

impl crate::private::Sealed for SMSG_ITEM_QUERY_SINGLE_RESPONSE {}
impl SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=1732).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0058, size: body_size });
        }

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // optional found
        let current_size = {
            4 // item: u32
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

            // required_spell: u32
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
                    *i = ItemStat::read(&mut r)?;
                }
                stats
            };

            // damages: ItemDamageType[5]
            let damages = {
                let mut damages = [ItemDamageType::default(); 5];
                for i in damages.iter_mut() {
                    *i = ItemDamageType::read(&mut r)?;
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
                    *i = ItemSpells::read(&mut r)?;
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
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ITEM_QUERY_SINGLE_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item).unwrap();
        if let Some(found) = &self.found {
            writeln!(s, "    class_and_sub_class = {};", found.class_and_sub_class.as_test_case_value()).unwrap();
            writeln!(s, "    name1 = \"{}\";", found.name1).unwrap();
            writeln!(s, "    name2 = \"{}\";", found.name2).unwrap();
            writeln!(s, "    name3 = \"{}\";", found.name3).unwrap();
            writeln!(s, "    name4 = \"{}\";", found.name4).unwrap();
            writeln!(s, "    display_id = {};", found.display_id).unwrap();
            writeln!(s, "    quality = {};", found.quality.as_test_case_value()).unwrap();
            writeln!(s, "    flags = {};", found.flags.as_test_case_value()).unwrap();
            writeln!(s, "    buy_price = {};", found.buy_price.as_int()).unwrap();
            writeln!(s, "    sell_price = {};", found.sell_price.as_int()).unwrap();
            writeln!(s, "    inventory_type = {};", found.inventory_type.as_test_case_value()).unwrap();
            writeln!(s, "    allowed_class = {};", found.allowed_class.as_test_case_value()).unwrap();
            writeln!(s, "    allowed_race = {};", found.allowed_race.as_test_case_value()).unwrap();
            writeln!(s, "    item_level = {};", found.item_level.as_int()).unwrap();
            writeln!(s, "    required_level = {};", found.required_level.as_int()).unwrap();
            writeln!(s, "    required_skill = {};", found.required_skill.as_test_case_value()).unwrap();
            writeln!(s, "    required_skill_rank = {};", found.required_skill_rank).unwrap();
            writeln!(s, "    required_spell = {};", found.required_spell).unwrap();
            writeln!(s, "    required_honor_rank = {};", found.required_honor_rank).unwrap();
            writeln!(s, "    required_city_rank = {};", found.required_city_rank).unwrap();
            writeln!(s, "    required_faction = {};", found.required_faction.as_test_case_value()).unwrap();
            writeln!(s, "    required_faction_rank = {};", found.required_faction_rank).unwrap();
            writeln!(s, "    max_count = {};", found.max_count).unwrap();
            writeln!(s, "    stackable = {};", found.stackable).unwrap();
            writeln!(s, "    container_slots = {};", found.container_slots).unwrap();
            write!(s, "    stats = [").unwrap();
            for v in found.stats.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "        stat_type = {};", v.stat_type.as_test_case_value()).unwrap();
                writeln!(s, "        value = {};", v.value).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();
            write!(s, "    damages = [").unwrap();
            for v in found.damages.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "    {}", if v.damage_minimum.to_string().contains('.') { v.damage_minimum.to_string() } else { format!("{}.0", v.damage_minimum) }).unwrap();
                writeln!(s, "    {}", if v.damage_maximum.to_string().contains('.') { v.damage_maximum.to_string() } else { format!("{}.0", v.damage_maximum) }).unwrap();
                writeln!(s, "        school = {};", v.school.as_test_case_value()).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();
            writeln!(s, "    armor = {};", found.armor).unwrap();
            writeln!(s, "    holy_resistance = {};", found.holy_resistance).unwrap();
            writeln!(s, "    fire_resistance = {};", found.fire_resistance).unwrap();
            writeln!(s, "    nature_resistance = {};", found.nature_resistance).unwrap();
            writeln!(s, "    frost_resistance = {};", found.frost_resistance).unwrap();
            writeln!(s, "    shadow_resistance = {};", found.shadow_resistance).unwrap();
            writeln!(s, "    arcane_resistance = {};", found.arcane_resistance).unwrap();
            writeln!(s, "    delay = {};", found.delay).unwrap();
            writeln!(s, "    ammo_type = {};", found.ammo_type).unwrap();
            writeln!(s, "    {}", if found.ranged_range_modification.to_string().contains('.') { found.ranged_range_modification.to_string() } else { format!("{}.0", found.ranged_range_modification) }).unwrap();
            write!(s, "    spells = [").unwrap();
            for v in found.spells.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "        spell = {};", v.spell).unwrap();
                writeln!(s, "        spell_trigger = {};", v.spell_trigger.as_test_case_value()).unwrap();
                writeln!(s, "        spell_charges = {};", v.spell_charges).unwrap();
                writeln!(s, "        spell_cooldown = {};", v.spell_cooldown).unwrap();
                writeln!(s, "        spell_category = {};", v.spell_category).unwrap();
                writeln!(s, "        spell_category_cooldown = {};", v.spell_category_cooldown).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();
            writeln!(s, "    bonding = {};", found.bonding.as_test_case_value()).unwrap();
            writeln!(s, "    description = \"{}\";", found.description).unwrap();
            writeln!(s, "    page_text = {};", found.page_text).unwrap();
            writeln!(s, "    language = {};", found.language.as_test_case_value()).unwrap();
            writeln!(s, "    page_text_material = {};", found.page_text_material.as_test_case_value()).unwrap();
            writeln!(s, "    start_quest = {};", found.start_quest).unwrap();
            writeln!(s, "    lock_id = {};", found.lock_id).unwrap();
            writeln!(s, "    material = {};", found.material).unwrap();
            writeln!(s, "    sheathe_type = {};", found.sheathe_type.as_test_case_value()).unwrap();
            writeln!(s, "    random_property = {};", found.random_property).unwrap();
            writeln!(s, "    block = {};", found.block).unwrap();
            writeln!(s, "    item_set = {};", found.item_set.as_test_case_value()).unwrap();
            writeln!(s, "    max_durability = {};", found.max_durability).unwrap();
            writeln!(s, "    area = {};", found.area.as_test_case_value()).unwrap();
            writeln!(s, "    map = {};", found.map.as_test_case_value()).unwrap();
            writeln!(s, "    bag_family = {};", found.bag_family.as_test_case_value()).unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 88_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
        if let Some(found) = &self.found {
            crate::util::write_bytes(&mut s, &mut bytes, 8, "class_and_sub_class", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name1.len() + 1, "name1", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name2.len() + 1, "name2", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name3.len() + 1, "name3", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name4.len() + 1, "name4", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "display_id", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "quality", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "buy_price", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "sell_price", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "inventory_type", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "allowed_class", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "allowed_race", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item_level", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "required_level", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "required_skill", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "required_skill_rank", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "required_spell", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "required_honor_rank", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "required_city_rank", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "required_faction", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "required_faction_rank", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "max_count", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "stackable", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "container_slots", "    ");
            writeln!(s, "    /* stats: ItemStat[10] start */").unwrap();
            for (i, v) in found.stats.iter().enumerate() {
                writeln!(s, "    /* stats: ItemStat[10] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "stat_type", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "value", "        ");
                writeln!(s, "    /* stats: ItemStat[10] {i} end */").unwrap();
            }
            writeln!(s, "    /* stats: ItemStat[10] end */").unwrap();
            writeln!(s, "    /* damages: ItemDamageType[5] start */").unwrap();
            for (i, v) in found.damages.iter().enumerate() {
                writeln!(s, "    /* damages: ItemDamageType[5] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "damage_minimum", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "damage_maximum", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "school", "        ");
                writeln!(s, "    /* damages: ItemDamageType[5] {i} end */").unwrap();
            }
            writeln!(s, "    /* damages: ItemDamageType[5] end */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "armor", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "holy_resistance", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "fire_resistance", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "nature_resistance", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "frost_resistance", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "shadow_resistance", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "arcane_resistance", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "delay", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "ammo_type", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "ranged_range_modification", "    ");
            writeln!(s, "    /* spells: ItemSpells[5] start */").unwrap();
            for (i, v) in found.spells.iter().enumerate() {
                writeln!(s, "    /* spells: ItemSpells[5] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_trigger", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_charges", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_cooldown", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_category", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_category_cooldown", "        ");
                writeln!(s, "    /* spells: ItemSpells[5] {i} end */").unwrap();
            }
            writeln!(s, "    /* spells: ItemSpells[5] end */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "bonding", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.description.len() + 1, "description", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "page_text", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "language", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "page_text_material", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "start_quest", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "lock_id", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "material", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "sheathe_type", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "random_property", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "block", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item_set", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "max_durability", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "bag_family", "    ");
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // optional found
        if let Some(v) = &self.found {
            // class_and_sub_class: ItemClassAndSubClass
            w.write_all(&(v.class_and_sub_class.as_int().to_le_bytes()))?;

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

            // required_spell: u32
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
                i.write_into_vec(&mut w)?;
            }

            // damages: ItemDamageType[5]
            for i in v.damages.iter() {
                i.write_into_vec(&mut w)?;
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
                i.write_into_vec(&mut w)?;
            }

            // bonding: Bonding
            w.write_all(&u32::from(v.bonding.as_int()).to_le_bytes())?;

            // description: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.description.as_bytes().iter().rev().next(), Some(&0_u8), "String `description` must not be null-terminated.");
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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ITEM_QUERY_SINGLE_RESPONSE {}

impl SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // item: u32
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
            + 4 // required_spell: u32
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

