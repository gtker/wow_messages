use crate::Guid;
use std::convert::TryInto;
use super::indices::*;
use crate::vanilla::{Race};
use crate::vanilla::{Class};
use crate::vanilla::{Gender};
use crate::vanilla::{Power};
use crate::vanilla::{UpdateContainer, UpdateContainerBuilder, UpdateCorpse, UpdateCorpseBuilder, UpdateDynamicObject, UpdateDynamicObjectBuilder, UpdateGameObject, UpdateGameObjectBuilder, UpdateItem, UpdateItemBuilder, UpdatePlayer, UpdatePlayerBuilder, UpdateUnit, UpdateUnitBuilder};

impl UpdateItemBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
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
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_DURATION(mut self, v: i32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_ENCHANTMENT(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(44, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_ITEM_TEXT_ID(mut self, v: i32) -> Self {
        self.header_set(45, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateContainerBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
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
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_DURATION(mut self, v: i32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_ENCHANTMENT(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(44, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_ITEM_TEXT_ID(mut self, v: i32) -> Self {
        self.header_set(45, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_container_NUM_SLOTS(mut self, v: i32) -> Self {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_container_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(50, v.guid() as u32);
        self.header_set(51, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateUnitBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
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

    pub fn set_unit_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_TARGET(mut self, v: Guid) -> Self {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_PERSUADED(mut self, v: Guid) -> Self {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_HEALTH(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AURA(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AURAFLAGS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(95, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURALEVELS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(101, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURAAPPLICATIONS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(113, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(132, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(133, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(134, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(135, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(136, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(137, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(138, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(139, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(140, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(141, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(142, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(143, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(144, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(148, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_TRAINING_POINTS(mut self, a: u16, b: u16) -> Self {
        self.header_set(149, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NORMAL_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_HOLY_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIRE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NATURE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FROST_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(159, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_SHADOW_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ARCANE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_2(mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) -> Self {
        self.header_set(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
        self
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(166, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(169, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(170, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdatePlayerBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
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

    pub fn set_unit_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_TARGET(mut self, v: Guid) -> Self {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_PERSUADED(mut self, v: Guid) -> Self {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_HEALTH(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AURA(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AURAFLAGS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(95, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURALEVELS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(101, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURAAPPLICATIONS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(113, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(132, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(133, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(134, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(135, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(136, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(137, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(138, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(139, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(140, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(141, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(142, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(143, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(144, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(148, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_TRAINING_POINTS(mut self, a: u16, b: u16) -> Self {
        self.header_set(149, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NORMAL_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_HOLY_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIRE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_NATURE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FROST_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(159, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_SHADOW_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ARCANE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_BYTES_2(mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) -> Self {
        self.header_set(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
        self
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(166, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(169, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(170, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_DUEL_ARBITER(mut self, v: Guid) -> Self {
        self.header_set(188, v.guid() as u32);
        self.header_set(189, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FLAGS(mut self, v: i32) -> Self {
        self.header_set(190, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_GUILDID(mut self, v: i32) -> Self {
        self.header_set(191, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_GUILDRANK(mut self, v: i32) -> Self {
        self.header_set(192, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FEATURES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(193, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(194, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_BYTES_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(195, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_DUEL_TEAM(mut self, v: i32) -> Self {
        self.header_set(196, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_GUILD_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(197, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_1_1(mut self, v: i32) -> Self {
        self.header_set(198, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_1_2(mut self, v: i32) -> Self {
        self.header_set(199, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_2_1(mut self, v: i32) -> Self {
        self.header_set(201, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_2_2(mut self, v: i32) -> Self {
        self.header_set(202, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_3_1(mut self, v: i32) -> Self {
        self.header_set(204, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_3_2(mut self, v: i32) -> Self {
        self.header_set(205, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_4_1(mut self, v: i32) -> Self {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_4_2(mut self, v: i32) -> Self {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_5_1(mut self, v: i32) -> Self {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_5_2(mut self, v: i32) -> Self {
        self.header_set(211, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_6_1(mut self, v: i32) -> Self {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_6_2(mut self, v: i32) -> Self {
        self.header_set(214, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_7_1(mut self, v: i32) -> Self {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_7_2(mut self, v: i32) -> Self {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_8_1(mut self, v: i32) -> Self {
        self.header_set(219, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_8_2(mut self, v: i32) -> Self {
        self.header_set(220, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_9_1(mut self, v: i32) -> Self {
        self.header_set(222, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_9_2(mut self, v: i32) -> Self {
        self.header_set(223, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_10_1(mut self, v: i32) -> Self {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_10_2(mut self, v: i32) -> Self {
        self.header_set(226, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_11_1(mut self, v: i32) -> Self {
        self.header_set(228, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_11_2(mut self, v: i32) -> Self {
        self.header_set(229, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_12_1(mut self, v: i32) -> Self {
        self.header_set(231, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_12_2(mut self, v: i32) -> Self {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_13_1(mut self, v: i32) -> Self {
        self.header_set(234, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_13_2(mut self, v: i32) -> Self {
        self.header_set(235, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_14_1(mut self, v: i32) -> Self {
        self.header_set(237, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_14_2(mut self, v: i32) -> Self {
        self.header_set(238, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_15_1(mut self, v: i32) -> Self {
        self.header_set(240, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_15_2(mut self, v: i32) -> Self {
        self.header_set(241, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_16_1(mut self, v: i32) -> Self {
        self.header_set(243, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_16_2(mut self, v: i32) -> Self {
        self.header_set(244, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_17_1(mut self, v: i32) -> Self {
        self.header_set(246, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_17_2(mut self, v: i32) -> Self {
        self.header_set(247, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_18_1(mut self, v: i32) -> Self {
        self.header_set(249, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_18_2(mut self, v: i32) -> Self {
        self.header_set(250, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_19_1(mut self, v: i32) -> Self {
        self.header_set(252, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_19_2(mut self, v: i32) -> Self {
        self.header_set(253, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_20_1(mut self, v: i32) -> Self {
        self.header_set(255, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_QUEST_LOG_20_2(mut self, v: i32) -> Self {
        self.header_set(256, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(258, v.guid() as u32);
        self.header_set(259, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_0(mut self, v: i32) -> Self {
        self.header_set(260, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(268, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(270, v.guid() as u32);
        self.header_set(271, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_0(mut self, v: i32) -> Self {
        self.header_set(272, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(280, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(282, v.guid() as u32);
        self.header_set(283, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_0(mut self, v: i32) -> Self {
        self.header_set(284, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(292, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(294, v.guid() as u32);
        self.header_set(295, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_0(mut self, v: i32) -> Self {
        self.header_set(296, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(304, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(306, v.guid() as u32);
        self.header_set(307, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_0(mut self, v: i32) -> Self {
        self.header_set(308, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(316, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(318, v.guid() as u32);
        self.header_set(319, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_0(mut self, v: i32) -> Self {
        self.header_set(320, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(328, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(330, v.guid() as u32);
        self.header_set(331, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_0(mut self, v: i32) -> Self {
        self.header_set(332, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(340, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(342, v.guid() as u32);
        self.header_set(343, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_0(mut self, v: i32) -> Self {
        self.header_set(344, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(352, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(354, v.guid() as u32);
        self.header_set(355, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_0(mut self, v: i32) -> Self {
        self.header_set(356, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(364, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(366, v.guid() as u32);
        self.header_set(367, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_0(mut self, v: i32) -> Self {
        self.header_set(368, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(376, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(378, v.guid() as u32);
        self.header_set(379, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_0(mut self, v: i32) -> Self {
        self.header_set(380, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(388, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(390, v.guid() as u32);
        self.header_set(391, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_0(mut self, v: i32) -> Self {
        self.header_set(392, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(400, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(402, v.guid() as u32);
        self.header_set(403, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_0(mut self, v: i32) -> Self {
        self.header_set(404, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(412, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(414, v.guid() as u32);
        self.header_set(415, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_0(mut self, v: i32) -> Self {
        self.header_set(416, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(424, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(426, v.guid() as u32);
        self.header_set(427, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_0(mut self, v: i32) -> Self {
        self.header_set(428, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(436, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(438, v.guid() as u32);
        self.header_set(439, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_0(mut self, v: i32) -> Self {
        self.header_set(440, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(448, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(450, v.guid() as u32);
        self.header_set(451, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_0(mut self, v: i32) -> Self {
        self.header_set(452, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(460, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(462, v.guid() as u32);
        self.header_set(463, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_0(mut self, v: i32) -> Self {
        self.header_set(464, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(472, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(474, v.guid() as u32);
        self.header_set(475, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_0(mut self, v: i32) -> Self {
        self.header_set(476, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(484, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_FIELD_INV_SLOT_HEAD(mut self, v: Guid) -> Self {
        self.header_set(486, v.guid() as u32);
        self.header_set(487, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_NECK(mut self, v: Guid) -> Self {
        self.header_set(488, v.guid() as u32);
        self.header_set(489, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_SHOULDERS(mut self, v: Guid) -> Self {
        self.header_set(490, v.guid() as u32);
        self.header_set(491, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_BODY(mut self, v: Guid) -> Self {
        self.header_set(492, v.guid() as u32);
        self.header_set(493, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_CHEST(mut self, v: Guid) -> Self {
        self.header_set(494, v.guid() as u32);
        self.header_set(495, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_WAIST(mut self, v: Guid) -> Self {
        self.header_set(496, v.guid() as u32);
        self.header_set(497, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_LEGS(mut self, v: Guid) -> Self {
        self.header_set(498, v.guid() as u32);
        self.header_set(499, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_FEET(mut self, v: Guid) -> Self {
        self.header_set(500, v.guid() as u32);
        self.header_set(501, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_WRISTS(mut self, v: Guid) -> Self {
        self.header_set(502, v.guid() as u32);
        self.header_set(503, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_HANDS(mut self, v: Guid) -> Self {
        self.header_set(504, v.guid() as u32);
        self.header_set(505, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_FINGER1(mut self, v: Guid) -> Self {
        self.header_set(506, v.guid() as u32);
        self.header_set(507, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_FINGER2(mut self, v: Guid) -> Self {
        self.header_set(508, v.guid() as u32);
        self.header_set(509, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_TRINKET1(mut self, v: Guid) -> Self {
        self.header_set(510, v.guid() as u32);
        self.header_set(511, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_TRINKET2(mut self, v: Guid) -> Self {
        self.header_set(512, v.guid() as u32);
        self.header_set(513, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_BACK(mut self, v: Guid) -> Self {
        self.header_set(514, v.guid() as u32);
        self.header_set(515, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_MAIN_HAND(mut self, v: Guid) -> Self {
        self.header_set(516, v.guid() as u32);
        self.header_set(517, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_OFF_HAND(mut self, v: Guid) -> Self {
        self.header_set(518, v.guid() as u32);
        self.header_set(519, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_RANGED(mut self, v: Guid) -> Self {
        self.header_set(520, v.guid() as u32);
        self.header_set(521, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_TABARD(mut self, v: Guid) -> Self {
        self.header_set(522, v.guid() as u32);
        self.header_set(523, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_BAG1(mut self, v: Guid) -> Self {
        self.header_set(524, v.guid() as u32);
        self.header_set(525, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_BAG2(mut self, v: Guid) -> Self {
        self.header_set(526, v.guid() as u32);
        self.header_set(527, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_BAG3(mut self, v: Guid) -> Self {
        self.header_set(528, v.guid() as u32);
        self.header_set(529, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_BAG4(mut self, v: Guid) -> Self {
        self.header_set(530, v.guid() as u32);
        self.header_set(531, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(532, v.guid() as u32);
        self.header_set(533, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_2(mut self, v: Guid) -> Self {
        self.header_set(534, v.guid() as u32);
        self.header_set(535, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_3(mut self, v: Guid) -> Self {
        self.header_set(536, v.guid() as u32);
        self.header_set(537, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_4(mut self, v: Guid) -> Self {
        self.header_set(538, v.guid() as u32);
        self.header_set(539, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_5(mut self, v: Guid) -> Self {
        self.header_set(540, v.guid() as u32);
        self.header_set(541, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_6(mut self, v: Guid) -> Self {
        self.header_set(542, v.guid() as u32);
        self.header_set(543, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_7(mut self, v: Guid) -> Self {
        self.header_set(544, v.guid() as u32);
        self.header_set(545, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_8(mut self, v: Guid) -> Self {
        self.header_set(546, v.guid() as u32);
        self.header_set(547, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_9(mut self, v: Guid) -> Self {
        self.header_set(548, v.guid() as u32);
        self.header_set(549, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_10(mut self, v: Guid) -> Self {
        self.header_set(550, v.guid() as u32);
        self.header_set(551, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_11(mut self, v: Guid) -> Self {
        self.header_set(552, v.guid() as u32);
        self.header_set(553, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_12(mut self, v: Guid) -> Self {
        self.header_set(554, v.guid() as u32);
        self.header_set(555, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_13(mut self, v: Guid) -> Self {
        self.header_set(556, v.guid() as u32);
        self.header_set(557, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_14(mut self, v: Guid) -> Self {
        self.header_set(558, v.guid() as u32);
        self.header_set(559, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_15(mut self, v: Guid) -> Self {
        self.header_set(560, v.guid() as u32);
        self.header_set(561, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_16(mut self, v: Guid) -> Self {
        self.header_set(562, v.guid() as u32);
        self.header_set(563, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(564, v.guid() as u32);
        self.header_set(565, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_2(mut self, v: Guid) -> Self {
        self.header_set(566, v.guid() as u32);
        self.header_set(567, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_3(mut self, v: Guid) -> Self {
        self.header_set(568, v.guid() as u32);
        self.header_set(569, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_4(mut self, v: Guid) -> Self {
        self.header_set(570, v.guid() as u32);
        self.header_set(571, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_5(mut self, v: Guid) -> Self {
        self.header_set(572, v.guid() as u32);
        self.header_set(573, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_6(mut self, v: Guid) -> Self {
        self.header_set(574, v.guid() as u32);
        self.header_set(575, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_7(mut self, v: Guid) -> Self {
        self.header_set(576, v.guid() as u32);
        self.header_set(577, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_8(mut self, v: Guid) -> Self {
        self.header_set(578, v.guid() as u32);
        self.header_set(579, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_9(mut self, v: Guid) -> Self {
        self.header_set(580, v.guid() as u32);
        self.header_set(581, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_10(mut self, v: Guid) -> Self {
        self.header_set(582, v.guid() as u32);
        self.header_set(583, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_11(mut self, v: Guid) -> Self {
        self.header_set(584, v.guid() as u32);
        self.header_set(585, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_12(mut self, v: Guid) -> Self {
        self.header_set(586, v.guid() as u32);
        self.header_set(587, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_13(mut self, v: Guid) -> Self {
        self.header_set(588, v.guid() as u32);
        self.header_set(589, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_14(mut self, v: Guid) -> Self {
        self.header_set(590, v.guid() as u32);
        self.header_set(591, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_15(mut self, v: Guid) -> Self {
        self.header_set(592, v.guid() as u32);
        self.header_set(593, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_16(mut self, v: Guid) -> Self {
        self.header_set(594, v.guid() as u32);
        self.header_set(595, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_17(mut self, v: Guid) -> Self {
        self.header_set(596, v.guid() as u32);
        self.header_set(597, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_18(mut self, v: Guid) -> Self {
        self.header_set(598, v.guid() as u32);
        self.header_set(599, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_19(mut self, v: Guid) -> Self {
        self.header_set(600, v.guid() as u32);
        self.header_set(601, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_20(mut self, v: Guid) -> Self {
        self.header_set(602, v.guid() as u32);
        self.header_set(603, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_21(mut self, v: Guid) -> Self {
        self.header_set(604, v.guid() as u32);
        self.header_set(605, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_22(mut self, v: Guid) -> Self {
        self.header_set(606, v.guid() as u32);
        self.header_set(607, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_23(mut self, v: Guid) -> Self {
        self.header_set(608, v.guid() as u32);
        self.header_set(609, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_24(mut self, v: Guid) -> Self {
        self.header_set(610, v.guid() as u32);
        self.header_set(611, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(612, v.guid() as u32);
        self.header_set(613, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_2(mut self, v: Guid) -> Self {
        self.header_set(614, v.guid() as u32);
        self.header_set(615, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_3(mut self, v: Guid) -> Self {
        self.header_set(616, v.guid() as u32);
        self.header_set(617, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_4(mut self, v: Guid) -> Self {
        self.header_set(618, v.guid() as u32);
        self.header_set(619, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_5(mut self, v: Guid) -> Self {
        self.header_set(620, v.guid() as u32);
        self.header_set(621, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_6(mut self, v: Guid) -> Self {
        self.header_set(622, v.guid() as u32);
        self.header_set(623, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(624, v.guid() as u32);
        self.header_set(625, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_2(mut self, v: Guid) -> Self {
        self.header_set(626, v.guid() as u32);
        self.header_set(627, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_3(mut self, v: Guid) -> Self {
        self.header_set(628, v.guid() as u32);
        self.header_set(629, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_4(mut self, v: Guid) -> Self {
        self.header_set(630, v.guid() as u32);
        self.header_set(631, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_5(mut self, v: Guid) -> Self {
        self.header_set(632, v.guid() as u32);
        self.header_set(633, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_6(mut self, v: Guid) -> Self {
        self.header_set(634, v.guid() as u32);
        self.header_set(635, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_7(mut self, v: Guid) -> Self {
        self.header_set(636, v.guid() as u32);
        self.header_set(637, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_8(mut self, v: Guid) -> Self {
        self.header_set(638, v.guid() as u32);
        self.header_set(639, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_9(mut self, v: Guid) -> Self {
        self.header_set(640, v.guid() as u32);
        self.header_set(641, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_10(mut self, v: Guid) -> Self {
        self.header_set(642, v.guid() as u32);
        self.header_set(643, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_11(mut self, v: Guid) -> Self {
        self.header_set(644, v.guid() as u32);
        self.header_set(645, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_12(mut self, v: Guid) -> Self {
        self.header_set(646, v.guid() as u32);
        self.header_set(647, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(648, v.guid() as u32);
        self.header_set(649, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_2(mut self, v: Guid) -> Self {
        self.header_set(650, v.guid() as u32);
        self.header_set(651, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_3(mut self, v: Guid) -> Self {
        self.header_set(652, v.guid() as u32);
        self.header_set(653, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_4(mut self, v: Guid) -> Self {
        self.header_set(654, v.guid() as u32);
        self.header_set(655, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_5(mut self, v: Guid) -> Self {
        self.header_set(656, v.guid() as u32);
        self.header_set(657, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_6(mut self, v: Guid) -> Self {
        self.header_set(658, v.guid() as u32);
        self.header_set(659, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_7(mut self, v: Guid) -> Self {
        self.header_set(660, v.guid() as u32);
        self.header_set(661, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_8(mut self, v: Guid) -> Self {
        self.header_set(662, v.guid() as u32);
        self.header_set(663, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_9(mut self, v: Guid) -> Self {
        self.header_set(664, v.guid() as u32);
        self.header_set(665, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_10(mut self, v: Guid) -> Self {
        self.header_set(666, v.guid() as u32);
        self.header_set(667, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_11(mut self, v: Guid) -> Self {
        self.header_set(668, v.guid() as u32);
        self.header_set(669, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_12(mut self, v: Guid) -> Self {
        self.header_set(670, v.guid() as u32);
        self.header_set(671, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_13(mut self, v: Guid) -> Self {
        self.header_set(672, v.guid() as u32);
        self.header_set(673, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_14(mut self, v: Guid) -> Self {
        self.header_set(674, v.guid() as u32);
        self.header_set(675, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_15(mut self, v: Guid) -> Self {
        self.header_set(676, v.guid() as u32);
        self.header_set(677, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_16(mut self, v: Guid) -> Self {
        self.header_set(678, v.guid() as u32);
        self.header_set(679, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_17(mut self, v: Guid) -> Self {
        self.header_set(680, v.guid() as u32);
        self.header_set(681, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_18(mut self, v: Guid) -> Self {
        self.header_set(682, v.guid() as u32);
        self.header_set(683, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_19(mut self, v: Guid) -> Self {
        self.header_set(684, v.guid() as u32);
        self.header_set(685, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_20(mut self, v: Guid) -> Self {
        self.header_set(686, v.guid() as u32);
        self.header_set(687, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_21(mut self, v: Guid) -> Self {
        self.header_set(688, v.guid() as u32);
        self.header_set(689, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_22(mut self, v: Guid) -> Self {
        self.header_set(690, v.guid() as u32);
        self.header_set(691, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_23(mut self, v: Guid) -> Self {
        self.header_set(692, v.guid() as u32);
        self.header_set(693, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_24(mut self, v: Guid) -> Self {
        self.header_set(694, v.guid() as u32);
        self.header_set(695, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_25(mut self, v: Guid) -> Self {
        self.header_set(696, v.guid() as u32);
        self.header_set(697, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_26(mut self, v: Guid) -> Self {
        self.header_set(698, v.guid() as u32);
        self.header_set(699, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_27(mut self, v: Guid) -> Self {
        self.header_set(700, v.guid() as u32);
        self.header_set(701, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_28(mut self, v: Guid) -> Self {
        self.header_set(702, v.guid() as u32);
        self.header_set(703, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_29(mut self, v: Guid) -> Self {
        self.header_set(704, v.guid() as u32);
        self.header_set(705, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_30(mut self, v: Guid) -> Self {
        self.header_set(706, v.guid() as u32);
        self.header_set(707, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_31(mut self, v: Guid) -> Self {
        self.header_set(708, v.guid() as u32);
        self.header_set(709, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_32(mut self, v: Guid) -> Self {
        self.header_set(710, v.guid() as u32);
        self.header_set(711, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FARSIGHT(mut self, v: Guid) -> Self {
        self.header_set(712, v.guid() as u32);
        self.header_set(713, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_COMBO_TARGET(mut self, v: Guid) -> Self {
        self.header_set(714, v.guid() as u32);
        self.header_set(715, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_XP(mut self, v: i32) -> Self {
        self.header_set(716, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_NEXT_LEVEL_XP(mut self, v: i32) -> Self {
        self.header_set(717, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SKILL_INFO(mut self, skill_info: crate::vanilla::SkillInfo, index: SkillInfoIndex) -> Self {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
        self
    }

    pub fn set_player_CHARACTER_POINTS1(mut self, v: i32) -> Self {
        self.header_set(1102, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_CHARACTER_POINTS2(mut self, v: i32) -> Self {
        self.header_set(1103, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_TRACK_CREATURES(mut self, v: i32) -> Self {
        self.header_set(1104, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_TRACK_RESOURCES(mut self, v: i32) -> Self {
        self.header_set(1105, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_BLOCK_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1106, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_DODGE_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1107, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_PARRY_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1108, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1109, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_RANGED_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1110, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_EXPLORED_ZONES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1111, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_REST_STATE_EXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(1175, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_COINAGE(mut self, v: i32) -> Self {
        self.header_set(1176, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_POSSTAT0(mut self, v: i32) -> Self {
        self.header_set(1177, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_POSSTAT1(mut self, v: i32) -> Self {
        self.header_set(1178, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_POSSTAT2(mut self, v: i32) -> Self {
        self.header_set(1179, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_POSSTAT3(mut self, v: i32) -> Self {
        self.header_set(1180, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_POSSTAT4(mut self, v: i32) -> Self {
        self.header_set(1181, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_NEGSTAT0(mut self, v: i32) -> Self {
        self.header_set(1182, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_NEGSTAT1(mut self, v: i32) -> Self {
        self.header_set(1183, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_NEGSTAT2(mut self, v: i32) -> Self {
        self.header_set(1184, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_NEGSTAT3(mut self, v: i32) -> Self {
        self.header_set(1185, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_NEGSTAT4(mut self, v: i32) -> Self {
        self.header_set(1186, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSPOSITIVE(mut self, v: i32) -> Self {
        self.header_set(1187, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSNEGATIVE(mut self, v: i32) -> Self {
        self.header_set(1194, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_POS(mut self, v: i32) -> Self {
        self.header_set(1201, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_NEG(mut self, v: i32) -> Self {
        self.header_set(1208, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_PCT(mut self, v: i32) -> Self {
        self.header_set(1215, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1222, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_AMMO_ID(mut self, v: i32) -> Self {
        self.header_set(1223, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SELF_RES_SPELL(mut self, v: i32) -> Self {
        self.header_set(1224, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_PVP_MEDALS(mut self, v: i32) -> Self {
        self.header_set(1225, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_BUYBACK_PRICE_1(mut self, v: i32) -> Self {
        self.header_set(1226, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_BUYBACK_TIMESTAMP_1(mut self, v: i32) -> Self {
        self.header_set(1238, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_SESSION_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1250, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_FIELD_YESTERDAY_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1251, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_FIELD_LAST_WEEK_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1252, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_FIELD_THIS_WEEK_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1253, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_FIELD_THIS_WEEK_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1254, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_LIFETIME_HONORBALE_KILLS(mut self, v: i32) -> Self {
        self.header_set(1255, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_LIFETIME_DISHONORBALE_KILLS(mut self, v: i32) -> Self {
        self.header_set(1256, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_YESTERDAY_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1257, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_LAST_WEEK_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1258, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_LAST_WEEK_RANK(mut self, v: i32) -> Self {
        self.header_set(1259, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_BYTES2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1260, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_FIELD_WATCHED_FACTION_INDEX(mut self, v: i32) -> Self {
        self.header_set(1261, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_COMBAT_RATING_1(mut self, v: i32) -> Self {
        self.header_set(1262, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateGameObjectBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_gameobject_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(8, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_FLAGS(mut self, v: i32) -> Self {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_ROTATION(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_STATE(mut self, v: i32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_POS_X(mut self, v: f32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_POS_Y(mut self, v: f32) -> Self {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_POS_Z(mut self, v: f32) -> Self {
        self.header_set(17, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_FACING(mut self, v: f32) -> Self {
        self.header_set(18, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_DYN_FLAGS(mut self, v: i32) -> Self {
        self.header_set(19, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_FACTION(mut self, v: i32) -> Self {
        self.header_set(20, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_TYPE_ID(mut self, v: i32) -> Self {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_LEVEL(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_ARTKIT(mut self, v: i32) -> Self {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_ANIMPROGRESS(mut self, v: i32) -> Self {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateDynamicObjectBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
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
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_RADIUS(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_POS_X(mut self, v: f32) -> Self {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_POS_Y(mut self, v: f32) -> Self {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_POS_Z(mut self, v: f32) -> Self {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_FACING(mut self, v: f32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateCorpseBuilder {
    pub fn set_object_GUID(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_FACING(mut self, v: f32) -> Self {
        self.header_set(8, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_POS_X(mut self, v: f32) -> Self {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_POS_Y(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_POS_Z(mut self, v: f32) -> Self {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_DISPLAY_ID(mut self, v: i32) -> Self {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_ITEM(mut self, v: i32) -> Self {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(32, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(33, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_GUILD(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_FLAGS(mut self, v: i32) -> Self {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
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

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
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
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_STACK_COUNT(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_item_DURATION(&mut self, v: i32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_DURATION(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_item_SPELL_CHARGES(&mut self, v: i32) {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_SPELL_CHARGES(&self) -> Option<i32> {
        self.values.get(&16).map(|v| *v as i32)
    }

    pub fn set_item_FLAGS(&mut self, v: i32) {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_FLAGS(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_ENCHANTMENT(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_PROPERTY_SEED(&mut self, v: i32) {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_PROPERTY_SEED(&self) -> Option<i32> {
        self.values.get(&43).map(|v| *v as i32)
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(&mut self, v: i32) {
        self.header_set(44, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_RANDOM_PROPERTIES_ID(&self) -> Option<i32> {
        self.values.get(&44).map(|v| *v as i32)
    }

    pub fn set_item_ITEM_TEXT_ID(&mut self, v: i32) {
        self.header_set(45, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_ITEM_TEXT_ID(&self) -> Option<i32> {
        self.values.get(&45).map(|v| *v as i32)
    }

    pub fn set_item_DURABILITY(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_DURABILITY(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_item_MAXDURABILITY(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_MAXDURABILITY(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
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

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
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
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_STACK_COUNT(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_item_DURATION(&mut self, v: i32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_DURATION(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_item_SPELL_CHARGES(&mut self, v: i32) {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_SPELL_CHARGES(&self) -> Option<i32> {
        self.values.get(&16).map(|v| *v as i32)
    }

    pub fn set_item_FLAGS(&mut self, v: i32) {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_FLAGS(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_item_ENCHANTMENT(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_ENCHANTMENT(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_PROPERTY_SEED(&mut self, v: i32) {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_PROPERTY_SEED(&self) -> Option<i32> {
        self.values.get(&43).map(|v| *v as i32)
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(&mut self, v: i32) {
        self.header_set(44, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_RANDOM_PROPERTIES_ID(&self) -> Option<i32> {
        self.values.get(&44).map(|v| *v as i32)
    }

    pub fn set_item_ITEM_TEXT_ID(&mut self, v: i32) {
        self.header_set(45, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_ITEM_TEXT_ID(&self) -> Option<i32> {
        self.values.get(&45).map(|v| *v as i32)
    }

    pub fn set_item_DURABILITY(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_DURABILITY(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_item_MAXDURABILITY(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_MAXDURABILITY(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
    }

    pub fn set_container_NUM_SLOTS(&mut self, v: i32) {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn container_NUM_SLOTS(&self) -> Option<i32> {
        self.values.get(&48).map(|v| *v as i32)
    }

    pub fn set_container_SLOT_1(&mut self, v: Guid) {
        self.header_set(50, v.guid() as u32);
        self.header_set(51, (v.guid() >> 32) as u32);
    }

    pub fn container_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&50);
        let upper = self.values.get(&51);

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

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
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

    pub fn set_unit_CHARMEDBY(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHARMEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_SUMMONEDBY(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn unit_SUMMONEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CREATEDBY(&mut self, v: Guid) {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
    }

    pub fn unit_CREATEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_TARGET(&mut self, v: Guid) {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
    }

    pub fn unit_TARGET(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_PERSUADED(&mut self, v: Guid) {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
    }

    pub fn unit_PERSUADED(&self) -> Option<Guid> {
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

    pub fn set_unit_HEALTH(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_HEALTH(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_POWER1(&mut self, v: i32) {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER1(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_unit_POWER2(&mut self, v: i32) {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER2(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_POWER3(&mut self, v: i32) {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER3(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_POWER4(&mut self, v: i32) {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER4(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_POWER5(&mut self, v: i32) {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER5(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_MAXHEALTH(&mut self, v: i32) {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXHEALTH(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER1(&mut self, v: i32) {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER1(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER2(&mut self, v: i32) {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER2(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER3(&mut self, v: i32) {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER3(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER4(&mut self, v: i32) {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER4(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER5(&mut self, v: i32) {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER5(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_LEVEL(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_LEVEL(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_FACTIONTEMPLATE(&mut self, v: i32) {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FACTIONTEMPLATE(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
    }

    pub fn unit_BYTES_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_VIRTUAL_ITEM_SLOT_DISPLAY(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_VIRTUAL_ITEM_INFO(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&40) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_FLAGS(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FLAGS(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_unit_AURA(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AURA(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
    }

    pub fn set_unit_AURAFLAGS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(95, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURAFLAGS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&95) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURALEVELS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(101, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURALEVELS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&101) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURAAPPLICATIONS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(113, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURAAPPLICATIONS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&113) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURASTATE(&mut self, v: i32) {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AURASTATE(&self) -> Option<i32> {
        self.values.get(&125).map(|v| *v as i32)
    }

    pub fn set_unit_BASEATTACKTIME(&mut self, v: i32) {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASEATTACKTIME(&self) -> Option<i32> {
        self.values.get(&126).map(|v| *v as i32)
    }

    pub fn set_unit_RANGEDATTACKTIME(&mut self, v: i32) {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGEDATTACKTIME(&self) -> Option<i32> {
        self.values.get(&128).map(|v| *v as i32)
    }

    pub fn set_unit_BOUNDINGRADIUS(&mut self, v: f32) {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BOUNDINGRADIUS(&self) -> Option<f32> {
        self.values.get(&129).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_COMBATREACH(&mut self, v: f32) {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_COMBATREACH(&self) -> Option<f32> {
        self.values.get(&130).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_DISPLAYID(&mut self, v: i32) {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_DISPLAYID(&self) -> Option<i32> {
        self.values.get(&131).map(|v| *v as i32)
    }

    pub fn set_unit_NATIVEDISPLAYID(&mut self, v: i32) {
        self.header_set(132, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NATIVEDISPLAYID(&self) -> Option<i32> {
        self.values.get(&132).map(|v| *v as i32)
    }

    pub fn set_unit_MOUNTDISPLAYID(&mut self, v: i32) {
        self.header_set(133, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MOUNTDISPLAYID(&self) -> Option<i32> {
        self.values.get(&133).map(|v| *v as i32)
    }

    pub fn set_unit_MINDAMAGE(&mut self, v: f32) {
        self.header_set(134, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINDAMAGE(&self) -> Option<f32> {
        self.values.get(&134).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXDAMAGE(&mut self, v: f32) {
        self.header_set(135, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXDAMAGE(&self) -> Option<f32> {
        self.values.get(&135).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(136, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&136).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(137, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&137).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_BYTES_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(138, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_BYTES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&138) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_PETNUMBER(&mut self, v: i32) {
        self.header_set(139, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETNUMBER(&self) -> Option<i32> {
        self.values.get(&139).map(|v| *v as i32)
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(&mut self, v: i32) {
        self.header_set(140, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PET_NAME_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&140).map(|v| *v as i32)
    }

    pub fn set_unit_PETEXPERIENCE(&mut self, v: i32) {
        self.header_set(141, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETEXPERIENCE(&self) -> Option<i32> {
        self.values.get(&141).map(|v| *v as i32)
    }

    pub fn set_unit_PETNEXTLEVELEXP(&mut self, v: i32) {
        self.header_set(142, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETNEXTLEVELEXP(&self) -> Option<i32> {
        self.values.get(&142).map(|v| *v as i32)
    }

    pub fn set_unit_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(143, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&143).map(|v| *v as i32)
    }

    pub fn set_unit_CHANNEL_SPELL(&mut self, v: i32) {
        self.header_set(144, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_CHANNEL_SPELL(&self) -> Option<i32> {
        self.values.get(&144).map(|v| *v as i32)
    }

    pub fn set_unit_MOD_CAST_SPEED(&mut self, v: f32) {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MOD_CAST_SPEED(&self) -> Option<f32> {
        self.values.get(&145).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_CREATED_BY_SPELL(&mut self, v: i32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_CREATED_BY_SPELL(&self) -> Option<i32> {
        self.values.get(&146).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_FLAGS(&mut self, v: i32) {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NPC_FLAGS(&self) -> Option<i32> {
        self.values.get(&147).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_EMOTESTATE(&mut self, v: i32) {
        self.header_set(148, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NPC_EMOTESTATE(&self) -> Option<i32> {
        self.values.get(&148).map(|v| *v as i32)
    }

    pub fn set_unit_TRAINING_POINTS(&mut self, a: u16, b: u16) {
        self.header_set(149, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_TRAINING_POINTS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&149) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_STRENGTH(&mut self, v: i32) {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_STRENGTH(&self) -> Option<i32> {
        self.values.get(&150).map(|v| *v as i32)
    }

    pub fn set_unit_AGILITY(&mut self, v: i32) {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AGILITY(&self) -> Option<i32> {
        self.values.get(&151).map(|v| *v as i32)
    }

    pub fn set_unit_STAMINA(&mut self, v: i32) {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_STAMINA(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_unit_INTELLECT(&mut self, v: i32) {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_INTELLECT(&self) -> Option<i32> {
        self.values.get(&153).map(|v| *v as i32)
    }

    pub fn set_unit_SPIRIT(&mut self, v: i32) {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_SPIRIT(&self) -> Option<i32> {
        self.values.get(&154).map(|v| *v as i32)
    }

    pub fn set_unit_NORMAL_RESISTANCE(&mut self, v: i32) {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NORMAL_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&155).map(|v| *v as i32)
    }

    pub fn set_unit_HOLY_RESISTANCE(&mut self, v: i32) {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_HOLY_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&156).map(|v| *v as i32)
    }

    pub fn set_unit_FIRE_RESISTANCE(&mut self, v: i32) {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FIRE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&157).map(|v| *v as i32)
    }

    pub fn set_unit_NATURE_RESISTANCE(&mut self, v: i32) {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NATURE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&158).map(|v| *v as i32)
    }

    pub fn set_unit_FROST_RESISTANCE(&mut self, v: i32) {
        self.header_set(159, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FROST_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&159).map(|v| *v as i32)
    }

    pub fn set_unit_SHADOW_RESISTANCE(&mut self, v: i32) {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_SHADOW_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&160).map(|v| *v as i32)
    }

    pub fn set_unit_ARCANE_RESISTANCE(&mut self, v: i32) {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ARCANE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&161).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_MANA(&mut self, v: i32) {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASE_MANA(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_HEALTH(&mut self, v: i32) {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASE_HEALTH(&self) -> Option<i32> {
        self.values.get(&163).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_2(&mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) {
        self.header_set(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
    }

    pub fn unit_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&164) {
            let v = v.to_le_bytes();
            let (facial_hair, unknown, bank_bag_slots, rested_state) = (v[0], v[1], v[2], v[3]);
            Some((facial_hair, unknown, bank_bag_slots, rested_state))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&165).map(|v| *v as i32)
    }

    pub fn set_unit_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(166, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&166) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&167).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_RANGED_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGED_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(169, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_RANGED_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&169) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(170, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGED_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&170).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&171).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&172).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_POWER_COST_MODIFIER(&mut self, v: i32) {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_COST_MODIFIER(&self) -> Option<i32> {
        self.values.get(&173).map(|v| *v as i32)
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(&mut self, v: f32) {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_COST_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&180).map(|v| f32::from_le_bytes(v.to_le_bytes()))
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

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
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

    pub fn set_unit_CHARMEDBY(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn unit_CHARMEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_SUMMONEDBY(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn unit_SUMMONEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_CREATEDBY(&mut self, v: Guid) {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
    }

    pub fn unit_CREATEDBY(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_TARGET(&mut self, v: Guid) {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
    }

    pub fn unit_TARGET(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_PERSUADED(&mut self, v: Guid) {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
    }

    pub fn unit_PERSUADED(&self) -> Option<Guid> {
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

    pub fn set_unit_HEALTH(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_HEALTH(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_POWER1(&mut self, v: i32) {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER1(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_unit_POWER2(&mut self, v: i32) {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER2(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_POWER3(&mut self, v: i32) {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER3(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_POWER4(&mut self, v: i32) {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER4(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_POWER5(&mut self, v: i32) {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER5(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_MAXHEALTH(&mut self, v: i32) {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXHEALTH(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER1(&mut self, v: i32) {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER1(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER2(&mut self, v: i32) {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER2(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER3(&mut self, v: i32) {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER3(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER4(&mut self, v: i32) {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER4(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER5(&mut self, v: i32) {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXPOWER5(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_LEVEL(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_LEVEL(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_FACTIONTEMPLATE(&mut self, v: i32) {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FACTIONTEMPLATE(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
    }

    pub fn unit_BYTES_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_VIRTUAL_ITEM_SLOT_DISPLAY(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_VIRTUAL_ITEM_INFO(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&40) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_FLAGS(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FLAGS(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_unit_AURA(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AURA(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
    }

    pub fn set_unit_AURAFLAGS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(95, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURAFLAGS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&95) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURALEVELS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(101, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURALEVELS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&101) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURAAPPLICATIONS(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(113, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_AURAAPPLICATIONS(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&113) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_AURASTATE(&mut self, v: i32) {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AURASTATE(&self) -> Option<i32> {
        self.values.get(&125).map(|v| *v as i32)
    }

    pub fn set_unit_BASEATTACKTIME(&mut self, v: i32) {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASEATTACKTIME(&self) -> Option<i32> {
        self.values.get(&126).map(|v| *v as i32)
    }

    pub fn set_unit_RANGEDATTACKTIME(&mut self, v: i32) {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGEDATTACKTIME(&self) -> Option<i32> {
        self.values.get(&128).map(|v| *v as i32)
    }

    pub fn set_unit_BOUNDINGRADIUS(&mut self, v: f32) {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BOUNDINGRADIUS(&self) -> Option<f32> {
        self.values.get(&129).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_COMBATREACH(&mut self, v: f32) {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_COMBATREACH(&self) -> Option<f32> {
        self.values.get(&130).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_DISPLAYID(&mut self, v: i32) {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_DISPLAYID(&self) -> Option<i32> {
        self.values.get(&131).map(|v| *v as i32)
    }

    pub fn set_unit_NATIVEDISPLAYID(&mut self, v: i32) {
        self.header_set(132, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NATIVEDISPLAYID(&self) -> Option<i32> {
        self.values.get(&132).map(|v| *v as i32)
    }

    pub fn set_unit_MOUNTDISPLAYID(&mut self, v: i32) {
        self.header_set(133, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MOUNTDISPLAYID(&self) -> Option<i32> {
        self.values.get(&133).map(|v| *v as i32)
    }

    pub fn set_unit_MINDAMAGE(&mut self, v: f32) {
        self.header_set(134, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINDAMAGE(&self) -> Option<f32> {
        self.values.get(&134).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXDAMAGE(&mut self, v: f32) {
        self.header_set(135, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXDAMAGE(&self) -> Option<f32> {
        self.values.get(&135).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(136, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&136).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(137, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXOFFHANDDAMAGE(&self) -> Option<f32> {
        self.values.get(&137).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_BYTES_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(138, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_BYTES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&138) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_PETNUMBER(&mut self, v: i32) {
        self.header_set(139, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETNUMBER(&self) -> Option<i32> {
        self.values.get(&139).map(|v| *v as i32)
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(&mut self, v: i32) {
        self.header_set(140, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PET_NAME_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&140).map(|v| *v as i32)
    }

    pub fn set_unit_PETEXPERIENCE(&mut self, v: i32) {
        self.header_set(141, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETEXPERIENCE(&self) -> Option<i32> {
        self.values.get(&141).map(|v| *v as i32)
    }

    pub fn set_unit_PETNEXTLEVELEXP(&mut self, v: i32) {
        self.header_set(142, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_PETNEXTLEVELEXP(&self) -> Option<i32> {
        self.values.get(&142).map(|v| *v as i32)
    }

    pub fn set_unit_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(143, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&143).map(|v| *v as i32)
    }

    pub fn set_unit_CHANNEL_SPELL(&mut self, v: i32) {
        self.header_set(144, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_CHANNEL_SPELL(&self) -> Option<i32> {
        self.values.get(&144).map(|v| *v as i32)
    }

    pub fn set_unit_MOD_CAST_SPEED(&mut self, v: f32) {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MOD_CAST_SPEED(&self) -> Option<f32> {
        self.values.get(&145).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_CREATED_BY_SPELL(&mut self, v: i32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_CREATED_BY_SPELL(&self) -> Option<i32> {
        self.values.get(&146).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_FLAGS(&mut self, v: i32) {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NPC_FLAGS(&self) -> Option<i32> {
        self.values.get(&147).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_EMOTESTATE(&mut self, v: i32) {
        self.header_set(148, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NPC_EMOTESTATE(&self) -> Option<i32> {
        self.values.get(&148).map(|v| *v as i32)
    }

    pub fn set_unit_TRAINING_POINTS(&mut self, a: u16, b: u16) {
        self.header_set(149, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_TRAINING_POINTS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&149) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_STRENGTH(&mut self, v: i32) {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_STRENGTH(&self) -> Option<i32> {
        self.values.get(&150).map(|v| *v as i32)
    }

    pub fn set_unit_AGILITY(&mut self, v: i32) {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_AGILITY(&self) -> Option<i32> {
        self.values.get(&151).map(|v| *v as i32)
    }

    pub fn set_unit_STAMINA(&mut self, v: i32) {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_STAMINA(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_unit_INTELLECT(&mut self, v: i32) {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_INTELLECT(&self) -> Option<i32> {
        self.values.get(&153).map(|v| *v as i32)
    }

    pub fn set_unit_SPIRIT(&mut self, v: i32) {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_SPIRIT(&self) -> Option<i32> {
        self.values.get(&154).map(|v| *v as i32)
    }

    pub fn set_unit_NORMAL_RESISTANCE(&mut self, v: i32) {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NORMAL_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&155).map(|v| *v as i32)
    }

    pub fn set_unit_HOLY_RESISTANCE(&mut self, v: i32) {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_HOLY_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&156).map(|v| *v as i32)
    }

    pub fn set_unit_FIRE_RESISTANCE(&mut self, v: i32) {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FIRE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&157).map(|v| *v as i32)
    }

    pub fn set_unit_NATURE_RESISTANCE(&mut self, v: i32) {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_NATURE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&158).map(|v| *v as i32)
    }

    pub fn set_unit_FROST_RESISTANCE(&mut self, v: i32) {
        self.header_set(159, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_FROST_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&159).map(|v| *v as i32)
    }

    pub fn set_unit_SHADOW_RESISTANCE(&mut self, v: i32) {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_SHADOW_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&160).map(|v| *v as i32)
    }

    pub fn set_unit_ARCANE_RESISTANCE(&mut self, v: i32) {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ARCANE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&161).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_MANA(&mut self, v: i32) {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASE_MANA(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_HEALTH(&mut self, v: i32) {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_BASE_HEALTH(&self) -> Option<i32> {
        self.values.get(&163).map(|v| *v as i32)
    }

    pub fn set_unit_BYTES_2(&mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) {
        self.header_set(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
    }

    pub fn unit_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&164) {
            let v = v.to_le_bytes();
            let (facial_hair, unknown, bank_bag_slots, rested_state) = (v[0], v[1], v[2], v[3]);
            Some((facial_hair, unknown, bank_bag_slots, rested_state))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&165).map(|v| *v as i32)
    }

    pub fn set_unit_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(166, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&166) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&167).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_RANGED_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGED_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(169, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_RANGED_ATTACK_POWER_MODS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&169) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(170, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_RANGED_ATTACK_POWER_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&170).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MINRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MINRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&171).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_MAXRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_MAXRANGEDDAMAGE(&self) -> Option<f32> {
        self.values.get(&172).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_POWER_COST_MODIFIER(&mut self, v: i32) {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_COST_MODIFIER(&self) -> Option<i32> {
        self.values.get(&173).map(|v| *v as i32)
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(&mut self, v: f32) {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_POWER_COST_MULTIPLIER(&self) -> Option<f32> {
        self.values.get(&180).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_DUEL_ARBITER(&mut self, v: Guid) {
        self.header_set(188, v.guid() as u32);
        self.header_set(189, (v.guid() >> 32) as u32);
    }

    pub fn player_DUEL_ARBITER(&self) -> Option<Guid> {
        let lower = self.values.get(&188);
        let upper = self.values.get(&189);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FLAGS(&mut self, v: i32) {
        self.header_set(190, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FLAGS(&self) -> Option<i32> {
        self.values.get(&190).map(|v| *v as i32)
    }

    pub fn set_player_GUILDID(&mut self, v: i32) {
        self.header_set(191, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_GUILDID(&self) -> Option<i32> {
        self.values.get(&191).map(|v| *v as i32)
    }

    pub fn set_player_GUILDRANK(&mut self, v: i32) {
        self.header_set(192, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_GUILDRANK(&self) -> Option<i32> {
        self.values.get(&192).map(|v| *v as i32)
    }

    pub fn set_player_FEATURES(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(193, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_FEATURES(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&193) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_BYTES_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(194, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&194) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_BYTES_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(195, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_BYTES_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&195) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_DUEL_TEAM(&mut self, v: i32) {
        self.header_set(196, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_DUEL_TEAM(&self) -> Option<i32> {
        self.values.get(&196).map(|v| *v as i32)
    }

    pub fn set_player_GUILD_TIMESTAMP(&mut self, v: i32) {
        self.header_set(197, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_GUILD_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&197).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_1_1(&mut self, v: i32) {
        self.header_set(198, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_1_1(&self) -> Option<i32> {
        self.values.get(&198).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_1_2(&mut self, v: i32) {
        self.header_set(199, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_1_2(&self) -> Option<i32> {
        self.values.get(&199).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_2_1(&mut self, v: i32) {
        self.header_set(201, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_2_1(&self) -> Option<i32> {
        self.values.get(&201).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_2_2(&mut self, v: i32) {
        self.header_set(202, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_2_2(&self) -> Option<i32> {
        self.values.get(&202).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_3_1(&mut self, v: i32) {
        self.header_set(204, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_3_1(&self) -> Option<i32> {
        self.values.get(&204).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_3_2(&mut self, v: i32) {
        self.header_set(205, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_3_2(&self) -> Option<i32> {
        self.values.get(&205).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_4_1(&mut self, v: i32) {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_4_1(&self) -> Option<i32> {
        self.values.get(&207).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_4_2(&mut self, v: i32) {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_4_2(&self) -> Option<i32> {
        self.values.get(&208).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_5_1(&mut self, v: i32) {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_5_1(&self) -> Option<i32> {
        self.values.get(&210).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_5_2(&mut self, v: i32) {
        self.header_set(211, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_5_2(&self) -> Option<i32> {
        self.values.get(&211).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_6_1(&mut self, v: i32) {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_6_1(&self) -> Option<i32> {
        self.values.get(&213).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_6_2(&mut self, v: i32) {
        self.header_set(214, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_6_2(&self) -> Option<i32> {
        self.values.get(&214).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_7_1(&mut self, v: i32) {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_7_1(&self) -> Option<i32> {
        self.values.get(&216).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_7_2(&mut self, v: i32) {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_7_2(&self) -> Option<i32> {
        self.values.get(&217).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_8_1(&mut self, v: i32) {
        self.header_set(219, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_8_1(&self) -> Option<i32> {
        self.values.get(&219).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_8_2(&mut self, v: i32) {
        self.header_set(220, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_8_2(&self) -> Option<i32> {
        self.values.get(&220).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_9_1(&mut self, v: i32) {
        self.header_set(222, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_9_1(&self) -> Option<i32> {
        self.values.get(&222).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_9_2(&mut self, v: i32) {
        self.header_set(223, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_9_2(&self) -> Option<i32> {
        self.values.get(&223).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_10_1(&mut self, v: i32) {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_10_1(&self) -> Option<i32> {
        self.values.get(&225).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_10_2(&mut self, v: i32) {
        self.header_set(226, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_10_2(&self) -> Option<i32> {
        self.values.get(&226).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_11_1(&mut self, v: i32) {
        self.header_set(228, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_11_1(&self) -> Option<i32> {
        self.values.get(&228).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_11_2(&mut self, v: i32) {
        self.header_set(229, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_11_2(&self) -> Option<i32> {
        self.values.get(&229).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_12_1(&mut self, v: i32) {
        self.header_set(231, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_12_1(&self) -> Option<i32> {
        self.values.get(&231).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_12_2(&mut self, v: i32) {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_12_2(&self) -> Option<i32> {
        self.values.get(&232).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_13_1(&mut self, v: i32) {
        self.header_set(234, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_13_1(&self) -> Option<i32> {
        self.values.get(&234).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_13_2(&mut self, v: i32) {
        self.header_set(235, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_13_2(&self) -> Option<i32> {
        self.values.get(&235).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_14_1(&mut self, v: i32) {
        self.header_set(237, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_14_1(&self) -> Option<i32> {
        self.values.get(&237).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_14_2(&mut self, v: i32) {
        self.header_set(238, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_14_2(&self) -> Option<i32> {
        self.values.get(&238).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_15_1(&mut self, v: i32) {
        self.header_set(240, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_15_1(&self) -> Option<i32> {
        self.values.get(&240).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_15_2(&mut self, v: i32) {
        self.header_set(241, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_15_2(&self) -> Option<i32> {
        self.values.get(&241).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_16_1(&mut self, v: i32) {
        self.header_set(243, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_16_1(&self) -> Option<i32> {
        self.values.get(&243).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_16_2(&mut self, v: i32) {
        self.header_set(244, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_16_2(&self) -> Option<i32> {
        self.values.get(&244).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_17_1(&mut self, v: i32) {
        self.header_set(246, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_17_1(&self) -> Option<i32> {
        self.values.get(&246).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_17_2(&mut self, v: i32) {
        self.header_set(247, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_17_2(&self) -> Option<i32> {
        self.values.get(&247).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_18_1(&mut self, v: i32) {
        self.header_set(249, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_18_1(&self) -> Option<i32> {
        self.values.get(&249).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_18_2(&mut self, v: i32) {
        self.header_set(250, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_18_2(&self) -> Option<i32> {
        self.values.get(&250).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_19_1(&mut self, v: i32) {
        self.header_set(252, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_19_1(&self) -> Option<i32> {
        self.values.get(&252).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_19_2(&mut self, v: i32) {
        self.header_set(253, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_19_2(&self) -> Option<i32> {
        self.values.get(&253).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_20_1(&mut self, v: i32) {
        self.header_set(255, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_20_1(&self) -> Option<i32> {
        self.values.get(&255).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_20_2(&mut self, v: i32) {
        self.header_set(256, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_QUEST_LOG_20_2(&self) -> Option<i32> {
        self.values.get(&256).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_1_CREATOR(&mut self, v: Guid) {
        self.header_set(258, v.guid() as u32);
        self.header_set(259, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_1_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&258);
        let upper = self.values.get(&259);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_1_0(&mut self, v: i32) {
        self.header_set(260, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_1_0(&self) -> Option<i32> {
        self.values.get(&260).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_1_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(268, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_1_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&268) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_2_CREATOR(&mut self, v: Guid) {
        self.header_set(270, v.guid() as u32);
        self.header_set(271, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_2_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&270);
        let upper = self.values.get(&271);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_2_0(&mut self, v: i32) {
        self.header_set(272, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_2_0(&self) -> Option<i32> {
        self.values.get(&272).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_2_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(280, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_2_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&280) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_3_CREATOR(&mut self, v: Guid) {
        self.header_set(282, v.guid() as u32);
        self.header_set(283, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_3_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&282);
        let upper = self.values.get(&283);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_3_0(&mut self, v: i32) {
        self.header_set(284, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_3_0(&self) -> Option<i32> {
        self.values.get(&284).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_3_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(292, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_3_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&292) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_4_CREATOR(&mut self, v: Guid) {
        self.header_set(294, v.guid() as u32);
        self.header_set(295, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_4_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&294);
        let upper = self.values.get(&295);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_4_0(&mut self, v: i32) {
        self.header_set(296, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_4_0(&self) -> Option<i32> {
        self.values.get(&296).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_4_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(304, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_4_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&304) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_5_CREATOR(&mut self, v: Guid) {
        self.header_set(306, v.guid() as u32);
        self.header_set(307, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_5_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&306);
        let upper = self.values.get(&307);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_5_0(&mut self, v: i32) {
        self.header_set(308, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_5_0(&self) -> Option<i32> {
        self.values.get(&308).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_5_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(316, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_5_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&316) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_6_CREATOR(&mut self, v: Guid) {
        self.header_set(318, v.guid() as u32);
        self.header_set(319, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_6_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&318);
        let upper = self.values.get(&319);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_6_0(&mut self, v: i32) {
        self.header_set(320, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_6_0(&self) -> Option<i32> {
        self.values.get(&320).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_6_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(328, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_6_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&328) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_7_CREATOR(&mut self, v: Guid) {
        self.header_set(330, v.guid() as u32);
        self.header_set(331, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_7_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&330);
        let upper = self.values.get(&331);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_7_0(&mut self, v: i32) {
        self.header_set(332, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_7_0(&self) -> Option<i32> {
        self.values.get(&332).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_7_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(340, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_7_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&340) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_8_CREATOR(&mut self, v: Guid) {
        self.header_set(342, v.guid() as u32);
        self.header_set(343, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_8_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&342);
        let upper = self.values.get(&343);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_8_0(&mut self, v: i32) {
        self.header_set(344, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_8_0(&self) -> Option<i32> {
        self.values.get(&344).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_8_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(352, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_8_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&352) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_9_CREATOR(&mut self, v: Guid) {
        self.header_set(354, v.guid() as u32);
        self.header_set(355, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_9_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&354);
        let upper = self.values.get(&355);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_9_0(&mut self, v: i32) {
        self.header_set(356, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_9_0(&self) -> Option<i32> {
        self.values.get(&356).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_9_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(364, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_9_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&364) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_10_CREATOR(&mut self, v: Guid) {
        self.header_set(366, v.guid() as u32);
        self.header_set(367, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_10_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&366);
        let upper = self.values.get(&367);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_10_0(&mut self, v: i32) {
        self.header_set(368, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_10_0(&self) -> Option<i32> {
        self.values.get(&368).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_10_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(376, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_10_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&376) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_11_CREATOR(&mut self, v: Guid) {
        self.header_set(378, v.guid() as u32);
        self.header_set(379, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_11_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&378);
        let upper = self.values.get(&379);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_11_0(&mut self, v: i32) {
        self.header_set(380, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_11_0(&self) -> Option<i32> {
        self.values.get(&380).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_11_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(388, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_11_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&388) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_12_CREATOR(&mut self, v: Guid) {
        self.header_set(390, v.guid() as u32);
        self.header_set(391, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_12_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&390);
        let upper = self.values.get(&391);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_12_0(&mut self, v: i32) {
        self.header_set(392, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_12_0(&self) -> Option<i32> {
        self.values.get(&392).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_12_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(400, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_12_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&400) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_13_CREATOR(&mut self, v: Guid) {
        self.header_set(402, v.guid() as u32);
        self.header_set(403, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_13_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&402);
        let upper = self.values.get(&403);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_13_0(&mut self, v: i32) {
        self.header_set(404, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_13_0(&self) -> Option<i32> {
        self.values.get(&404).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_13_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(412, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_13_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&412) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_14_CREATOR(&mut self, v: Guid) {
        self.header_set(414, v.guid() as u32);
        self.header_set(415, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_14_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&414);
        let upper = self.values.get(&415);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_14_0(&mut self, v: i32) {
        self.header_set(416, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_14_0(&self) -> Option<i32> {
        self.values.get(&416).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_14_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(424, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_14_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&424) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_15_CREATOR(&mut self, v: Guid) {
        self.header_set(426, v.guid() as u32);
        self.header_set(427, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_15_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&426);
        let upper = self.values.get(&427);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_15_0(&mut self, v: i32) {
        self.header_set(428, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_15_0(&self) -> Option<i32> {
        self.values.get(&428).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_15_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(436, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_15_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&436) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_16_CREATOR(&mut self, v: Guid) {
        self.header_set(438, v.guid() as u32);
        self.header_set(439, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_16_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&438);
        let upper = self.values.get(&439);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_16_0(&mut self, v: i32) {
        self.header_set(440, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_16_0(&self) -> Option<i32> {
        self.values.get(&440).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_16_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(448, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_16_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&448) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_17_CREATOR(&mut self, v: Guid) {
        self.header_set(450, v.guid() as u32);
        self.header_set(451, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_17_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&450);
        let upper = self.values.get(&451);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_17_0(&mut self, v: i32) {
        self.header_set(452, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_17_0(&self) -> Option<i32> {
        self.values.get(&452).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_17_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(460, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_17_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&460) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_18_CREATOR(&mut self, v: Guid) {
        self.header_set(462, v.guid() as u32);
        self.header_set(463, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_18_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&462);
        let upper = self.values.get(&463);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_18_0(&mut self, v: i32) {
        self.header_set(464, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_18_0(&self) -> Option<i32> {
        self.values.get(&464).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_18_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(472, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_18_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&472) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_VISIBLE_ITEM_19_CREATOR(&mut self, v: Guid) {
        self.header_set(474, v.guid() as u32);
        self.header_set(475, (v.guid() >> 32) as u32);
    }

    pub fn player_VISIBLE_ITEM_19_CREATOR(&self) -> Option<Guid> {
        let lower = self.values.get(&474);
        let upper = self.values.get(&475);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_VISIBLE_ITEM_19_0(&mut self, v: i32) {
        self.header_set(476, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_VISIBLE_ITEM_19_0(&self) -> Option<i32> {
        self.values.get(&476).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_19_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(484, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_VISIBLE_ITEM_19_PROPERTIES(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&484) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_INV_SLOT_HEAD(&mut self, v: Guid) {
        self.header_set(486, v.guid() as u32);
        self.header_set(487, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_HEAD(&self) -> Option<Guid> {
        let lower = self.values.get(&486);
        let upper = self.values.get(&487);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_NECK(&mut self, v: Guid) {
        self.header_set(488, v.guid() as u32);
        self.header_set(489, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_NECK(&self) -> Option<Guid> {
        let lower = self.values.get(&488);
        let upper = self.values.get(&489);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_SHOULDERS(&mut self, v: Guid) {
        self.header_set(490, v.guid() as u32);
        self.header_set(491, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_SHOULDERS(&self) -> Option<Guid> {
        let lower = self.values.get(&490);
        let upper = self.values.get(&491);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_BODY(&mut self, v: Guid) {
        self.header_set(492, v.guid() as u32);
        self.header_set(493, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_BODY(&self) -> Option<Guid> {
        let lower = self.values.get(&492);
        let upper = self.values.get(&493);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_CHEST(&mut self, v: Guid) {
        self.header_set(494, v.guid() as u32);
        self.header_set(495, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_CHEST(&self) -> Option<Guid> {
        let lower = self.values.get(&494);
        let upper = self.values.get(&495);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_WAIST(&mut self, v: Guid) {
        self.header_set(496, v.guid() as u32);
        self.header_set(497, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_WAIST(&self) -> Option<Guid> {
        let lower = self.values.get(&496);
        let upper = self.values.get(&497);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_LEGS(&mut self, v: Guid) {
        self.header_set(498, v.guid() as u32);
        self.header_set(499, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_LEGS(&self) -> Option<Guid> {
        let lower = self.values.get(&498);
        let upper = self.values.get(&499);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_FEET(&mut self, v: Guid) {
        self.header_set(500, v.guid() as u32);
        self.header_set(501, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_FEET(&self) -> Option<Guid> {
        let lower = self.values.get(&500);
        let upper = self.values.get(&501);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_WRISTS(&mut self, v: Guid) {
        self.header_set(502, v.guid() as u32);
        self.header_set(503, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_WRISTS(&self) -> Option<Guid> {
        let lower = self.values.get(&502);
        let upper = self.values.get(&503);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_HANDS(&mut self, v: Guid) {
        self.header_set(504, v.guid() as u32);
        self.header_set(505, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_HANDS(&self) -> Option<Guid> {
        let lower = self.values.get(&504);
        let upper = self.values.get(&505);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_FINGER1(&mut self, v: Guid) {
        self.header_set(506, v.guid() as u32);
        self.header_set(507, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_FINGER1(&self) -> Option<Guid> {
        let lower = self.values.get(&506);
        let upper = self.values.get(&507);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_FINGER2(&mut self, v: Guid) {
        self.header_set(508, v.guid() as u32);
        self.header_set(509, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_FINGER2(&self) -> Option<Guid> {
        let lower = self.values.get(&508);
        let upper = self.values.get(&509);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_TRINKET1(&mut self, v: Guid) {
        self.header_set(510, v.guid() as u32);
        self.header_set(511, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_TRINKET1(&self) -> Option<Guid> {
        let lower = self.values.get(&510);
        let upper = self.values.get(&511);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_TRINKET2(&mut self, v: Guid) {
        self.header_set(512, v.guid() as u32);
        self.header_set(513, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_TRINKET2(&self) -> Option<Guid> {
        let lower = self.values.get(&512);
        let upper = self.values.get(&513);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_BACK(&mut self, v: Guid) {
        self.header_set(514, v.guid() as u32);
        self.header_set(515, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_BACK(&self) -> Option<Guid> {
        let lower = self.values.get(&514);
        let upper = self.values.get(&515);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_MAIN_HAND(&mut self, v: Guid) {
        self.header_set(516, v.guid() as u32);
        self.header_set(517, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_MAIN_HAND(&self) -> Option<Guid> {
        let lower = self.values.get(&516);
        let upper = self.values.get(&517);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_OFF_HAND(&mut self, v: Guid) {
        self.header_set(518, v.guid() as u32);
        self.header_set(519, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_OFF_HAND(&self) -> Option<Guid> {
        let lower = self.values.get(&518);
        let upper = self.values.get(&519);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_RANGED(&mut self, v: Guid) {
        self.header_set(520, v.guid() as u32);
        self.header_set(521, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_RANGED(&self) -> Option<Guid> {
        let lower = self.values.get(&520);
        let upper = self.values.get(&521);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_TABARD(&mut self, v: Guid) {
        self.header_set(522, v.guid() as u32);
        self.header_set(523, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_TABARD(&self) -> Option<Guid> {
        let lower = self.values.get(&522);
        let upper = self.values.get(&523);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_BAG1(&mut self, v: Guid) {
        self.header_set(524, v.guid() as u32);
        self.header_set(525, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_BAG1(&self) -> Option<Guid> {
        let lower = self.values.get(&524);
        let upper = self.values.get(&525);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_BAG2(&mut self, v: Guid) {
        self.header_set(526, v.guid() as u32);
        self.header_set(527, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_BAG2(&self) -> Option<Guid> {
        let lower = self.values.get(&526);
        let upper = self.values.get(&527);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_BAG3(&mut self, v: Guid) {
        self.header_set(528, v.guid() as u32);
        self.header_set(529, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_BAG3(&self) -> Option<Guid> {
        let lower = self.values.get(&528);
        let upper = self.values.get(&529);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_INV_SLOT_BAG4(&mut self, v: Guid) {
        self.header_set(530, v.guid() as u32);
        self.header_set(531, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_INV_SLOT_BAG4(&self) -> Option<Guid> {
        let lower = self.values.get(&530);
        let upper = self.values.get(&531);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_1(&mut self, v: Guid) {
        self.header_set(532, v.guid() as u32);
        self.header_set(533, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&532);
        let upper = self.values.get(&533);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_2(&mut self, v: Guid) {
        self.header_set(534, v.guid() as u32);
        self.header_set(535, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_2(&self) -> Option<Guid> {
        let lower = self.values.get(&534);
        let upper = self.values.get(&535);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_3(&mut self, v: Guid) {
        self.header_set(536, v.guid() as u32);
        self.header_set(537, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_3(&self) -> Option<Guid> {
        let lower = self.values.get(&536);
        let upper = self.values.get(&537);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_4(&mut self, v: Guid) {
        self.header_set(538, v.guid() as u32);
        self.header_set(539, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_4(&self) -> Option<Guid> {
        let lower = self.values.get(&538);
        let upper = self.values.get(&539);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_5(&mut self, v: Guid) {
        self.header_set(540, v.guid() as u32);
        self.header_set(541, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_5(&self) -> Option<Guid> {
        let lower = self.values.get(&540);
        let upper = self.values.get(&541);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_6(&mut self, v: Guid) {
        self.header_set(542, v.guid() as u32);
        self.header_set(543, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_6(&self) -> Option<Guid> {
        let lower = self.values.get(&542);
        let upper = self.values.get(&543);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_7(&mut self, v: Guid) {
        self.header_set(544, v.guid() as u32);
        self.header_set(545, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_7(&self) -> Option<Guid> {
        let lower = self.values.get(&544);
        let upper = self.values.get(&545);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_8(&mut self, v: Guid) {
        self.header_set(546, v.guid() as u32);
        self.header_set(547, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_8(&self) -> Option<Guid> {
        let lower = self.values.get(&546);
        let upper = self.values.get(&547);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_9(&mut self, v: Guid) {
        self.header_set(548, v.guid() as u32);
        self.header_set(549, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_9(&self) -> Option<Guid> {
        let lower = self.values.get(&548);
        let upper = self.values.get(&549);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_10(&mut self, v: Guid) {
        self.header_set(550, v.guid() as u32);
        self.header_set(551, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_10(&self) -> Option<Guid> {
        let lower = self.values.get(&550);
        let upper = self.values.get(&551);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_11(&mut self, v: Guid) {
        self.header_set(552, v.guid() as u32);
        self.header_set(553, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_11(&self) -> Option<Guid> {
        let lower = self.values.get(&552);
        let upper = self.values.get(&553);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_12(&mut self, v: Guid) {
        self.header_set(554, v.guid() as u32);
        self.header_set(555, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_12(&self) -> Option<Guid> {
        let lower = self.values.get(&554);
        let upper = self.values.get(&555);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_13(&mut self, v: Guid) {
        self.header_set(556, v.guid() as u32);
        self.header_set(557, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_13(&self) -> Option<Guid> {
        let lower = self.values.get(&556);
        let upper = self.values.get(&557);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_14(&mut self, v: Guid) {
        self.header_set(558, v.guid() as u32);
        self.header_set(559, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_14(&self) -> Option<Guid> {
        let lower = self.values.get(&558);
        let upper = self.values.get(&559);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_15(&mut self, v: Guid) {
        self.header_set(560, v.guid() as u32);
        self.header_set(561, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_15(&self) -> Option<Guid> {
        let lower = self.values.get(&560);
        let upper = self.values.get(&561);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_PACK_SLOT_16(&mut self, v: Guid) {
        self.header_set(562, v.guid() as u32);
        self.header_set(563, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_16(&self) -> Option<Guid> {
        let lower = self.values.get(&562);
        let upper = self.values.get(&563);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_1(&mut self, v: Guid) {
        self.header_set(564, v.guid() as u32);
        self.header_set(565, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&564);
        let upper = self.values.get(&565);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_2(&mut self, v: Guid) {
        self.header_set(566, v.guid() as u32);
        self.header_set(567, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_2(&self) -> Option<Guid> {
        let lower = self.values.get(&566);
        let upper = self.values.get(&567);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_3(&mut self, v: Guid) {
        self.header_set(568, v.guid() as u32);
        self.header_set(569, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_3(&self) -> Option<Guid> {
        let lower = self.values.get(&568);
        let upper = self.values.get(&569);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_4(&mut self, v: Guid) {
        self.header_set(570, v.guid() as u32);
        self.header_set(571, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_4(&self) -> Option<Guid> {
        let lower = self.values.get(&570);
        let upper = self.values.get(&571);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_5(&mut self, v: Guid) {
        self.header_set(572, v.guid() as u32);
        self.header_set(573, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_5(&self) -> Option<Guid> {
        let lower = self.values.get(&572);
        let upper = self.values.get(&573);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_6(&mut self, v: Guid) {
        self.header_set(574, v.guid() as u32);
        self.header_set(575, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_6(&self) -> Option<Guid> {
        let lower = self.values.get(&574);
        let upper = self.values.get(&575);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_7(&mut self, v: Guid) {
        self.header_set(576, v.guid() as u32);
        self.header_set(577, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_7(&self) -> Option<Guid> {
        let lower = self.values.get(&576);
        let upper = self.values.get(&577);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_8(&mut self, v: Guid) {
        self.header_set(578, v.guid() as u32);
        self.header_set(579, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_8(&self) -> Option<Guid> {
        let lower = self.values.get(&578);
        let upper = self.values.get(&579);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_9(&mut self, v: Guid) {
        self.header_set(580, v.guid() as u32);
        self.header_set(581, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_9(&self) -> Option<Guid> {
        let lower = self.values.get(&580);
        let upper = self.values.get(&581);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_10(&mut self, v: Guid) {
        self.header_set(582, v.guid() as u32);
        self.header_set(583, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_10(&self) -> Option<Guid> {
        let lower = self.values.get(&582);
        let upper = self.values.get(&583);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_11(&mut self, v: Guid) {
        self.header_set(584, v.guid() as u32);
        self.header_set(585, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_11(&self) -> Option<Guid> {
        let lower = self.values.get(&584);
        let upper = self.values.get(&585);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_12(&mut self, v: Guid) {
        self.header_set(586, v.guid() as u32);
        self.header_set(587, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_12(&self) -> Option<Guid> {
        let lower = self.values.get(&586);
        let upper = self.values.get(&587);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_13(&mut self, v: Guid) {
        self.header_set(588, v.guid() as u32);
        self.header_set(589, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_13(&self) -> Option<Guid> {
        let lower = self.values.get(&588);
        let upper = self.values.get(&589);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_14(&mut self, v: Guid) {
        self.header_set(590, v.guid() as u32);
        self.header_set(591, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_14(&self) -> Option<Guid> {
        let lower = self.values.get(&590);
        let upper = self.values.get(&591);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_15(&mut self, v: Guid) {
        self.header_set(592, v.guid() as u32);
        self.header_set(593, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_15(&self) -> Option<Guid> {
        let lower = self.values.get(&592);
        let upper = self.values.get(&593);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_16(&mut self, v: Guid) {
        self.header_set(594, v.guid() as u32);
        self.header_set(595, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_16(&self) -> Option<Guid> {
        let lower = self.values.get(&594);
        let upper = self.values.get(&595);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_17(&mut self, v: Guid) {
        self.header_set(596, v.guid() as u32);
        self.header_set(597, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_17(&self) -> Option<Guid> {
        let lower = self.values.get(&596);
        let upper = self.values.get(&597);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_18(&mut self, v: Guid) {
        self.header_set(598, v.guid() as u32);
        self.header_set(599, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_18(&self) -> Option<Guid> {
        let lower = self.values.get(&598);
        let upper = self.values.get(&599);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_19(&mut self, v: Guid) {
        self.header_set(600, v.guid() as u32);
        self.header_set(601, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_19(&self) -> Option<Guid> {
        let lower = self.values.get(&600);
        let upper = self.values.get(&601);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_20(&mut self, v: Guid) {
        self.header_set(602, v.guid() as u32);
        self.header_set(603, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_20(&self) -> Option<Guid> {
        let lower = self.values.get(&602);
        let upper = self.values.get(&603);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_21(&mut self, v: Guid) {
        self.header_set(604, v.guid() as u32);
        self.header_set(605, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_21(&self) -> Option<Guid> {
        let lower = self.values.get(&604);
        let upper = self.values.get(&605);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_22(&mut self, v: Guid) {
        self.header_set(606, v.guid() as u32);
        self.header_set(607, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_22(&self) -> Option<Guid> {
        let lower = self.values.get(&606);
        let upper = self.values.get(&607);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_23(&mut self, v: Guid) {
        self.header_set(608, v.guid() as u32);
        self.header_set(609, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_23(&self) -> Option<Guid> {
        let lower = self.values.get(&608);
        let upper = self.values.get(&609);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANK_SLOT_24(&mut self, v: Guid) {
        self.header_set(610, v.guid() as u32);
        self.header_set(611, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANK_SLOT_24(&self) -> Option<Guid> {
        let lower = self.values.get(&610);
        let upper = self.values.get(&611);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_1(&mut self, v: Guid) {
        self.header_set(612, v.guid() as u32);
        self.header_set(613, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANKBAG_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&612);
        let upper = self.values.get(&613);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_2(&mut self, v: Guid) {
        self.header_set(614, v.guid() as u32);
        self.header_set(615, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANKBAG_SLOT_2(&self) -> Option<Guid> {
        let lower = self.values.get(&614);
        let upper = self.values.get(&615);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_3(&mut self, v: Guid) {
        self.header_set(616, v.guid() as u32);
        self.header_set(617, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANKBAG_SLOT_3(&self) -> Option<Guid> {
        let lower = self.values.get(&616);
        let upper = self.values.get(&617);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_4(&mut self, v: Guid) {
        self.header_set(618, v.guid() as u32);
        self.header_set(619, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANKBAG_SLOT_4(&self) -> Option<Guid> {
        let lower = self.values.get(&618);
        let upper = self.values.get(&619);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_5(&mut self, v: Guid) {
        self.header_set(620, v.guid() as u32);
        self.header_set(621, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANKBAG_SLOT_5(&self) -> Option<Guid> {
        let lower = self.values.get(&620);
        let upper = self.values.get(&621);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_6(&mut self, v: Guid) {
        self.header_set(622, v.guid() as u32);
        self.header_set(623, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANKBAG_SLOT_6(&self) -> Option<Guid> {
        let lower = self.values.get(&622);
        let upper = self.values.get(&623);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_1(&mut self, v: Guid) {
        self.header_set(624, v.guid() as u32);
        self.header_set(625, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&624);
        let upper = self.values.get(&625);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_2(&mut self, v: Guid) {
        self.header_set(626, v.guid() as u32);
        self.header_set(627, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_2(&self) -> Option<Guid> {
        let lower = self.values.get(&626);
        let upper = self.values.get(&627);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_3(&mut self, v: Guid) {
        self.header_set(628, v.guid() as u32);
        self.header_set(629, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_3(&self) -> Option<Guid> {
        let lower = self.values.get(&628);
        let upper = self.values.get(&629);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_4(&mut self, v: Guid) {
        self.header_set(630, v.guid() as u32);
        self.header_set(631, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_4(&self) -> Option<Guid> {
        let lower = self.values.get(&630);
        let upper = self.values.get(&631);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_5(&mut self, v: Guid) {
        self.header_set(632, v.guid() as u32);
        self.header_set(633, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_5(&self) -> Option<Guid> {
        let lower = self.values.get(&632);
        let upper = self.values.get(&633);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_6(&mut self, v: Guid) {
        self.header_set(634, v.guid() as u32);
        self.header_set(635, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_6(&self) -> Option<Guid> {
        let lower = self.values.get(&634);
        let upper = self.values.get(&635);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_7(&mut self, v: Guid) {
        self.header_set(636, v.guid() as u32);
        self.header_set(637, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_7(&self) -> Option<Guid> {
        let lower = self.values.get(&636);
        let upper = self.values.get(&637);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_8(&mut self, v: Guid) {
        self.header_set(638, v.guid() as u32);
        self.header_set(639, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_8(&self) -> Option<Guid> {
        let lower = self.values.get(&638);
        let upper = self.values.get(&639);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_9(&mut self, v: Guid) {
        self.header_set(640, v.guid() as u32);
        self.header_set(641, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_9(&self) -> Option<Guid> {
        let lower = self.values.get(&640);
        let upper = self.values.get(&641);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_10(&mut self, v: Guid) {
        self.header_set(642, v.guid() as u32);
        self.header_set(643, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_10(&self) -> Option<Guid> {
        let lower = self.values.get(&642);
        let upper = self.values.get(&643);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_11(&mut self, v: Guid) {
        self.header_set(644, v.guid() as u32);
        self.header_set(645, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_11(&self) -> Option<Guid> {
        let lower = self.values.get(&644);
        let upper = self.values.get(&645);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_12(&mut self, v: Guid) {
        self.header_set(646, v.guid() as u32);
        self.header_set(647, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_VENDORBUYBACK_SLOT_12(&self) -> Option<Guid> {
        let lower = self.values.get(&646);
        let upper = self.values.get(&647);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_1(&mut self, v: Guid) {
        self.header_set(648, v.guid() as u32);
        self.header_set(649, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&648);
        let upper = self.values.get(&649);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_2(&mut self, v: Guid) {
        self.header_set(650, v.guid() as u32);
        self.header_set(651, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_2(&self) -> Option<Guid> {
        let lower = self.values.get(&650);
        let upper = self.values.get(&651);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_3(&mut self, v: Guid) {
        self.header_set(652, v.guid() as u32);
        self.header_set(653, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_3(&self) -> Option<Guid> {
        let lower = self.values.get(&652);
        let upper = self.values.get(&653);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_4(&mut self, v: Guid) {
        self.header_set(654, v.guid() as u32);
        self.header_set(655, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_4(&self) -> Option<Guid> {
        let lower = self.values.get(&654);
        let upper = self.values.get(&655);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_5(&mut self, v: Guid) {
        self.header_set(656, v.guid() as u32);
        self.header_set(657, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_5(&self) -> Option<Guid> {
        let lower = self.values.get(&656);
        let upper = self.values.get(&657);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_6(&mut self, v: Guid) {
        self.header_set(658, v.guid() as u32);
        self.header_set(659, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_6(&self) -> Option<Guid> {
        let lower = self.values.get(&658);
        let upper = self.values.get(&659);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_7(&mut self, v: Guid) {
        self.header_set(660, v.guid() as u32);
        self.header_set(661, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_7(&self) -> Option<Guid> {
        let lower = self.values.get(&660);
        let upper = self.values.get(&661);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_8(&mut self, v: Guid) {
        self.header_set(662, v.guid() as u32);
        self.header_set(663, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_8(&self) -> Option<Guid> {
        let lower = self.values.get(&662);
        let upper = self.values.get(&663);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_9(&mut self, v: Guid) {
        self.header_set(664, v.guid() as u32);
        self.header_set(665, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_9(&self) -> Option<Guid> {
        let lower = self.values.get(&664);
        let upper = self.values.get(&665);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_10(&mut self, v: Guid) {
        self.header_set(666, v.guid() as u32);
        self.header_set(667, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_10(&self) -> Option<Guid> {
        let lower = self.values.get(&666);
        let upper = self.values.get(&667);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_11(&mut self, v: Guid) {
        self.header_set(668, v.guid() as u32);
        self.header_set(669, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_11(&self) -> Option<Guid> {
        let lower = self.values.get(&668);
        let upper = self.values.get(&669);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_12(&mut self, v: Guid) {
        self.header_set(670, v.guid() as u32);
        self.header_set(671, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_12(&self) -> Option<Guid> {
        let lower = self.values.get(&670);
        let upper = self.values.get(&671);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_13(&mut self, v: Guid) {
        self.header_set(672, v.guid() as u32);
        self.header_set(673, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_13(&self) -> Option<Guid> {
        let lower = self.values.get(&672);
        let upper = self.values.get(&673);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_14(&mut self, v: Guid) {
        self.header_set(674, v.guid() as u32);
        self.header_set(675, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_14(&self) -> Option<Guid> {
        let lower = self.values.get(&674);
        let upper = self.values.get(&675);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_15(&mut self, v: Guid) {
        self.header_set(676, v.guid() as u32);
        self.header_set(677, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_15(&self) -> Option<Guid> {
        let lower = self.values.get(&676);
        let upper = self.values.get(&677);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_16(&mut self, v: Guid) {
        self.header_set(678, v.guid() as u32);
        self.header_set(679, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_16(&self) -> Option<Guid> {
        let lower = self.values.get(&678);
        let upper = self.values.get(&679);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_17(&mut self, v: Guid) {
        self.header_set(680, v.guid() as u32);
        self.header_set(681, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_17(&self) -> Option<Guid> {
        let lower = self.values.get(&680);
        let upper = self.values.get(&681);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_18(&mut self, v: Guid) {
        self.header_set(682, v.guid() as u32);
        self.header_set(683, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_18(&self) -> Option<Guid> {
        let lower = self.values.get(&682);
        let upper = self.values.get(&683);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_19(&mut self, v: Guid) {
        self.header_set(684, v.guid() as u32);
        self.header_set(685, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_19(&self) -> Option<Guid> {
        let lower = self.values.get(&684);
        let upper = self.values.get(&685);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_20(&mut self, v: Guid) {
        self.header_set(686, v.guid() as u32);
        self.header_set(687, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_20(&self) -> Option<Guid> {
        let lower = self.values.get(&686);
        let upper = self.values.get(&687);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_21(&mut self, v: Guid) {
        self.header_set(688, v.guid() as u32);
        self.header_set(689, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_21(&self) -> Option<Guid> {
        let lower = self.values.get(&688);
        let upper = self.values.get(&689);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_22(&mut self, v: Guid) {
        self.header_set(690, v.guid() as u32);
        self.header_set(691, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_22(&self) -> Option<Guid> {
        let lower = self.values.get(&690);
        let upper = self.values.get(&691);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_23(&mut self, v: Guid) {
        self.header_set(692, v.guid() as u32);
        self.header_set(693, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_23(&self) -> Option<Guid> {
        let lower = self.values.get(&692);
        let upper = self.values.get(&693);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_24(&mut self, v: Guid) {
        self.header_set(694, v.guid() as u32);
        self.header_set(695, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_24(&self) -> Option<Guid> {
        let lower = self.values.get(&694);
        let upper = self.values.get(&695);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_25(&mut self, v: Guid) {
        self.header_set(696, v.guid() as u32);
        self.header_set(697, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_25(&self) -> Option<Guid> {
        let lower = self.values.get(&696);
        let upper = self.values.get(&697);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_26(&mut self, v: Guid) {
        self.header_set(698, v.guid() as u32);
        self.header_set(699, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_26(&self) -> Option<Guid> {
        let lower = self.values.get(&698);
        let upper = self.values.get(&699);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_27(&mut self, v: Guid) {
        self.header_set(700, v.guid() as u32);
        self.header_set(701, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_27(&self) -> Option<Guid> {
        let lower = self.values.get(&700);
        let upper = self.values.get(&701);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_28(&mut self, v: Guid) {
        self.header_set(702, v.guid() as u32);
        self.header_set(703, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_28(&self) -> Option<Guid> {
        let lower = self.values.get(&702);
        let upper = self.values.get(&703);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_29(&mut self, v: Guid) {
        self.header_set(704, v.guid() as u32);
        self.header_set(705, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_29(&self) -> Option<Guid> {
        let lower = self.values.get(&704);
        let upper = self.values.get(&705);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_30(&mut self, v: Guid) {
        self.header_set(706, v.guid() as u32);
        self.header_set(707, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_30(&self) -> Option<Guid> {
        let lower = self.values.get(&706);
        let upper = self.values.get(&707);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_31(&mut self, v: Guid) {
        self.header_set(708, v.guid() as u32);
        self.header_set(709, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_31(&self) -> Option<Guid> {
        let lower = self.values.get(&708);
        let upper = self.values.get(&709);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_KEYRING_SLOT_32(&mut self, v: Guid) {
        self.header_set(710, v.guid() as u32);
        self.header_set(711, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_32(&self) -> Option<Guid> {
        let lower = self.values.get(&710);
        let upper = self.values.get(&711);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FARSIGHT(&mut self, v: Guid) {
        self.header_set(712, v.guid() as u32);
        self.header_set(713, (v.guid() >> 32) as u32);
    }

    pub fn player_FARSIGHT(&self) -> Option<Guid> {
        let lower = self.values.get(&712);
        let upper = self.values.get(&713);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_FIELD_COMBO_TARGET(&mut self, v: Guid) {
        self.header_set(714, v.guid() as u32);
        self.header_set(715, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_COMBO_TARGET(&self) -> Option<Guid> {
        let lower = self.values.get(&714);
        let upper = self.values.get(&715);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_XP(&mut self, v: i32) {
        self.header_set(716, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_XP(&self) -> Option<i32> {
        self.values.get(&716).map(|v| *v as i32)
    }

    pub fn set_player_NEXT_LEVEL_XP(&mut self, v: i32) {
        self.header_set(717, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_NEXT_LEVEL_XP(&self) -> Option<i32> {
        self.values.get(&717).map(|v| *v as i32)
    }

    pub fn set_player_SKILL_INFO(&mut self, skill_info: crate::vanilla::SkillInfo, index: SkillInfoIndex) {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
    }

    pub fn player_SKILL_INFO(&self, index: SkillInfoIndex) -> Option<crate::vanilla::SkillInfo> {
        crate::vanilla::SkillInfo::from_range(self.values.range(index.first()..=index.last()))
    }

    pub fn set_player_CHARACTER_POINTS1(&mut self, v: i32) {
        self.header_set(1102, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_CHARACTER_POINTS1(&self) -> Option<i32> {
        self.values.get(&1102).map(|v| *v as i32)
    }

    pub fn set_player_CHARACTER_POINTS2(&mut self, v: i32) {
        self.header_set(1103, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_CHARACTER_POINTS2(&self) -> Option<i32> {
        self.values.get(&1103).map(|v| *v as i32)
    }

    pub fn set_player_TRACK_CREATURES(&mut self, v: i32) {
        self.header_set(1104, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_TRACK_CREATURES(&self) -> Option<i32> {
        self.values.get(&1104).map(|v| *v as i32)
    }

    pub fn set_player_TRACK_RESOURCES(&mut self, v: i32) {
        self.header_set(1105, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_TRACK_RESOURCES(&self) -> Option<i32> {
        self.values.get(&1105).map(|v| *v as i32)
    }

    pub fn set_player_BLOCK_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1106, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_BLOCK_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1106).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_DODGE_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1107, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_DODGE_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1107).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_PARRY_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1108, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_PARRY_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1108).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_CRIT_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1109, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_CRIT_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1109).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_RANGED_CRIT_PERCENTAGE(&mut self, v: f32) {
        self.header_set(1110, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_RANGED_CRIT_PERCENTAGE(&self) -> Option<f32> {
        self.values.get(&1110).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_EXPLORED_ZONES_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1111, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_EXPLORED_ZONES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1111) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_REST_STATE_EXPERIENCE(&mut self, v: i32) {
        self.header_set(1175, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_REST_STATE_EXPERIENCE(&self) -> Option<i32> {
        self.values.get(&1175).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_COINAGE(&mut self, v: i32) {
        self.header_set(1176, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_COINAGE(&self) -> Option<i32> {
        self.values.get(&1176).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_POSSTAT0(&mut self, v: i32) {
        self.header_set(1177, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_POSSTAT0(&self) -> Option<i32> {
        self.values.get(&1177).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_POSSTAT1(&mut self, v: i32) {
        self.header_set(1178, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_POSSTAT1(&self) -> Option<i32> {
        self.values.get(&1178).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_POSSTAT2(&mut self, v: i32) {
        self.header_set(1179, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_POSSTAT2(&self) -> Option<i32> {
        self.values.get(&1179).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_POSSTAT3(&mut self, v: i32) {
        self.header_set(1180, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_POSSTAT3(&self) -> Option<i32> {
        self.values.get(&1180).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_POSSTAT4(&mut self, v: i32) {
        self.header_set(1181, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_POSSTAT4(&self) -> Option<i32> {
        self.values.get(&1181).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_NEGSTAT0(&mut self, v: i32) {
        self.header_set(1182, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_NEGSTAT0(&self) -> Option<i32> {
        self.values.get(&1182).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_NEGSTAT1(&mut self, v: i32) {
        self.header_set(1183, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_NEGSTAT1(&self) -> Option<i32> {
        self.values.get(&1183).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_NEGSTAT2(&mut self, v: i32) {
        self.header_set(1184, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_NEGSTAT2(&self) -> Option<i32> {
        self.values.get(&1184).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_NEGSTAT3(&mut self, v: i32) {
        self.header_set(1185, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_NEGSTAT3(&self) -> Option<i32> {
        self.values.get(&1185).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_NEGSTAT4(&mut self, v: i32) {
        self.header_set(1186, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_NEGSTAT4(&self) -> Option<i32> {
        self.values.get(&1186).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSPOSITIVE(&mut self, v: i32) {
        self.header_set(1187, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_RESISTANCEBUFFMODSPOSITIVE(&self) -> Option<i32> {
        self.values.get(&1187).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSNEGATIVE(&mut self, v: i32) {
        self.header_set(1194, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_RESISTANCEBUFFMODSNEGATIVE(&self) -> Option<i32> {
        self.values.get(&1194).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_POS(&mut self, v: i32) {
        self.header_set(1201, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_MOD_DAMAGE_DONE_POS(&self) -> Option<i32> {
        self.values.get(&1201).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_NEG(&mut self, v: i32) {
        self.header_set(1208, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_MOD_DAMAGE_DONE_NEG(&self) -> Option<i32> {
        self.values.get(&1208).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_PCT(&mut self, v: i32) {
        self.header_set(1215, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_MOD_DAMAGE_DONE_PCT(&self) -> Option<i32> {
        self.values.get(&1215).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_BYTES(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1222, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_FIELD_BYTES(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1222) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_AMMO_ID(&mut self, v: i32) {
        self.header_set(1223, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_AMMO_ID(&self) -> Option<i32> {
        self.values.get(&1223).map(|v| *v as i32)
    }

    pub fn set_player_SELF_RES_SPELL(&mut self, v: i32) {
        self.header_set(1224, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_SELF_RES_SPELL(&self) -> Option<i32> {
        self.values.get(&1224).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_PVP_MEDALS(&mut self, v: i32) {
        self.header_set(1225, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_PVP_MEDALS(&self) -> Option<i32> {
        self.values.get(&1225).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_BUYBACK_PRICE_1(&mut self, v: i32) {
        self.header_set(1226, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_BUYBACK_PRICE_1(&self) -> Option<i32> {
        self.values.get(&1226).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_BUYBACK_TIMESTAMP_1(&mut self, v: i32) {
        self.header_set(1238, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_BUYBACK_TIMESTAMP_1(&self) -> Option<i32> {
        self.values.get(&1238).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_SESSION_KILLS(&mut self, a: u16, b: u16) {
        self.header_set(1250, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_FIELD_SESSION_KILLS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1250) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_YESTERDAY_KILLS(&mut self, a: u16, b: u16) {
        self.header_set(1251, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_FIELD_YESTERDAY_KILLS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1251) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_LAST_WEEK_KILLS(&mut self, a: u16, b: u16) {
        self.header_set(1252, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_FIELD_LAST_WEEK_KILLS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1252) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_THIS_WEEK_KILLS(&mut self, a: u16, b: u16) {
        self.header_set(1253, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_FIELD_THIS_WEEK_KILLS(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1253) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_THIS_WEEK_CONTRIBUTION(&mut self, v: i32) {
        self.header_set(1254, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_THIS_WEEK_CONTRIBUTION(&self) -> Option<i32> {
        self.values.get(&1254).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_LIFETIME_HONORBALE_KILLS(&mut self, v: i32) {
        self.header_set(1255, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_LIFETIME_HONORBALE_KILLS(&self) -> Option<i32> {
        self.values.get(&1255).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_LIFETIME_DISHONORBALE_KILLS(&mut self, v: i32) {
        self.header_set(1256, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_LIFETIME_DISHONORBALE_KILLS(&self) -> Option<i32> {
        self.values.get(&1256).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_YESTERDAY_CONTRIBUTION(&mut self, v: i32) {
        self.header_set(1257, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_YESTERDAY_CONTRIBUTION(&self) -> Option<i32> {
        self.values.get(&1257).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_LAST_WEEK_CONTRIBUTION(&mut self, v: i32) {
        self.header_set(1258, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_LAST_WEEK_CONTRIBUTION(&self) -> Option<i32> {
        self.values.get(&1258).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_LAST_WEEK_RANK(&mut self, v: i32) {
        self.header_set(1259, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_LAST_WEEK_RANK(&self) -> Option<i32> {
        self.values.get(&1259).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_BYTES2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1260, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_FIELD_BYTES2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1260) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_FIELD_WATCHED_FACTION_INDEX(&mut self, v: i32) {
        self.header_set(1261, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_WATCHED_FACTION_INDEX(&self) -> Option<i32> {
        self.values.get(&1261).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_COMBAT_RATING_1(&mut self, v: i32) {
        self.header_set(1262, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_FIELD_COMBAT_RATING_1(&self) -> Option<i32> {
        self.values.get(&1262).map(|v| *v as i32)
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

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
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

    pub fn set_gameobject_CREATED_BY(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn gameobject_CREATED_BY(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_gameobject_DISPLAYID(&mut self, v: i32) {
        self.header_set(8, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_DISPLAYID(&self) -> Option<i32> {
        self.values.get(&8).map(|v| *v as i32)
    }

    pub fn set_gameobject_FLAGS(&mut self, v: i32) {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_FLAGS(&self) -> Option<i32> {
        self.values.get(&9).map(|v| *v as i32)
    }

    pub fn set_gameobject_ROTATION(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_ROTATION(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_STATE(&mut self, v: i32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_STATE(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_gameobject_POS_X(&mut self, v: f32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_POS_X(&self) -> Option<f32> {
        self.values.get(&15).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_POS_Y(&mut self, v: f32) {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_POS_Y(&self) -> Option<f32> {
        self.values.get(&16).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_POS_Z(&mut self, v: f32) {
        self.header_set(17, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_POS_Z(&self) -> Option<f32> {
        self.values.get(&17).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_FACING(&mut self, v: f32) {
        self.header_set(18, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_FACING(&self) -> Option<f32> {
        self.values.get(&18).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_DYN_FLAGS(&mut self, v: i32) {
        self.header_set(19, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_DYN_FLAGS(&self) -> Option<i32> {
        self.values.get(&19).map(|v| *v as i32)
    }

    pub fn set_gameobject_FACTION(&mut self, v: i32) {
        self.header_set(20, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_FACTION(&self) -> Option<i32> {
        self.values.get(&20).map(|v| *v as i32)
    }

    pub fn set_gameobject_TYPE_ID(&mut self, v: i32) {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_TYPE_ID(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_gameobject_LEVEL(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_LEVEL(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_gameobject_ARTKIT(&mut self, v: i32) {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_ARTKIT(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_gameobject_ANIMPROGRESS(&mut self, v: i32) {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_ANIMPROGRESS(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
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

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
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
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
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

    pub fn set_dynamicobject_POS_X(&mut self, v: f32) {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_POS_X(&self) -> Option<f32> {
        self.values.get(&11).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_POS_Y(&mut self, v: f32) {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_POS_Y(&self) -> Option<f32> {
        self.values.get(&12).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_POS_Z(&mut self, v: f32) {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_POS_Z(&self) -> Option<f32> {
        self.values.get(&13).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_FACING(&mut self, v: f32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_FACING(&self) -> Option<f32> {
        self.values.get(&14).map(|v| f32::from_le_bytes(v.to_le_bytes()))
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

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
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

    pub fn set_corpse_OWNER(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn corpse_OWNER(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_corpse_FACING(&mut self, v: f32) {
        self.header_set(8, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_FACING(&self) -> Option<f32> {
        self.values.get(&8).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_POS_X(&mut self, v: f32) {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_POS_X(&self) -> Option<f32> {
        self.values.get(&9).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_POS_Y(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_POS_Y(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_POS_Z(&mut self, v: f32) {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_POS_Z(&self) -> Option<f32> {
        self.values.get(&11).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_DISPLAY_ID(&mut self, v: i32) {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_DISPLAY_ID(&self) -> Option<i32> {
        self.values.get(&12).map(|v| *v as i32)
    }

    pub fn set_corpse_ITEM(&mut self, v: i32) {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_ITEM(&self) -> Option<i32> {
        self.values.get(&13).map(|v| *v as i32)
    }

    pub fn set_corpse_BYTES_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(32, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_BYTES_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&32) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_BYTES_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(33, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_BYTES_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&33) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_GUILD(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_GUILD(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_corpse_FLAGS(&mut self, v: i32) {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_FLAGS(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_corpse_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&36).map(|v| *v as i32)
    }

}

