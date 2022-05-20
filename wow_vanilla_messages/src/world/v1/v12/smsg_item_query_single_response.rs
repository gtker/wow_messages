use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{InventoryType, InventoryTypeError};
use crate::world::v1::v12::{ItemClass, ItemClassError};
use crate::world::v1::v12::ItemDamageType;
use crate::world::v1::v12::{ItemQuality, ItemQualityError};
use crate::world::v1::v12::ItemSpells;
use crate::world::v1::v12::ItemStat;
use crate::world::v1::v12::{Map, MapError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    pub item: u32,
    pub found: Option<SMSG_ITEM_QUERY_SINGLE_RESPONSEfound>,
}

impl ServerMessageWrite for SMSG_ITEM_QUERY_SINGLE_RESPONSE {}

impl SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // optional found
        if let Some(v) = &self.found {
            // item_class: ItemClass
            w.write_all(&(v.item_class.as_int() as u32).to_le_bytes())?;

            // item_sub_class: u32
            w.write_all(&v.item_sub_class.to_le_bytes())?;

            // name1: CString
            w.write_all(v.name1.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name2: CString
            w.write_all(v.name2.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name3: CString
            w.write_all(v.name3.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name4: CString
            w.write_all(v.name4.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // item_display_info: u32
            w.write_all(&v.item_display_info.to_le_bytes())?;

            // quality: ItemQuality
            w.write_all(&(v.quality.as_int() as u32).to_le_bytes())?;

            // flags: u32
            w.write_all(&v.flags.to_le_bytes())?;

            // buy_price: f64
            w.write_all(&v.buy_price.to_le_bytes())?;

            // sell_price: f64
            w.write_all(&v.sell_price.to_le_bytes())?;

            // inventory_type: InventoryType
            w.write_all(&(v.inventory_type.as_int() as u8).to_le_bytes())?;

            // allowed_class: u32
            w.write_all(&v.allowed_class.to_le_bytes())?;

            // allowed_race: u32
            w.write_all(&v.allowed_race.to_le_bytes())?;

            // item_level: u32
            w.write_all(&v.item_level.to_le_bytes())?;

            // required_level: u32
            w.write_all(&v.required_level.to_le_bytes())?;

            // required_skill: u32
            w.write_all(&v.required_skill.to_le_bytes())?;

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
                w.write_all(&(i.as_bytes()?))?;
            }

            // damages: ItemDamageType[5]
            for i in v.damages.iter() {
                w.write_all(&(i.as_bytes()?))?;
            }

            // armor: u32
            w.write_all(&v.armor.to_le_bytes())?;

            // holy_resistance: u32
            w.write_all(&v.holy_resistance.to_le_bytes())?;

            // fire_resistance: u32
            w.write_all(&v.fire_resistance.to_le_bytes())?;

            // nature_resistance: u32
            w.write_all(&v.nature_resistance.to_le_bytes())?;

            // frost_resistance: u32
            w.write_all(&v.frost_resistance.to_le_bytes())?;

            // shadow_resistance: u32
            w.write_all(&v.shadow_resistance.to_le_bytes())?;

            // arcane_resistance: u32
            w.write_all(&v.arcane_resistance.to_le_bytes())?;

            // delay: u32
            w.write_all(&v.delay.to_le_bytes())?;

            // ammo_type: u32
            w.write_all(&v.ammo_type.to_le_bytes())?;

            // ranged_range_modification: f32
            w.write_all(&v.ranged_range_modification.to_le_bytes())?;

            // spells: ItemSpells[5]
            for i in v.spells.iter() {
                w.write_all(&(i.as_bytes()?))?;
            }

            // bonding: u32
            w.write_all(&v.bonding.to_le_bytes())?;

            // description: CString
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

        }

        Ok(w)
    }
}

impl MessageBody for SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    const OPCODE: u16 = 0x0058;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_ITEM_QUERY_SINGLE_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // optional found
        let current_size = {
            0
            + 4 // item: u32
        };
        let found = if current_size < body_size as usize {
            // item_class: ItemClass
            let item_class: ItemClass = (crate::util::read_u32_le(r)? as u8).try_into()?;

            // item_sub_class: u32
            let item_sub_class = crate::util::read_u32_le(r)?;

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

            // buy_price: f64
            let buy_price = crate::util::read_f64_le(r)?;
            // sell_price: f64
            let sell_price = crate::util::read_f64_le(r)?;
            // inventory_type: InventoryType
            let inventory_type: InventoryType = crate::util::read_u8_le(r)?.try_into()?;

            // allowed_class: u32
            let allowed_class = crate::util::read_u32_le(r)?;

            // allowed_race: u32
            let allowed_race = crate::util::read_u32_le(r)?;

            // item_level: u32
            let item_level = crate::util::read_u32_le(r)?;

            // required_level: u32
            let required_level = crate::util::read_u32_le(r)?;

            // required_skill: u32
            let required_skill = crate::util::read_u32_le(r)?;

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
            let mut stats = Vec::with_capacity(10 as usize);
            for i in 0..10 {
                stats.push(ItemStat::read(r)?);
            }
            let stats = stats.try_into().unwrap();

            // damages: ItemDamageType[5]
            let mut damages = Vec::with_capacity(5 as usize);
            for i in 0..5 {
                damages.push(ItemDamageType::read(r)?);
            }
            let damages = damages.try_into().unwrap();

            // armor: u32
            let armor = crate::util::read_u32_le(r)?;

            // holy_resistance: u32
            let holy_resistance = crate::util::read_u32_le(r)?;

            // fire_resistance: u32
            let fire_resistance = crate::util::read_u32_le(r)?;

            // nature_resistance: u32
            let nature_resistance = crate::util::read_u32_le(r)?;

            // frost_resistance: u32
            let frost_resistance = crate::util::read_u32_le(r)?;

            // shadow_resistance: u32
            let shadow_resistance = crate::util::read_u32_le(r)?;

            // arcane_resistance: u32
            let arcane_resistance = crate::util::read_u32_le(r)?;

            // delay: u32
            let delay = crate::util::read_u32_le(r)?;

            // ammo_type: u32
            let ammo_type = crate::util::read_u32_le(r)?;

            // ranged_range_modification: f32
            let ranged_range_modification = crate::util::read_f32_le(r)?;
            // spells: ItemSpells[5]
            let mut spells = Vec::with_capacity(5 as usize);
            for i in 0..5 {
                spells.push(ItemSpells::read(r)?);
            }
            let spells = spells.try_into().unwrap();

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

            Some(SMSG_ITEM_QUERY_SINGLE_RESPONSEfound {
                item_class,
                item_sub_class,
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
            })
        } else {
            None
        };

        Ok(Self {
            item,
            found,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // item: u32
            let item = crate::util::tokio_read_u32_le(r).await?;

            // optional found
            let current_size = {
                0
                + 4 // item: u32
            };
            let found = if current_size < body_size as usize {
                // item_class: ItemClass
                let item_class: ItemClass = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

                // item_sub_class: u32
                let item_sub_class = crate::util::tokio_read_u32_le(r).await?;

                // name1: CString
                let name1 = crate::util::tokio_read_c_string_to_vec(r).await?;
                let name1 = String::from_utf8(name1)?;

                // name2: CString
                let name2 = crate::util::tokio_read_c_string_to_vec(r).await?;
                let name2 = String::from_utf8(name2)?;

                // name3: CString
                let name3 = crate::util::tokio_read_c_string_to_vec(r).await?;
                let name3 = String::from_utf8(name3)?;

                // name4: CString
                let name4 = crate::util::tokio_read_c_string_to_vec(r).await?;
                let name4 = String::from_utf8(name4)?;

                // item_display_info: u32
                let item_display_info = crate::util::tokio_read_u32_le(r).await?;

                // quality: ItemQuality
                let quality: ItemQuality = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

                // flags: u32
                let flags = crate::util::tokio_read_u32_le(r).await?;

                // buy_price: f64
                let buy_price = crate::util::tokio_read_f64_le(r).await?;
                // sell_price: f64
                let sell_price = crate::util::tokio_read_f64_le(r).await?;
                // inventory_type: InventoryType
                let inventory_type: InventoryType = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                // allowed_class: u32
                let allowed_class = crate::util::tokio_read_u32_le(r).await?;

                // allowed_race: u32
                let allowed_race = crate::util::tokio_read_u32_le(r).await?;

                // item_level: u32
                let item_level = crate::util::tokio_read_u32_le(r).await?;

                // required_level: u32
                let required_level = crate::util::tokio_read_u32_le(r).await?;

                // required_skill: u32
                let required_skill = crate::util::tokio_read_u32_le(r).await?;

                // required_skill_rank: u32
                let required_skill_rank = crate::util::tokio_read_u32_le(r).await?;

                // required_spell: u32
                let required_spell = crate::util::tokio_read_u32_le(r).await?;

                // required_honor_rank: u32
                let required_honor_rank = crate::util::tokio_read_u32_le(r).await?;

                // required_city_rank: u32
                let required_city_rank = crate::util::tokio_read_u32_le(r).await?;

                // required_reputation_faction: u32
                let required_reputation_faction = crate::util::tokio_read_u32_le(r).await?;

                // required_reputation_rank: u32
                let required_reputation_rank = crate::util::tokio_read_u32_le(r).await?;

                // max_count: u32
                let max_count = crate::util::tokio_read_u32_le(r).await?;

                // stackable: u32
                let stackable = crate::util::tokio_read_u32_le(r).await?;

                // container_slots: u32
                let container_slots = crate::util::tokio_read_u32_le(r).await?;

                // stats: ItemStat[10]
                let mut stats = Vec::with_capacity(10 as usize);
                for i in 0..10 {
                    stats.push(ItemStat::tokio_read(r).await?);
                }
                let stats = stats.try_into().unwrap();

                // damages: ItemDamageType[5]
                let mut damages = Vec::with_capacity(5 as usize);
                for i in 0..5 {
                    damages.push(ItemDamageType::tokio_read(r).await?);
                }
                let damages = damages.try_into().unwrap();

                // armor: u32
                let armor = crate::util::tokio_read_u32_le(r).await?;

                // holy_resistance: u32
                let holy_resistance = crate::util::tokio_read_u32_le(r).await?;

                // fire_resistance: u32
                let fire_resistance = crate::util::tokio_read_u32_le(r).await?;

                // nature_resistance: u32
                let nature_resistance = crate::util::tokio_read_u32_le(r).await?;

                // frost_resistance: u32
                let frost_resistance = crate::util::tokio_read_u32_le(r).await?;

                // shadow_resistance: u32
                let shadow_resistance = crate::util::tokio_read_u32_le(r).await?;

                // arcane_resistance: u32
                let arcane_resistance = crate::util::tokio_read_u32_le(r).await?;

                // delay: u32
                let delay = crate::util::tokio_read_u32_le(r).await?;

                // ammo_type: u32
                let ammo_type = crate::util::tokio_read_u32_le(r).await?;

                // ranged_range_modification: f32
                let ranged_range_modification = crate::util::tokio_read_f32_le(r).await?;
                // spells: ItemSpells[5]
                let mut spells = Vec::with_capacity(5 as usize);
                for i in 0..5 {
                    spells.push(ItemSpells::tokio_read(r).await?);
                }
                let spells = spells.try_into().unwrap();

                // bonding: u32
                let bonding = crate::util::tokio_read_u32_le(r).await?;

                // description: CString
                let description = crate::util::tokio_read_c_string_to_vec(r).await?;
                let description = String::from_utf8(description)?;

                // page_text: u32
                let page_text = crate::util::tokio_read_u32_le(r).await?;

                // language_id: u32
                let language_id = crate::util::tokio_read_u32_le(r).await?;

                // page_material: u32
                let page_material = crate::util::tokio_read_u32_le(r).await?;

                // start_quest: u32
                let start_quest = crate::util::tokio_read_u32_le(r).await?;

                // lock_id: u32
                let lock_id = crate::util::tokio_read_u32_le(r).await?;

                // material: u32
                let material = crate::util::tokio_read_u32_le(r).await?;

                // sheath: u32
                let sheath = crate::util::tokio_read_u32_le(r).await?;

                // random_property: u32
                let random_property = crate::util::tokio_read_u32_le(r).await?;

                // block: u32
                let block = crate::util::tokio_read_u32_le(r).await?;

                // item_set: u32
                let item_set = crate::util::tokio_read_u32_le(r).await?;

                // max_durability: u32
                let max_durability = crate::util::tokio_read_u32_le(r).await?;

                // area: Area
                let area: Area = crate::util::tokio_read_u32_le(r).await?.try_into()?;

                // map: Map
                let map: Map = crate::util::tokio_read_u32_le(r).await?.try_into()?;

                // bag_family: u32
                let bag_family = crate::util::tokio_read_u32_le(r).await?;

                Some(SMSG_ITEM_QUERY_SINGLE_RESPONSEfound {
                    item_class,
                    item_sub_class,
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
                })
            } else {
                None
            };

            Ok(Self {
                item,
                found,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // item: u32
            let item = crate::util::astd_read_u32_le(r).await?;

            // optional found
            let current_size = {
                0
                + 4 // item: u32
            };
            let found = if current_size < body_size as usize {
                // item_class: ItemClass
                let item_class: ItemClass = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

                // item_sub_class: u32
                let item_sub_class = crate::util::astd_read_u32_le(r).await?;

                // name1: CString
                let name1 = crate::util::astd_read_c_string_to_vec(r).await?;
                let name1 = String::from_utf8(name1)?;

                // name2: CString
                let name2 = crate::util::astd_read_c_string_to_vec(r).await?;
                let name2 = String::from_utf8(name2)?;

                // name3: CString
                let name3 = crate::util::astd_read_c_string_to_vec(r).await?;
                let name3 = String::from_utf8(name3)?;

                // name4: CString
                let name4 = crate::util::astd_read_c_string_to_vec(r).await?;
                let name4 = String::from_utf8(name4)?;

                // item_display_info: u32
                let item_display_info = crate::util::astd_read_u32_le(r).await?;

                // quality: ItemQuality
                let quality: ItemQuality = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

                // flags: u32
                let flags = crate::util::astd_read_u32_le(r).await?;

                // buy_price: f64
                let buy_price = crate::util::astd_read_f64_le(r).await?;
                // sell_price: f64
                let sell_price = crate::util::astd_read_f64_le(r).await?;
                // inventory_type: InventoryType
                let inventory_type: InventoryType = crate::util::astd_read_u8_le(r).await?.try_into()?;

                // allowed_class: u32
                let allowed_class = crate::util::astd_read_u32_le(r).await?;

                // allowed_race: u32
                let allowed_race = crate::util::astd_read_u32_le(r).await?;

                // item_level: u32
                let item_level = crate::util::astd_read_u32_le(r).await?;

                // required_level: u32
                let required_level = crate::util::astd_read_u32_le(r).await?;

                // required_skill: u32
                let required_skill = crate::util::astd_read_u32_le(r).await?;

                // required_skill_rank: u32
                let required_skill_rank = crate::util::astd_read_u32_le(r).await?;

                // required_spell: u32
                let required_spell = crate::util::astd_read_u32_le(r).await?;

                // required_honor_rank: u32
                let required_honor_rank = crate::util::astd_read_u32_le(r).await?;

                // required_city_rank: u32
                let required_city_rank = crate::util::astd_read_u32_le(r).await?;

                // required_reputation_faction: u32
                let required_reputation_faction = crate::util::astd_read_u32_le(r).await?;

                // required_reputation_rank: u32
                let required_reputation_rank = crate::util::astd_read_u32_le(r).await?;

                // max_count: u32
                let max_count = crate::util::astd_read_u32_le(r).await?;

                // stackable: u32
                let stackable = crate::util::astd_read_u32_le(r).await?;

                // container_slots: u32
                let container_slots = crate::util::astd_read_u32_le(r).await?;

                // stats: ItemStat[10]
                let mut stats = Vec::with_capacity(10 as usize);
                for i in 0..10 {
                    stats.push(ItemStat::astd_read(r).await?);
                }
                let stats = stats.try_into().unwrap();

                // damages: ItemDamageType[5]
                let mut damages = Vec::with_capacity(5 as usize);
                for i in 0..5 {
                    damages.push(ItemDamageType::astd_read(r).await?);
                }
                let damages = damages.try_into().unwrap();

                // armor: u32
                let armor = crate::util::astd_read_u32_le(r).await?;

                // holy_resistance: u32
                let holy_resistance = crate::util::astd_read_u32_le(r).await?;

                // fire_resistance: u32
                let fire_resistance = crate::util::astd_read_u32_le(r).await?;

                // nature_resistance: u32
                let nature_resistance = crate::util::astd_read_u32_le(r).await?;

                // frost_resistance: u32
                let frost_resistance = crate::util::astd_read_u32_le(r).await?;

                // shadow_resistance: u32
                let shadow_resistance = crate::util::astd_read_u32_le(r).await?;

                // arcane_resistance: u32
                let arcane_resistance = crate::util::astd_read_u32_le(r).await?;

                // delay: u32
                let delay = crate::util::astd_read_u32_le(r).await?;

                // ammo_type: u32
                let ammo_type = crate::util::astd_read_u32_le(r).await?;

                // ranged_range_modification: f32
                let ranged_range_modification = crate::util::astd_read_f32_le(r).await?;
                // spells: ItemSpells[5]
                let mut spells = Vec::with_capacity(5 as usize);
                for i in 0..5 {
                    spells.push(ItemSpells::astd_read(r).await?);
                }
                let spells = spells.try_into().unwrap();

                // bonding: u32
                let bonding = crate::util::astd_read_u32_le(r).await?;

                // description: CString
                let description = crate::util::astd_read_c_string_to_vec(r).await?;
                let description = String::from_utf8(description)?;

                // page_text: u32
                let page_text = crate::util::astd_read_u32_le(r).await?;

                // language_id: u32
                let language_id = crate::util::astd_read_u32_le(r).await?;

                // page_material: u32
                let page_material = crate::util::astd_read_u32_le(r).await?;

                // start_quest: u32
                let start_quest = crate::util::astd_read_u32_le(r).await?;

                // lock_id: u32
                let lock_id = crate::util::astd_read_u32_le(r).await?;

                // material: u32
                let material = crate::util::astd_read_u32_le(r).await?;

                // sheath: u32
                let sheath = crate::util::astd_read_u32_le(r).await?;

                // random_property: u32
                let random_property = crate::util::astd_read_u32_le(r).await?;

                // block: u32
                let block = crate::util::astd_read_u32_le(r).await?;

                // item_set: u32
                let item_set = crate::util::astd_read_u32_le(r).await?;

                // max_durability: u32
                let max_durability = crate::util::astd_read_u32_le(r).await?;

                // area: Area
                let area: Area = crate::util::astd_read_u32_le(r).await?.try_into()?;

                // map: Map
                let map: Map = crate::util::astd_read_u32_le(r).await?.try_into()?;

                // bag_family: u32
                let bag_family = crate::util::astd_read_u32_le(r).await?;

                Some(SMSG_ITEM_QUERY_SINGLE_RESPONSEfound {
                    item_class,
                    item_sub_class,
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
                })
            } else {
                None
            };

            Ok(Self {
                item,
                found,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_ITEM_QUERY_SINGLE_RESPONSE {
    pub fn size(&self) -> usize {
        0
        + 4 // item: u32
        + if let Some(found) = &self.found {
            0
            + 4 // item_class: ItemClass
            + 4 // item_sub_class: u32
            + found.name1.len() + 1 // name1: CString
            + found.name2.len() + 1 // name2: CString
            + found.name3.len() + 1 // name3: CString
            + found.name4.len() + 1 // name4: CString
            + 4 // item_display_info: u32
            + 4 // quality: ItemQuality
            + 4 // flags: u32
            + 8 // buy_price: f64
            + 8 // sell_price: f64
            + 1 // inventory_type: InventoryType
            + 4 // allowed_class: u32
            + 4 // allowed_race: u32
            + 4 // item_level: u32
            + 4 // required_level: u32
            + 4 // required_skill: u32
            + 4 // required_skill_rank: u32
            + 4 // required_spell: u32
            + 4 // required_honor_rank: u32
            + 4 // required_city_rank: u32
            + 4 // required_reputation_faction: u32
            + 4 // required_reputation_rank: u32
            + 4 // max_count: u32
            + 4 // stackable: u32
            + 4 // container_slots: u32
            + 10 * ItemStat::size() // stats: ItemStat[10]
            + 5 * ItemDamageType::size() // damages: ItemDamageType[5]
            + 4 // armor: u32
            + 4 // holy_resistance: u32
            + 4 // fire_resistance: u32
            + 4 // nature_resistance: u32
            + 4 // frost_resistance: u32
            + 4 // shadow_resistance: u32
            + 4 // arcane_resistance: u32
            + 4 // delay: u32
            + 4 // ammo_type: u32
            + 4 // ranged_range_modification: f32
            + 5 * ItemSpells::size() // spells: ItemSpells[5]
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
        } else {
            0
        }
    }
}

#[derive(Debug)]
pub enum SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Area(AreaError),
    InventoryType(InventoryTypeError),
    ItemClass(ItemClassError),
    ItemQuality(ItemQualityError),
    Map(MapError),
}

impl std::error::Error for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {}
impl std::fmt::Display for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::InventoryType(i) => i.fmt(f),
            Self::ItemClass(i) => i.fmt(f),
            Self::ItemQuality(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<AreaError> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<InventoryTypeError> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: InventoryTypeError) -> Self {
        Self::InventoryType(e)
    }
}

impl From<ItemClassError> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: ItemClassError) -> Self {
        Self::ItemClass(e)
    }
}

impl From<ItemQualityError> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: ItemQualityError) -> Self {
        Self::ItemQuality(e)
    }
}

impl From<MapError> for SMSG_ITEM_QUERY_SINGLE_RESPONSEError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SMSG_ITEM_QUERY_SINGLE_RESPONSEfound {
    pub item_class: ItemClass,
    pub item_sub_class: u32,
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub name4: String,
    pub item_display_info: u32,
    pub quality: ItemQuality,
    pub flags: u32,
    pub buy_price: f64,
    pub sell_price: f64,
    pub inventory_type: InventoryType,
    pub allowed_class: u32,
    pub allowed_race: u32,
    pub item_level: u32,
    pub required_level: u32,
    pub required_skill: u32,
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
    pub armor: u32,
    pub holy_resistance: u32,
    pub fire_resistance: u32,
    pub nature_resistance: u32,
    pub frost_resistance: u32,
    pub shadow_resistance: u32,
    pub arcane_resistance: u32,
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
}

impl SMSG_ITEM_QUERY_SINGLE_RESPONSEfound {
    pub(crate) fn size(&self) -> usize {
        4 // item_class: ItemClass
        + 4 // item_sub_class: u32
        + self.name1.len() + 1 // name1: CString
        + self.name2.len() + 1 // name2: CString
        + self.name3.len() + 1 // name3: CString
        + self.name4.len() + 1 // name4: CString
        + 4 // item_display_info: u32
        + 4 // quality: ItemQuality
        + 4 // flags: u32
        + 8 // buy_price: f64
        + 8 // sell_price: f64
        + 1 // inventory_type: InventoryType
        + 4 // allowed_class: u32
        + 4 // allowed_race: u32
        + 4 // item_level: u32
        + 4 // required_level: u32
        + 4 // required_skill: u32
        + 4 // required_skill_rank: u32
        + 4 // required_spell: u32
        + 4 // required_honor_rank: u32
        + 4 // required_city_rank: u32
        + 4 // required_reputation_faction: u32
        + 4 // required_reputation_rank: u32
        + 4 // max_count: u32
        + 4 // stackable: u32
        + 4 // container_slots: u32
        + 10 * ItemStat::size() // stats: ItemStat[10]
        + 5 * ItemDamageType::size() // damages: ItemDamageType[5]
        + 4 // armor: u32
        + 4 // holy_resistance: u32
        + 4 // fire_resistance: u32
        + 4 // nature_resistance: u32
        + 4 // frost_resistance: u32
        + 4 // shadow_resistance: u32
        + 4 // arcane_resistance: u32
        + 4 // delay: u32
        + 4 // ammo_type: u32
        + 4 // ranged_range_modification: f32
        + 5 * ItemSpells::size() // spells: ItemSpells[5]
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
    }

}

