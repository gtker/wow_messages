use crate::Guid;
use std::convert::TryInto;
use crate::wrath::{Race};
use crate::wrath::{Class};
use crate::wrath::{Gender};
use crate::wrath::{Power};
use crate::wrath::{UnitStandState};
use crate::wrath::{UpdateContainer, UpdateContainerBuilder, UpdateCorpse, UpdateCorpseBuilder, UpdateDynamicObject, UpdateDynamicObjectBuilder, UpdateGameObject, UpdateGameObjectBuilder, UpdateItem, UpdateItemBuilder, UpdatePlayer, UpdatePlayerBuilder, UpdateUnit, UpdateUnitBuilder};

impl UpdateItemBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CONTAINED(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_GIFTCREATOR(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_STACK_COUNT(mut self, v: i32) -> Self {
        self.header_set(14, v as u32);
        self
    }

    pub fn set_item_DURATION(mut self, v: i32) -> Self {
        self.header_set(15, v as u32);
        self
    }

    pub fn set_item_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16, v as u32);
        self
    }

    pub fn set_item_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_1_1(mut self, v: i32) -> Self {
        self.header_set(22, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_1_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(24, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_2_1(mut self, v: i32) -> Self {
        self.header_set(25, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_2_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(27, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_3_1(mut self, v: i32) -> Self {
        self.header_set(28, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_3_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(30, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_4_1(mut self, v: i32) -> Self {
        self.header_set(31, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_4_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(33, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_5_1(mut self, v: i32) -> Self {
        self.header_set(34, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_5_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(36, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_6_1(mut self, v: i32) -> Self {
        self.header_set(37, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_6_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(39, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_7_1(mut self, v: i32) -> Self {
        self.header_set(40, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_7_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(42, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_8_1(mut self, v: i32) -> Self {
        self.header_set(43, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_8_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(45, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_9_1(mut self, v: i32) -> Self {
        self.header_set(46, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_9_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(48, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_10_1(mut self, v: i32) -> Self {
        self.header_set(49, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_10_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(51, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_11_1(mut self, v: i32) -> Self {
        self.header_set(52, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_11_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(54, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_12_1(mut self, v: i32) -> Self {
        self.header_set(55, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_12_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(57, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(58, v as u32);
        self
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(59, v as u32);
        self
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(60, v as u32);
        self
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(61, v as u32);
        self
    }

    pub fn set_item_CREATE_PLAYED_TIME(mut self, v: i32) -> Self {
        self.header_set(62, v as u32);
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateContainerBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CONTAINED(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_GIFTCREATOR(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_STACK_COUNT(mut self, v: i32) -> Self {
        self.header_set(14, v as u32);
        self
    }

    pub fn set_item_DURATION(mut self, v: i32) -> Self {
        self.header_set(15, v as u32);
        self
    }

    pub fn set_item_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16, v as u32);
        self
    }

    pub fn set_item_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_1_1(mut self, v: i32) -> Self {
        self.header_set(22, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_1_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(24, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_2_1(mut self, v: i32) -> Self {
        self.header_set(25, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_2_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(27, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_3_1(mut self, v: i32) -> Self {
        self.header_set(28, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_3_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(30, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_4_1(mut self, v: i32) -> Self {
        self.header_set(31, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_4_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(33, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_5_1(mut self, v: i32) -> Self {
        self.header_set(34, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_5_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(36, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_6_1(mut self, v: i32) -> Self {
        self.header_set(37, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_6_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(39, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_7_1(mut self, v: i32) -> Self {
        self.header_set(40, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_7_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(42, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_8_1(mut self, v: i32) -> Self {
        self.header_set(43, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_8_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(45, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_9_1(mut self, v: i32) -> Self {
        self.header_set(46, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_9_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(48, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_10_1(mut self, v: i32) -> Self {
        self.header_set(49, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_10_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(51, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_11_1(mut self, v: i32) -> Self {
        self.header_set(52, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_11_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(54, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_12_1(mut self, v: i32) -> Self {
        self.header_set(55, v as u32);
        self
    }

    pub fn set_item_ENCHANTMENT_12_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(57, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(58, v as u32);
        self
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(59, v as u32);
        self
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(60, v as u32);
        self
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(61, v as u32);
        self
    }

    pub fn set_item_CREATE_PLAYED_TIME(mut self, v: i32) -> Self {
        self.header_set(62, v as u32);
        self
    }

    pub fn set_container_NUM_SLOTS(mut self, v: i32) -> Self {
        self.header_set(64, v as u32);
        self
    }

    pub fn set_container_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(66, v.guid() as u32);
        self.header_set(67, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateUnitBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CHARM(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMON(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CRITTER(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_TARGET(mut self, v: Guid) -> Self {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(22, v as u32);
        self
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(23, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_HEALTH(mut self, v: i32) -> Self {
        self.header_set(24, v as u32);
        self
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(25, v as u32);
        self
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(26, v as u32);
        self
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(27, v as u32);
        self
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(28, v as u32);
        self
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(29, v as u32);
        self
    }

    pub fn set_unit_POWER6(mut self, v: i32) -> Self {
        self.header_set(30, v as u32);
        self
    }

    pub fn set_unit_POWER7(mut self, v: i32) -> Self {
        self.header_set(31, v as u32);
        self
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(32, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(33, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(34, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(35, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(36, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(37, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER6(mut self, v: i32) -> Self {
        self.header_set(38, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER7(mut self, v: i32) -> Self {
        self.header_set(39, v as u32);
        self
    }

    pub fn set_unit_POWER_REGEN_FLAT_MODIFIER(mut self, v: f32) -> Self {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_REGEN_INTERRUPTED_FLAT_MODIFIER(mut self, v: f32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(54, v as u32);
        self
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(55, v as u32);
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_ID(mut self, v: i32) -> Self {
        self.header_set(56, v as u32);
        self
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(59, v as u32);
        self
    }

    pub fn set_unit_FLAGS_2(mut self, v: i32) -> Self {
        self.header_set(60, v as u32);
        self
    }

    pub fn set_unit_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(61, v as u32);
        self
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(62, v as u32);
        self
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(64, v as u32);
        self
    }

    pub fn set_unit_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(65, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(66, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(67, v as u32);
        self
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(68, v as u32);
        self
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(69, v as u32);
        self
    }

    pub fn set_unit_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(70, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(71, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(72, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(73, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_1(mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) -> Self {
        self.header_set(74, u32::from_le_bytes([stand_state.as_int(), unknown1, unknown2, unknown3]));
        self
    }

    pub fn set_unit_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(75, v as u32);
        self
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(76, v as u32);
        self
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(77, v as u32);
        self
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(78, v as u32);
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(79, v as u32);
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(80, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(81, v as u32);
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(82, v as u32);
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(83, v as u32);
        self
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(84, v as u32);
        self
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(85, v as u32);
        self
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(86, v as u32);
        self
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(87, v as u32);
        self
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(88, v as u32);
        self
    }

    pub fn set_unit_POSSTAT0(mut self, v: i32) -> Self {
        self.header_set(89, v as u32);
        self
    }

    pub fn set_unit_POSSTAT1(mut self, v: i32) -> Self {
        self.header_set(90, v as u32);
        self
    }

    pub fn set_unit_POSSTAT2(mut self, v: i32) -> Self {
        self.header_set(91, v as u32);
        self
    }

    pub fn set_unit_POSSTAT3(mut self, v: i32) -> Self {
        self.header_set(92, v as u32);
        self
    }

    pub fn set_unit_POSSTAT4(mut self, v: i32) -> Self {
        self.header_set(93, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT0(mut self, v: i32) -> Self {
        self.header_set(94, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT1(mut self, v: i32) -> Self {
        self.header_set(95, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT2(mut self, v: i32) -> Self {
        self.header_set(96, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT3(mut self, v: i32) -> Self {
        self.header_set(97, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT4(mut self, v: i32) -> Self {
        self.header_set(98, v as u32);
        self
    }

    pub fn set_unit_RESISTANCES(mut self, v: i32) -> Self {
        self.header_set(99, v as u32);
        self
    }

    pub fn set_unit_RESISTANCEBUFFMODSPOSITIVE(mut self, v: i32) -> Self {
        self.header_set(106, v as u32);
        self
    }

    pub fn set_unit_RESISTANCEBUFFMODSNEGATIVE(mut self, v: i32) -> Self {
        self.header_set(113, v as u32);
        self
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(120, v as u32);
        self
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(121, v as u32);
        self
    }

    pub fn set_unit_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(122, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(123, v as u32);
        self
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(124, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(126, v as u32);
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(127, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(131, v as u32);
        self
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(138, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXHEALTHMODIFIER(mut self, v: f32) -> Self {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_HOVERHEIGHT(mut self, v: f32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdatePlayerBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CHARM(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMON(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CRITTER(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_TARGET(mut self, v: Guid) -> Self {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(22, v as u32);
        self
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(23, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_HEALTH(mut self, v: i32) -> Self {
        self.header_set(24, v as u32);
        self
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(25, v as u32);
        self
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(26, v as u32);
        self
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(27, v as u32);
        self
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(28, v as u32);
        self
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(29, v as u32);
        self
    }

    pub fn set_unit_POWER6(mut self, v: i32) -> Self {
        self.header_set(30, v as u32);
        self
    }

    pub fn set_unit_POWER7(mut self, v: i32) -> Self {
        self.header_set(31, v as u32);
        self
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(32, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(33, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(34, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(35, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(36, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(37, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER6(mut self, v: i32) -> Self {
        self.header_set(38, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER7(mut self, v: i32) -> Self {
        self.header_set(39, v as u32);
        self
    }

    pub fn set_unit_POWER_REGEN_FLAT_MODIFIER(mut self, v: f32) -> Self {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_REGEN_INTERRUPTED_FLAT_MODIFIER(mut self, v: f32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(54, v as u32);
        self
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(55, v as u32);
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_ID(mut self, v: i32) -> Self {
        self.header_set(56, v as u32);
        self
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(59, v as u32);
        self
    }

    pub fn set_unit_FLAGS_2(mut self, v: i32) -> Self {
        self.header_set(60, v as u32);
        self
    }

    pub fn set_unit_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(61, v as u32);
        self
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(62, v as u32);
        self
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(64, v as u32);
        self
    }

    pub fn set_unit_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(65, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(66, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(67, v as u32);
        self
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(68, v as u32);
        self
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(69, v as u32);
        self
    }

    pub fn set_unit_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(70, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(71, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(72, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(73, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_1(mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) -> Self {
        self.header_set(74, u32::from_le_bytes([stand_state.as_int(), unknown1, unknown2, unknown3]));
        self
    }

    pub fn set_unit_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(75, v as u32);
        self
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(76, v as u32);
        self
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(77, v as u32);
        self
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(78, v as u32);
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(79, v as u32);
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(80, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(81, v as u32);
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(82, v as u32);
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(83, v as u32);
        self
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(84, v as u32);
        self
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(85, v as u32);
        self
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(86, v as u32);
        self
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(87, v as u32);
        self
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(88, v as u32);
        self
    }

    pub fn set_unit_POSSTAT0(mut self, v: i32) -> Self {
        self.header_set(89, v as u32);
        self
    }

    pub fn set_unit_POSSTAT1(mut self, v: i32) -> Self {
        self.header_set(90, v as u32);
        self
    }

    pub fn set_unit_POSSTAT2(mut self, v: i32) -> Self {
        self.header_set(91, v as u32);
        self
    }

    pub fn set_unit_POSSTAT3(mut self, v: i32) -> Self {
        self.header_set(92, v as u32);
        self
    }

    pub fn set_unit_POSSTAT4(mut self, v: i32) -> Self {
        self.header_set(93, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT0(mut self, v: i32) -> Self {
        self.header_set(94, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT1(mut self, v: i32) -> Self {
        self.header_set(95, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT2(mut self, v: i32) -> Self {
        self.header_set(96, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT3(mut self, v: i32) -> Self {
        self.header_set(97, v as u32);
        self
    }

    pub fn set_unit_NEGSTAT4(mut self, v: i32) -> Self {
        self.header_set(98, v as u32);
        self
    }

    pub fn set_unit_RESISTANCES(mut self, v: i32) -> Self {
        self.header_set(99, v as u32);
        self
    }

    pub fn set_unit_RESISTANCEBUFFMODSPOSITIVE(mut self, v: i32) -> Self {
        self.header_set(106, v as u32);
        self
    }

    pub fn set_unit_RESISTANCEBUFFMODSNEGATIVE(mut self, v: i32) -> Self {
        self.header_set(113, v as u32);
        self
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(120, v as u32);
        self
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(121, v as u32);
        self
    }

    pub fn set_unit_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(122, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(123, v as u32);
        self
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(124, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(126, v as u32);
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(127, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(131, v as u32);
        self
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(138, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXHEALTHMODIFIER(mut self, v: f32) -> Self {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_HOVERHEIGHT(mut self, v: f32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_DUEL_ARBITER(mut self, v: Guid) -> Self {
        self.header_set(148, v.guid() as u32);
        self.header_set(149, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FLAGS(mut self, v: i32) -> Self {
        self.header_set(150, v as u32);
        self
    }

    pub fn set_player_GUILDID(mut self, v: i32) -> Self {
        self.header_set(151, v as u32);
        self
    }

    pub fn set_player_GUILDRANK(mut self, v: i32) -> Self {
        self.header_set(152, v as u32);
        self
    }

    pub fn set_player_FIELD_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(153, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(154, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_BYTES_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(155, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_DUEL_TEAM(mut self, v: i32) -> Self {
        self.header_set(156, v as u32);
        self
    }

    pub fn set_player_GUILD_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(157, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_1(mut self, v: i32) -> Self {
        self.header_set(158, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_2(mut self, v: i32) -> Self {
        self.header_set(159, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(160, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_4(mut self, v: i32) -> Self {
        self.header_set(162, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_1(mut self, v: i32) -> Self {
        self.header_set(163, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_2(mut self, v: i32) -> Self {
        self.header_set(164, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(165, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_5(mut self, v: i32) -> Self {
        self.header_set(167, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_1(mut self, v: i32) -> Self {
        self.header_set(168, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_2(mut self, v: i32) -> Self {
        self.header_set(169, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(170, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_5(mut self, v: i32) -> Self {
        self.header_set(172, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_1(mut self, v: i32) -> Self {
        self.header_set(173, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_2(mut self, v: i32) -> Self {
        self.header_set(174, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(175, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_5(mut self, v: i32) -> Self {
        self.header_set(177, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_1(mut self, v: i32) -> Self {
        self.header_set(178, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_2(mut self, v: i32) -> Self {
        self.header_set(179, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(180, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_5(mut self, v: i32) -> Self {
        self.header_set(182, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_1(mut self, v: i32) -> Self {
        self.header_set(183, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_2(mut self, v: i32) -> Self {
        self.header_set(184, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(185, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_5(mut self, v: i32) -> Self {
        self.header_set(187, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_1(mut self, v: i32) -> Self {
        self.header_set(188, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_2(mut self, v: i32) -> Self {
        self.header_set(189, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(190, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_5(mut self, v: i32) -> Self {
        self.header_set(192, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_1(mut self, v: i32) -> Self {
        self.header_set(193, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_2(mut self, v: i32) -> Self {
        self.header_set(194, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(195, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_5(mut self, v: i32) -> Self {
        self.header_set(197, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_1(mut self, v: i32) -> Self {
        self.header_set(198, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_2(mut self, v: i32) -> Self {
        self.header_set(199, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(200, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_5(mut self, v: i32) -> Self {
        self.header_set(202, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_1(mut self, v: i32) -> Self {
        self.header_set(203, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_2(mut self, v: i32) -> Self {
        self.header_set(204, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(205, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_5(mut self, v: i32) -> Self {
        self.header_set(207, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_1(mut self, v: i32) -> Self {
        self.header_set(208, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_2(mut self, v: i32) -> Self {
        self.header_set(209, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(210, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_5(mut self, v: i32) -> Self {
        self.header_set(212, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_1(mut self, v: i32) -> Self {
        self.header_set(213, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_2(mut self, v: i32) -> Self {
        self.header_set(214, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(215, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_5(mut self, v: i32) -> Self {
        self.header_set(217, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_1(mut self, v: i32) -> Self {
        self.header_set(218, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_2(mut self, v: i32) -> Self {
        self.header_set(219, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(220, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_5(mut self, v: i32) -> Self {
        self.header_set(222, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_1(mut self, v: i32) -> Self {
        self.header_set(223, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_2(mut self, v: i32) -> Self {
        self.header_set(224, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(225, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_5(mut self, v: i32) -> Self {
        self.header_set(227, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_1(mut self, v: i32) -> Self {
        self.header_set(228, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_2(mut self, v: i32) -> Self {
        self.header_set(229, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(230, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_5(mut self, v: i32) -> Self {
        self.header_set(232, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_1(mut self, v: i32) -> Self {
        self.header_set(233, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_2(mut self, v: i32) -> Self {
        self.header_set(234, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(235, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_5(mut self, v: i32) -> Self {
        self.header_set(237, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_1(mut self, v: i32) -> Self {
        self.header_set(238, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_2(mut self, v: i32) -> Self {
        self.header_set(239, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(240, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_5(mut self, v: i32) -> Self {
        self.header_set(242, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_1(mut self, v: i32) -> Self {
        self.header_set(243, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_2(mut self, v: i32) -> Self {
        self.header_set(244, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(245, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_5(mut self, v: i32) -> Self {
        self.header_set(247, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_1(mut self, v: i32) -> Self {
        self.header_set(248, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_2(mut self, v: i32) -> Self {
        self.header_set(249, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(250, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_5(mut self, v: i32) -> Self {
        self.header_set(252, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_1(mut self, v: i32) -> Self {
        self.header_set(253, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_2(mut self, v: i32) -> Self {
        self.header_set(254, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(255, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_5(mut self, v: i32) -> Self {
        self.header_set(257, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_21_1(mut self, v: i32) -> Self {
        self.header_set(258, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_21_2(mut self, v: i32) -> Self {
        self.header_set(259, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_21_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(260, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_21_5(mut self, v: i32) -> Self {
        self.header_set(262, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_22_1(mut self, v: i32) -> Self {
        self.header_set(263, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_22_2(mut self, v: i32) -> Self {
        self.header_set(264, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_22_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(265, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_22_5(mut self, v: i32) -> Self {
        self.header_set(267, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_23_1(mut self, v: i32) -> Self {
        self.header_set(268, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_23_2(mut self, v: i32) -> Self {
        self.header_set(269, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_23_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(270, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_23_5(mut self, v: i32) -> Self {
        self.header_set(272, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_24_1(mut self, v: i32) -> Self {
        self.header_set(273, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_24_2(mut self, v: i32) -> Self {
        self.header_set(274, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_24_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(275, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_24_5(mut self, v: i32) -> Self {
        self.header_set(277, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_25_1(mut self, v: i32) -> Self {
        self.header_set(278, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_25_2(mut self, v: i32) -> Self {
        self.header_set(279, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_25_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(280, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_QUEST_LOG_25_5(mut self, v: i32) -> Self {
        self.header_set(282, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(283, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(284, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(285, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(286, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(287, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(288, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(289, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(290, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(291, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(292, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(293, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(294, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(295, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(296, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(297, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(298, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(299, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(300, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(301, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(302, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(303, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(304, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(305, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(306, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(307, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(308, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(309, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(310, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(311, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(312, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(313, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(314, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(315, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(316, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(317, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(318, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_ENTRYID(mut self, v: i32) -> Self {
        self.header_set(319, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_ENCHANTMENT(mut self, a: u16, b: u16) -> Self {
        self.header_set(320, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_CHOSEN_TITLE(mut self, v: i32) -> Self {
        self.header_set(321, v as u32);
        self
    }

    pub fn set_player_FAKE_INEBRIATION(mut self, v: i32) -> Self {
        self.header_set(322, v as u32);
        self
    }

    pub fn set_player_INV_SLOT_HEAD(mut self, v: Guid) -> Self {
        self.header_set(324, v.guid() as u32);
        self.header_set(325, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_PACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(370, v.guid() as u32);
        self.header_set(371, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(402, v.guid() as u32);
        self.header_set(403, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_BANKBAG_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(458, v.guid() as u32);
        self.header_set(459, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VENDORBUYBACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(472, v.guid() as u32);
        self.header_set(473, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KEYRING_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(496, v.guid() as u32);
        self.header_set(497, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_CURRENCYTOKEN_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(560, v.guid() as u32);
        self.header_set(561, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FARSIGHT(mut self, v: Guid) -> Self {
        self.header_set(624, v.guid() as u32);
        self.header_set(625, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KNOWN_TITLES(mut self, v: Guid) -> Self {
        self.header_set(626, v.guid() as u32);
        self.header_set(627, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KNOWN_TITLES1(mut self, v: Guid) -> Self {
        self.header_set(628, v.guid() as u32);
        self.header_set(629, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KNOWN_TITLES2(mut self, v: Guid) -> Self {
        self.header_set(630, v.guid() as u32);
        self.header_set(631, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_KNOWN_CURRENCIES(mut self, v: Guid) -> Self {
        self.header_set(632, v.guid() as u32);
        self.header_set(633, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_XP(mut self, v: i32) -> Self {
        self.header_set(634, v as u32);
        self
    }

    pub fn set_player_NEXT_LEVEL_XP(mut self, v: i32) -> Self {
        self.header_set(635, v as u32);
        self
    }

    pub fn set_player_SKILL_INFO_1_1(mut self, v: i32) -> Self {
        self.header_set(636, v as u32);
        self
    }

    pub fn set_player_SKILL_INFO_1_2(mut self, a: u16, b: u16) -> Self {
        self.header_set(637, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_SKILL_INFO_1_3(mut self, v: i32) -> Self {
        self.header_set(638, v as u32);
        self
    }

    pub fn set_player_CHARACTER_POINTS1(mut self, v: i32) -> Self {
        self.header_set(1020, v as u32);
        self
    }

    pub fn set_player_CHARACTER_POINTS2(mut self, v: i32) -> Self {
        self.header_set(1021, v as u32);
        self
    }

    pub fn set_player_TRACK_CREATURES(mut self, v: i32) -> Self {
        self.header_set(1022, v as u32);
        self
    }

    pub fn set_player_TRACK_RESOURCES(mut self, v: i32) -> Self {
        self.header_set(1023, v as u32);
        self
    }

    pub fn set_player_BLOCK_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1024, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_DODGE_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1025, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_PARRY_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1026, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_EXPERTISE(mut self, v: i32) -> Self {
        self.header_set(1027, v as u32);
        self
    }

    pub fn set_player_OFFHAND_EXPERTISE(mut self, v: i32) -> Self {
        self.header_set(1028, v as u32);
        self
    }

    pub fn set_player_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1029, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_RANGED_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1030, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_OFFHAND_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1031, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SPELL_CRIT_PERCENTAGE1(mut self, v: f32) -> Self {
        self.header_set(1032, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SHIELD_BLOCK(mut self, v: i32) -> Self {
        self.header_set(1039, v as u32);
        self
    }

    pub fn set_player_SHIELD_BLOCK_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1040, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_EXPLORED_ZONES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1041, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_REST_STATE_EXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(1169, v as u32);
        self
    }

    pub fn set_player_COINAGE(mut self, v: i32) -> Self {
        self.header_set(1170, v as u32);
        self
    }

    pub fn set_player_MOD_DAMAGE_DONE_POS(mut self, v: i32) -> Self {
        self.header_set(1171, v as u32);
        self
    }

    pub fn set_player_MOD_DAMAGE_DONE_NEG(mut self, v: i32) -> Self {
        self.header_set(1178, v as u32);
        self
    }

    pub fn set_player_MOD_DAMAGE_DONE_PCT(mut self, v: i32) -> Self {
        self.header_set(1185, v as u32);
        self
    }

    pub fn set_player_MOD_HEALING_DONE_POS(mut self, v: i32) -> Self {
        self.header_set(1192, v as u32);
        self
    }

    pub fn set_player_MOD_HEALING_PCT(mut self, v: f32) -> Self {
        self.header_set(1193, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_HEALING_DONE_PCT(mut self, v: f32) -> Self {
        self.header_set(1194, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_MOD_TARGET_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(1195, v as u32);
        self
    }

    pub fn set_player_MOD_TARGET_PHYSICAL_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(1196, v as u32);
        self
    }

    pub fn set_player_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1197, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_AMMO_ID(mut self, v: i32) -> Self {
        self.header_set(1198, v as u32);
        self
    }

    pub fn set_player_SELF_RES_SPELL(mut self, v: i32) -> Self {
        self.header_set(1199, v as u32);
        self
    }

    pub fn set_player_PVP_MEDALS(mut self, v: i32) -> Self {
        self.header_set(1200, v as u32);
        self
    }

    pub fn set_player_BUYBACK_PRICE_1(mut self, v: i32) -> Self {
        self.header_set(1201, v as u32);
        self
    }

    pub fn set_player_BUYBACK_TIMESTAMP_1(mut self, v: i32) -> Self {
        self.header_set(1213, v as u32);
        self
    }

    pub fn set_player_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1225, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_TODAY_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1226, v as u32);
        self
    }

    pub fn set_player_YESTERDAY_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1227, v as u32);
        self
    }

    pub fn set_player_LIFETIME_HONORBALE_KILLS(mut self, v: i32) -> Self {
        self.header_set(1228, v as u32);
        self
    }

    pub fn set_player_BYTES2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1229, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_WATCHED_FACTION_INDEX(mut self, v: i32) -> Self {
        self.header_set(1230, v as u32);
        self
    }

    pub fn set_player_COMBAT_RATING_1(mut self, v: i32) -> Self {
        self.header_set(1231, v as u32);
        self
    }

    pub fn set_player_ARENA_TEAM_INFO_1_1(mut self, v: i32) -> Self {
        self.header_set(1256, v as u32);
        self
    }

    pub fn set_player_HONOR_CURRENCY(mut self, v: i32) -> Self {
        self.header_set(1277, v as u32);
        self
    }

    pub fn set_player_ARENA_CURRENCY(mut self, v: i32) -> Self {
        self.header_set(1278, v as u32);
        self
    }

    pub fn set_player_MAX_LEVEL(mut self, v: i32) -> Self {
        self.header_set(1279, v as u32);
        self
    }

    pub fn set_player_DAILY_QUESTS_1(mut self, v: i32) -> Self {
        self.header_set(1280, v as u32);
        self
    }

    pub fn set_player_RUNE_REGEN_1(mut self, v: f32) -> Self {
        self.header_set(1305, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_NO_REAGENT_COST_1(mut self, v: i32) -> Self {
        self.header_set(1309, v as u32);
        self
    }

    pub fn set_player_GLYPH_SLOTS_1(mut self, v: i32) -> Self {
        self.header_set(1312, v as u32);
        self
    }

    pub fn set_player_GLYPHS_1(mut self, v: i32) -> Self {
        self.header_set(1318, v as u32);
        self
    }

    pub fn set_player_GLYPHS_ENABLED(mut self, v: i32) -> Self {
        self.header_set(1324, v as u32);
        self
    }

    pub fn set_player_PET_SPELL_POWER(mut self, v: i32) -> Self {
        self.header_set(1325, v as u32);
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateGameObjectBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_gameobject_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(8, v as u32);
        self
    }

    pub fn set_gameobject_FLAGS(mut self, v: i32) -> Self {
        self.header_set(9, v as u32);
        self
    }

    pub fn set_gameobject_PARENTROTATION(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_DYNAMIC(mut self, a: u16, b: u16) -> Self {
        self.header_set(14, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_gameobject_FACTION(mut self, v: i32) -> Self {
        self.header_set(15, v as u32);
        self
    }

    pub fn set_gameobject_LEVEL(mut self, v: i32) -> Self {
        self.header_set(16, v as u32);
        self
    }

    pub fn set_gameobject_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(17, u32::from_le_bytes([a, b, c, d]));
        self
    }

}

impl UpdateDynamicObjectBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_dynamicobject_CASTER(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_dynamicobject_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(8, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_dynamicobject_SPELLID(mut self, v: i32) -> Self {
        self.header_set(9, v as u32);
        self
    }

    pub fn set_dynamicobject_RADIUS(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_CASTTIME(mut self, v: i32) -> Self {
        self.header_set(11, v as u32);
        self
    }

}

impl UpdateCorpseBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_TYPE(mut self, v: i32) -> Self {
        self.header_set(2, v as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, v as u32);
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_PARTY(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_DISPLAY_ID(mut self, v: i32) -> Self {
        self.header_set(10, v as u32);
        self
    }

    pub fn set_corpse_ITEM(mut self, v: i32) -> Self {
        self.header_set(11, v as u32);
        self
    }

    pub fn set_corpse_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(30, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(31, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_GUILD(mut self, v: i32) -> Self {
        self.header_set(32, v as u32);
        self
    }

    pub fn set_corpse_FLAGS(mut self, v: i32) -> Self {
        self.header_set(33, v as u32);
        self
    }

    pub fn set_corpse_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(34, v as u32);
        self
    }

}

impl UpdateItem {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, v as u32);
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, v as u32);
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_item_OWNER(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn item_OWNER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_CONTAINED(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn item_CONTAINED(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_CREATOR(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn item_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_GIFTCREATOR(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn item_GIFTCREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_STACK_COUNT(&mut self, v: i32) {
        self.header_set(14, v as u32);
    }

    pub fn item_STACK_COUNT(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_item_DURATION(&mut self, v: i32) {
        self.header_set(15, v as u32);
    }

    pub fn item_DURATION(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_item_SPELL_CHARGES(&mut self, v: i32) {
        self.header_set(16, v as u32);
    }

    pub fn item_SPELL_CHARGES(&self) -> Option<i32> {
        self.values.get(&16).map(|v| *v as i32)
    }

    pub fn set_item_FLAGS(&mut self, v: i32) {
        self.header_set(21, v as u32);
    }

    pub fn item_FLAGS(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_1_1(&mut self, v: i32) {
        self.header_set(22, v as u32);
    }

    pub fn item_ENCHANTMENT_1_1(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_1_3(&mut self, a: u16, b: u16) {
        self.header_set(24, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_1_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&24) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_2_1(&mut self, v: i32) {
        self.header_set(25, v as u32);
    }

    pub fn item_ENCHANTMENT_2_1(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_2_3(&mut self, a: u16, b: u16) {
        self.header_set(27, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_2_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&27) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_3_1(&mut self, v: i32) {
        self.header_set(28, v as u32);
    }

    pub fn item_ENCHANTMENT_3_1(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_3_3(&mut self, a: u16, b: u16) {
        self.header_set(30, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_3_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&30) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_4_1(&mut self, v: i32) {
        self.header_set(31, v as u32);
    }

    pub fn item_ENCHANTMENT_4_1(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_4_3(&mut self, a: u16, b: u16) {
        self.header_set(33, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_4_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&33) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_5_1(&mut self, v: i32) {
        self.header_set(34, v as u32);
    }

    pub fn item_ENCHANTMENT_5_1(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_5_3(&mut self, a: u16, b: u16) {
        self.header_set(36, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_5_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_6_1(&mut self, v: i32) {
        self.header_set(37, v as u32);
    }

    pub fn item_ENCHANTMENT_6_1(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_6_3(&mut self, a: u16, b: u16) {
        self.header_set(39, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_6_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&39) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_7_1(&mut self, v: i32) {
        self.header_set(40, v as u32);
    }

    pub fn item_ENCHANTMENT_7_1(&self) -> Option<i32> {
        self.values.get(&40).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_7_3(&mut self, a: u16, b: u16) {
        self.header_set(42, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_7_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&42) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_8_1(&mut self, v: i32) {
        self.header_set(43, v as u32);
    }

    pub fn item_ENCHANTMENT_8_1(&self) -> Option<i32> {
        self.values.get(&43).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_8_3(&mut self, a: u16, b: u16) {
        self.header_set(45, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_8_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&45) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_9_1(&mut self, v: i32) {
        self.header_set(46, v as u32);
    }

    pub fn item_ENCHANTMENT_9_1(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_9_3(&mut self, a: u16, b: u16) {
        self.header_set(48, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_9_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&48) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_10_1(&mut self, v: i32) {
        self.header_set(49, v as u32);
    }

    pub fn item_ENCHANTMENT_10_1(&self) -> Option<i32> {
        self.values.get(&49).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_10_3(&mut self, a: u16, b: u16) {
        self.header_set(51, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_10_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&51) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_11_1(&mut self, v: i32) {
        self.header_set(52, v as u32);
    }

    pub fn item_ENCHANTMENT_11_1(&self) -> Option<i32> {
        self.values.get(&52).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_11_3(&mut self, a: u16, b: u16) {
        self.header_set(54, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_11_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&54) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_12_1(&mut self, v: i32) {
        self.header_set(55, v as u32);
    }

    pub fn item_ENCHANTMENT_12_1(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_12_3(&mut self, a: u16, b: u16) {
        self.header_set(57, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_12_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&57) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_PROPERTY_SEED(&mut self, v: i32) {
        self.header_set(58, v as u32);
    }

    pub fn item_PROPERTY_SEED(&self) -> Option<i32> {
        self.values.get(&58).map(|v| *v as i32)
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(&mut self, v: i32) {
        self.header_set(59, v as u32);
    }

    pub fn item_RANDOM_PROPERTIES_ID(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
    }

    pub fn set_item_DURABILITY(&mut self, v: i32) {
        self.header_set(60, v as u32);
    }

    pub fn item_DURABILITY(&self) -> Option<i32> {
        self.values.get(&60).map(|v| *v as i32)
    }

    pub fn set_item_MAXDURABILITY(&mut self, v: i32) {
        self.header_set(61, v as u32);
    }

    pub fn item_MAXDURABILITY(&self) -> Option<i32> {
        self.values.get(&61).map(|v| *v as i32)
    }

    pub fn set_item_CREATE_PLAYED_TIME(&mut self, v: i32) {
        self.header_set(62, v as u32);
    }

    pub fn item_CREATE_PLAYED_TIME(&self) -> Option<i32> {
        self.values.get(&62).map(|v| *v as i32)
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdateContainer {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, v as u32);
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, v as u32);
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_item_OWNER(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn item_OWNER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_CONTAINED(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn item_CONTAINED(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_CREATOR(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn item_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_GIFTCREATOR(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn item_GIFTCREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_STACK_COUNT(&mut self, v: i32) {
        self.header_set(14, v as u32);
    }

    pub fn item_STACK_COUNT(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_item_DURATION(&mut self, v: i32) {
        self.header_set(15, v as u32);
    }

    pub fn item_DURATION(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_item_SPELL_CHARGES(&mut self, v: i32) {
        self.header_set(16, v as u32);
    }

    pub fn item_SPELL_CHARGES(&self) -> Option<i32> {
        self.values.get(&16).map(|v| *v as i32)
    }

    pub fn set_item_FLAGS(&mut self, v: i32) {
        self.header_set(21, v as u32);
    }

    pub fn item_FLAGS(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_1_1(&mut self, v: i32) {
        self.header_set(22, v as u32);
    }

    pub fn item_ENCHANTMENT_1_1(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_1_3(&mut self, a: u16, b: u16) {
        self.header_set(24, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_1_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&24) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_2_1(&mut self, v: i32) {
        self.header_set(25, v as u32);
    }

    pub fn item_ENCHANTMENT_2_1(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_2_3(&mut self, a: u16, b: u16) {
        self.header_set(27, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_2_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&27) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_3_1(&mut self, v: i32) {
        self.header_set(28, v as u32);
    }

    pub fn item_ENCHANTMENT_3_1(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_3_3(&mut self, a: u16, b: u16) {
        self.header_set(30, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_3_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&30) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_4_1(&mut self, v: i32) {
        self.header_set(31, v as u32);
    }

    pub fn item_ENCHANTMENT_4_1(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_4_3(&mut self, a: u16, b: u16) {
        self.header_set(33, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_4_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&33) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_5_1(&mut self, v: i32) {
        self.header_set(34, v as u32);
    }

    pub fn item_ENCHANTMENT_5_1(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_5_3(&mut self, a: u16, b: u16) {
        self.header_set(36, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_5_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_6_1(&mut self, v: i32) {
        self.header_set(37, v as u32);
    }

    pub fn item_ENCHANTMENT_6_1(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_6_3(&mut self, a: u16, b: u16) {
        self.header_set(39, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_6_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&39) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_7_1(&mut self, v: i32) {
        self.header_set(40, v as u32);
    }

    pub fn item_ENCHANTMENT_7_1(&self) -> Option<i32> {
        self.values.get(&40).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_7_3(&mut self, a: u16, b: u16) {
        self.header_set(42, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_7_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&42) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_8_1(&mut self, v: i32) {
        self.header_set(43, v as u32);
    }

    pub fn item_ENCHANTMENT_8_1(&self) -> Option<i32> {
        self.values.get(&43).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_8_3(&mut self, a: u16, b: u16) {
        self.header_set(45, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_8_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&45) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_9_1(&mut self, v: i32) {
        self.header_set(46, v as u32);
    }

    pub fn item_ENCHANTMENT_9_1(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_9_3(&mut self, a: u16, b: u16) {
        self.header_set(48, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_9_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&48) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_10_1(&mut self, v: i32) {
        self.header_set(49, v as u32);
    }

    pub fn item_ENCHANTMENT_10_1(&self) -> Option<i32> {
        self.values.get(&49).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_10_3(&mut self, a: u16, b: u16) {
        self.header_set(51, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_10_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&51) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_11_1(&mut self, v: i32) {
        self.header_set(52, v as u32);
    }

    pub fn item_ENCHANTMENT_11_1(&self) -> Option<i32> {
        self.values.get(&52).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_11_3(&mut self, a: u16, b: u16) {
        self.header_set(54, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_11_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&54) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_ENCHANTMENT_12_1(&mut self, v: i32) {
        self.header_set(55, v as u32);
    }

    pub fn item_ENCHANTMENT_12_1(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT_12_3(&mut self, a: u16, b: u16) {
        self.header_set(57, (a as u32) << 16 | b as u32);
    }

    pub fn item_ENCHANTMENT_12_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&57) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_PROPERTY_SEED(&mut self, v: i32) {
        self.header_set(58, v as u32);
    }

    pub fn item_PROPERTY_SEED(&self) -> Option<i32> {
        self.values.get(&58).map(|v| *v as i32)
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(&mut self, v: i32) {
        self.header_set(59, v as u32);
    }

    pub fn item_RANDOM_PROPERTIES_ID(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
    }

    pub fn set_item_DURABILITY(&mut self, v: i32) {
        self.header_set(60, v as u32);
    }

    pub fn item_DURABILITY(&self) -> Option<i32> {
        self.values.get(&60).map(|v| *v as i32)
    }

    pub fn set_item_MAXDURABILITY(&mut self, v: i32) {
        self.header_set(61, v as u32);
    }

    pub fn item_MAXDURABILITY(&self) -> Option<i32> {
        self.values.get(&61).map(|v| *v as i32)
    }

    pub fn set_item_CREATE_PLAYED_TIME(&mut self, v: i32) {
        self.header_set(62, v as u32);
    }

    pub fn item_CREATE_PLAYED_TIME(&self) -> Option<i32> {
        self.values.get(&62).map(|v| *v as i32)
    }

    pub fn set_container_NUM_SLOTS(&mut self, v: i32) {
        self.header_set(64, v as u32);
    }

    pub fn container_NUM_SLOTS(&self) -> Option<i32> {
        self.values.get(&64).map(|v| *v as i32)
    }

    pub fn set_container_SLOT_1(&mut self, v: Guid) {
        self.header_set(66, v.guid() as u32);
        self.header_set(67, (v.guid() >> 32) as u32);
    }

    pub fn container_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&66);
        let upper = self.values.get(&67);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdateUnit {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, v as u32);
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, v as u32);
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_CHARM(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHARM(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_SUMMON(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn unit_SUMMON(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CRITTER(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn unit_CRITTER(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CHARMEDBY(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHARMEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_SUMMONEDBY(&mut self, v: Guid) {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
    }

    pub fn unit_SUMMONEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CREATEDBY(&mut self, v: Guid) {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
    }

    pub fn unit_CREATEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_TARGET(&mut self, v: Guid) {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
    }

    pub fn unit_TARGET(&self) -> Option<Guid> {
        let lower = self.values.get(&18);
        let upper = self.values.get(&19);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CHANNEL_OBJECT(&mut self, v: Guid) {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHANNEL_OBJECT(&self) -> Option<Guid> {
        let lower = self.values.get(&20);
        let upper = self.values.get(&21);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CHANNEL_SPELL(&mut self, v: i32) {
        self.header_set(22, v as u32);
    }

    pub fn unit_CHANNEL_SPELL(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.header_set(23, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
    }

    pub fn unit_BYTES_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&23) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_HEALTH(&mut self, v: i32) {
        self.header_set(24, v as u32);
    }

    pub fn unit_HEALTH(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_POWER1(&mut self, v: i32) {
        self.header_set(25, v as u32);
    }

    pub fn unit_POWER1(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_POWER2(&mut self, v: i32) {
        self.header_set(26, v as u32);
    }

    pub fn unit_POWER2(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_POWER3(&mut self, v: i32) {
        self.header_set(27, v as u32);
    }

    pub fn unit_POWER3(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_POWER4(&mut self, v: i32) {
        self.header_set(28, v as u32);
    }

    pub fn unit_POWER4(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_POWER5(&mut self, v: i32) {
        self.header_set(29, v as u32);
    }

    pub fn unit_POWER5(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_POWER6(&mut self, v: i32) {
        self.header_set(30, v as u32);
    }

    pub fn unit_POWER6(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_POWER7(&mut self, v: i32) {
        self.header_set(31, v as u32);
    }

    pub fn unit_POWER7(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_MAXHEALTH(&mut self, v: i32) {
        self.header_set(32, v as u32);
    }

    pub fn unit_MAXHEALTH(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER1(&mut self, v: i32) {
        self.header_set(33, v as u32);
    }

    pub fn unit_MAXPOWER1(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER2(&mut self, v: i32) {
        self.header_set(34, v as u32);
    }

    pub fn unit_MAXPOWER2(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER3(&mut self, v: i32) {
        self.header_set(35, v as u32);
    }

    pub fn unit_MAXPOWER3(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER4(&mut self, v: i32) {
        self.header_set(36, v as u32);
    }

    pub fn unit_MAXPOWER4(&self) -> Option<i32> {
        self.values.get(&36).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER5(&mut self, v: i32) {
        self.header_set(37, v as u32);
    }

    pub fn unit_MAXPOWER5(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER6(&mut self, v: i32) {
        self.header_set(38, v as u32);
    }

    pub fn unit_MAXPOWER6(&self) -> Option<i32> {
        self.values.get(&38).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER7(&mut self, v: i32) {
        self.header_set(39, v as u32);
    }

    pub fn unit_MAXPOWER7(&self) -> Option<i32> {
        self.values.get(&39).map(|v| *v as i32)
    }

    pub fn set_unit_POWER_REGEN_FLAT_MODIFIER(&mut self, v: f32) {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_REGEN_FLAT_MODIFIER(&self) -> Option<f32> {
        self.values.get(&40).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_POWER_REGEN_INTERRUPTED_FLAT_MODIFIER(&mut self, v: f32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_REGEN_INTERRUPTED_FLAT_MODIFIER(&self) -> Option<f32> {
        self.values.get(&47).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_LEVEL(&mut self, v: i32) {
        self.header_set(54, v as u32);
    }

    pub fn unit_LEVEL(&self) -> Option<i32> {
        self.values.get(&54).map(|v| *v as i32)
    }

    pub fn set_unit_FACTIONTEMPLATE(&mut self, v: i32) {
        self.header_set(55, v as u32);
    }

    pub fn unit_FACTIONTEMPLATE(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_ID(&mut self, v: i32) {
        self.header_set(56, v as u32);
    }

    pub fn unit_VIRTUAL_ITEM_SLOT_ID(&self) -> Option<i32> {
        self.values.get(&56).map(|v| *v as i32)
    }

    pub fn set_unit_FLAGS(&mut self, v: i32) {
        self.header_set(59, v as u32);
    }

    pub fn unit_FLAGS(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
    }

    pub fn set_unit_FLAGS_2(&mut self, v: i32) {
        self.header_set(60, v as u32);
    }

    pub fn unit_FLAGS_2(&self) -> Option<i32> {
        self.values.get(&60).map(|v| *v as i32)
    }

    pub fn set_unit_AURASTATE(&mut self, v: i32) {
        self.header_set(61, v as u32);
    }

    pub fn unit_AURASTATE(&self) -> Option<i32> {
        self.values.get(&61).map(|v| *v as i32)
    }

    pub fn set_unit_BASEATTACKTIME(&mut self, v: i32) {
        self.header_set(62, v as u32);
    }

    pub fn unit_BASEATTACKTIME(&self) -> Option<i32> {
        self.values.get(&62).map(|v| *v as i32)
    }

    pub fn set_unit_RANGEDATTACKTIME(&mut self, v: i32) {
        self.header_set(64, v as u32);
    }

    pub fn unit_RANGEDATTACKTIME(&self) -> Option<i32> {
        self.values.get(&64).map(|v| *v as i32)
    }

    pub fn set_unit_BOUNDINGRADIUS(&mut self, v: f32) {
        self.header_set(65, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BOUNDINGRADIUS(&self) -> Option<f32> {
        self.values.get(&65).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_COMBATREACH(&mut self, v: f32) {
        self.header_set(66, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_COMBATREACH(&self) -> Option<f32> {
        self.values.get(&66).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_DISPLAYID(&mut self, v: i32) {
        self.header_set(67, v as u32);
    }

    pub fn unit_DISPLAYID(&self) -> Option<i32> {
        self.values.get(&67).map(|v| *v as i32)
    }

    pub fn set_unit_NATIVEDISPLAYID(&mut self, v: i32) {
        self.header_set(68, v as u32);
    }

    pub fn unit_NATIVEDISPLAYID(&self) -> Option<i32> {
        self.values.get(&68).map(|v| *v as i32)
    }

    pub fn set_unit_MOUNTDISPLAYID(&mut self, v: i32) {
        self.header_set(69, v as u32);
    }

    pub fn unit_MOUNTDISPLAYID(&self) -> Option<i32> {
        self.values.get(&69).map(|v| *v as i32)
    }

    pub fn set_unit_MINDAMAGE(&mut self, v: f32) {
        self.header_set(70, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINDAMAGE(&self) -> Option<f32> {
        self.values.get(&70).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXDAMAGE(&mut self, v: f32) {
        self.header_set(71, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXDAMAGE(&self) -> Option<f32> {
        self.values.get(&71).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(72, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&72).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(73, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&73).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_BYTES_1(&mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) {
        self.header_set(74, u32::from_le_bytes([stand_state.as_int(), unknown1, unknown2, unknown3]));
    }

    pub fn unit_BYTES_1(&self) -> Option<(UnitStandState, u8, u8, u8)> {
        if let Some(v) = self.values.get(&74) {
            let v = v.to_le_bytes();
            let (stand_state, unknown1, unknown2, unknown3) = (v[0], v[1], v[2], v[3]);
            Some((stand_state.try_into().unwrap(), unknown1, unknown2, unknown3))
        } else {
            None
        }
    }

    pub fn set_unit_PETNUMBER(&mut self, v: i32) {
        self.header_set(75, v as u32);
    }

    pub fn unit_PETNUMBER(&self) -> Option<i32> {
        self.values.get(&75).map(|v| *v as i32)
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(&mut self, v: i32) {
        self.header_set(76, v as u32);
    }

    pub fn unit_PET_NAME_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&76).map(|v| *v as i32)
    }

    pub fn set_unit_PETEXPERIENCE(&mut self, v: i32) {
        self.header_set(77, v as u32);
    }

    pub fn unit_PETEXPERIENCE(&self) -> Option<i32> {
        self.values.get(&77).map(|v| *v as i32)
    }

    pub fn set_unit_PETNEXTLEVELEXP(&mut self, v: i32) {
        self.header_set(78, v as u32);
    }

    pub fn unit_PETNEXTLEVELEXP(&self) -> Option<i32> {
        self.values.get(&78).map(|v| *v as i32)
    }

    pub fn set_unit_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(79, v as u32);
    }

    pub fn unit_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&79).map(|v| *v as i32)
    }

    pub fn set_unit_MOD_CAST_SPEED(&mut self, v: f32) {
        self.header_set(80, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MOD_CAST_SPEED(&self) -> Option<f32> {
        self.values.get(&80).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_CREATED_BY_SPELL(&mut self, v: i32) {
        self.header_set(81, v as u32);
    }

    pub fn unit_CREATED_BY_SPELL(&self) -> Option<i32> {
        self.values.get(&81).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_FLAGS(&mut self, v: i32) {
        self.header_set(82, v as u32);
    }

    pub fn unit_NPC_FLAGS(&self) -> Option<i32> {
        self.values.get(&82).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_EMOTESTATE(&mut self, v: i32) {
        self.header_set(83, v as u32);
    }

    pub fn unit_NPC_EMOTESTATE(&self) -> Option<i32> {
        self.values.get(&83).map(|v| *v as i32)
    }

    pub fn set_unit_STRENGTH(&mut self, v: i32) {
        self.header_set(84, v as u32);
    }

    pub fn unit_STRENGTH(&self) -> Option<i32> {
        self.values.get(&84).map(|v| *v as i32)
    }

    pub fn set_unit_AGILITY(&mut self, v: i32) {
        self.header_set(85, v as u32);
    }

    pub fn unit_AGILITY(&self) -> Option<i32> {
        self.values.get(&85).map(|v| *v as i32)
    }

    pub fn set_unit_STAMINA(&mut self, v: i32) {
        self.header_set(86, v as u32);
    }

    pub fn unit_STAMINA(&self) -> Option<i32> {
        self.values.get(&86).map(|v| *v as i32)
    }

    pub fn set_unit_INTELLECT(&mut self, v: i32) {
        self.header_set(87, v as u32);
    }

    pub fn unit_INTELLECT(&self) -> Option<i32> {
        self.values.get(&87).map(|v| *v as i32)
    }

    pub fn set_unit_SPIRIT(&mut self, v: i32) {
        self.header_set(88, v as u32);
    }

    pub fn unit_SPIRIT(&self) -> Option<i32> {
        self.values.get(&88).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT0(&mut self, v: i32) {
        self.header_set(89, v as u32);
    }

    pub fn unit_POSSTAT0(&self) -> Option<i32> {
        self.values.get(&89).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT1(&mut self, v: i32) {
        self.header_set(90, v as u32);
    }

    pub fn unit_POSSTAT1(&self) -> Option<i32> {
        self.values.get(&90).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT2(&mut self, v: i32) {
        self.header_set(91, v as u32);
    }

    pub fn unit_POSSTAT2(&self) -> Option<i32> {
        self.values.get(&91).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT3(&mut self, v: i32) {
        self.header_set(92, v as u32);
    }

    pub fn unit_POSSTAT3(&self) -> Option<i32> {
        self.values.get(&92).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT4(&mut self, v: i32) {
        self.header_set(93, v as u32);
    }

    pub fn unit_POSSTAT4(&self) -> Option<i32> {
        self.values.get(&93).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT0(&mut self, v: i32) {
        self.header_set(94, v as u32);
    }

    pub fn unit_NEGSTAT0(&self) -> Option<i32> {
        self.values.get(&94).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT1(&mut self, v: i32) {
        self.header_set(95, v as u32);
    }

    pub fn unit_NEGSTAT1(&self) -> Option<i32> {
        self.values.get(&95).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT2(&mut self, v: i32) {
        self.header_set(96, v as u32);
    }

    pub fn unit_NEGSTAT2(&self) -> Option<i32> {
        self.values.get(&96).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT3(&mut self, v: i32) {
        self.header_set(97, v as u32);
    }

    pub fn unit_NEGSTAT3(&self) -> Option<i32> {
        self.values.get(&97).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT4(&mut self, v: i32) {
        self.header_set(98, v as u32);
    }

    pub fn unit_NEGSTAT4(&self) -> Option<i32> {
        self.values.get(&98).map(|v| *v as i32)
    }

    pub fn set_unit_RESISTANCES(&mut self, v: i32) {
        self.header_set(99, v as u32);
    }

    pub fn unit_RESISTANCES(&self) -> Option<i32> {
        self.values.get(&99).map(|v| *v as i32)
    }

    pub fn set_unit_RESISTANCEBUFFMODSPOSITIVE(&mut self, v: i32) {
        self.header_set(106, v as u32);
    }

    pub fn unit_RESISTANCEBUFFMODSPOSITIVE(&self) -> Option<i32> {
        self.values.get(&106).map(|v| *v as i32)
    }

    pub fn set_unit_RESISTANCEBUFFMODSNEGATIVE(&mut self, v: i32) {
        self.header_set(113, v as u32);
    }

    pub fn unit_RESISTANCEBUFFMODSNEGATIVE(&self) -> Option<i32> {
        self.values.get(&113).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_MANA(&mut self, v: i32) {
        self.header_set(120, v as u32);
    }

    pub fn unit_BASE_MANA(&self) -> Option<i32> {
        self.values.get(&120).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_HEALTH(&mut self, v: i32) {
        self.header_set(121, v as u32);
    }

    pub fn unit_BASE_HEALTH(&self) -> Option<i32> {
        self.values.get(&121).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(122, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&122) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(123, v as u32);
    }

    pub fn unit_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&123).map(|v| *v as i32)
    }

    pub fn set_unit_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(124, (a as u32) << 16 | b as u32);
    }

    pub fn unit_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&124) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&125).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_RANGED_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(126, v as u32);
    }

    pub fn unit_RANGED_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&126).map(|v| *v as i32)
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(127, (a as u32) << 16 | b as u32);
    }

    pub fn unit_RANGED_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&127) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGED_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&128).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&129).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&130).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_POWER_COST_MODIFIER(&mut self, v: i32) {
        self.header_set(131, v as u32);
    }

    pub fn unit_POWER_COST_MODIFIER(&self) -> Option<i32> {
        self.values.get(&131).map(|v| *v as i32)
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(&mut self, v: f32) {
        self.header_set(138, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_COST_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&138).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXHEALTHMODIFIER(&mut self, v: f32) {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXHEALTHMODIFIER(&self) -> Option<f32> {
        self.values.get(&145).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_HOVERHEIGHT(&mut self, v: f32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_HOVERHEIGHT(&self) -> Option<f32> {
        self.values.get(&146).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdatePlayer {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, v as u32);
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, v as u32);
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_CHARM(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHARM(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_SUMMON(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn unit_SUMMON(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CRITTER(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn unit_CRITTER(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CHARMEDBY(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHARMEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_SUMMONEDBY(&mut self, v: Guid) {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
    }

    pub fn unit_SUMMONEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CREATEDBY(&mut self, v: Guid) {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
    }

    pub fn unit_CREATEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_TARGET(&mut self, v: Guid) {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
    }

    pub fn unit_TARGET(&self) -> Option<Guid> {
        let lower = self.values.get(&18);
        let upper = self.values.get(&19);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CHANNEL_OBJECT(&mut self, v: Guid) {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHANNEL_OBJECT(&self) -> Option<Guid> {
        let lower = self.values.get(&20);
        let upper = self.values.get(&21);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CHANNEL_SPELL(&mut self, v: i32) {
        self.header_set(22, v as u32);
    }

    pub fn unit_CHANNEL_SPELL(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.header_set(23, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
    }

    pub fn unit_BYTES_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&23) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_HEALTH(&mut self, v: i32) {
        self.header_set(24, v as u32);
    }

    pub fn unit_HEALTH(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_POWER1(&mut self, v: i32) {
        self.header_set(25, v as u32);
    }

    pub fn unit_POWER1(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_POWER2(&mut self, v: i32) {
        self.header_set(26, v as u32);
    }

    pub fn unit_POWER2(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_POWER3(&mut self, v: i32) {
        self.header_set(27, v as u32);
    }

    pub fn unit_POWER3(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_POWER4(&mut self, v: i32) {
        self.header_set(28, v as u32);
    }

    pub fn unit_POWER4(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_POWER5(&mut self, v: i32) {
        self.header_set(29, v as u32);
    }

    pub fn unit_POWER5(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_POWER6(&mut self, v: i32) {
        self.header_set(30, v as u32);
    }

    pub fn unit_POWER6(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_POWER7(&mut self, v: i32) {
        self.header_set(31, v as u32);
    }

    pub fn unit_POWER7(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_MAXHEALTH(&mut self, v: i32) {
        self.header_set(32, v as u32);
    }

    pub fn unit_MAXHEALTH(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER1(&mut self, v: i32) {
        self.header_set(33, v as u32);
    }

    pub fn unit_MAXPOWER1(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER2(&mut self, v: i32) {
        self.header_set(34, v as u32);
    }

    pub fn unit_MAXPOWER2(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER3(&mut self, v: i32) {
        self.header_set(35, v as u32);
    }

    pub fn unit_MAXPOWER3(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER4(&mut self, v: i32) {
        self.header_set(36, v as u32);
    }

    pub fn unit_MAXPOWER4(&self) -> Option<i32> {
        self.values.get(&36).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER5(&mut self, v: i32) {
        self.header_set(37, v as u32);
    }

    pub fn unit_MAXPOWER5(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER6(&mut self, v: i32) {
        self.header_set(38, v as u32);
    }

    pub fn unit_MAXPOWER6(&self) -> Option<i32> {
        self.values.get(&38).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER7(&mut self, v: i32) {
        self.header_set(39, v as u32);
    }

    pub fn unit_MAXPOWER7(&self) -> Option<i32> {
        self.values.get(&39).map(|v| *v as i32)
    }

    pub fn set_unit_POWER_REGEN_FLAT_MODIFIER(&mut self, v: f32) {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_REGEN_FLAT_MODIFIER(&self) -> Option<f32> {
        self.values.get(&40).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_POWER_REGEN_INTERRUPTED_FLAT_MODIFIER(&mut self, v: f32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_REGEN_INTERRUPTED_FLAT_MODIFIER(&self) -> Option<f32> {
        self.values.get(&47).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_LEVEL(&mut self, v: i32) {
        self.header_set(54, v as u32);
    }

    pub fn unit_LEVEL(&self) -> Option<i32> {
        self.values.get(&54).map(|v| *v as i32)
    }

    pub fn set_unit_FACTIONTEMPLATE(&mut self, v: i32) {
        self.header_set(55, v as u32);
    }

    pub fn unit_FACTIONTEMPLATE(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_ID(&mut self, v: i32) {
        self.header_set(56, v as u32);
    }

    pub fn unit_VIRTUAL_ITEM_SLOT_ID(&self) -> Option<i32> {
        self.values.get(&56).map(|v| *v as i32)
    }

    pub fn set_unit_FLAGS(&mut self, v: i32) {
        self.header_set(59, v as u32);
    }

    pub fn unit_FLAGS(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
    }

    pub fn set_unit_FLAGS_2(&mut self, v: i32) {
        self.header_set(60, v as u32);
    }

    pub fn unit_FLAGS_2(&self) -> Option<i32> {
        self.values.get(&60).map(|v| *v as i32)
    }

    pub fn set_unit_AURASTATE(&mut self, v: i32) {
        self.header_set(61, v as u32);
    }

    pub fn unit_AURASTATE(&self) -> Option<i32> {
        self.values.get(&61).map(|v| *v as i32)
    }

    pub fn set_unit_BASEATTACKTIME(&mut self, v: i32) {
        self.header_set(62, v as u32);
    }

    pub fn unit_BASEATTACKTIME(&self) -> Option<i32> {
        self.values.get(&62).map(|v| *v as i32)
    }

    pub fn set_unit_RANGEDATTACKTIME(&mut self, v: i32) {
        self.header_set(64, v as u32);
    }

    pub fn unit_RANGEDATTACKTIME(&self) -> Option<i32> {
        self.values.get(&64).map(|v| *v as i32)
    }

    pub fn set_unit_BOUNDINGRADIUS(&mut self, v: f32) {
        self.header_set(65, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BOUNDINGRADIUS(&self) -> Option<f32> {
        self.values.get(&65).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_COMBATREACH(&mut self, v: f32) {
        self.header_set(66, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_COMBATREACH(&self) -> Option<f32> {
        self.values.get(&66).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_DISPLAYID(&mut self, v: i32) {
        self.header_set(67, v as u32);
    }

    pub fn unit_DISPLAYID(&self) -> Option<i32> {
        self.values.get(&67).map(|v| *v as i32)
    }

    pub fn set_unit_NATIVEDISPLAYID(&mut self, v: i32) {
        self.header_set(68, v as u32);
    }

    pub fn unit_NATIVEDISPLAYID(&self) -> Option<i32> {
        self.values.get(&68).map(|v| *v as i32)
    }

    pub fn set_unit_MOUNTDISPLAYID(&mut self, v: i32) {
        self.header_set(69, v as u32);
    }

    pub fn unit_MOUNTDISPLAYID(&self) -> Option<i32> {
        self.values.get(&69).map(|v| *v as i32)
    }

    pub fn set_unit_MINDAMAGE(&mut self, v: f32) {
        self.header_set(70, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINDAMAGE(&self) -> Option<f32> {
        self.values.get(&70).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXDAMAGE(&mut self, v: f32) {
        self.header_set(71, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXDAMAGE(&self) -> Option<f32> {
        self.values.get(&71).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(72, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&72).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(73, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&73).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_BYTES_1(&mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) {
        self.header_set(74, u32::from_le_bytes([stand_state.as_int(), unknown1, unknown2, unknown3]));
    }

    pub fn unit_BYTES_1(&self) -> Option<(UnitStandState, u8, u8, u8)> {
        if let Some(v) = self.values.get(&74) {
            let v = v.to_le_bytes();
            let (stand_state, unknown1, unknown2, unknown3) = (v[0], v[1], v[2], v[3]);
            Some((stand_state.try_into().unwrap(), unknown1, unknown2, unknown3))
        } else {
            None
        }
    }

    pub fn set_unit_PETNUMBER(&mut self, v: i32) {
        self.header_set(75, v as u32);
    }

    pub fn unit_PETNUMBER(&self) -> Option<i32> {
        self.values.get(&75).map(|v| *v as i32)
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(&mut self, v: i32) {
        self.header_set(76, v as u32);
    }

    pub fn unit_PET_NAME_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&76).map(|v| *v as i32)
    }

    pub fn set_unit_PETEXPERIENCE(&mut self, v: i32) {
        self.header_set(77, v as u32);
    }

    pub fn unit_PETEXPERIENCE(&self) -> Option<i32> {
        self.values.get(&77).map(|v| *v as i32)
    }

    pub fn set_unit_PETNEXTLEVELEXP(&mut self, v: i32) {
        self.header_set(78, v as u32);
    }

    pub fn unit_PETNEXTLEVELEXP(&self) -> Option<i32> {
        self.values.get(&78).map(|v| *v as i32)
    }

    pub fn set_unit_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(79, v as u32);
    }

    pub fn unit_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&79).map(|v| *v as i32)
    }

    pub fn set_unit_MOD_CAST_SPEED(&mut self, v: f32) {
        self.header_set(80, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MOD_CAST_SPEED(&self) -> Option<f32> {
        self.values.get(&80).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_CREATED_BY_SPELL(&mut self, v: i32) {
        self.header_set(81, v as u32);
    }

    pub fn unit_CREATED_BY_SPELL(&self) -> Option<i32> {
        self.values.get(&81).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_FLAGS(&mut self, v: i32) {
        self.header_set(82, v as u32);
    }

    pub fn unit_NPC_FLAGS(&self) -> Option<i32> {
        self.values.get(&82).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_EMOTESTATE(&mut self, v: i32) {
        self.header_set(83, v as u32);
    }

    pub fn unit_NPC_EMOTESTATE(&self) -> Option<i32> {
        self.values.get(&83).map(|v| *v as i32)
    }

    pub fn set_unit_STRENGTH(&mut self, v: i32) {
        self.header_set(84, v as u32);
    }

    pub fn unit_STRENGTH(&self) -> Option<i32> {
        self.values.get(&84).map(|v| *v as i32)
    }

    pub fn set_unit_AGILITY(&mut self, v: i32) {
        self.header_set(85, v as u32);
    }

    pub fn unit_AGILITY(&self) -> Option<i32> {
        self.values.get(&85).map(|v| *v as i32)
    }

    pub fn set_unit_STAMINA(&mut self, v: i32) {
        self.header_set(86, v as u32);
    }

    pub fn unit_STAMINA(&self) -> Option<i32> {
        self.values.get(&86).map(|v| *v as i32)
    }

    pub fn set_unit_INTELLECT(&mut self, v: i32) {
        self.header_set(87, v as u32);
    }

    pub fn unit_INTELLECT(&self) -> Option<i32> {
        self.values.get(&87).map(|v| *v as i32)
    }

    pub fn set_unit_SPIRIT(&mut self, v: i32) {
        self.header_set(88, v as u32);
    }

    pub fn unit_SPIRIT(&self) -> Option<i32> {
        self.values.get(&88).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT0(&mut self, v: i32) {
        self.header_set(89, v as u32);
    }

    pub fn unit_POSSTAT0(&self) -> Option<i32> {
        self.values.get(&89).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT1(&mut self, v: i32) {
        self.header_set(90, v as u32);
    }

    pub fn unit_POSSTAT1(&self) -> Option<i32> {
        self.values.get(&90).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT2(&mut self, v: i32) {
        self.header_set(91, v as u32);
    }

    pub fn unit_POSSTAT2(&self) -> Option<i32> {
        self.values.get(&91).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT3(&mut self, v: i32) {
        self.header_set(92, v as u32);
    }

    pub fn unit_POSSTAT3(&self) -> Option<i32> {
        self.values.get(&92).map(|v| *v as i32)
    }

    pub fn set_unit_POSSTAT4(&mut self, v: i32) {
        self.header_set(93, v as u32);
    }

    pub fn unit_POSSTAT4(&self) -> Option<i32> {
        self.values.get(&93).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT0(&mut self, v: i32) {
        self.header_set(94, v as u32);
    }

    pub fn unit_NEGSTAT0(&self) -> Option<i32> {
        self.values.get(&94).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT1(&mut self, v: i32) {
        self.header_set(95, v as u32);
    }

    pub fn unit_NEGSTAT1(&self) -> Option<i32> {
        self.values.get(&95).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT2(&mut self, v: i32) {
        self.header_set(96, v as u32);
    }

    pub fn unit_NEGSTAT2(&self) -> Option<i32> {
        self.values.get(&96).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT3(&mut self, v: i32) {
        self.header_set(97, v as u32);
    }

    pub fn unit_NEGSTAT3(&self) -> Option<i32> {
        self.values.get(&97).map(|v| *v as i32)
    }

    pub fn set_unit_NEGSTAT4(&mut self, v: i32) {
        self.header_set(98, v as u32);
    }

    pub fn unit_NEGSTAT4(&self) -> Option<i32> {
        self.values.get(&98).map(|v| *v as i32)
    }

    pub fn set_unit_RESISTANCES(&mut self, v: i32) {
        self.header_set(99, v as u32);
    }

    pub fn unit_RESISTANCES(&self) -> Option<i32> {
        self.values.get(&99).map(|v| *v as i32)
    }

    pub fn set_unit_RESISTANCEBUFFMODSPOSITIVE(&mut self, v: i32) {
        self.header_set(106, v as u32);
    }

    pub fn unit_RESISTANCEBUFFMODSPOSITIVE(&self) -> Option<i32> {
        self.values.get(&106).map(|v| *v as i32)
    }

    pub fn set_unit_RESISTANCEBUFFMODSNEGATIVE(&mut self, v: i32) {
        self.header_set(113, v as u32);
    }

    pub fn unit_RESISTANCEBUFFMODSNEGATIVE(&self) -> Option<i32> {
        self.values.get(&113).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_MANA(&mut self, v: i32) {
        self.header_set(120, v as u32);
    }

    pub fn unit_BASE_MANA(&self) -> Option<i32> {
        self.values.get(&120).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_HEALTH(&mut self, v: i32) {
        self.header_set(121, v as u32);
    }

    pub fn unit_BASE_HEALTH(&self) -> Option<i32> {
        self.values.get(&121).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(122, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&122) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(123, v as u32);
    }

    pub fn unit_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&123).map(|v| *v as i32)
    }

    pub fn set_unit_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(124, (a as u32) << 16 | b as u32);
    }

    pub fn unit_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&124) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&125).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_RANGED_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(126, v as u32);
    }

    pub fn unit_RANGED_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&126).map(|v| *v as i32)
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(127, (a as u32) << 16 | b as u32);
    }

    pub fn unit_RANGED_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&127) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGED_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&128).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&129).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&130).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_POWER_COST_MODIFIER(&mut self, v: i32) {
        self.header_set(131, v as u32);
    }

    pub fn unit_POWER_COST_MODIFIER(&self) -> Option<i32> {
        self.values.get(&131).map(|v| *v as i32)
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(&mut self, v: f32) {
        self.header_set(138, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_COST_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&138).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXHEALTHMODIFIER(&mut self, v: f32) {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXHEALTHMODIFIER(&self) -> Option<f32> {
        self.values.get(&145).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_HOVERHEIGHT(&mut self, v: f32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_HOVERHEIGHT(&self) -> Option<f32> {
        self.values.get(&146).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_DUEL_ARBITER(&mut self, v: Guid) {
        self.header_set(148, v.guid() as u32);
        self.header_set(149, (v.guid() >> 32) as u32);
    }

    pub fn player_DUEL_ARBITER(&self) -> Option<Guid> {
        let lower = self.values.get(&148);
        let upper = self.values.get(&149);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FLAGS(&mut self, v: i32) {
        self.header_set(150, v as u32);
    }

    pub fn player_FLAGS(&self) -> Option<i32> {
        self.values.get(&150).map(|v| *v as i32)
    }

    pub fn set_player_GUILDID(&mut self, v: i32) {
        self.header_set(151, v as u32);
    }

    pub fn player_GUILDID(&self) -> Option<i32> {
        self.values.get(&151).map(|v| *v as i32)
    }

    pub fn set_player_GUILDRANK(&mut self, v: i32) {
        self.header_set(152, v as u32);
    }

    pub fn player_GUILDRANK(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_BYTES(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(153, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_FIELD_BYTES(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&153) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_BYTES_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(154, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&154) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_BYTES_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(155, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_BYTES_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&155) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_DUEL_TEAM(&mut self, v: i32) {
        self.header_set(156, v as u32);
    }

    pub fn player_DUEL_TEAM(&self) -> Option<i32> {
        self.values.get(&156).map(|v| *v as i32)
    }

    pub fn set_player_GUILD_TIMESTAMP(&mut self, v: i32) {
        self.header_set(157, v as u32);
    }

    pub fn player_GUILD_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&157).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_1_1(&mut self, v: i32) {
        self.header_set(158, v as u32);
    }

    pub fn player_QUEST_LOG_1_1(&self) -> Option<i32> {
        self.values.get(&158).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_1_2(&mut self, v: i32) {
        self.header_set(159, v as u32);
    }

    pub fn player_QUEST_LOG_1_2(&self) -> Option<i32> {
        self.values.get(&159).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_1_3(&mut self, a: u16, b: u16) {
        self.header_set(160, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_1_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&160) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_1_4(&mut self, v: i32) {
        self.header_set(162, v as u32);
    }

    pub fn player_QUEST_LOG_1_4(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_2_1(&mut self, v: i32) {
        self.header_set(163, v as u32);
    }

    pub fn player_QUEST_LOG_2_1(&self) -> Option<i32> {
        self.values.get(&163).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_2_2(&mut self, v: i32) {
        self.header_set(164, v as u32);
    }

    pub fn player_QUEST_LOG_2_2(&self) -> Option<i32> {
        self.values.get(&164).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_2_3(&mut self, a: u16, b: u16) {
        self.header_set(165, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_2_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&165) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_2_5(&mut self, v: i32) {
        self.header_set(167, v as u32);
    }

    pub fn player_QUEST_LOG_2_5(&self) -> Option<i32> {
        self.values.get(&167).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_3_1(&mut self, v: i32) {
        self.header_set(168, v as u32);
    }

    pub fn player_QUEST_LOG_3_1(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_3_2(&mut self, v: i32) {
        self.header_set(169, v as u32);
    }

    pub fn player_QUEST_LOG_3_2(&self) -> Option<i32> {
        self.values.get(&169).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_3_3(&mut self, a: u16, b: u16) {
        self.header_set(170, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_3_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&170) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_3_5(&mut self, v: i32) {
        self.header_set(172, v as u32);
    }

    pub fn player_QUEST_LOG_3_5(&self) -> Option<i32> {
        self.values.get(&172).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_4_1(&mut self, v: i32) {
        self.header_set(173, v as u32);
    }

    pub fn player_QUEST_LOG_4_1(&self) -> Option<i32> {
        self.values.get(&173).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_4_2(&mut self, v: i32) {
        self.header_set(174, v as u32);
    }

    pub fn player_QUEST_LOG_4_2(&self) -> Option<i32> {
        self.values.get(&174).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_4_3(&mut self, a: u16, b: u16) {
        self.header_set(175, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_4_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&175) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_4_5(&mut self, v: i32) {
        self.header_set(177, v as u32);
    }

    pub fn player_QUEST_LOG_4_5(&self) -> Option<i32> {
        self.values.get(&177).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_5_1(&mut self, v: i32) {
        self.header_set(178, v as u32);
    }

    pub fn player_QUEST_LOG_5_1(&self) -> Option<i32> {
        self.values.get(&178).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_5_2(&mut self, v: i32) {
        self.header_set(179, v as u32);
    }

    pub fn player_QUEST_LOG_5_2(&self) -> Option<i32> {
        self.values.get(&179).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_5_3(&mut self, a: u16, b: u16) {
        self.header_set(180, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_5_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&180) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_5_5(&mut self, v: i32) {
        self.header_set(182, v as u32);
    }

    pub fn player_QUEST_LOG_5_5(&self) -> Option<i32> {
        self.values.get(&182).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_6_1(&mut self, v: i32) {
        self.header_set(183, v as u32);
    }

    pub fn player_QUEST_LOG_6_1(&self) -> Option<i32> {
        self.values.get(&183).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_6_2(&mut self, v: i32) {
        self.header_set(184, v as u32);
    }

    pub fn player_QUEST_LOG_6_2(&self) -> Option<i32> {
        self.values.get(&184).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_6_3(&mut self, a: u16, b: u16) {
        self.header_set(185, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_6_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&185) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_6_5(&mut self, v: i32) {
        self.header_set(187, v as u32);
    }

    pub fn player_QUEST_LOG_6_5(&self) -> Option<i32> {
        self.values.get(&187).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_7_1(&mut self, v: i32) {
        self.header_set(188, v as u32);
    }

    pub fn player_QUEST_LOG_7_1(&self) -> Option<i32> {
        self.values.get(&188).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_7_2(&mut self, v: i32) {
        self.header_set(189, v as u32);
    }

    pub fn player_QUEST_LOG_7_2(&self) -> Option<i32> {
        self.values.get(&189).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_7_3(&mut self, a: u16, b: u16) {
        self.header_set(190, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_7_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&190) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_7_5(&mut self, v: i32) {
        self.header_set(192, v as u32);
    }

    pub fn player_QUEST_LOG_7_5(&self) -> Option<i32> {
        self.values.get(&192).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_8_1(&mut self, v: i32) {
        self.header_set(193, v as u32);
    }

    pub fn player_QUEST_LOG_8_1(&self) -> Option<i32> {
        self.values.get(&193).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_8_2(&mut self, v: i32) {
        self.header_set(194, v as u32);
    }

    pub fn player_QUEST_LOG_8_2(&self) -> Option<i32> {
        self.values.get(&194).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_8_3(&mut self, a: u16, b: u16) {
        self.header_set(195, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_8_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&195) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_8_5(&mut self, v: i32) {
        self.header_set(197, v as u32);
    }

    pub fn player_QUEST_LOG_8_5(&self) -> Option<i32> {
        self.values.get(&197).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_9_1(&mut self, v: i32) {
        self.header_set(198, v as u32);
    }

    pub fn player_QUEST_LOG_9_1(&self) -> Option<i32> {
        self.values.get(&198).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_9_2(&mut self, v: i32) {
        self.header_set(199, v as u32);
    }

    pub fn player_QUEST_LOG_9_2(&self) -> Option<i32> {
        self.values.get(&199).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_9_3(&mut self, a: u16, b: u16) {
        self.header_set(200, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_9_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&200) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_9_5(&mut self, v: i32) {
        self.header_set(202, v as u32);
    }

    pub fn player_QUEST_LOG_9_5(&self) -> Option<i32> {
        self.values.get(&202).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_10_1(&mut self, v: i32) {
        self.header_set(203, v as u32);
    }

    pub fn player_QUEST_LOG_10_1(&self) -> Option<i32> {
        self.values.get(&203).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_10_2(&mut self, v: i32) {
        self.header_set(204, v as u32);
    }

    pub fn player_QUEST_LOG_10_2(&self) -> Option<i32> {
        self.values.get(&204).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_10_3(&mut self, a: u16, b: u16) {
        self.header_set(205, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_10_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&205) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_10_5(&mut self, v: i32) {
        self.header_set(207, v as u32);
    }

    pub fn player_QUEST_LOG_10_5(&self) -> Option<i32> {
        self.values.get(&207).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_11_1(&mut self, v: i32) {
        self.header_set(208, v as u32);
    }

    pub fn player_QUEST_LOG_11_1(&self) -> Option<i32> {
        self.values.get(&208).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_11_2(&mut self, v: i32) {
        self.header_set(209, v as u32);
    }

    pub fn player_QUEST_LOG_11_2(&self) -> Option<i32> {
        self.values.get(&209).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_11_3(&mut self, a: u16, b: u16) {
        self.header_set(210, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_11_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&210) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_11_5(&mut self, v: i32) {
        self.header_set(212, v as u32);
    }

    pub fn player_QUEST_LOG_11_5(&self) -> Option<i32> {
        self.values.get(&212).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_12_1(&mut self, v: i32) {
        self.header_set(213, v as u32);
    }

    pub fn player_QUEST_LOG_12_1(&self) -> Option<i32> {
        self.values.get(&213).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_12_2(&mut self, v: i32) {
        self.header_set(214, v as u32);
    }

    pub fn player_QUEST_LOG_12_2(&self) -> Option<i32> {
        self.values.get(&214).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_12_3(&mut self, a: u16, b: u16) {
        self.header_set(215, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_12_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&215) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_12_5(&mut self, v: i32) {
        self.header_set(217, v as u32);
    }

    pub fn player_QUEST_LOG_12_5(&self) -> Option<i32> {
        self.values.get(&217).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_13_1(&mut self, v: i32) {
        self.header_set(218, v as u32);
    }

    pub fn player_QUEST_LOG_13_1(&self) -> Option<i32> {
        self.values.get(&218).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_13_2(&mut self, v: i32) {
        self.header_set(219, v as u32);
    }

    pub fn player_QUEST_LOG_13_2(&self) -> Option<i32> {
        self.values.get(&219).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_13_3(&mut self, a: u16, b: u16) {
        self.header_set(220, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_13_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&220) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_13_5(&mut self, v: i32) {
        self.header_set(222, v as u32);
    }

    pub fn player_QUEST_LOG_13_5(&self) -> Option<i32> {
        self.values.get(&222).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_14_1(&mut self, v: i32) {
        self.header_set(223, v as u32);
    }

    pub fn player_QUEST_LOG_14_1(&self) -> Option<i32> {
        self.values.get(&223).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_14_2(&mut self, v: i32) {
        self.header_set(224, v as u32);
    }

    pub fn player_QUEST_LOG_14_2(&self) -> Option<i32> {
        self.values.get(&224).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_14_3(&mut self, a: u16, b: u16) {
        self.header_set(225, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_14_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&225) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_14_5(&mut self, v: i32) {
        self.header_set(227, v as u32);
    }

    pub fn player_QUEST_LOG_14_5(&self) -> Option<i32> {
        self.values.get(&227).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_15_1(&mut self, v: i32) {
        self.header_set(228, v as u32);
    }

    pub fn player_QUEST_LOG_15_1(&self) -> Option<i32> {
        self.values.get(&228).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_15_2(&mut self, v: i32) {
        self.header_set(229, v as u32);
    }

    pub fn player_QUEST_LOG_15_2(&self) -> Option<i32> {
        self.values.get(&229).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_15_3(&mut self, a: u16, b: u16) {
        self.header_set(230, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_15_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&230) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_15_5(&mut self, v: i32) {
        self.header_set(232, v as u32);
    }

    pub fn player_QUEST_LOG_15_5(&self) -> Option<i32> {
        self.values.get(&232).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_16_1(&mut self, v: i32) {
        self.header_set(233, v as u32);
    }

    pub fn player_QUEST_LOG_16_1(&self) -> Option<i32> {
        self.values.get(&233).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_16_2(&mut self, v: i32) {
        self.header_set(234, v as u32);
    }

    pub fn player_QUEST_LOG_16_2(&self) -> Option<i32> {
        self.values.get(&234).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_16_3(&mut self, a: u16, b: u16) {
        self.header_set(235, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_16_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&235) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_16_5(&mut self, v: i32) {
        self.header_set(237, v as u32);
    }

    pub fn player_QUEST_LOG_16_5(&self) -> Option<i32> {
        self.values.get(&237).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_17_1(&mut self, v: i32) {
        self.header_set(238, v as u32);
    }

    pub fn player_QUEST_LOG_17_1(&self) -> Option<i32> {
        self.values.get(&238).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_17_2(&mut self, v: i32) {
        self.header_set(239, v as u32);
    }

    pub fn player_QUEST_LOG_17_2(&self) -> Option<i32> {
        self.values.get(&239).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_17_3(&mut self, a: u16, b: u16) {
        self.header_set(240, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_17_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&240) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_17_5(&mut self, v: i32) {
        self.header_set(242, v as u32);
    }

    pub fn player_QUEST_LOG_17_5(&self) -> Option<i32> {
        self.values.get(&242).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_18_1(&mut self, v: i32) {
        self.header_set(243, v as u32);
    }

    pub fn player_QUEST_LOG_18_1(&self) -> Option<i32> {
        self.values.get(&243).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_18_2(&mut self, v: i32) {
        self.header_set(244, v as u32);
    }

    pub fn player_QUEST_LOG_18_2(&self) -> Option<i32> {
        self.values.get(&244).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_18_3(&mut self, a: u16, b: u16) {
        self.header_set(245, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_18_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&245) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_18_5(&mut self, v: i32) {
        self.header_set(247, v as u32);
    }

    pub fn player_QUEST_LOG_18_5(&self) -> Option<i32> {
        self.values.get(&247).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_19_1(&mut self, v: i32) {
        self.header_set(248, v as u32);
    }

    pub fn player_QUEST_LOG_19_1(&self) -> Option<i32> {
        self.values.get(&248).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_19_2(&mut self, v: i32) {
        self.header_set(249, v as u32);
    }

    pub fn player_QUEST_LOG_19_2(&self) -> Option<i32> {
        self.values.get(&249).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_19_3(&mut self, a: u16, b: u16) {
        self.header_set(250, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_19_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&250) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_19_5(&mut self, v: i32) {
        self.header_set(252, v as u32);
    }

    pub fn player_QUEST_LOG_19_5(&self) -> Option<i32> {
        self.values.get(&252).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_20_1(&mut self, v: i32) {
        self.header_set(253, v as u32);
    }

    pub fn player_QUEST_LOG_20_1(&self) -> Option<i32> {
        self.values.get(&253).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_20_2(&mut self, v: i32) {
        self.header_set(254, v as u32);
    }

    pub fn player_QUEST_LOG_20_2(&self) -> Option<i32> {
        self.values.get(&254).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_20_3(&mut self, a: u16, b: u16) {
        self.header_set(255, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_20_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&255) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_20_5(&mut self, v: i32) {
        self.header_set(257, v as u32);
    }

    pub fn player_QUEST_LOG_20_5(&self) -> Option<i32> {
        self.values.get(&257).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_21_1(&mut self, v: i32) {
        self.header_set(258, v as u32);
    }

    pub fn player_QUEST_LOG_21_1(&self) -> Option<i32> {
        self.values.get(&258).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_21_2(&mut self, v: i32) {
        self.header_set(259, v as u32);
    }

    pub fn player_QUEST_LOG_21_2(&self) -> Option<i32> {
        self.values.get(&259).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_21_3(&mut self, a: u16, b: u16) {
        self.header_set(260, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_21_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&260) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_21_5(&mut self, v: i32) {
        self.header_set(262, v as u32);
    }

    pub fn player_QUEST_LOG_21_5(&self) -> Option<i32> {
        self.values.get(&262).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_22_1(&mut self, v: i32) {
        self.header_set(263, v as u32);
    }

    pub fn player_QUEST_LOG_22_1(&self) -> Option<i32> {
        self.values.get(&263).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_22_2(&mut self, v: i32) {
        self.header_set(264, v as u32);
    }

    pub fn player_QUEST_LOG_22_2(&self) -> Option<i32> {
        self.values.get(&264).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_22_3(&mut self, a: u16, b: u16) {
        self.header_set(265, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_22_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&265) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_22_5(&mut self, v: i32) {
        self.header_set(267, v as u32);
    }

    pub fn player_QUEST_LOG_22_5(&self) -> Option<i32> {
        self.values.get(&267).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_23_1(&mut self, v: i32) {
        self.header_set(268, v as u32);
    }

    pub fn player_QUEST_LOG_23_1(&self) -> Option<i32> {
        self.values.get(&268).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_23_2(&mut self, v: i32) {
        self.header_set(269, v as u32);
    }

    pub fn player_QUEST_LOG_23_2(&self) -> Option<i32> {
        self.values.get(&269).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_23_3(&mut self, a: u16, b: u16) {
        self.header_set(270, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_23_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&270) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_23_5(&mut self, v: i32) {
        self.header_set(272, v as u32);
    }

    pub fn player_QUEST_LOG_23_5(&self) -> Option<i32> {
        self.values.get(&272).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_24_1(&mut self, v: i32) {
        self.header_set(273, v as u32);
    }

    pub fn player_QUEST_LOG_24_1(&self) -> Option<i32> {
        self.values.get(&273).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_24_2(&mut self, v: i32) {
        self.header_set(274, v as u32);
    }

    pub fn player_QUEST_LOG_24_2(&self) -> Option<i32> {
        self.values.get(&274).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_24_3(&mut self, a: u16, b: u16) {
        self.header_set(275, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_24_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&275) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_24_5(&mut self, v: i32) {
        self.header_set(277, v as u32);
    }

    pub fn player_QUEST_LOG_24_5(&self) -> Option<i32> {
        self.values.get(&277).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_25_1(&mut self, v: i32) {
        self.header_set(278, v as u32);
    }

    pub fn player_QUEST_LOG_25_1(&self) -> Option<i32> {
        self.values.get(&278).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_25_2(&mut self, v: i32) {
        self.header_set(279, v as u32);
    }

    pub fn player_QUEST_LOG_25_2(&self) -> Option<i32> {
        self.values.get(&279).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_25_3(&mut self, a: u16, b: u16) {
        self.header_set(280, (a as u32) << 16 | b as u32);
    }

    pub fn player_QUEST_LOG_25_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&280) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_QUEST_LOG_25_5(&mut self, v: i32) {
        self.header_set(282, v as u32);
    }

    pub fn player_QUEST_LOG_25_5(&self) -> Option<i32> {
        self.values.get(&282).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_1_ENTRYID(&mut self, v: i32) {
        self.header_set(283, v as u32);
    }

    pub fn player_VISIBLE_ITEM_1_ENTRYID(&self) -> Option<i32> {
        self.values.get(&283).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_1_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(284, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_1_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&284) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_2_ENTRYID(&mut self, v: i32) {
        self.header_set(285, v as u32);
    }

    pub fn player_VISIBLE_ITEM_2_ENTRYID(&self) -> Option<i32> {
        self.values.get(&285).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_2_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(286, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_2_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&286) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_3_ENTRYID(&mut self, v: i32) {
        self.header_set(287, v as u32);
    }

    pub fn player_VISIBLE_ITEM_3_ENTRYID(&self) -> Option<i32> {
        self.values.get(&287).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_3_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(288, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_3_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&288) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_4_ENTRYID(&mut self, v: i32) {
        self.header_set(289, v as u32);
    }

    pub fn player_VISIBLE_ITEM_4_ENTRYID(&self) -> Option<i32> {
        self.values.get(&289).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_4_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(290, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_4_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&290) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_5_ENTRYID(&mut self, v: i32) {
        self.header_set(291, v as u32);
    }

    pub fn player_VISIBLE_ITEM_5_ENTRYID(&self) -> Option<i32> {
        self.values.get(&291).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_5_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(292, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_5_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&292) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_6_ENTRYID(&mut self, v: i32) {
        self.header_set(293, v as u32);
    }

    pub fn player_VISIBLE_ITEM_6_ENTRYID(&self) -> Option<i32> {
        self.values.get(&293).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_6_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(294, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_6_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&294) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_7_ENTRYID(&mut self, v: i32) {
        self.header_set(295, v as u32);
    }

    pub fn player_VISIBLE_ITEM_7_ENTRYID(&self) -> Option<i32> {
        self.values.get(&295).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_7_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(296, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_7_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&296) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_8_ENTRYID(&mut self, v: i32) {
        self.header_set(297, v as u32);
    }

    pub fn player_VISIBLE_ITEM_8_ENTRYID(&self) -> Option<i32> {
        self.values.get(&297).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_8_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(298, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_8_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&298) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_9_ENTRYID(&mut self, v: i32) {
        self.header_set(299, v as u32);
    }

    pub fn player_VISIBLE_ITEM_9_ENTRYID(&self) -> Option<i32> {
        self.values.get(&299).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_9_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(300, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_9_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&300) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_10_ENTRYID(&mut self, v: i32) {
        self.header_set(301, v as u32);
    }

    pub fn player_VISIBLE_ITEM_10_ENTRYID(&self) -> Option<i32> {
        self.values.get(&301).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_10_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(302, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_10_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&302) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_11_ENTRYID(&mut self, v: i32) {
        self.header_set(303, v as u32);
    }

    pub fn player_VISIBLE_ITEM_11_ENTRYID(&self) -> Option<i32> {
        self.values.get(&303).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_11_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(304, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_11_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&304) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_12_ENTRYID(&mut self, v: i32) {
        self.header_set(305, v as u32);
    }

    pub fn player_VISIBLE_ITEM_12_ENTRYID(&self) -> Option<i32> {
        self.values.get(&305).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_12_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(306, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_12_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&306) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_13_ENTRYID(&mut self, v: i32) {
        self.header_set(307, v as u32);
    }

    pub fn player_VISIBLE_ITEM_13_ENTRYID(&self) -> Option<i32> {
        self.values.get(&307).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_13_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(308, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_13_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&308) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_14_ENTRYID(&mut self, v: i32) {
        self.header_set(309, v as u32);
    }

    pub fn player_VISIBLE_ITEM_14_ENTRYID(&self) -> Option<i32> {
        self.values.get(&309).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_14_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(310, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_14_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&310) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_15_ENTRYID(&mut self, v: i32) {
        self.header_set(311, v as u32);
    }

    pub fn player_VISIBLE_ITEM_15_ENTRYID(&self) -> Option<i32> {
        self.values.get(&311).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_15_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(312, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_15_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&312) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_16_ENTRYID(&mut self, v: i32) {
        self.header_set(313, v as u32);
    }

    pub fn player_VISIBLE_ITEM_16_ENTRYID(&self) -> Option<i32> {
        self.values.get(&313).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_16_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(314, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_16_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&314) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_17_ENTRYID(&mut self, v: i32) {
        self.header_set(315, v as u32);
    }

    pub fn player_VISIBLE_ITEM_17_ENTRYID(&self) -> Option<i32> {
        self.values.get(&315).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_17_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(316, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_17_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&316) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_18_ENTRYID(&mut self, v: i32) {
        self.header_set(317, v as u32);
    }

    pub fn player_VISIBLE_ITEM_18_ENTRYID(&self) -> Option<i32> {
        self.values.get(&317).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_18_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(318, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_18_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&318) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_19_ENTRYID(&mut self, v: i32) {
        self.header_set(319, v as u32);
    }

    pub fn player_VISIBLE_ITEM_19_ENTRYID(&self) -> Option<i32> {
        self.values.get(&319).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_19_ENCHANTMENT(&mut self, a: u16, b: u16) {
        self.header_set(320, (a as u32) << 16 | b as u32);
    }

    pub fn player_VISIBLE_ITEM_19_ENCHANTMENT(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&320) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_CHOSEN_TITLE(&mut self, v: i32) {
        self.header_set(321, v as u32);
    }

    pub fn player_CHOSEN_TITLE(&self) -> Option<i32> {
        self.values.get(&321).map(|v| *v as i32)
    }

    pub fn set_player_FAKE_INEBRIATION(&mut self, v: i32) {
        self.header_set(322, v as u32);
    }

    pub fn player_FAKE_INEBRIATION(&self) -> Option<i32> {
        self.values.get(&322).map(|v| *v as i32)
    }

    pub fn set_player_INV_SLOT_HEAD(&mut self, v: Guid) {
        self.header_set(324, v.guid() as u32);
        self.header_set(325, (v.guid() >> 32) as u32);
    }

    pub fn player_INV_SLOT_HEAD(&self) -> Option<Guid> {
        let lower = self.values.get(&324);
        let upper = self.values.get(&325);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_PACK_SLOT_1(&mut self, v: Guid) {
        self.header_set(370, v.guid() as u32);
        self.header_set(371, (v.guid() >> 32) as u32);
    }

    pub fn player_PACK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&370);
        let upper = self.values.get(&371);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANK_SLOT_1(&mut self, v: Guid) {
        self.header_set(402, v.guid() as u32);
        self.header_set(403, (v.guid() >> 32) as u32);
    }

    pub fn player_BANK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&402);
        let upper = self.values.get(&403);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_BANKBAG_SLOT_1(&mut self, v: Guid) {
        self.header_set(458, v.guid() as u32);
        self.header_set(459, (v.guid() >> 32) as u32);
    }

    pub fn player_BANKBAG_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&458);
        let upper = self.values.get(&459);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VENDORBUYBACK_SLOT_1(&mut self, v: Guid) {
        self.header_set(472, v.guid() as u32);
        self.header_set(473, (v.guid() >> 32) as u32);
    }

    pub fn player_VENDORBUYBACK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&472);
        let upper = self.values.get(&473);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KEYRING_SLOT_1(&mut self, v: Guid) {
        self.header_set(496, v.guid() as u32);
        self.header_set(497, (v.guid() >> 32) as u32);
    }

    pub fn player_KEYRING_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&496);
        let upper = self.values.get(&497);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_CURRENCYTOKEN_SLOT_1(&mut self, v: Guid) {
        self.header_set(560, v.guid() as u32);
        self.header_set(561, (v.guid() >> 32) as u32);
    }

    pub fn player_CURRENCYTOKEN_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&560);
        let upper = self.values.get(&561);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FARSIGHT(&mut self, v: Guid) {
        self.header_set(624, v.guid() as u32);
        self.header_set(625, (v.guid() >> 32) as u32);
    }

    pub fn player_FARSIGHT(&self) -> Option<Guid> {
        let lower = self.values.get(&624);
        let upper = self.values.get(&625);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KNOWN_TITLES(&mut self, v: Guid) {
        self.header_set(626, v.guid() as u32);
        self.header_set(627, (v.guid() >> 32) as u32);
    }

    pub fn player_KNOWN_TITLES(&self) -> Option<Guid> {
        let lower = self.values.get(&626);
        let upper = self.values.get(&627);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KNOWN_TITLES1(&mut self, v: Guid) {
        self.header_set(628, v.guid() as u32);
        self.header_set(629, (v.guid() >> 32) as u32);
    }

    pub fn player_KNOWN_TITLES1(&self) -> Option<Guid> {
        let lower = self.values.get(&628);
        let upper = self.values.get(&629);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KNOWN_TITLES2(&mut self, v: Guid) {
        self.header_set(630, v.guid() as u32);
        self.header_set(631, (v.guid() >> 32) as u32);
    }

    pub fn player_KNOWN_TITLES2(&self) -> Option<Guid> {
        let lower = self.values.get(&630);
        let upper = self.values.get(&631);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_KNOWN_CURRENCIES(&mut self, v: Guid) {
        self.header_set(632, v.guid() as u32);
        self.header_set(633, (v.guid() >> 32) as u32);
    }

    pub fn player_KNOWN_CURRENCIES(&self) -> Option<Guid> {
        let lower = self.values.get(&632);
        let upper = self.values.get(&633);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_XP(&mut self, v: i32) {
        self.header_set(634, v as u32);
    }

    pub fn player_XP(&self) -> Option<i32> {
        self.values.get(&634).map(|v| *v as i32)
    }

    pub fn set_player_NEXT_LEVEL_XP(&mut self, v: i32) {
        self.header_set(635, v as u32);
    }

    pub fn player_NEXT_LEVEL_XP(&self) -> Option<i32> {
        self.values.get(&635).map(|v| *v as i32)
    }

    pub fn set_player_SKILL_INFO_1_1(&mut self, v: i32) {
        self.header_set(636, v as u32);
    }

    pub fn player_SKILL_INFO_1_1(&self) -> Option<i32> {
        self.values.get(&636).map(|v| *v as i32)
    }

    pub fn set_player_SKILL_INFO_1_2(&mut self, a: u16, b: u16) {
        self.header_set(637, (a as u32) << 16 | b as u32);
    }

    pub fn player_SKILL_INFO_1_2(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&637) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_SKILL_INFO_1_3(&mut self, v: i32) {
        self.header_set(638, v as u32);
    }

    pub fn player_SKILL_INFO_1_3(&self) -> Option<i32> {
        self.values.get(&638).map(|v| *v as i32)
    }

    pub fn set_player_CHARACTER_POINTS1(&mut self, v: i32) {
        self.header_set(1020, v as u32);
    }

    pub fn player_CHARACTER_POINTS1(&self) -> Option<i32> {
        self.values.get(&1020).map(|v| *v as i32)
    }

    pub fn set_player_CHARACTER_POINTS2(&mut self, v: i32) {
        self.header_set(1021, v as u32);
    }

    pub fn player_CHARACTER_POINTS2(&self) -> Option<i32> {
        self.values.get(&1021).map(|v| *v as i32)
    }

    pub fn set_player_TRACK_CREATURES(&mut self, v: i32) {
        self.header_set(1022, v as u32);
    }

    pub fn player_TRACK_CREATURES(&self) -> Option<i32> {
        self.values.get(&1022).map(|v| *v as i32)
    }

    pub fn set_player_TRACK_RESOURCES(&mut self, v: i32) {
        self.header_set(1023, v as u32);
    }

    pub fn player_TRACK_RESOURCES(&self) -> Option<i32> {
        self.values.get(&1023).map(|v| *v as i32)
    }

    pub fn set_player_BLOCK_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1024, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_BLOCK_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1024).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_DODGE_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1025, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_DODGE_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1025).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_PARRY_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1026, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_PARRY_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1026).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_EXPERTISE(&mut self, v: i32) {
        self.header_set(1027, v as u32);
    }

    pub fn player_EXPERTISE(&self) -> Option<i32> {
        self.values.get(&1027).map(|v| *v as i32)
    }

    pub fn set_player_OFFHAND_EXPERTISE(&mut self, v: i32) {
        self.header_set(1028, v as u32);
    }

    pub fn player_OFFHAND_EXPERTISE(&self) -> Option<i32> {
        self.values.get(&1028).map(|v| *v as i32)
    }

    pub fn set_player_CRIT_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1029, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_CRIT_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1029).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_RANGED_CRIT_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1030, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_RANGED_CRIT_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1030).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_OFFHAND_CRIT_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1031, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_OFFHAND_CRIT_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1031).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_SPELL_CRIT_PERCENTAGE1(&mut self, v: f32) {
        self.header_set(1032, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_SPELL_CRIT_PERCENTAGE1(&self) -> Option<f32> {
        self.values.get(&1032).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_SHIELD_BLOCK(&mut self, v: i32) {
        self.header_set(1039, v as u32);
    }

    pub fn player_SHIELD_BLOCK(&self) -> Option<i32> {
        self.values.get(&1039).map(|v| *v as i32)
    }

    pub fn set_player_SHIELD_BLOCK_CRIT_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1040, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_SHIELD_BLOCK_CRIT_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1040).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_EXPLORED_ZONES_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1041, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_EXPLORED_ZONES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1041) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_REST_STATE_EXPERIENCE(&mut self, v: i32) {
        self.header_set(1169, v as u32);
    }

    pub fn player_REST_STATE_EXPERIENCE(&self) -> Option<i32> {
        self.values.get(&1169).map(|v| *v as i32)
    }

    pub fn set_player_COINAGE(&mut self, v: i32) {
        self.header_set(1170, v as u32);
    }

    pub fn player_COINAGE(&self) -> Option<i32> {
        self.values.get(&1170).map(|v| *v as i32)
    }

    pub fn set_player_MOD_DAMAGE_DONE_POS(&mut self, v: i32) {
        self.header_set(1171, v as u32);
    }

    pub fn player_MOD_DAMAGE_DONE_POS(&self) -> Option<i32> {
        self.values.get(&1171).map(|v| *v as i32)
    }

    pub fn set_player_MOD_DAMAGE_DONE_NEG(&mut self, v: i32) {
        self.header_set(1178, v as u32);
    }

    pub fn player_MOD_DAMAGE_DONE_NEG(&self) -> Option<i32> {
        self.values.get(&1178).map(|v| *v as i32)
    }

    pub fn set_player_MOD_DAMAGE_DONE_PCT(&mut self, v: i32) {
        self.header_set(1185, v as u32);
    }

    pub fn player_MOD_DAMAGE_DONE_PCT(&self) -> Option<i32> {
        self.values.get(&1185).map(|v| *v as i32)
    }

    pub fn set_player_MOD_HEALING_DONE_POS(&mut self, v: i32) {
        self.header_set(1192, v as u32);
    }

    pub fn player_MOD_HEALING_DONE_POS(&self) -> Option<i32> {
        self.values.get(&1192).map(|v| *v as i32)
    }

    pub fn set_player_MOD_HEALING_PCT(&mut self, v: f32) {
        self.header_set(1193, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_MOD_HEALING_PCT(&self) -> Option<f32> {
        self.values.get(&1193).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_MOD_HEALING_DONE_PCT(&mut self, v: f32) {
        self.header_set(1194, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_MOD_HEALING_DONE_PCT(&self) -> Option<f32> {
        self.values.get(&1194).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_MOD_TARGET_RESISTANCE(&mut self, v: i32) {
        self.header_set(1195, v as u32);
    }

    pub fn player_MOD_TARGET_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&1195).map(|v| *v as i32)
    }

    pub fn set_player_MOD_TARGET_PHYSICAL_RESISTANCE(&mut self, v: i32) {
        self.header_set(1196, v as u32);
    }

    pub fn player_MOD_TARGET_PHYSICAL_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&1196).map(|v| *v as i32)
    }

    pub fn set_player_BYTES(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1197, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_BYTES(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1197) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_AMMO_ID(&mut self, v: i32) {
        self.header_set(1198, v as u32);
    }

    pub fn player_AMMO_ID(&self) -> Option<i32> {
        self.values.get(&1198).map(|v| *v as i32)
    }

    pub fn set_player_SELF_RES_SPELL(&mut self, v: i32) {
        self.header_set(1199, v as u32);
    }

    pub fn player_SELF_RES_SPELL(&self) -> Option<i32> {
        self.values.get(&1199).map(|v| *v as i32)
    }

    pub fn set_player_PVP_MEDALS(&mut self, v: i32) {
        self.header_set(1200, v as u32);
    }

    pub fn player_PVP_MEDALS(&self) -> Option<i32> {
        self.values.get(&1200).map(|v| *v as i32)
    }

    pub fn set_player_BUYBACK_PRICE_1(&mut self, v: i32) {
        self.header_set(1201, v as u32);
    }

    pub fn player_BUYBACK_PRICE_1(&self) -> Option<i32> {
        self.values.get(&1201).map(|v| *v as i32)
    }

    pub fn set_player_BUYBACK_TIMESTAMP_1(&mut self, v: i32) {
        self.header_set(1213, v as u32);
    }

    pub fn player_BUYBACK_TIMESTAMP_1(&self) -> Option<i32> {
        self.values.get(&1213).map(|v| *v as i32)
    }

    pub fn set_player_KILLS(&mut self, a: u16, b: u16) {
        self.header_set(1225, (a as u32) << 16 | b as u32);
    }

    pub fn player_KILLS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1225) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_TODAY_CONTRIBUTION(&mut self, v: i32) {
        self.header_set(1226, v as u32);
    }

    pub fn player_TODAY_CONTRIBUTION(&self) -> Option<i32> {
        self.values.get(&1226).map(|v| *v as i32)
    }

    pub fn set_player_YESTERDAY_CONTRIBUTION(&mut self, v: i32) {
        self.header_set(1227, v as u32);
    }

    pub fn player_YESTERDAY_CONTRIBUTION(&self) -> Option<i32> {
        self.values.get(&1227).map(|v| *v as i32)
    }

    pub fn set_player_LIFETIME_HONORBALE_KILLS(&mut self, v: i32) {
        self.header_set(1228, v as u32);
    }

    pub fn player_LIFETIME_HONORBALE_KILLS(&self) -> Option<i32> {
        self.values.get(&1228).map(|v| *v as i32)
    }

    pub fn set_player_BYTES2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1229, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_BYTES2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1229) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_WATCHED_FACTION_INDEX(&mut self, v: i32) {
        self.header_set(1230, v as u32);
    }

    pub fn player_WATCHED_FACTION_INDEX(&self) -> Option<i32> {
        self.values.get(&1230).map(|v| *v as i32)
    }

    pub fn set_player_COMBAT_RATING_1(&mut self, v: i32) {
        self.header_set(1231, v as u32);
    }

    pub fn player_COMBAT_RATING_1(&self) -> Option<i32> {
        self.values.get(&1231).map(|v| *v as i32)
    }

    pub fn set_player_ARENA_TEAM_INFO_1_1(&mut self, v: i32) {
        self.header_set(1256, v as u32);
    }

    pub fn player_ARENA_TEAM_INFO_1_1(&self) -> Option<i32> {
        self.values.get(&1256).map(|v| *v as i32)
    }

    pub fn set_player_HONOR_CURRENCY(&mut self, v: i32) {
        self.header_set(1277, v as u32);
    }

    pub fn player_HONOR_CURRENCY(&self) -> Option<i32> {
        self.values.get(&1277).map(|v| *v as i32)
    }

    pub fn set_player_ARENA_CURRENCY(&mut self, v: i32) {
        self.header_set(1278, v as u32);
    }

    pub fn player_ARENA_CURRENCY(&self) -> Option<i32> {
        self.values.get(&1278).map(|v| *v as i32)
    }

    pub fn set_player_MAX_LEVEL(&mut self, v: i32) {
        self.header_set(1279, v as u32);
    }

    pub fn player_MAX_LEVEL(&self) -> Option<i32> {
        self.values.get(&1279).map(|v| *v as i32)
    }

    pub fn set_player_DAILY_QUESTS_1(&mut self, v: i32) {
        self.header_set(1280, v as u32);
    }

    pub fn player_DAILY_QUESTS_1(&self) -> Option<i32> {
        self.values.get(&1280).map(|v| *v as i32)
    }

    pub fn set_player_RUNE_REGEN_1(&mut self, v: f32) {
        self.header_set(1305, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_RUNE_REGEN_1(&self) -> Option<f32> {
        self.values.get(&1305).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_NO_REAGENT_COST_1(&mut self, v: i32) {
        self.header_set(1309, v as u32);
    }

    pub fn player_NO_REAGENT_COST_1(&self) -> Option<i32> {
        self.values.get(&1309).map(|v| *v as i32)
    }

    pub fn set_player_GLYPH_SLOTS_1(&mut self, v: i32) {
        self.header_set(1312, v as u32);
    }

    pub fn player_GLYPH_SLOTS_1(&self) -> Option<i32> {
        self.values.get(&1312).map(|v| *v as i32)
    }

    pub fn set_player_GLYPHS_1(&mut self, v: i32) {
        self.header_set(1318, v as u32);
    }

    pub fn player_GLYPHS_1(&self) -> Option<i32> {
        self.values.get(&1318).map(|v| *v as i32)
    }

    pub fn set_player_GLYPHS_ENABLED(&mut self, v: i32) {
        self.header_set(1324, v as u32);
    }

    pub fn player_GLYPHS_ENABLED(&self) -> Option<i32> {
        self.values.get(&1324).map(|v| *v as i32)
    }

    pub fn set_player_PET_SPELL_POWER(&mut self, v: i32) {
        self.header_set(1325, v as u32);
    }

    pub fn player_PET_SPELL_POWER(&self) -> Option<i32> {
        self.values.get(&1325).map(|v| *v as i32)
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdateGameObject {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, v as u32);
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, v as u32);
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_gameobject_DISPLAYID(&mut self, v: i32) {
        self.header_set(8, v as u32);
    }

    pub fn gameobject_DISPLAYID(&self) -> Option<i32> {
        self.values.get(&8).map(|v| *v as i32)
    }

    pub fn set_gameobject_FLAGS(&mut self, v: i32) {
        self.header_set(9, v as u32);
    }

    pub fn gameobject_FLAGS(&self) -> Option<i32> {
        self.values.get(&9).map(|v| *v as i32)
    }

    pub fn set_gameobject_PARENTROTATION(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_PARENTROTATION(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_DYNAMIC(&mut self, a: u16, b: u16) {
        self.header_set(14, (a as u32) << 16 | b as u32);
    }

    pub fn gameobject_DYNAMIC(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&14) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_gameobject_FACTION(&mut self, v: i32) {
        self.header_set(15, v as u32);
    }

    pub fn gameobject_FACTION(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_gameobject_LEVEL(&mut self, v: i32) {
        self.header_set(16, v as u32);
    }

    pub fn gameobject_LEVEL(&self) -> Option<i32> {
        self.values.get(&16).map(|v| *v as i32)
    }

    pub fn set_gameobject_BYTES_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(17, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn gameobject_BYTES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&17) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

}

impl UpdateDynamicObject {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, v as u32);
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, v as u32);
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_dynamicobject_CASTER(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn dynamicobject_CASTER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_dynamicobject_BYTES(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(8, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn dynamicobject_BYTES(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&8) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_dynamicobject_SPELLID(&mut self, v: i32) {
        self.header_set(9, v as u32);
    }

    pub fn dynamicobject_SPELLID(&self) -> Option<i32> {
        self.values.get(&9).map(|v| *v as i32)
    }

    pub fn set_dynamicobject_RADIUS(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_RADIUS(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_CASTTIME(&mut self, v: i32) {
        self.header_set(11, v as u32);
    }

    pub fn dynamicobject_CASTTIME(&self) -> Option<i32> {
        self.values.get(&11).map(|v| *v as i32)
    }

}

impl UpdateCorpse {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_GUID(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2, v as u32);
    }

    pub fn object_TYPE(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, v as u32);
    }

    pub fn object_ENTRY(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_SCALE_X(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_corpse_OWNER(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn corpse_OWNER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_corpse_PARTY(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn corpse_PARTY(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_corpse_DISPLAY_ID(&mut self, v: i32) {
        self.header_set(10, v as u32);
    }

    pub fn corpse_DISPLAY_ID(&self) -> Option<i32> {
        self.values.get(&10).map(|v| *v as i32)
    }

    pub fn set_corpse_ITEM(&mut self, v: i32) {
        self.header_set(11, v as u32);
    }

    pub fn corpse_ITEM(&self) -> Option<i32> {
        self.values.get(&11).map(|v| *v as i32)
    }

    pub fn set_corpse_BYTES_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(30, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_BYTES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&30) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_BYTES_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(31, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&31) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_GUILD(&mut self, v: i32) {
        self.header_set(32, v as u32);
    }

    pub fn corpse_GUILD(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_corpse_FLAGS(&mut self, v: i32) {
        self.header_set(33, v as u32);
    }

    pub fn corpse_FLAGS(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_corpse_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(34, v as u32);
    }

    pub fn corpse_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

}

