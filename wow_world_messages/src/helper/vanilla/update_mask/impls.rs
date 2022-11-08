use crate::Guid;
use std::convert::TryInto;
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

    pub fn set_item_ENCHANTMENT(mut self, v: i32) -> Self {
        self.header_set(22, v as u32);
        self
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(43, v as u32);
        self
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(44, v as u32);
        self
    }

    pub fn set_item_ITEM_TEXT_ID(mut self, v: i32) -> Self {
        self.header_set(45, v as u32);
        self
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(46, v as u32);
        self
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(47, v as u32);
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

    pub fn set_item_ENCHANTMENT(mut self, v: i32) -> Self {
        self.header_set(22, v as u32);
        self
    }

    pub fn set_item_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(43, v as u32);
        self
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(44, v as u32);
        self
    }

    pub fn set_item_ITEM_TEXT_ID(mut self, v: i32) -> Self {
        self.header_set(45, v as u32);
        self
    }

    pub fn set_item_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(46, v as u32);
        self
    }

    pub fn set_item_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(47, v as u32);
        self
    }

    pub fn set_container_NUM_SLOTS(mut self, v: i32) -> Self {
        self.header_set(48, v as u32);
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
        self.header_set(22, v as u32);
        self
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(23, v as u32);
        self
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(24, v as u32);
        self
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(25, v as u32);
        self
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(26, v as u32);
        self
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(27, v as u32);
        self
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(28, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(29, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(30, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(31, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(32, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(33, v as u32);
        self
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(34, v as u32);
        self
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(35, v as u32);
        self
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(mut self, v: i32) -> Self {
        self.header_set(37, v as u32);
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(46, v as u32);
        self
    }

    pub fn set_unit_AURA(mut self, v: i32) -> Self {
        self.header_set(47, v as u32);
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
        self.header_set(125, v as u32);
        self
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(126, v as u32);
        self
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(128, v as u32);
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
        self.header_set(131, v as u32);
        self
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(132, v as u32);
        self
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(133, v as u32);
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
        self.header_set(139, v as u32);
        self
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(140, v as u32);
        self
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(141, v as u32);
        self
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(142, v as u32);
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(143, v as u32);
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(144, v as u32);
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(146, v as u32);
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(147, v as u32);
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(148, v as u32);
        self
    }

    pub fn set_unit_TRAINING_POINTS(mut self, a: u16, b: u16) -> Self {
        self.header_set(149, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(150, v as u32);
        self
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(151, v as u32);
        self
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(152, v as u32);
        self
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(153, v as u32);
        self
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(154, v as u32);
        self
    }

    pub fn set_unit_NORMAL_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(155, v as u32);
        self
    }

    pub fn set_unit_HOLY_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(156, v as u32);
        self
    }

    pub fn set_unit_FIRE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(157, v as u32);
        self
    }

    pub fn set_unit_NATURE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(158, v as u32);
        self
    }

    pub fn set_unit_FROST_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(159, v as u32);
        self
    }

    pub fn set_unit_SHADOW_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(160, v as u32);
        self
    }

    pub fn set_unit_ARCANE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(161, v as u32);
        self
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(162, v as u32);
        self
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(163, v as u32);
        self
    }

    pub fn set_unit_BYTES_2(mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) -> Self {
        self.header_set(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
        self
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(165, v as u32);
        self
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(166, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(168, v as u32);
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(169, (a as u32) << 16 | b as u32);
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
        self.header_set(173, v as u32);
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
        self.header_set(22, v as u32);
        self
    }

    pub fn set_unit_POWER1(mut self, v: i32) -> Self {
        self.header_set(23, v as u32);
        self
    }

    pub fn set_unit_POWER2(mut self, v: i32) -> Self {
        self.header_set(24, v as u32);
        self
    }

    pub fn set_unit_POWER3(mut self, v: i32) -> Self {
        self.header_set(25, v as u32);
        self
    }

    pub fn set_unit_POWER4(mut self, v: i32) -> Self {
        self.header_set(26, v as u32);
        self
    }

    pub fn set_unit_POWER5(mut self, v: i32) -> Self {
        self.header_set(27, v as u32);
        self
    }

    pub fn set_unit_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(28, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(29, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(30, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(31, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(32, v as u32);
        self
    }

    pub fn set_unit_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(33, v as u32);
        self
    }

    pub fn set_unit_LEVEL(mut self, v: i32) -> Self {
        self.header_set(34, v as u32);
        self
    }

    pub fn set_unit_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(35, v as u32);
        self
    }

    pub fn set_unit_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(mut self, v: i32) -> Self {
        self.header_set(37, v as u32);
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FLAGS(mut self, v: i32) -> Self {
        self.header_set(46, v as u32);
        self
    }

    pub fn set_unit_AURA(mut self, v: i32) -> Self {
        self.header_set(47, v as u32);
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
        self.header_set(125, v as u32);
        self
    }

    pub fn set_unit_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(126, v as u32);
        self
    }

    pub fn set_unit_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(128, v as u32);
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
        self.header_set(131, v as u32);
        self
    }

    pub fn set_unit_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(132, v as u32);
        self
    }

    pub fn set_unit_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(133, v as u32);
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
        self.header_set(139, v as u32);
        self
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(140, v as u32);
        self
    }

    pub fn set_unit_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(141, v as u32);
        self
    }

    pub fn set_unit_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(142, v as u32);
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(143, v as u32);
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(144, v as u32);
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(146, v as u32);
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(147, v as u32);
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(148, v as u32);
        self
    }

    pub fn set_unit_TRAINING_POINTS(mut self, a: u16, b: u16) -> Self {
        self.header_set(149, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_unit_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(150, v as u32);
        self
    }

    pub fn set_unit_AGILITY(mut self, v: i32) -> Self {
        self.header_set(151, v as u32);
        self
    }

    pub fn set_unit_STAMINA(mut self, v: i32) -> Self {
        self.header_set(152, v as u32);
        self
    }

    pub fn set_unit_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(153, v as u32);
        self
    }

    pub fn set_unit_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(154, v as u32);
        self
    }

    pub fn set_unit_NORMAL_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(155, v as u32);
        self
    }

    pub fn set_unit_HOLY_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(156, v as u32);
        self
    }

    pub fn set_unit_FIRE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(157, v as u32);
        self
    }

    pub fn set_unit_NATURE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(158, v as u32);
        self
    }

    pub fn set_unit_FROST_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(159, v as u32);
        self
    }

    pub fn set_unit_SHADOW_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(160, v as u32);
        self
    }

    pub fn set_unit_ARCANE_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(161, v as u32);
        self
    }

    pub fn set_unit_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(162, v as u32);
        self
    }

    pub fn set_unit_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(163, v as u32);
        self
    }

    pub fn set_unit_BYTES_2(mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) -> Self {
        self.header_set(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
        self
    }

    pub fn set_unit_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(165, v as u32);
        self
    }

    pub fn set_unit_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(166, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(168, v as u32);
        self
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(mut self, a: u16, b: u16) -> Self {
        self.header_set(169, (a as u32) << 16 | b as u32);
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
        self.header_set(173, v as u32);
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
        self.header_set(190, v as u32);
        self
    }

    pub fn set_player_GUILDID(mut self, v: i32) -> Self {
        self.header_set(191, v as u32);
        self
    }

    pub fn set_player_GUILDRANK(mut self, v: i32) -> Self {
        self.header_set(192, v as u32);
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
        self.header_set(196, v as u32);
        self
    }

    pub fn set_player_GUILD_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(197, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_1(mut self, v: i32) -> Self {
        self.header_set(198, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_2(mut self, v: i32) -> Self {
        self.header_set(199, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_1(mut self, v: i32) -> Self {
        self.header_set(201, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_2(mut self, v: i32) -> Self {
        self.header_set(202, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_1(mut self, v: i32) -> Self {
        self.header_set(204, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_2(mut self, v: i32) -> Self {
        self.header_set(205, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_1(mut self, v: i32) -> Self {
        self.header_set(207, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_2(mut self, v: i32) -> Self {
        self.header_set(208, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_1(mut self, v: i32) -> Self {
        self.header_set(210, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_2(mut self, v: i32) -> Self {
        self.header_set(211, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_1(mut self, v: i32) -> Self {
        self.header_set(213, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_2(mut self, v: i32) -> Self {
        self.header_set(214, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_1(mut self, v: i32) -> Self {
        self.header_set(216, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_2(mut self, v: i32) -> Self {
        self.header_set(217, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_1(mut self, v: i32) -> Self {
        self.header_set(219, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_2(mut self, v: i32) -> Self {
        self.header_set(220, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_1(mut self, v: i32) -> Self {
        self.header_set(222, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_2(mut self, v: i32) -> Self {
        self.header_set(223, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_1(mut self, v: i32) -> Self {
        self.header_set(225, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_2(mut self, v: i32) -> Self {
        self.header_set(226, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_1(mut self, v: i32) -> Self {
        self.header_set(228, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_2(mut self, v: i32) -> Self {
        self.header_set(229, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_1(mut self, v: i32) -> Self {
        self.header_set(231, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_2(mut self, v: i32) -> Self {
        self.header_set(232, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_1(mut self, v: i32) -> Self {
        self.header_set(234, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_2(mut self, v: i32) -> Self {
        self.header_set(235, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_1(mut self, v: i32) -> Self {
        self.header_set(237, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_2(mut self, v: i32) -> Self {
        self.header_set(238, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_1(mut self, v: i32) -> Self {
        self.header_set(240, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_2(mut self, v: i32) -> Self {
        self.header_set(241, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_1(mut self, v: i32) -> Self {
        self.header_set(243, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_2(mut self, v: i32) -> Self {
        self.header_set(244, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_1(mut self, v: i32) -> Self {
        self.header_set(246, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_2(mut self, v: i32) -> Self {
        self.header_set(247, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_1(mut self, v: i32) -> Self {
        self.header_set(249, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_2(mut self, v: i32) -> Self {
        self.header_set(250, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_1(mut self, v: i32) -> Self {
        self.header_set(252, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_2(mut self, v: i32) -> Self {
        self.header_set(253, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_1(mut self, v: i32) -> Self {
        self.header_set(255, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_2(mut self, v: i32) -> Self {
        self.header_set(256, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(258, v.guid() as u32);
        self.header_set(259, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_0(mut self, v: i32) -> Self {
        self.header_set(260, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(268, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(270, v.guid() as u32);
        self.header_set(271, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_0(mut self, v: i32) -> Self {
        self.header_set(272, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(280, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(282, v.guid() as u32);
        self.header_set(283, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_0(mut self, v: i32) -> Self {
        self.header_set(284, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(292, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(294, v.guid() as u32);
        self.header_set(295, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_0(mut self, v: i32) -> Self {
        self.header_set(296, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(304, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(306, v.guid() as u32);
        self.header_set(307, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_0(mut self, v: i32) -> Self {
        self.header_set(308, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(316, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(318, v.guid() as u32);
        self.header_set(319, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_0(mut self, v: i32) -> Self {
        self.header_set(320, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(328, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(330, v.guid() as u32);
        self.header_set(331, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_0(mut self, v: i32) -> Self {
        self.header_set(332, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(340, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(342, v.guid() as u32);
        self.header_set(343, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_0(mut self, v: i32) -> Self {
        self.header_set(344, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(352, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(354, v.guid() as u32);
        self.header_set(355, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_0(mut self, v: i32) -> Self {
        self.header_set(356, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(364, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(366, v.guid() as u32);
        self.header_set(367, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_0(mut self, v: i32) -> Self {
        self.header_set(368, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(376, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(378, v.guid() as u32);
        self.header_set(379, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_0(mut self, v: i32) -> Self {
        self.header_set(380, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(388, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(390, v.guid() as u32);
        self.header_set(391, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_0(mut self, v: i32) -> Self {
        self.header_set(392, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(400, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(402, v.guid() as u32);
        self.header_set(403, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_0(mut self, v: i32) -> Self {
        self.header_set(404, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(412, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(414, v.guid() as u32);
        self.header_set(415, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_0(mut self, v: i32) -> Self {
        self.header_set(416, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(424, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(426, v.guid() as u32);
        self.header_set(427, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_0(mut self, v: i32) -> Self {
        self.header_set(428, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(436, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(438, v.guid() as u32);
        self.header_set(439, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_0(mut self, v: i32) -> Self {
        self.header_set(440, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(448, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(450, v.guid() as u32);
        self.header_set(451, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_0(mut self, v: i32) -> Self {
        self.header_set(452, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(460, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(462, v.guid() as u32);
        self.header_set(463, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_0(mut self, v: i32) -> Self {
        self.header_set(464, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(472, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(474, v.guid() as u32);
        self.header_set(475, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_0(mut self, v: i32) -> Self {
        self.header_set(476, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_PROPERTIES(mut self, a: u16, b: u16) -> Self {
        self.header_set(484, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_HEAD(mut self, v: Guid) -> Self {
        self.header_set(486, v.guid() as u32);
        self.header_set(487, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(532, v.guid() as u32);
        self.header_set(533, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(564, v.guid() as u32);
        self.header_set(565, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(612, v.guid() as u32);
        self.header_set(613, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(624, v.guid() as u32);
        self.header_set(625, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(648, v.guid() as u32);
        self.header_set(649, (v.guid() >> 32) as u32);
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
        self.header_set(716, v as u32);
        self
    }

    pub fn set_player_NEXT_LEVEL_XP(mut self, v: i32) -> Self {
        self.header_set(717, v as u32);
        self
    }

    pub fn set_player_SKILL_INFO_1_1(mut self, v: i32) -> Self {
        self.header_set(718, v as u32);
        self
    }

    pub fn set_player_SKILL_INFO_1_2(mut self, a: u16, b: u16) -> Self {
        self.header_set(719, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_SKILL_INFO_1_3(mut self, v: i32) -> Self {
        self.header_set(720, v as u32);
        self
    }

    pub fn set_player_CHARACTER_POINTS1(mut self, v: i32) -> Self {
        self.header_set(1102, v as u32);
        self
    }

    pub fn set_player_CHARACTER_POINTS2(mut self, v: i32) -> Self {
        self.header_set(1103, v as u32);
        self
    }

    pub fn set_player_TRACK_CREATURES(mut self, v: i32) -> Self {
        self.header_set(1104, v as u32);
        self
    }

    pub fn set_player_TRACK_RESOURCES(mut self, v: i32) -> Self {
        self.header_set(1105, v as u32);
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
        self.header_set(1175, v as u32);
        self
    }

    pub fn set_player_FIELD_COINAGE(mut self, v: i32) -> Self {
        self.header_set(1176, v as u32);
        self
    }

    pub fn set_player_FIELD_POSSTAT0(mut self, v: i32) -> Self {
        self.header_set(1177, v as u32);
        self
    }

    pub fn set_player_FIELD_POSSTAT1(mut self, v: i32) -> Self {
        self.header_set(1178, v as u32);
        self
    }

    pub fn set_player_FIELD_POSSTAT2(mut self, v: i32) -> Self {
        self.header_set(1179, v as u32);
        self
    }

    pub fn set_player_FIELD_POSSTAT3(mut self, v: i32) -> Self {
        self.header_set(1180, v as u32);
        self
    }

    pub fn set_player_FIELD_POSSTAT4(mut self, v: i32) -> Self {
        self.header_set(1181, v as u32);
        self
    }

    pub fn set_player_FIELD_NEGSTAT0(mut self, v: i32) -> Self {
        self.header_set(1182, v as u32);
        self
    }

    pub fn set_player_FIELD_NEGSTAT1(mut self, v: i32) -> Self {
        self.header_set(1183, v as u32);
        self
    }

    pub fn set_player_FIELD_NEGSTAT2(mut self, v: i32) -> Self {
        self.header_set(1184, v as u32);
        self
    }

    pub fn set_player_FIELD_NEGSTAT3(mut self, v: i32) -> Self {
        self.header_set(1185, v as u32);
        self
    }

    pub fn set_player_FIELD_NEGSTAT4(mut self, v: i32) -> Self {
        self.header_set(1186, v as u32);
        self
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSPOSITIVE(mut self, v: i32) -> Self {
        self.header_set(1187, v as u32);
        self
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSNEGATIVE(mut self, v: i32) -> Self {
        self.header_set(1194, v as u32);
        self
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_POS(mut self, v: i32) -> Self {
        self.header_set(1201, v as u32);
        self
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_NEG(mut self, v: i32) -> Self {
        self.header_set(1208, v as u32);
        self
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_PCT(mut self, v: i32) -> Self {
        self.header_set(1215, v as u32);
        self
    }

    pub fn set_player_FIELD_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1222, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_AMMO_ID(mut self, v: i32) -> Self {
        self.header_set(1223, v as u32);
        self
    }

    pub fn set_player_SELF_RES_SPELL(mut self, v: i32) -> Self {
        self.header_set(1224, v as u32);
        self
    }

    pub fn set_player_FIELD_PVP_MEDALS(mut self, v: i32) -> Self {
        self.header_set(1225, v as u32);
        self
    }

    pub fn set_player_FIELD_BUYBACK_PRICE_1(mut self, v: i32) -> Self {
        self.header_set(1226, v as u32);
        self
    }

    pub fn set_player_FIELD_BUYBACK_TIMESTAMP_1(mut self, v: i32) -> Self {
        self.header_set(1238, v as u32);
        self
    }

    pub fn set_player_FIELD_SESSION_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1250, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_FIELD_YESTERDAY_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1251, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_FIELD_LAST_WEEK_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1252, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_FIELD_THIS_WEEK_KILLS(mut self, a: u16, b: u16) -> Self {
        self.header_set(1253, (a as u32) << 16 | b as u32);
        self
    }

    pub fn set_player_FIELD_THIS_WEEK_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1254, v as u32);
        self
    }

    pub fn set_player_FIELD_LIFETIME_HONORBALE_KILLS(mut self, v: i32) -> Self {
        self.header_set(1255, v as u32);
        self
    }

    pub fn set_player_FIELD_LIFETIME_DISHONORBALE_KILLS(mut self, v: i32) -> Self {
        self.header_set(1256, v as u32);
        self
    }

    pub fn set_player_FIELD_YESTERDAY_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1257, v as u32);
        self
    }

    pub fn set_player_FIELD_LAST_WEEK_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1258, v as u32);
        self
    }

    pub fn set_player_FIELD_LAST_WEEK_RANK(mut self, v: i32) -> Self {
        self.header_set(1259, v as u32);
        self
    }

    pub fn set_player_FIELD_BYTES2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1260, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_FIELD_WATCHED_FACTION_INDEX(mut self, v: i32) -> Self {
        self.header_set(1261, v as u32);
        self
    }

    pub fn set_player_FIELD_COMBAT_RATING_1(mut self, v: i32) -> Self {
        self.header_set(1262, v as u32);
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
        self.header_set(3, v as u32);
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
        self.header_set(8, v as u32);
        self
    }

    pub fn set_gameobject_FLAGS(mut self, v: i32) -> Self {
        self.header_set(9, v as u32);
        self
    }

    pub fn set_gameobject_ROTATION(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_STATE(mut self, v: i32) -> Self {
        self.header_set(14, v as u32);
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
        self.header_set(19, v as u32);
        self
    }

    pub fn set_gameobject_FACTION(mut self, v: i32) -> Self {
        self.header_set(20, v as u32);
        self
    }

    pub fn set_gameobject_TYPE_ID(mut self, v: i32) -> Self {
        self.header_set(21, v as u32);
        self
    }

    pub fn set_gameobject_LEVEL(mut self, v: i32) -> Self {
        self.header_set(22, v as u32);
        self
    }

    pub fn set_gameobject_ARTKIT(mut self, v: i32) -> Self {
        self.header_set(23, v as u32);
        self
    }

    pub fn set_gameobject_ANIMPROGRESS(mut self, v: i32) -> Self {
        self.header_set(24, v as u32);
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
        self.header_set(3, v as u32);
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
        self.header_set(9, v as u32);
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
        self.header_set(3, v as u32);
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
        self.header_set(12, v as u32);
        self
    }

    pub fn set_corpse_ITEM(mut self, v: i32) -> Self {
        self.header_set(13, v as u32);
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
        self.header_set(34, v as u32);
        self
    }

    pub fn set_corpse_FLAGS(mut self, v: i32) -> Self {
        self.header_set(35, v as u32);
        self
    }

    pub fn set_corpse_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(36, v as u32);
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

    pub fn set_item_ENCHANTMENT(&mut self, v: i32) {
        self.header_set(22, v as u32);
    }

    pub fn item_ENCHANTMENT(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_PROPERTY_SEED(&mut self, v: i32) {
        self.header_set(43, v as u32);
    }

    pub fn item_PROPERTY_SEED(&self) -> Option<i32> {
        self.values.get(&43).map(|v| *v as i32)
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(&mut self, v: i32) {
        self.header_set(44, v as u32);
    }

    pub fn item_RANDOM_PROPERTIES_ID(&self) -> Option<i32> {
        self.values.get(&44).map(|v| *v as i32)
    }

    pub fn set_item_ITEM_TEXT_ID(&mut self, v: i32) {
        self.header_set(45, v as u32);
    }

    pub fn item_ITEM_TEXT_ID(&self) -> Option<i32> {
        self.values.get(&45).map(|v| *v as i32)
    }

    pub fn set_item_DURABILITY(&mut self, v: i32) {
        self.header_set(46, v as u32);
    }

    pub fn item_DURABILITY(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_item_MAXDURABILITY(&mut self, v: i32) {
        self.header_set(47, v as u32);
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

    pub fn set_item_ENCHANTMENT(&mut self, v: i32) {
        self.header_set(22, v as u32);
    }

    pub fn item_ENCHANTMENT(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_PROPERTY_SEED(&mut self, v: i32) {
        self.header_set(43, v as u32);
    }

    pub fn item_PROPERTY_SEED(&self) -> Option<i32> {
        self.values.get(&43).map(|v| *v as i32)
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(&mut self, v: i32) {
        self.header_set(44, v as u32);
    }

    pub fn item_RANDOM_PROPERTIES_ID(&self) -> Option<i32> {
        self.values.get(&44).map(|v| *v as i32)
    }

    pub fn set_item_ITEM_TEXT_ID(&mut self, v: i32) {
        self.header_set(45, v as u32);
    }

    pub fn item_ITEM_TEXT_ID(&self) -> Option<i32> {
        self.values.get(&45).map(|v| *v as i32)
    }

    pub fn set_item_DURABILITY(&mut self, v: i32) {
        self.header_set(46, v as u32);
    }

    pub fn item_DURABILITY(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_item_MAXDURABILITY(&mut self, v: i32) {
        self.header_set(47, v as u32);
    }

    pub fn item_MAXDURABILITY(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
    }

    pub fn set_container_NUM_SLOTS(&mut self, v: i32) {
        self.header_set(48, v as u32);
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
        self.header_set(22, v as u32);
    }

    pub fn unit_HEALTH(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_POWER1(&mut self, v: i32) {
        self.header_set(23, v as u32);
    }

    pub fn unit_POWER1(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_unit_POWER2(&mut self, v: i32) {
        self.header_set(24, v as u32);
    }

    pub fn unit_POWER2(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_POWER3(&mut self, v: i32) {
        self.header_set(25, v as u32);
    }

    pub fn unit_POWER3(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_POWER4(&mut self, v: i32) {
        self.header_set(26, v as u32);
    }

    pub fn unit_POWER4(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_POWER5(&mut self, v: i32) {
        self.header_set(27, v as u32);
    }

    pub fn unit_POWER5(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_MAXHEALTH(&mut self, v: i32) {
        self.header_set(28, v as u32);
    }

    pub fn unit_MAXHEALTH(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER1(&mut self, v: i32) {
        self.header_set(29, v as u32);
    }

    pub fn unit_MAXPOWER1(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER2(&mut self, v: i32) {
        self.header_set(30, v as u32);
    }

    pub fn unit_MAXPOWER2(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER3(&mut self, v: i32) {
        self.header_set(31, v as u32);
    }

    pub fn unit_MAXPOWER3(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER4(&mut self, v: i32) {
        self.header_set(32, v as u32);
    }

    pub fn unit_MAXPOWER4(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER5(&mut self, v: i32) {
        self.header_set(33, v as u32);
    }

    pub fn unit_MAXPOWER5(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_LEVEL(&mut self, v: i32) {
        self.header_set(34, v as u32);
    }

    pub fn unit_LEVEL(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_FACTIONTEMPLATE(&mut self, v: i32) {
        self.header_set(35, v as u32);
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
        self.header_set(37, v as u32);
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
        self.header_set(46, v as u32);
    }

    pub fn unit_FLAGS(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_unit_AURA(&mut self, v: i32) {
        self.header_set(47, v as u32);
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
        self.header_set(125, v as u32);
    }

    pub fn unit_AURASTATE(&self) -> Option<i32> {
        self.values.get(&125).map(|v| *v as i32)
    }

    pub fn set_unit_BASEATTACKTIME(&mut self, v: i32) {
        self.header_set(126, v as u32);
    }

    pub fn unit_BASEATTACKTIME(&self) -> Option<i32> {
        self.values.get(&126).map(|v| *v as i32)
    }

    pub fn set_unit_RANGEDATTACKTIME(&mut self, v: i32) {
        self.header_set(128, v as u32);
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
        self.header_set(131, v as u32);
    }

    pub fn unit_DISPLAYID(&self) -> Option<i32> {
        self.values.get(&131).map(|v| *v as i32)
    }

    pub fn set_unit_NATIVEDISPLAYID(&mut self, v: i32) {
        self.header_set(132, v as u32);
    }

    pub fn unit_NATIVEDISPLAYID(&self) -> Option<i32> {
        self.values.get(&132).map(|v| *v as i32)
    }

    pub fn set_unit_MOUNTDISPLAYID(&mut self, v: i32) {
        self.header_set(133, v as u32);
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
        self.header_set(139, v as u32);
    }

    pub fn unit_PETNUMBER(&self) -> Option<i32> {
        self.values.get(&139).map(|v| *v as i32)
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(&mut self, v: i32) {
        self.header_set(140, v as u32);
    }

    pub fn unit_PET_NAME_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&140).map(|v| *v as i32)
    }

    pub fn set_unit_PETEXPERIENCE(&mut self, v: i32) {
        self.header_set(141, v as u32);
    }

    pub fn unit_PETEXPERIENCE(&self) -> Option<i32> {
        self.values.get(&141).map(|v| *v as i32)
    }

    pub fn set_unit_PETNEXTLEVELEXP(&mut self, v: i32) {
        self.header_set(142, v as u32);
    }

    pub fn unit_PETNEXTLEVELEXP(&self) -> Option<i32> {
        self.values.get(&142).map(|v| *v as i32)
    }

    pub fn set_unit_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(143, v as u32);
    }

    pub fn unit_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&143).map(|v| *v as i32)
    }

    pub fn set_unit_CHANNEL_SPELL(&mut self, v: i32) {
        self.header_set(144, v as u32);
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
        self.header_set(146, v as u32);
    }

    pub fn unit_CREATED_BY_SPELL(&self) -> Option<i32> {
        self.values.get(&146).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_FLAGS(&mut self, v: i32) {
        self.header_set(147, v as u32);
    }

    pub fn unit_NPC_FLAGS(&self) -> Option<i32> {
        self.values.get(&147).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_EMOTESTATE(&mut self, v: i32) {
        self.header_set(148, v as u32);
    }

    pub fn unit_NPC_EMOTESTATE(&self) -> Option<i32> {
        self.values.get(&148).map(|v| *v as i32)
    }

    pub fn set_unit_TRAINING_POINTS(&mut self, a: u16, b: u16) {
        self.header_set(149, (a as u32) << 16 | b as u32);
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
        self.header_set(150, v as u32);
    }

    pub fn unit_STRENGTH(&self) -> Option<i32> {
        self.values.get(&150).map(|v| *v as i32)
    }

    pub fn set_unit_AGILITY(&mut self, v: i32) {
        self.header_set(151, v as u32);
    }

    pub fn unit_AGILITY(&self) -> Option<i32> {
        self.values.get(&151).map(|v| *v as i32)
    }

    pub fn set_unit_STAMINA(&mut self, v: i32) {
        self.header_set(152, v as u32);
    }

    pub fn unit_STAMINA(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_unit_INTELLECT(&mut self, v: i32) {
        self.header_set(153, v as u32);
    }

    pub fn unit_INTELLECT(&self) -> Option<i32> {
        self.values.get(&153).map(|v| *v as i32)
    }

    pub fn set_unit_SPIRIT(&mut self, v: i32) {
        self.header_set(154, v as u32);
    }

    pub fn unit_SPIRIT(&self) -> Option<i32> {
        self.values.get(&154).map(|v| *v as i32)
    }

    pub fn set_unit_NORMAL_RESISTANCE(&mut self, v: i32) {
        self.header_set(155, v as u32);
    }

    pub fn unit_NORMAL_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&155).map(|v| *v as i32)
    }

    pub fn set_unit_HOLY_RESISTANCE(&mut self, v: i32) {
        self.header_set(156, v as u32);
    }

    pub fn unit_HOLY_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&156).map(|v| *v as i32)
    }

    pub fn set_unit_FIRE_RESISTANCE(&mut self, v: i32) {
        self.header_set(157, v as u32);
    }

    pub fn unit_FIRE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&157).map(|v| *v as i32)
    }

    pub fn set_unit_NATURE_RESISTANCE(&mut self, v: i32) {
        self.header_set(158, v as u32);
    }

    pub fn unit_NATURE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&158).map(|v| *v as i32)
    }

    pub fn set_unit_FROST_RESISTANCE(&mut self, v: i32) {
        self.header_set(159, v as u32);
    }

    pub fn unit_FROST_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&159).map(|v| *v as i32)
    }

    pub fn set_unit_SHADOW_RESISTANCE(&mut self, v: i32) {
        self.header_set(160, v as u32);
    }

    pub fn unit_SHADOW_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&160).map(|v| *v as i32)
    }

    pub fn set_unit_ARCANE_RESISTANCE(&mut self, v: i32) {
        self.header_set(161, v as u32);
    }

    pub fn unit_ARCANE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&161).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_MANA(&mut self, v: i32) {
        self.header_set(162, v as u32);
    }

    pub fn unit_BASE_MANA(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_HEALTH(&mut self, v: i32) {
        self.header_set(163, v as u32);
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
        self.header_set(165, v as u32);
    }

    pub fn unit_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&165).map(|v| *v as i32)
    }

    pub fn set_unit_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(166, (a as u32) << 16 | b as u32);
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
        self.header_set(168, v as u32);
    }

    pub fn unit_RANGED_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(169, (a as u32) << 16 | b as u32);
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
        self.header_set(173, v as u32);
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
        self.header_set(22, v as u32);
    }

    pub fn unit_HEALTH(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_POWER1(&mut self, v: i32) {
        self.header_set(23, v as u32);
    }

    pub fn unit_POWER1(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_unit_POWER2(&mut self, v: i32) {
        self.header_set(24, v as u32);
    }

    pub fn unit_POWER2(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_POWER3(&mut self, v: i32) {
        self.header_set(25, v as u32);
    }

    pub fn unit_POWER3(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_POWER4(&mut self, v: i32) {
        self.header_set(26, v as u32);
    }

    pub fn unit_POWER4(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_POWER5(&mut self, v: i32) {
        self.header_set(27, v as u32);
    }

    pub fn unit_POWER5(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_MAXHEALTH(&mut self, v: i32) {
        self.header_set(28, v as u32);
    }

    pub fn unit_MAXHEALTH(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER1(&mut self, v: i32) {
        self.header_set(29, v as u32);
    }

    pub fn unit_MAXPOWER1(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER2(&mut self, v: i32) {
        self.header_set(30, v as u32);
    }

    pub fn unit_MAXPOWER2(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER3(&mut self, v: i32) {
        self.header_set(31, v as u32);
    }

    pub fn unit_MAXPOWER3(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER4(&mut self, v: i32) {
        self.header_set(32, v as u32);
    }

    pub fn unit_MAXPOWER4(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_MAXPOWER5(&mut self, v: i32) {
        self.header_set(33, v as u32);
    }

    pub fn unit_MAXPOWER5(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_LEVEL(&mut self, v: i32) {
        self.header_set(34, v as u32);
    }

    pub fn unit_LEVEL(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_FACTIONTEMPLATE(&mut self, v: i32) {
        self.header_set(35, v as u32);
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
        self.header_set(37, v as u32);
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
        self.header_set(46, v as u32);
    }

    pub fn unit_FLAGS(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_unit_AURA(&mut self, v: i32) {
        self.header_set(47, v as u32);
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
        self.header_set(125, v as u32);
    }

    pub fn unit_AURASTATE(&self) -> Option<i32> {
        self.values.get(&125).map(|v| *v as i32)
    }

    pub fn set_unit_BASEATTACKTIME(&mut self, v: i32) {
        self.header_set(126, v as u32);
    }

    pub fn unit_BASEATTACKTIME(&self) -> Option<i32> {
        self.values.get(&126).map(|v| *v as i32)
    }

    pub fn set_unit_RANGEDATTACKTIME(&mut self, v: i32) {
        self.header_set(128, v as u32);
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
        self.header_set(131, v as u32);
    }

    pub fn unit_DISPLAYID(&self) -> Option<i32> {
        self.values.get(&131).map(|v| *v as i32)
    }

    pub fn set_unit_NATIVEDISPLAYID(&mut self, v: i32) {
        self.header_set(132, v as u32);
    }

    pub fn unit_NATIVEDISPLAYID(&self) -> Option<i32> {
        self.values.get(&132).map(|v| *v as i32)
    }

    pub fn set_unit_MOUNTDISPLAYID(&mut self, v: i32) {
        self.header_set(133, v as u32);
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
        self.header_set(139, v as u32);
    }

    pub fn unit_PETNUMBER(&self) -> Option<i32> {
        self.values.get(&139).map(|v| *v as i32)
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(&mut self, v: i32) {
        self.header_set(140, v as u32);
    }

    pub fn unit_PET_NAME_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&140).map(|v| *v as i32)
    }

    pub fn set_unit_PETEXPERIENCE(&mut self, v: i32) {
        self.header_set(141, v as u32);
    }

    pub fn unit_PETEXPERIENCE(&self) -> Option<i32> {
        self.values.get(&141).map(|v| *v as i32)
    }

    pub fn set_unit_PETNEXTLEVELEXP(&mut self, v: i32) {
        self.header_set(142, v as u32);
    }

    pub fn unit_PETNEXTLEVELEXP(&self) -> Option<i32> {
        self.values.get(&142).map(|v| *v as i32)
    }

    pub fn set_unit_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(143, v as u32);
    }

    pub fn unit_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&143).map(|v| *v as i32)
    }

    pub fn set_unit_CHANNEL_SPELL(&mut self, v: i32) {
        self.header_set(144, v as u32);
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
        self.header_set(146, v as u32);
    }

    pub fn unit_CREATED_BY_SPELL(&self) -> Option<i32> {
        self.values.get(&146).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_FLAGS(&mut self, v: i32) {
        self.header_set(147, v as u32);
    }

    pub fn unit_NPC_FLAGS(&self) -> Option<i32> {
        self.values.get(&147).map(|v| *v as i32)
    }

    pub fn set_unit_NPC_EMOTESTATE(&mut self, v: i32) {
        self.header_set(148, v as u32);
    }

    pub fn unit_NPC_EMOTESTATE(&self) -> Option<i32> {
        self.values.get(&148).map(|v| *v as i32)
    }

    pub fn set_unit_TRAINING_POINTS(&mut self, a: u16, b: u16) {
        self.header_set(149, (a as u32) << 16 | b as u32);
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
        self.header_set(150, v as u32);
    }

    pub fn unit_STRENGTH(&self) -> Option<i32> {
        self.values.get(&150).map(|v| *v as i32)
    }

    pub fn set_unit_AGILITY(&mut self, v: i32) {
        self.header_set(151, v as u32);
    }

    pub fn unit_AGILITY(&self) -> Option<i32> {
        self.values.get(&151).map(|v| *v as i32)
    }

    pub fn set_unit_STAMINA(&mut self, v: i32) {
        self.header_set(152, v as u32);
    }

    pub fn unit_STAMINA(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_unit_INTELLECT(&mut self, v: i32) {
        self.header_set(153, v as u32);
    }

    pub fn unit_INTELLECT(&self) -> Option<i32> {
        self.values.get(&153).map(|v| *v as i32)
    }

    pub fn set_unit_SPIRIT(&mut self, v: i32) {
        self.header_set(154, v as u32);
    }

    pub fn unit_SPIRIT(&self) -> Option<i32> {
        self.values.get(&154).map(|v| *v as i32)
    }

    pub fn set_unit_NORMAL_RESISTANCE(&mut self, v: i32) {
        self.header_set(155, v as u32);
    }

    pub fn unit_NORMAL_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&155).map(|v| *v as i32)
    }

    pub fn set_unit_HOLY_RESISTANCE(&mut self, v: i32) {
        self.header_set(156, v as u32);
    }

    pub fn unit_HOLY_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&156).map(|v| *v as i32)
    }

    pub fn set_unit_FIRE_RESISTANCE(&mut self, v: i32) {
        self.header_set(157, v as u32);
    }

    pub fn unit_FIRE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&157).map(|v| *v as i32)
    }

    pub fn set_unit_NATURE_RESISTANCE(&mut self, v: i32) {
        self.header_set(158, v as u32);
    }

    pub fn unit_NATURE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&158).map(|v| *v as i32)
    }

    pub fn set_unit_FROST_RESISTANCE(&mut self, v: i32) {
        self.header_set(159, v as u32);
    }

    pub fn unit_FROST_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&159).map(|v| *v as i32)
    }

    pub fn set_unit_SHADOW_RESISTANCE(&mut self, v: i32) {
        self.header_set(160, v as u32);
    }

    pub fn unit_SHADOW_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&160).map(|v| *v as i32)
    }

    pub fn set_unit_ARCANE_RESISTANCE(&mut self, v: i32) {
        self.header_set(161, v as u32);
    }

    pub fn unit_ARCANE_RESISTANCE(&self) -> Option<i32> {
        self.values.get(&161).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_MANA(&mut self, v: i32) {
        self.header_set(162, v as u32);
    }

    pub fn unit_BASE_MANA(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_unit_BASE_HEALTH(&mut self, v: i32) {
        self.header_set(163, v as u32);
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
        self.header_set(165, v as u32);
    }

    pub fn unit_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&165).map(|v| *v as i32)
    }

    pub fn set_unit_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(166, (a as u32) << 16 | b as u32);
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
        self.header_set(168, v as u32);
    }

    pub fn unit_RANGED_ATTACK_POWER(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(&mut self, a: u16, b: u16) {
        self.header_set(169, (a as u32) << 16 | b as u32);
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
        self.header_set(173, v as u32);
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
        self.header_set(190, v as u32);
    }

    pub fn player_FLAGS(&self) -> Option<i32> {
        self.values.get(&190).map(|v| *v as i32)
    }

    pub fn set_player_GUILDID(&mut self, v: i32) {
        self.header_set(191, v as u32);
    }

    pub fn player_GUILDID(&self) -> Option<i32> {
        self.values.get(&191).map(|v| *v as i32)
    }

    pub fn set_player_GUILDRANK(&mut self, v: i32) {
        self.header_set(192, v as u32);
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
        self.header_set(196, v as u32);
    }

    pub fn player_DUEL_TEAM(&self) -> Option<i32> {
        self.values.get(&196).map(|v| *v as i32)
    }

    pub fn set_player_GUILD_TIMESTAMP(&mut self, v: i32) {
        self.header_set(197, v as u32);
    }

    pub fn player_GUILD_TIMESTAMP(&self) -> Option<i32> {
        self.values.get(&197).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_1_1(&mut self, v: i32) {
        self.header_set(198, v as u32);
    }

    pub fn player_QUEST_LOG_1_1(&self) -> Option<i32> {
        self.values.get(&198).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_1_2(&mut self, v: i32) {
        self.header_set(199, v as u32);
    }

    pub fn player_QUEST_LOG_1_2(&self) -> Option<i32> {
        self.values.get(&199).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_2_1(&mut self, v: i32) {
        self.header_set(201, v as u32);
    }

    pub fn player_QUEST_LOG_2_1(&self) -> Option<i32> {
        self.values.get(&201).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_2_2(&mut self, v: i32) {
        self.header_set(202, v as u32);
    }

    pub fn player_QUEST_LOG_2_2(&self) -> Option<i32> {
        self.values.get(&202).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_3_1(&mut self, v: i32) {
        self.header_set(204, v as u32);
    }

    pub fn player_QUEST_LOG_3_1(&self) -> Option<i32> {
        self.values.get(&204).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_3_2(&mut self, v: i32) {
        self.header_set(205, v as u32);
    }

    pub fn player_QUEST_LOG_3_2(&self) -> Option<i32> {
        self.values.get(&205).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_4_1(&mut self, v: i32) {
        self.header_set(207, v as u32);
    }

    pub fn player_QUEST_LOG_4_1(&self) -> Option<i32> {
        self.values.get(&207).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_4_2(&mut self, v: i32) {
        self.header_set(208, v as u32);
    }

    pub fn player_QUEST_LOG_4_2(&self) -> Option<i32> {
        self.values.get(&208).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_5_1(&mut self, v: i32) {
        self.header_set(210, v as u32);
    }

    pub fn player_QUEST_LOG_5_1(&self) -> Option<i32> {
        self.values.get(&210).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_5_2(&mut self, v: i32) {
        self.header_set(211, v as u32);
    }

    pub fn player_QUEST_LOG_5_2(&self) -> Option<i32> {
        self.values.get(&211).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_6_1(&mut self, v: i32) {
        self.header_set(213, v as u32);
    }

    pub fn player_QUEST_LOG_6_1(&self) -> Option<i32> {
        self.values.get(&213).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_6_2(&mut self, v: i32) {
        self.header_set(214, v as u32);
    }

    pub fn player_QUEST_LOG_6_2(&self) -> Option<i32> {
        self.values.get(&214).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_7_1(&mut self, v: i32) {
        self.header_set(216, v as u32);
    }

    pub fn player_QUEST_LOG_7_1(&self) -> Option<i32> {
        self.values.get(&216).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_7_2(&mut self, v: i32) {
        self.header_set(217, v as u32);
    }

    pub fn player_QUEST_LOG_7_2(&self) -> Option<i32> {
        self.values.get(&217).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_8_1(&mut self, v: i32) {
        self.header_set(219, v as u32);
    }

    pub fn player_QUEST_LOG_8_1(&self) -> Option<i32> {
        self.values.get(&219).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_8_2(&mut self, v: i32) {
        self.header_set(220, v as u32);
    }

    pub fn player_QUEST_LOG_8_2(&self) -> Option<i32> {
        self.values.get(&220).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_9_1(&mut self, v: i32) {
        self.header_set(222, v as u32);
    }

    pub fn player_QUEST_LOG_9_1(&self) -> Option<i32> {
        self.values.get(&222).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_9_2(&mut self, v: i32) {
        self.header_set(223, v as u32);
    }

    pub fn player_QUEST_LOG_9_2(&self) -> Option<i32> {
        self.values.get(&223).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_10_1(&mut self, v: i32) {
        self.header_set(225, v as u32);
    }

    pub fn player_QUEST_LOG_10_1(&self) -> Option<i32> {
        self.values.get(&225).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_10_2(&mut self, v: i32) {
        self.header_set(226, v as u32);
    }

    pub fn player_QUEST_LOG_10_2(&self) -> Option<i32> {
        self.values.get(&226).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_11_1(&mut self, v: i32) {
        self.header_set(228, v as u32);
    }

    pub fn player_QUEST_LOG_11_1(&self) -> Option<i32> {
        self.values.get(&228).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_11_2(&mut self, v: i32) {
        self.header_set(229, v as u32);
    }

    pub fn player_QUEST_LOG_11_2(&self) -> Option<i32> {
        self.values.get(&229).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_12_1(&mut self, v: i32) {
        self.header_set(231, v as u32);
    }

    pub fn player_QUEST_LOG_12_1(&self) -> Option<i32> {
        self.values.get(&231).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_12_2(&mut self, v: i32) {
        self.header_set(232, v as u32);
    }

    pub fn player_QUEST_LOG_12_2(&self) -> Option<i32> {
        self.values.get(&232).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_13_1(&mut self, v: i32) {
        self.header_set(234, v as u32);
    }

    pub fn player_QUEST_LOG_13_1(&self) -> Option<i32> {
        self.values.get(&234).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_13_2(&mut self, v: i32) {
        self.header_set(235, v as u32);
    }

    pub fn player_QUEST_LOG_13_2(&self) -> Option<i32> {
        self.values.get(&235).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_14_1(&mut self, v: i32) {
        self.header_set(237, v as u32);
    }

    pub fn player_QUEST_LOG_14_1(&self) -> Option<i32> {
        self.values.get(&237).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_14_2(&mut self, v: i32) {
        self.header_set(238, v as u32);
    }

    pub fn player_QUEST_LOG_14_2(&self) -> Option<i32> {
        self.values.get(&238).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_15_1(&mut self, v: i32) {
        self.header_set(240, v as u32);
    }

    pub fn player_QUEST_LOG_15_1(&self) -> Option<i32> {
        self.values.get(&240).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_15_2(&mut self, v: i32) {
        self.header_set(241, v as u32);
    }

    pub fn player_QUEST_LOG_15_2(&self) -> Option<i32> {
        self.values.get(&241).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_16_1(&mut self, v: i32) {
        self.header_set(243, v as u32);
    }

    pub fn player_QUEST_LOG_16_1(&self) -> Option<i32> {
        self.values.get(&243).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_16_2(&mut self, v: i32) {
        self.header_set(244, v as u32);
    }

    pub fn player_QUEST_LOG_16_2(&self) -> Option<i32> {
        self.values.get(&244).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_17_1(&mut self, v: i32) {
        self.header_set(246, v as u32);
    }

    pub fn player_QUEST_LOG_17_1(&self) -> Option<i32> {
        self.values.get(&246).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_17_2(&mut self, v: i32) {
        self.header_set(247, v as u32);
    }

    pub fn player_QUEST_LOG_17_2(&self) -> Option<i32> {
        self.values.get(&247).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_18_1(&mut self, v: i32) {
        self.header_set(249, v as u32);
    }

    pub fn player_QUEST_LOG_18_1(&self) -> Option<i32> {
        self.values.get(&249).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_18_2(&mut self, v: i32) {
        self.header_set(250, v as u32);
    }

    pub fn player_QUEST_LOG_18_2(&self) -> Option<i32> {
        self.values.get(&250).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_19_1(&mut self, v: i32) {
        self.header_set(252, v as u32);
    }

    pub fn player_QUEST_LOG_19_1(&self) -> Option<i32> {
        self.values.get(&252).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_19_2(&mut self, v: i32) {
        self.header_set(253, v as u32);
    }

    pub fn player_QUEST_LOG_19_2(&self) -> Option<i32> {
        self.values.get(&253).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_20_1(&mut self, v: i32) {
        self.header_set(255, v as u32);
    }

    pub fn player_QUEST_LOG_20_1(&self) -> Option<i32> {
        self.values.get(&255).map(|v| *v as i32)
    }

    pub fn set_player_QUEST_LOG_20_2(&mut self, v: i32) {
        self.header_set(256, v as u32);
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
        self.header_set(260, v as u32);
    }

    pub fn player_VISIBLE_ITEM_1_0(&self) -> Option<i32> {
        self.values.get(&260).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_1_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(268, (a as u32) << 16 | b as u32);
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
        self.header_set(272, v as u32);
    }

    pub fn player_VISIBLE_ITEM_2_0(&self) -> Option<i32> {
        self.values.get(&272).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_2_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(280, (a as u32) << 16 | b as u32);
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
        self.header_set(284, v as u32);
    }

    pub fn player_VISIBLE_ITEM_3_0(&self) -> Option<i32> {
        self.values.get(&284).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_3_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(292, (a as u32) << 16 | b as u32);
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
        self.header_set(296, v as u32);
    }

    pub fn player_VISIBLE_ITEM_4_0(&self) -> Option<i32> {
        self.values.get(&296).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_4_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(304, (a as u32) << 16 | b as u32);
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
        self.header_set(308, v as u32);
    }

    pub fn player_VISIBLE_ITEM_5_0(&self) -> Option<i32> {
        self.values.get(&308).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_5_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(316, (a as u32) << 16 | b as u32);
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
        self.header_set(320, v as u32);
    }

    pub fn player_VISIBLE_ITEM_6_0(&self) -> Option<i32> {
        self.values.get(&320).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_6_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(328, (a as u32) << 16 | b as u32);
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
        self.header_set(332, v as u32);
    }

    pub fn player_VISIBLE_ITEM_7_0(&self) -> Option<i32> {
        self.values.get(&332).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_7_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(340, (a as u32) << 16 | b as u32);
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
        self.header_set(344, v as u32);
    }

    pub fn player_VISIBLE_ITEM_8_0(&self) -> Option<i32> {
        self.values.get(&344).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_8_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(352, (a as u32) << 16 | b as u32);
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
        self.header_set(356, v as u32);
    }

    pub fn player_VISIBLE_ITEM_9_0(&self) -> Option<i32> {
        self.values.get(&356).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_9_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(364, (a as u32) << 16 | b as u32);
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
        self.header_set(368, v as u32);
    }

    pub fn player_VISIBLE_ITEM_10_0(&self) -> Option<i32> {
        self.values.get(&368).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_10_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(376, (a as u32) << 16 | b as u32);
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
        self.header_set(380, v as u32);
    }

    pub fn player_VISIBLE_ITEM_11_0(&self) -> Option<i32> {
        self.values.get(&380).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_11_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(388, (a as u32) << 16 | b as u32);
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
        self.header_set(392, v as u32);
    }

    pub fn player_VISIBLE_ITEM_12_0(&self) -> Option<i32> {
        self.values.get(&392).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_12_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(400, (a as u32) << 16 | b as u32);
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
        self.header_set(404, v as u32);
    }

    pub fn player_VISIBLE_ITEM_13_0(&self) -> Option<i32> {
        self.values.get(&404).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_13_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(412, (a as u32) << 16 | b as u32);
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
        self.header_set(416, v as u32);
    }

    pub fn player_VISIBLE_ITEM_14_0(&self) -> Option<i32> {
        self.values.get(&416).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_14_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(424, (a as u32) << 16 | b as u32);
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
        self.header_set(428, v as u32);
    }

    pub fn player_VISIBLE_ITEM_15_0(&self) -> Option<i32> {
        self.values.get(&428).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_15_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(436, (a as u32) << 16 | b as u32);
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
        self.header_set(440, v as u32);
    }

    pub fn player_VISIBLE_ITEM_16_0(&self) -> Option<i32> {
        self.values.get(&440).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_16_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(448, (a as u32) << 16 | b as u32);
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
        self.header_set(452, v as u32);
    }

    pub fn player_VISIBLE_ITEM_17_0(&self) -> Option<i32> {
        self.values.get(&452).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_17_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(460, (a as u32) << 16 | b as u32);
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
        self.header_set(464, v as u32);
    }

    pub fn player_VISIBLE_ITEM_18_0(&self) -> Option<i32> {
        self.values.get(&464).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_18_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(472, (a as u32) << 16 | b as u32);
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
        self.header_set(476, v as u32);
    }

    pub fn player_VISIBLE_ITEM_19_0(&self) -> Option<i32> {
        self.values.get(&476).map(|v| *v as i32)
    }

    pub fn set_player_VISIBLE_ITEM_19_PROPERTIES(&mut self, a: u16, b: u16) {
        self.header_set(484, (a as u32) << 16 | b as u32);
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

    pub fn set_player_FIELD_PACK_SLOT_1(&mut self, v: Guid) {
        self.header_set(532, v.guid() as u32);
        self.header_set(533, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_PACK_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&532);
        let upper = self.values.get(&533);

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

    pub fn set_player_FIELD_BANKBAG_SLOT_1(&mut self, v: Guid) {
        self.header_set(612, v.guid() as u32);
        self.header_set(613, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_BANKBAG_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&612);
        let upper = self.values.get(&613);

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

    pub fn set_player_FIELD_KEYRING_SLOT_1(&mut self, v: Guid) {
        self.header_set(648, v.guid() as u32);
        self.header_set(649, (v.guid() >> 32) as u32);
    }

    pub fn player_FIELD_KEYRING_SLOT_1(&self) -> Option<Guid> {
        let lower = self.values.get(&648);
        let upper = self.values.get(&649);

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
        self.header_set(716, v as u32);
    }

    pub fn player_XP(&self) -> Option<i32> {
        self.values.get(&716).map(|v| *v as i32)
    }

    pub fn set_player_NEXT_LEVEL_XP(&mut self, v: i32) {
        self.header_set(717, v as u32);
    }

    pub fn player_NEXT_LEVEL_XP(&self) -> Option<i32> {
        self.values.get(&717).map(|v| *v as i32)
    }

    pub fn set_player_SKILL_INFO_1_1(&mut self, v: i32) {
        self.header_set(718, v as u32);
    }

    pub fn player_SKILL_INFO_1_1(&self) -> Option<i32> {
        self.values.get(&718).map(|v| *v as i32)
    }

    pub fn set_player_SKILL_INFO_1_2(&mut self, a: u16, b: u16) {
        self.header_set(719, (a as u32) << 16 | b as u32);
    }

    pub fn player_SKILL_INFO_1_2(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&719) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_SKILL_INFO_1_3(&mut self, v: i32) {
        self.header_set(720, v as u32);
    }

    pub fn player_SKILL_INFO_1_3(&self) -> Option<i32> {
        self.values.get(&720).map(|v| *v as i32)
    }

    pub fn set_player_CHARACTER_POINTS1(&mut self, v: i32) {
        self.header_set(1102, v as u32);
    }

    pub fn player_CHARACTER_POINTS1(&self) -> Option<i32> {
        self.values.get(&1102).map(|v| *v as i32)
    }

    pub fn set_player_CHARACTER_POINTS2(&mut self, v: i32) {
        self.header_set(1103, v as u32);
    }

    pub fn player_CHARACTER_POINTS2(&self) -> Option<i32> {
        self.values.get(&1103).map(|v| *v as i32)
    }

    pub fn set_player_TRACK_CREATURES(&mut self, v: i32) {
        self.header_set(1104, v as u32);
    }

    pub fn player_TRACK_CREATURES(&self) -> Option<i32> {
        self.values.get(&1104).map(|v| *v as i32)
    }

    pub fn set_player_TRACK_RESOURCES(&mut self, v: i32) {
        self.header_set(1105, v as u32);
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
        self.header_set(1175, v as u32);
    }

    pub fn player_REST_STATE_EXPERIENCE(&self) -> Option<i32> {
        self.values.get(&1175).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_COINAGE(&mut self, v: i32) {
        self.header_set(1176, v as u32);
    }

    pub fn player_FIELD_COINAGE(&self) -> Option<i32> {
        self.values.get(&1176).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_POSSTAT0(&mut self, v: i32) {
        self.header_set(1177, v as u32);
    }

    pub fn player_FIELD_POSSTAT0(&self) -> Option<i32> {
        self.values.get(&1177).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_POSSTAT1(&mut self, v: i32) {
        self.header_set(1178, v as u32);
    }

    pub fn player_FIELD_POSSTAT1(&self) -> Option<i32> {
        self.values.get(&1178).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_POSSTAT2(&mut self, v: i32) {
        self.header_set(1179, v as u32);
    }

    pub fn player_FIELD_POSSTAT2(&self) -> Option<i32> {
        self.values.get(&1179).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_POSSTAT3(&mut self, v: i32) {
        self.header_set(1180, v as u32);
    }

    pub fn player_FIELD_POSSTAT3(&self) -> Option<i32> {
        self.values.get(&1180).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_POSSTAT4(&mut self, v: i32) {
        self.header_set(1181, v as u32);
    }

    pub fn player_FIELD_POSSTAT4(&self) -> Option<i32> {
        self.values.get(&1181).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_NEGSTAT0(&mut self, v: i32) {
        self.header_set(1182, v as u32);
    }

    pub fn player_FIELD_NEGSTAT0(&self) -> Option<i32> {
        self.values.get(&1182).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_NEGSTAT1(&mut self, v: i32) {
        self.header_set(1183, v as u32);
    }

    pub fn player_FIELD_NEGSTAT1(&self) -> Option<i32> {
        self.values.get(&1183).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_NEGSTAT2(&mut self, v: i32) {
        self.header_set(1184, v as u32);
    }

    pub fn player_FIELD_NEGSTAT2(&self) -> Option<i32> {
        self.values.get(&1184).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_NEGSTAT3(&mut self, v: i32) {
        self.header_set(1185, v as u32);
    }

    pub fn player_FIELD_NEGSTAT3(&self) -> Option<i32> {
        self.values.get(&1185).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_NEGSTAT4(&mut self, v: i32) {
        self.header_set(1186, v as u32);
    }

    pub fn player_FIELD_NEGSTAT4(&self) -> Option<i32> {
        self.values.get(&1186).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSPOSITIVE(&mut self, v: i32) {
        self.header_set(1187, v as u32);
    }

    pub fn player_FIELD_RESISTANCEBUFFMODSPOSITIVE(&self) -> Option<i32> {
        self.values.get(&1187).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSNEGATIVE(&mut self, v: i32) {
        self.header_set(1194, v as u32);
    }

    pub fn player_FIELD_RESISTANCEBUFFMODSNEGATIVE(&self) -> Option<i32> {
        self.values.get(&1194).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_POS(&mut self, v: i32) {
        self.header_set(1201, v as u32);
    }

    pub fn player_FIELD_MOD_DAMAGE_DONE_POS(&self) -> Option<i32> {
        self.values.get(&1201).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_NEG(&mut self, v: i32) {
        self.header_set(1208, v as u32);
    }

    pub fn player_FIELD_MOD_DAMAGE_DONE_NEG(&self) -> Option<i32> {
        self.values.get(&1208).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_PCT(&mut self, v: i32) {
        self.header_set(1215, v as u32);
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
        self.header_set(1223, v as u32);
    }

    pub fn player_AMMO_ID(&self) -> Option<i32> {
        self.values.get(&1223).map(|v| *v as i32)
    }

    pub fn set_player_SELF_RES_SPELL(&mut self, v: i32) {
        self.header_set(1224, v as u32);
    }

    pub fn player_SELF_RES_SPELL(&self) -> Option<i32> {
        self.values.get(&1224).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_PVP_MEDALS(&mut self, v: i32) {
        self.header_set(1225, v as u32);
    }

    pub fn player_FIELD_PVP_MEDALS(&self) -> Option<i32> {
        self.values.get(&1225).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_BUYBACK_PRICE_1(&mut self, v: i32) {
        self.header_set(1226, v as u32);
    }

    pub fn player_FIELD_BUYBACK_PRICE_1(&self) -> Option<i32> {
        self.values.get(&1226).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_BUYBACK_TIMESTAMP_1(&mut self, v: i32) {
        self.header_set(1238, v as u32);
    }

    pub fn player_FIELD_BUYBACK_TIMESTAMP_1(&self) -> Option<i32> {
        self.values.get(&1238).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_SESSION_KILLS(&mut self, a: u16, b: u16) {
        self.header_set(1250, (a as u32) << 16 | b as u32);
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
        self.header_set(1251, (a as u32) << 16 | b as u32);
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
        self.header_set(1252, (a as u32) << 16 | b as u32);
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
        self.header_set(1253, (a as u32) << 16 | b as u32);
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
        self.header_set(1254, v as u32);
    }

    pub fn player_FIELD_THIS_WEEK_CONTRIBUTION(&self) -> Option<i32> {
        self.values.get(&1254).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_LIFETIME_HONORBALE_KILLS(&mut self, v: i32) {
        self.header_set(1255, v as u32);
    }

    pub fn player_FIELD_LIFETIME_HONORBALE_KILLS(&self) -> Option<i32> {
        self.values.get(&1255).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_LIFETIME_DISHONORBALE_KILLS(&mut self, v: i32) {
        self.header_set(1256, v as u32);
    }

    pub fn player_FIELD_LIFETIME_DISHONORBALE_KILLS(&self) -> Option<i32> {
        self.values.get(&1256).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_YESTERDAY_CONTRIBUTION(&mut self, v: i32) {
        self.header_set(1257, v as u32);
    }

    pub fn player_FIELD_YESTERDAY_CONTRIBUTION(&self) -> Option<i32> {
        self.values.get(&1257).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_LAST_WEEK_CONTRIBUTION(&mut self, v: i32) {
        self.header_set(1258, v as u32);
    }

    pub fn player_FIELD_LAST_WEEK_CONTRIBUTION(&self) -> Option<i32> {
        self.values.get(&1258).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_LAST_WEEK_RANK(&mut self, v: i32) {
        self.header_set(1259, v as u32);
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
        self.header_set(1261, v as u32);
    }

    pub fn player_FIELD_WATCHED_FACTION_INDEX(&self) -> Option<i32> {
        self.values.get(&1261).map(|v| *v as i32)
    }

    pub fn set_player_FIELD_COMBAT_RATING_1(&mut self, v: i32) {
        self.header_set(1262, v as u32);
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

    pub fn set_gameobject_ROTATION(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_ROTATION(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_STATE(&mut self, v: i32) {
        self.header_set(14, v as u32);
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
        self.header_set(19, v as u32);
    }

    pub fn gameobject_DYN_FLAGS(&self) -> Option<i32> {
        self.values.get(&19).map(|v| *v as i32)
    }

    pub fn set_gameobject_FACTION(&mut self, v: i32) {
        self.header_set(20, v as u32);
    }

    pub fn gameobject_FACTION(&self) -> Option<i32> {
        self.values.get(&20).map(|v| *v as i32)
    }

    pub fn set_gameobject_TYPE_ID(&mut self, v: i32) {
        self.header_set(21, v as u32);
    }

    pub fn gameobject_TYPE_ID(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_gameobject_LEVEL(&mut self, v: i32) {
        self.header_set(22, v as u32);
    }

    pub fn gameobject_LEVEL(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_gameobject_ARTKIT(&mut self, v: i32) {
        self.header_set(23, v as u32);
    }

    pub fn gameobject_ARTKIT(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_gameobject_ANIMPROGRESS(&mut self, v: i32) {
        self.header_set(24, v as u32);
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
        self.header_set(12, v as u32);
    }

    pub fn corpse_DISPLAY_ID(&self) -> Option<i32> {
        self.values.get(&12).map(|v| *v as i32)
    }

    pub fn set_corpse_ITEM(&mut self, v: i32) {
        self.header_set(13, v as u32);
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
        self.header_set(34, v as u32);
    }

    pub fn corpse_GUILD(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_corpse_FLAGS(&mut self, v: i32) {
        self.header_set(35, v as u32);
    }

    pub fn corpse_FLAGS(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_corpse_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(36, v as u32);
    }

    pub fn corpse_DYNAMIC_FLAGS(&self) -> Option<i32> {
        self.values.get(&36).map(|v| *v as i32)
    }

}

