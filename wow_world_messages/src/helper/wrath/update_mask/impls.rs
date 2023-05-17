use crate::Guid;
use std::convert::TryInto;
use super::indices::*;
use crate::wrath::{Race};
use crate::wrath::{Class};
use crate::wrath::{Gender};
use crate::wrath::{Power};
use crate::wrath::{UnitStandState};
use crate::wrath::{UpdateContainer, UpdateContainerBuilder, UpdateCorpse, UpdateCorpseBuilder, UpdateDynamicObject, UpdateDynamicObjectBuilder, UpdateGameObject, UpdateGameObjectBuilder, UpdateItem, UpdateItemBuilder, UpdatePlayer, UpdatePlayerBuilder, UpdateUnit, UpdateUnitBuilder};

impl UpdateItemBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_owner(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_contained(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_creator(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_giftcreator(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_stack_count(mut self, v: i32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_duration(mut self, v: i32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_spell_charges(mut self, v: i32) -> Self {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_flags(mut self, v: i32) -> Self {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_1_1(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_1_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(24, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_2_1(mut self, v: i32) -> Self {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_2_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(27, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_3_1(mut self, v: i32) -> Self {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_3_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(30, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_4_1(mut self, v: i32) -> Self {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_4_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(33, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_5_1(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_5_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(36, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_6_1(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_6_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(39, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_7_1(mut self, v: i32) -> Self {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_7_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(42, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_8_1(mut self, v: i32) -> Self {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_8_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(45, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_9_1(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_9_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(48, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_10_1(mut self, v: i32) -> Self {
        self.header_set(49, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_10_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(51, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_11_1(mut self, v: i32) -> Self {
        self.header_set(52, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_11_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(54, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_12_1(mut self, v: i32) -> Self {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_12_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(57, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.header_set(61, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_create_played_time(mut self, v: i32) -> Self {
        self.header_set(62, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateContainerBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_owner(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_contained(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_creator(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_giftcreator(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_stack_count(mut self, v: i32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_duration(mut self, v: i32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_spell_charges(mut self, v: i32) -> Self {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_flags(mut self, v: i32) -> Self {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_1_1(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_1_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(24, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_2_1(mut self, v: i32) -> Self {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_2_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(27, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_3_1(mut self, v: i32) -> Self {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_3_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(30, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_4_1(mut self, v: i32) -> Self {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_4_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(33, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_5_1(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_5_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(36, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_6_1(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_6_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(39, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_7_1(mut self, v: i32) -> Self {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_7_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(42, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_8_1(mut self, v: i32) -> Self {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_8_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(45, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_9_1(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_9_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(48, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_10_1(mut self, v: i32) -> Self {
        self.header_set(49, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_10_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(51, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_11_1(mut self, v: i32) -> Self {
        self.header_set(52, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_11_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(54, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_enchantment_12_1(mut self, v: i32) -> Self {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_enchantment_12_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(57, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.header_set(61, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_create_played_time(mut self, v: i32) -> Self {
        self.header_set(62, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_container_num_slots(mut self, v: i32) -> Self {
        self.header_set(64, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_container_slot_1(mut self, v: Guid) -> Self {
        self.header_set(66, v.guid() as u32);
        self.header_set(67, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateUnitBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_charm(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_summon(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_critter(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_charmedby(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_summonedby(mut self, v: Guid) -> Self {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_createdby(mut self, v: Guid) -> Self {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_target(mut self, v: Guid) -> Self {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_channel_object(mut self, v: Guid) -> Self {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(23, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_health(mut self, v: i32) -> Self {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power1(mut self, v: i32) -> Self {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power2(mut self, v: i32) -> Self {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power3(mut self, v: i32) -> Self {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power4(mut self, v: i32) -> Self {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power5(mut self, v: i32) -> Self {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power6(mut self, v: i32) -> Self {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power7(mut self, v: i32) -> Self {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxhealth(mut self, v: i32) -> Self {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower1(mut self, v: i32) -> Self {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower2(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower3(mut self, v: i32) -> Self {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower4(mut self, v: i32) -> Self {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower5(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower6(mut self, v: i32) -> Self {
        self.header_set(38, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower7(mut self, v: i32) -> Self {
        self.header_set(39, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_regen_flat_modifier(mut self, v: f32) -> Self {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_regen_interrupted_flat_modifier(mut self, v: f32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_level(mut self, v: i32) -> Self {
        self.header_set(54, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_factiontemplate(mut self, v: i32) -> Self {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_virtual_item_slot_id(mut self, v: i32) -> Self {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_flags(mut self, v: i32) -> Self {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_flags_2(mut self, v: i32) -> Self {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.header_set(61, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.header_set(62, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.header_set(64, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.header_set(65, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.header_set(66, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.header_set(67, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.header_set(68, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.header_set(69, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.header_set(70, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.header_set(71, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(72, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(73, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_1(mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) -> Self {
        self.header_set(74, u32::from_le_bytes([stand_state.as_int(), unknown1, unknown2, unknown3]));
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.header_set(75, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.header_set(76, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.header_set(77, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.header_set(78, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.header_set(79, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.header_set(80, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.header_set(81, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.header_set(82, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.header_set(83, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.header_set(84, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.header_set(85, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.header_set(86, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.header_set(87, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.header_set(88, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat0(mut self, v: i32) -> Self {
        self.header_set(89, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat1(mut self, v: i32) -> Self {
        self.header_set(90, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat2(mut self, v: i32) -> Self {
        self.header_set(91, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat3(mut self, v: i32) -> Self {
        self.header_set(92, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat4(mut self, v: i32) -> Self {
        self.header_set(93, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat0(mut self, v: i32) -> Self {
        self.header_set(94, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat1(mut self, v: i32) -> Self {
        self.header_set(95, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat2(mut self, v: i32) -> Self {
        self.header_set(96, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat3(mut self, v: i32) -> Self {
        self.header_set(97, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat4(mut self, v: i32) -> Self {
        self.header_set(98, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_resistances(mut self, v: i32) -> Self {
        self.header_set(99, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_resistancebuffmodspositive(mut self, v: i32) -> Self {
        self.header_set(106, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_resistancebuffmodsnegative(mut self, v: i32) -> Self {
        self.header_set(113, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.header_set(120, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.header_set(121, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(122, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.header_set(123, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(124, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(127, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.header_set(138, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxhealthmodifier(mut self, v: f32) -> Self {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_hoverheight(mut self, v: f32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdatePlayerBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_charm(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_summon(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_critter(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_charmedby(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_summonedby(mut self, v: Guid) -> Self {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_createdby(mut self, v: Guid) -> Self {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_target(mut self, v: Guid) -> Self {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_channel_object(mut self, v: Guid) -> Self {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(23, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_health(mut self, v: i32) -> Self {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power1(mut self, v: i32) -> Self {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power2(mut self, v: i32) -> Self {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power3(mut self, v: i32) -> Self {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power4(mut self, v: i32) -> Self {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power5(mut self, v: i32) -> Self {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power6(mut self, v: i32) -> Self {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power7(mut self, v: i32) -> Self {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxhealth(mut self, v: i32) -> Self {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower1(mut self, v: i32) -> Self {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower2(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower3(mut self, v: i32) -> Self {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower4(mut self, v: i32) -> Self {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower5(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower6(mut self, v: i32) -> Self {
        self.header_set(38, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower7(mut self, v: i32) -> Self {
        self.header_set(39, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_regen_flat_modifier(mut self, v: f32) -> Self {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_regen_interrupted_flat_modifier(mut self, v: f32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_level(mut self, v: i32) -> Self {
        self.header_set(54, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_factiontemplate(mut self, v: i32) -> Self {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_virtual_item_slot_id(mut self, v: i32) -> Self {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_flags(mut self, v: i32) -> Self {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_flags_2(mut self, v: i32) -> Self {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.header_set(61, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.header_set(62, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.header_set(64, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.header_set(65, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.header_set(66, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.header_set(67, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.header_set(68, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.header_set(69, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.header_set(70, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.header_set(71, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(72, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(73, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_1(mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) -> Self {
        self.header_set(74, u32::from_le_bytes([stand_state.as_int(), unknown1, unknown2, unknown3]));
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.header_set(75, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.header_set(76, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.header_set(77, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.header_set(78, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.header_set(79, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.header_set(80, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.header_set(81, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.header_set(82, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.header_set(83, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.header_set(84, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.header_set(85, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.header_set(86, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.header_set(87, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.header_set(88, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat0(mut self, v: i32) -> Self {
        self.header_set(89, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat1(mut self, v: i32) -> Self {
        self.header_set(90, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat2(mut self, v: i32) -> Self {
        self.header_set(91, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat3(mut self, v: i32) -> Self {
        self.header_set(92, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat4(mut self, v: i32) -> Self {
        self.header_set(93, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat0(mut self, v: i32) -> Self {
        self.header_set(94, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat1(mut self, v: i32) -> Self {
        self.header_set(95, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat2(mut self, v: i32) -> Self {
        self.header_set(96, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat3(mut self, v: i32) -> Self {
        self.header_set(97, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat4(mut self, v: i32) -> Self {
        self.header_set(98, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_resistances(mut self, v: i32) -> Self {
        self.header_set(99, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_resistancebuffmodspositive(mut self, v: i32) -> Self {
        self.header_set(106, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_resistancebuffmodsnegative(mut self, v: i32) -> Self {
        self.header_set(113, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.header_set(120, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.header_set(121, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(122, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.header_set(123, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(124, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(127, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.header_set(138, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxhealthmodifier(mut self, v: f32) -> Self {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_hoverheight(mut self, v: f32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_duel_arbiter(mut self, v: Guid) -> Self {
        self.header_set(148, v.guid() as u32);
        self.header_set(149, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_flags(mut self, v: i32) -> Self {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_guildid(mut self, v: i32) -> Self {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_guildrank(mut self, v: i32) -> Self {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_bytes(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(153, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(154, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_bytes_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(155, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_duel_team(mut self, v: i32) -> Self {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_guild_timestamp(mut self, v: i32) -> Self {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_1_1(mut self, v: i32) -> Self {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_1_2(mut self, v: i32) -> Self {
        self.header_set(159, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_1_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(160, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_1_4(mut self, v: i32) -> Self {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_2_1(mut self, v: i32) -> Self {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_2_2(mut self, v: i32) -> Self {
        self.header_set(164, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_2_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(165, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_2_5(mut self, v: i32) -> Self {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_3_1(mut self, v: i32) -> Self {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_3_2(mut self, v: i32) -> Self {
        self.header_set(169, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_3_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(170, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_3_5(mut self, v: i32) -> Self {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_4_1(mut self, v: i32) -> Self {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_4_2(mut self, v: i32) -> Self {
        self.header_set(174, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_4_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(175, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_4_5(mut self, v: i32) -> Self {
        self.header_set(177, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_5_1(mut self, v: i32) -> Self {
        self.header_set(178, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_5_2(mut self, v: i32) -> Self {
        self.header_set(179, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_5_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(180, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_5_5(mut self, v: i32) -> Self {
        self.header_set(182, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_6_1(mut self, v: i32) -> Self {
        self.header_set(183, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_6_2(mut self, v: i32) -> Self {
        self.header_set(184, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_6_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(185, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_6_5(mut self, v: i32) -> Self {
        self.header_set(187, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_7_1(mut self, v: i32) -> Self {
        self.header_set(188, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_7_2(mut self, v: i32) -> Self {
        self.header_set(189, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_7_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(190, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_7_5(mut self, v: i32) -> Self {
        self.header_set(192, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_8_1(mut self, v: i32) -> Self {
        self.header_set(193, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_8_2(mut self, v: i32) -> Self {
        self.header_set(194, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_8_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(195, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_8_5(mut self, v: i32) -> Self {
        self.header_set(197, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_9_1(mut self, v: i32) -> Self {
        self.header_set(198, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_9_2(mut self, v: i32) -> Self {
        self.header_set(199, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_9_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(200, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_9_5(mut self, v: i32) -> Self {
        self.header_set(202, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_10_1(mut self, v: i32) -> Self {
        self.header_set(203, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_10_2(mut self, v: i32) -> Self {
        self.header_set(204, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_10_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(205, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_10_5(mut self, v: i32) -> Self {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_11_1(mut self, v: i32) -> Self {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_11_2(mut self, v: i32) -> Self {
        self.header_set(209, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_11_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(210, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_11_5(mut self, v: i32) -> Self {
        self.header_set(212, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_12_1(mut self, v: i32) -> Self {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_12_2(mut self, v: i32) -> Self {
        self.header_set(214, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_12_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(215, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_12_5(mut self, v: i32) -> Self {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_13_1(mut self, v: i32) -> Self {
        self.header_set(218, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_13_2(mut self, v: i32) -> Self {
        self.header_set(219, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_13_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(220, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_13_5(mut self, v: i32) -> Self {
        self.header_set(222, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_14_1(mut self, v: i32) -> Self {
        self.header_set(223, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_14_2(mut self, v: i32) -> Self {
        self.header_set(224, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_14_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(225, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_14_5(mut self, v: i32) -> Self {
        self.header_set(227, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_15_1(mut self, v: i32) -> Self {
        self.header_set(228, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_15_2(mut self, v: i32) -> Self {
        self.header_set(229, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_15_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(230, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_15_5(mut self, v: i32) -> Self {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_16_1(mut self, v: i32) -> Self {
        self.header_set(233, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_16_2(mut self, v: i32) -> Self {
        self.header_set(234, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_16_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(235, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_16_5(mut self, v: i32) -> Self {
        self.header_set(237, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_17_1(mut self, v: i32) -> Self {
        self.header_set(238, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_17_2(mut self, v: i32) -> Self {
        self.header_set(239, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_17_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(240, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_17_5(mut self, v: i32) -> Self {
        self.header_set(242, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_18_1(mut self, v: i32) -> Self {
        self.header_set(243, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_18_2(mut self, v: i32) -> Self {
        self.header_set(244, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_18_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(245, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_18_5(mut self, v: i32) -> Self {
        self.header_set(247, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_19_1(mut self, v: i32) -> Self {
        self.header_set(248, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_19_2(mut self, v: i32) -> Self {
        self.header_set(249, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_19_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(250, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_19_5(mut self, v: i32) -> Self {
        self.header_set(252, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_20_1(mut self, v: i32) -> Self {
        self.header_set(253, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_20_2(mut self, v: i32) -> Self {
        self.header_set(254, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_20_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(255, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_20_5(mut self, v: i32) -> Self {
        self.header_set(257, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_21_1(mut self, v: i32) -> Self {
        self.header_set(258, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_21_2(mut self, v: i32) -> Self {
        self.header_set(259, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_21_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(260, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_21_5(mut self, v: i32) -> Self {
        self.header_set(262, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_22_1(mut self, v: i32) -> Self {
        self.header_set(263, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_22_2(mut self, v: i32) -> Self {
        self.header_set(264, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_22_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(265, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_22_5(mut self, v: i32) -> Self {
        self.header_set(267, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_23_1(mut self, v: i32) -> Self {
        self.header_set(268, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_23_2(mut self, v: i32) -> Self {
        self.header_set(269, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_23_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(270, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_23_5(mut self, v: i32) -> Self {
        self.header_set(272, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_24_1(mut self, v: i32) -> Self {
        self.header_set(273, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_24_2(mut self, v: i32) -> Self {
        self.header_set(274, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_24_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(275, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_24_5(mut self, v: i32) -> Self {
        self.header_set(277, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_25_1(mut self, v: i32) -> Self {
        self.header_set(278, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_25_2(mut self, v: i32) -> Self {
        self.header_set(279, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_25_3(mut self, a: u16, b: u16) -> Self {
        self.header_set(280, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_quest_log_25_5(mut self, v: i32) -> Self {
        self.header_set(282, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_1_entryid(mut self, v: i32) -> Self {
        self.header_set(283, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_1_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(284, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_2_entryid(mut self, v: i32) -> Self {
        self.header_set(285, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_2_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(286, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_3_entryid(mut self, v: i32) -> Self {
        self.header_set(287, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_3_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(288, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_4_entryid(mut self, v: i32) -> Self {
        self.header_set(289, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_4_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(290, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_5_entryid(mut self, v: i32) -> Self {
        self.header_set(291, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_5_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(292, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_6_entryid(mut self, v: i32) -> Self {
        self.header_set(293, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_6_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(294, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_7_entryid(mut self, v: i32) -> Self {
        self.header_set(295, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_7_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(296, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_8_entryid(mut self, v: i32) -> Self {
        self.header_set(297, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_8_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(298, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_9_entryid(mut self, v: i32) -> Self {
        self.header_set(299, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_9_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(300, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_10_entryid(mut self, v: i32) -> Self {
        self.header_set(301, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_10_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(302, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_11_entryid(mut self, v: i32) -> Self {
        self.header_set(303, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_11_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(304, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_12_entryid(mut self, v: i32) -> Self {
        self.header_set(305, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_12_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(306, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_13_entryid(mut self, v: i32) -> Self {
        self.header_set(307, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_13_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(308, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_14_entryid(mut self, v: i32) -> Self {
        self.header_set(309, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_14_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(310, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_15_entryid(mut self, v: i32) -> Self {
        self.header_set(311, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_15_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(312, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_16_entryid(mut self, v: i32) -> Self {
        self.header_set(313, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_16_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(314, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_17_entryid(mut self, v: i32) -> Self {
        self.header_set(315, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_17_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(316, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_18_entryid(mut self, v: i32) -> Self {
        self.header_set(317, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_18_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(318, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_19_entryid(mut self, v: i32) -> Self {
        self.header_set(319, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_19_enchantment(mut self, a: u16, b: u16) -> Self {
        self.header_set(320, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_chosen_title(mut self, v: i32) -> Self {
        self.header_set(321, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_fake_inebriation(mut self, v: i32) -> Self {
        self.header_set(322, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_inv_slot_head1(mut self, v: Guid) -> Self {
        self.header_set(324, v.guid() as u32);
        self.header_set(325, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head2(mut self, v: Guid) -> Self {
        self.header_set(326, v.guid() as u32);
        self.header_set(327, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head3(mut self, v: Guid) -> Self {
        self.header_set(328, v.guid() as u32);
        self.header_set(329, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head4(mut self, v: Guid) -> Self {
        self.header_set(330, v.guid() as u32);
        self.header_set(331, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head5(mut self, v: Guid) -> Self {
        self.header_set(332, v.guid() as u32);
        self.header_set(333, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head6(mut self, v: Guid) -> Self {
        self.header_set(334, v.guid() as u32);
        self.header_set(335, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head7(mut self, v: Guid) -> Self {
        self.header_set(336, v.guid() as u32);
        self.header_set(337, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head8(mut self, v: Guid) -> Self {
        self.header_set(338, v.guid() as u32);
        self.header_set(339, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head9(mut self, v: Guid) -> Self {
        self.header_set(340, v.guid() as u32);
        self.header_set(341, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head10(mut self, v: Guid) -> Self {
        self.header_set(342, v.guid() as u32);
        self.header_set(343, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head11(mut self, v: Guid) -> Self {
        self.header_set(344, v.guid() as u32);
        self.header_set(345, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head12(mut self, v: Guid) -> Self {
        self.header_set(346, v.guid() as u32);
        self.header_set(347, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head13(mut self, v: Guid) -> Self {
        self.header_set(348, v.guid() as u32);
        self.header_set(349, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head14(mut self, v: Guid) -> Self {
        self.header_set(350, v.guid() as u32);
        self.header_set(351, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head15(mut self, v: Guid) -> Self {
        self.header_set(352, v.guid() as u32);
        self.header_set(353, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head16(mut self, v: Guid) -> Self {
        self.header_set(354, v.guid() as u32);
        self.header_set(355, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head17(mut self, v: Guid) -> Self {
        self.header_set(356, v.guid() as u32);
        self.header_set(357, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head18(mut self, v: Guid) -> Self {
        self.header_set(358, v.guid() as u32);
        self.header_set(359, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head19(mut self, v: Guid) -> Self {
        self.header_set(360, v.guid() as u32);
        self.header_set(361, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head20(mut self, v: Guid) -> Self {
        self.header_set(362, v.guid() as u32);
        self.header_set(363, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head21(mut self, v: Guid) -> Self {
        self.header_set(364, v.guid() as u32);
        self.header_set(365, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head22(mut self, v: Guid) -> Self {
        self.header_set(366, v.guid() as u32);
        self.header_set(367, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_head23(mut self, v: Guid) -> Self {
        self.header_set(368, v.guid() as u32);
        self.header_set(369, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_1(mut self, v: Guid) -> Self {
        self.header_set(370, v.guid() as u32);
        self.header_set(371, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_2(mut self, v: Guid) -> Self {
        self.header_set(372, v.guid() as u32);
        self.header_set(373, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_3(mut self, v: Guid) -> Self {
        self.header_set(374, v.guid() as u32);
        self.header_set(375, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_4(mut self, v: Guid) -> Self {
        self.header_set(376, v.guid() as u32);
        self.header_set(377, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_5(mut self, v: Guid) -> Self {
        self.header_set(378, v.guid() as u32);
        self.header_set(379, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_6(mut self, v: Guid) -> Self {
        self.header_set(380, v.guid() as u32);
        self.header_set(381, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_7(mut self, v: Guid) -> Self {
        self.header_set(382, v.guid() as u32);
        self.header_set(383, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_8(mut self, v: Guid) -> Self {
        self.header_set(384, v.guid() as u32);
        self.header_set(385, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_9(mut self, v: Guid) -> Self {
        self.header_set(386, v.guid() as u32);
        self.header_set(387, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_10(mut self, v: Guid) -> Self {
        self.header_set(388, v.guid() as u32);
        self.header_set(389, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_11(mut self, v: Guid) -> Self {
        self.header_set(390, v.guid() as u32);
        self.header_set(391, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_12(mut self, v: Guid) -> Self {
        self.header_set(392, v.guid() as u32);
        self.header_set(393, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_13(mut self, v: Guid) -> Self {
        self.header_set(394, v.guid() as u32);
        self.header_set(395, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_14(mut self, v: Guid) -> Self {
        self.header_set(396, v.guid() as u32);
        self.header_set(397, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_15(mut self, v: Guid) -> Self {
        self.header_set(398, v.guid() as u32);
        self.header_set(399, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_16(mut self, v: Guid) -> Self {
        self.header_set(400, v.guid() as u32);
        self.header_set(401, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_1(mut self, v: Guid) -> Self {
        self.header_set(402, v.guid() as u32);
        self.header_set(403, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_2(mut self, v: Guid) -> Self {
        self.header_set(404, v.guid() as u32);
        self.header_set(405, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_3(mut self, v: Guid) -> Self {
        self.header_set(406, v.guid() as u32);
        self.header_set(407, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_4(mut self, v: Guid) -> Self {
        self.header_set(408, v.guid() as u32);
        self.header_set(409, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_5(mut self, v: Guid) -> Self {
        self.header_set(410, v.guid() as u32);
        self.header_set(411, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_6(mut self, v: Guid) -> Self {
        self.header_set(412, v.guid() as u32);
        self.header_set(413, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_7(mut self, v: Guid) -> Self {
        self.header_set(414, v.guid() as u32);
        self.header_set(415, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_8(mut self, v: Guid) -> Self {
        self.header_set(416, v.guid() as u32);
        self.header_set(417, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_9(mut self, v: Guid) -> Self {
        self.header_set(418, v.guid() as u32);
        self.header_set(419, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_10(mut self, v: Guid) -> Self {
        self.header_set(420, v.guid() as u32);
        self.header_set(421, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_11(mut self, v: Guid) -> Self {
        self.header_set(422, v.guid() as u32);
        self.header_set(423, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_12(mut self, v: Guid) -> Self {
        self.header_set(424, v.guid() as u32);
        self.header_set(425, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_13(mut self, v: Guid) -> Self {
        self.header_set(426, v.guid() as u32);
        self.header_set(427, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_14(mut self, v: Guid) -> Self {
        self.header_set(428, v.guid() as u32);
        self.header_set(429, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_15(mut self, v: Guid) -> Self {
        self.header_set(430, v.guid() as u32);
        self.header_set(431, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_16(mut self, v: Guid) -> Self {
        self.header_set(432, v.guid() as u32);
        self.header_set(433, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_17(mut self, v: Guid) -> Self {
        self.header_set(434, v.guid() as u32);
        self.header_set(435, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_18(mut self, v: Guid) -> Self {
        self.header_set(436, v.guid() as u32);
        self.header_set(437, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_19(mut self, v: Guid) -> Self {
        self.header_set(438, v.guid() as u32);
        self.header_set(439, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_20(mut self, v: Guid) -> Self {
        self.header_set(440, v.guid() as u32);
        self.header_set(441, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_21(mut self, v: Guid) -> Self {
        self.header_set(442, v.guid() as u32);
        self.header_set(443, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_22(mut self, v: Guid) -> Self {
        self.header_set(444, v.guid() as u32);
        self.header_set(445, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_23(mut self, v: Guid) -> Self {
        self.header_set(446, v.guid() as u32);
        self.header_set(447, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_24(mut self, v: Guid) -> Self {
        self.header_set(448, v.guid() as u32);
        self.header_set(449, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_25(mut self, v: Guid) -> Self {
        self.header_set(450, v.guid() as u32);
        self.header_set(451, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_26(mut self, v: Guid) -> Self {
        self.header_set(452, v.guid() as u32);
        self.header_set(453, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_27(mut self, v: Guid) -> Self {
        self.header_set(454, v.guid() as u32);
        self.header_set(455, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_28(mut self, v: Guid) -> Self {
        self.header_set(456, v.guid() as u32);
        self.header_set(457, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_1(mut self, v: Guid) -> Self {
        self.header_set(458, v.guid() as u32);
        self.header_set(459, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_2(mut self, v: Guid) -> Self {
        self.header_set(460, v.guid() as u32);
        self.header_set(461, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_3(mut self, v: Guid) -> Self {
        self.header_set(462, v.guid() as u32);
        self.header_set(463, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_4(mut self, v: Guid) -> Self {
        self.header_set(464, v.guid() as u32);
        self.header_set(465, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_5(mut self, v: Guid) -> Self {
        self.header_set(466, v.guid() as u32);
        self.header_set(467, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_6(mut self, v: Guid) -> Self {
        self.header_set(468, v.guid() as u32);
        self.header_set(469, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_7(mut self, v: Guid) -> Self {
        self.header_set(470, v.guid() as u32);
        self.header_set(471, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_1(mut self, v: Guid) -> Self {
        self.header_set(472, v.guid() as u32);
        self.header_set(473, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_2(mut self, v: Guid) -> Self {
        self.header_set(474, v.guid() as u32);
        self.header_set(475, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_3(mut self, v: Guid) -> Self {
        self.header_set(476, v.guid() as u32);
        self.header_set(477, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_4(mut self, v: Guid) -> Self {
        self.header_set(478, v.guid() as u32);
        self.header_set(479, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_5(mut self, v: Guid) -> Self {
        self.header_set(480, v.guid() as u32);
        self.header_set(481, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_6(mut self, v: Guid) -> Self {
        self.header_set(482, v.guid() as u32);
        self.header_set(483, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_7(mut self, v: Guid) -> Self {
        self.header_set(484, v.guid() as u32);
        self.header_set(485, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_8(mut self, v: Guid) -> Self {
        self.header_set(486, v.guid() as u32);
        self.header_set(487, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_9(mut self, v: Guid) -> Self {
        self.header_set(488, v.guid() as u32);
        self.header_set(489, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_10(mut self, v: Guid) -> Self {
        self.header_set(490, v.guid() as u32);
        self.header_set(491, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_11(mut self, v: Guid) -> Self {
        self.header_set(492, v.guid() as u32);
        self.header_set(493, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_12(mut self, v: Guid) -> Self {
        self.header_set(494, v.guid() as u32);
        self.header_set(495, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_1(mut self, v: Guid) -> Self {
        self.header_set(496, v.guid() as u32);
        self.header_set(497, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_2(mut self, v: Guid) -> Self {
        self.header_set(498, v.guid() as u32);
        self.header_set(499, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_3(mut self, v: Guid) -> Self {
        self.header_set(500, v.guid() as u32);
        self.header_set(501, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_4(mut self, v: Guid) -> Self {
        self.header_set(502, v.guid() as u32);
        self.header_set(503, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_5(mut self, v: Guid) -> Self {
        self.header_set(504, v.guid() as u32);
        self.header_set(505, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_6(mut self, v: Guid) -> Self {
        self.header_set(506, v.guid() as u32);
        self.header_set(507, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_7(mut self, v: Guid) -> Self {
        self.header_set(508, v.guid() as u32);
        self.header_set(509, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_8(mut self, v: Guid) -> Self {
        self.header_set(510, v.guid() as u32);
        self.header_set(511, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_9(mut self, v: Guid) -> Self {
        self.header_set(512, v.guid() as u32);
        self.header_set(513, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_10(mut self, v: Guid) -> Self {
        self.header_set(514, v.guid() as u32);
        self.header_set(515, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_11(mut self, v: Guid) -> Self {
        self.header_set(516, v.guid() as u32);
        self.header_set(517, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_12(mut self, v: Guid) -> Self {
        self.header_set(518, v.guid() as u32);
        self.header_set(519, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_13(mut self, v: Guid) -> Self {
        self.header_set(520, v.guid() as u32);
        self.header_set(521, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_14(mut self, v: Guid) -> Self {
        self.header_set(522, v.guid() as u32);
        self.header_set(523, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_15(mut self, v: Guid) -> Self {
        self.header_set(524, v.guid() as u32);
        self.header_set(525, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_16(mut self, v: Guid) -> Self {
        self.header_set(526, v.guid() as u32);
        self.header_set(527, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_17(mut self, v: Guid) -> Self {
        self.header_set(528, v.guid() as u32);
        self.header_set(529, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_18(mut self, v: Guid) -> Self {
        self.header_set(530, v.guid() as u32);
        self.header_set(531, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_19(mut self, v: Guid) -> Self {
        self.header_set(532, v.guid() as u32);
        self.header_set(533, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_20(mut self, v: Guid) -> Self {
        self.header_set(534, v.guid() as u32);
        self.header_set(535, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_21(mut self, v: Guid) -> Self {
        self.header_set(536, v.guid() as u32);
        self.header_set(537, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_22(mut self, v: Guid) -> Self {
        self.header_set(538, v.guid() as u32);
        self.header_set(539, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_23(mut self, v: Guid) -> Self {
        self.header_set(540, v.guid() as u32);
        self.header_set(541, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_24(mut self, v: Guid) -> Self {
        self.header_set(542, v.guid() as u32);
        self.header_set(543, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_25(mut self, v: Guid) -> Self {
        self.header_set(544, v.guid() as u32);
        self.header_set(545, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_26(mut self, v: Guid) -> Self {
        self.header_set(546, v.guid() as u32);
        self.header_set(547, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_27(mut self, v: Guid) -> Self {
        self.header_set(548, v.guid() as u32);
        self.header_set(549, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_28(mut self, v: Guid) -> Self {
        self.header_set(550, v.guid() as u32);
        self.header_set(551, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_29(mut self, v: Guid) -> Self {
        self.header_set(552, v.guid() as u32);
        self.header_set(553, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_30(mut self, v: Guid) -> Self {
        self.header_set(554, v.guid() as u32);
        self.header_set(555, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_31(mut self, v: Guid) -> Self {
        self.header_set(556, v.guid() as u32);
        self.header_set(557, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_32(mut self, v: Guid) -> Self {
        self.header_set(558, v.guid() as u32);
        self.header_set(559, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_1(mut self, v: Guid) -> Self {
        self.header_set(560, v.guid() as u32);
        self.header_set(561, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_2(mut self, v: Guid) -> Self {
        self.header_set(562, v.guid() as u32);
        self.header_set(563, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_3(mut self, v: Guid) -> Self {
        self.header_set(564, v.guid() as u32);
        self.header_set(565, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_4(mut self, v: Guid) -> Self {
        self.header_set(566, v.guid() as u32);
        self.header_set(567, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_5(mut self, v: Guid) -> Self {
        self.header_set(568, v.guid() as u32);
        self.header_set(569, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_6(mut self, v: Guid) -> Self {
        self.header_set(570, v.guid() as u32);
        self.header_set(571, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_7(mut self, v: Guid) -> Self {
        self.header_set(572, v.guid() as u32);
        self.header_set(573, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_8(mut self, v: Guid) -> Self {
        self.header_set(574, v.guid() as u32);
        self.header_set(575, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_9(mut self, v: Guid) -> Self {
        self.header_set(576, v.guid() as u32);
        self.header_set(577, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_10(mut self, v: Guid) -> Self {
        self.header_set(578, v.guid() as u32);
        self.header_set(579, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_11(mut self, v: Guid) -> Self {
        self.header_set(580, v.guid() as u32);
        self.header_set(581, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_12(mut self, v: Guid) -> Self {
        self.header_set(582, v.guid() as u32);
        self.header_set(583, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_13(mut self, v: Guid) -> Self {
        self.header_set(584, v.guid() as u32);
        self.header_set(585, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_14(mut self, v: Guid) -> Self {
        self.header_set(586, v.guid() as u32);
        self.header_set(587, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_15(mut self, v: Guid) -> Self {
        self.header_set(588, v.guid() as u32);
        self.header_set(589, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_16(mut self, v: Guid) -> Self {
        self.header_set(590, v.guid() as u32);
        self.header_set(591, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_17(mut self, v: Guid) -> Self {
        self.header_set(592, v.guid() as u32);
        self.header_set(593, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_18(mut self, v: Guid) -> Self {
        self.header_set(594, v.guid() as u32);
        self.header_set(595, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_19(mut self, v: Guid) -> Self {
        self.header_set(596, v.guid() as u32);
        self.header_set(597, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_20(mut self, v: Guid) -> Self {
        self.header_set(598, v.guid() as u32);
        self.header_set(599, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_21(mut self, v: Guid) -> Self {
        self.header_set(600, v.guid() as u32);
        self.header_set(601, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_22(mut self, v: Guid) -> Self {
        self.header_set(602, v.guid() as u32);
        self.header_set(603, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_23(mut self, v: Guid) -> Self {
        self.header_set(604, v.guid() as u32);
        self.header_set(605, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_24(mut self, v: Guid) -> Self {
        self.header_set(606, v.guid() as u32);
        self.header_set(607, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_25(mut self, v: Guid) -> Self {
        self.header_set(608, v.guid() as u32);
        self.header_set(609, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_26(mut self, v: Guid) -> Self {
        self.header_set(610, v.guid() as u32);
        self.header_set(611, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_27(mut self, v: Guid) -> Self {
        self.header_set(612, v.guid() as u32);
        self.header_set(613, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_28(mut self, v: Guid) -> Self {
        self.header_set(614, v.guid() as u32);
        self.header_set(615, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_29(mut self, v: Guid) -> Self {
        self.header_set(616, v.guid() as u32);
        self.header_set(617, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_30(mut self, v: Guid) -> Self {
        self.header_set(618, v.guid() as u32);
        self.header_set(619, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_31(mut self, v: Guid) -> Self {
        self.header_set(620, v.guid() as u32);
        self.header_set(621, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_currencytoken_slot_32(mut self, v: Guid) -> Self {
        self.header_set(622, v.guid() as u32);
        self.header_set(623, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_farsight(mut self, v: Guid) -> Self {
        self.header_set(624, v.guid() as u32);
        self.header_set(625, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_known_titles(mut self, v: Guid) -> Self {
        self.header_set(626, v.guid() as u32);
        self.header_set(627, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_known_titles1(mut self, v: Guid) -> Self {
        self.header_set(628, v.guid() as u32);
        self.header_set(629, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_known_titles2(mut self, v: Guid) -> Self {
        self.header_set(630, v.guid() as u32);
        self.header_set(631, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_known_currencies(mut self, v: Guid) -> Self {
        self.header_set(632, v.guid() as u32);
        self.header_set(633, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_xp(mut self, v: i32) -> Self {
        self.header_set(634, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_next_level_xp(mut self, v: i32) -> Self {
        self.header_set(635, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_skill_info(mut self, skill_info: crate::wrath::SkillInfo, index: SkillInfoIndex) -> Self {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
        self
    }

    pub fn set_player_character_points1(mut self, v: i32) -> Self {
        self.header_set(1020, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_character_points2(mut self, v: i32) -> Self {
        self.header_set(1021, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_track_creatures(mut self, v: i32) -> Self {
        self.header_set(1022, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_track_resources(mut self, v: i32) -> Self {
        self.header_set(1023, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_block_percentage(mut self, v: f32) -> Self {
        self.header_set(1024, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_dodge_percentage(mut self, v: f32) -> Self {
        self.header_set(1025, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_parry_percentage(mut self, v: f32) -> Self {
        self.header_set(1026, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_expertise(mut self, v: i32) -> Self {
        self.header_set(1027, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_offhand_expertise(mut self, v: i32) -> Self {
        self.header_set(1028, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_crit_percentage(mut self, v: f32) -> Self {
        self.header_set(1029, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_ranged_crit_percentage(mut self, v: f32) -> Self {
        self.header_set(1030, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_offhand_crit_percentage(mut self, v: f32) -> Self {
        self.header_set(1031, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_spell_crit_percentage1(mut self, v: f32) -> Self {
        self.header_set(1032, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_shield_block(mut self, v: i32) -> Self {
        self.header_set(1039, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_shield_block_crit_percentage(mut self, v: f32) -> Self {
        self.header_set(1040, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_explored_zones_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1041, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_rest_state_experience(mut self, v: i32) -> Self {
        self.header_set(1169, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_coinage(mut self, v: i32) -> Self {
        self.header_set(1170, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_damage_done_pos(mut self, v: i32) -> Self {
        self.header_set(1171, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_damage_done_neg(mut self, v: i32) -> Self {
        self.header_set(1178, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_damage_done_pct(mut self, v: i32) -> Self {
        self.header_set(1185, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_healing_done_pos(mut self, v: i32) -> Self {
        self.header_set(1192, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_healing_pct(mut self, v: f32) -> Self {
        self.header_set(1193, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_healing_done_pct(mut self, v: f32) -> Self {
        self.header_set(1194, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_target_resistance(mut self, v: i32) -> Self {
        self.header_set(1195, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_target_physical_resistance(mut self, v: i32) -> Self {
        self.header_set(1196, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_features(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1197, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_ammo_id(mut self, v: i32) -> Self {
        self.header_set(1198, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_self_res_spell(mut self, v: i32) -> Self {
        self.header_set(1199, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_pvp_medals(mut self, v: i32) -> Self {
        self.header_set(1200, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_buyback_price_1(mut self, v: i32) -> Self {
        self.header_set(1201, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_buyback_timestamp_1(mut self, v: i32) -> Self {
        self.header_set(1213, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_kills(mut self, a: u16, b: u16) -> Self {
        self.header_set(1225, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_today_contribution(mut self, v: i32) -> Self {
        self.header_set(1226, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_yesterday_contribution(mut self, v: i32) -> Self {
        self.header_set(1227, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_lifetime_honorbale_kills(mut self, v: i32) -> Self {
        self.header_set(1228, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_bytes2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1229, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_watched_faction_index(mut self, v: i32) -> Self {
        self.header_set(1230, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_combat_rating_1(mut self, v: i32) -> Self {
        self.header_set(1231, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_arena_team_info_1_1(mut self, v: i32) -> Self {
        self.header_set(1256, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_honor_currency(mut self, v: i32) -> Self {
        self.header_set(1277, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_arena_currency(mut self, v: i32) -> Self {
        self.header_set(1278, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_max_level(mut self, v: i32) -> Self {
        self.header_set(1279, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_daily_quests_1(mut self, v: i32) -> Self {
        self.header_set(1280, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_rune_regen_1(mut self, v: f32) -> Self {
        self.header_set(1305, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_no_reagent_cost_1(mut self, v: i32) -> Self {
        self.header_set(1309, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_glyph_slots_1(mut self, v: i32) -> Self {
        self.header_set(1312, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_glyphs_1(mut self, v: i32) -> Self {
        self.header_set(1318, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_glyphs_enabled(mut self, v: i32) -> Self {
        self.header_set(1324, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_pet_spell_power(mut self, v: i32) -> Self {
        self.header_set(1325, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateGameObjectBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_gameobject_displayid(mut self, v: i32) -> Self {
        self.header_set(8, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_flags(mut self, v: i32) -> Self {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_parentrotation(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_dynamic(mut self, a: u16, b: u16) -> Self {
        self.header_set(14, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_gameobject_faction(mut self, v: i32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_level(mut self, v: i32) -> Self {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(17, u32::from_le_bytes([a, b, c, d]));
        self
    }

}

impl UpdateDynamicObjectBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_dynamicobject_caster(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_dynamicobject_bytes(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(8, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_dynamicobject_spellid(mut self, v: i32) -> Self {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_radius(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_casttime(mut self, v: i32) -> Self {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateCorpseBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_owner(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_party(mut self, v: Guid) -> Self {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_display_id(mut self, v: i32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_item(mut self, v: i32) -> Self {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(30, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(31, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_guild(mut self, v: i32) -> Self {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_flags(mut self, v: i32) -> Self {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_dynamic_flags(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateItem {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_type(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_item_owner(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn item_owner(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_contained(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn item_contained(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_creator(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn item_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_giftcreator(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn item_giftcreator(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_stack_count(&mut self, v: i32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_stack_count(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_item_duration(&mut self, v: i32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_duration(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_item_spell_charges(&mut self, v: i32) {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_spell_charges(&self) -> Option<i32> {
        self.values.get(&16).map(|v| *v as i32)
    }

    pub fn set_item_flags(&mut self, v: i32) {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_flags(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_1_1(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_1_1(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_1_3(&mut self, a: u16, b: u16) {
        self.header_set(24, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_1_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&24) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_2_1(&mut self, v: i32) {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_2_1(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_2_3(&mut self, a: u16, b: u16) {
        self.header_set(27, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_2_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&27) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_3_1(&mut self, v: i32) {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_3_1(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_3_3(&mut self, a: u16, b: u16) {
        self.header_set(30, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_3_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&30) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_4_1(&mut self, v: i32) {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_4_1(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_4_3(&mut self, a: u16, b: u16) {
        self.header_set(33, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_4_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&33) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_5_1(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_5_1(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_5_3(&mut self, a: u16, b: u16) {
        self.header_set(36, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_5_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_6_1(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_6_1(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_6_3(&mut self, a: u16, b: u16) {
        self.header_set(39, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_6_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&39) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_7_1(&mut self, v: i32) {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_7_1(&self) -> Option<i32> {
        self.values.get(&40).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_7_3(&mut self, a: u16, b: u16) {
        self.header_set(42, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_7_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&42) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_8_1(&mut self, v: i32) {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_8_1(&self) -> Option<i32> {
        self.values.get(&43).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_8_3(&mut self, a: u16, b: u16) {
        self.header_set(45, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_8_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&45) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_9_1(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_9_1(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_9_3(&mut self, a: u16, b: u16) {
        self.header_set(48, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_9_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&48) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_10_1(&mut self, v: i32) {
        self.header_set(49, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_10_1(&self) -> Option<i32> {
        self.values.get(&49).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_10_3(&mut self, a: u16, b: u16) {
        self.header_set(51, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_10_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&51) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_11_1(&mut self, v: i32) {
        self.header_set(52, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_11_1(&self) -> Option<i32> {
        self.values.get(&52).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_11_3(&mut self, a: u16, b: u16) {
        self.header_set(54, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_11_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&54) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_12_1(&mut self, v: i32) {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_12_1(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_12_3(&mut self, a: u16, b: u16) {
        self.header_set(57, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_12_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&57) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.values.get(&58).map(|v| *v as i32)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.values.get(&60).map(|v| *v as i32)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.header_set(61, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.values.get(&61).map(|v| *v as i32)
    }

    pub fn set_item_create_played_time(&mut self, v: i32) {
        self.header_set(62, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_create_played_time(&self) -> Option<i32> {
        self.values.get(&62).map(|v| *v as i32)
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdateContainer {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_type(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_item_owner(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn item_owner(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_contained(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn item_contained(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_creator(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn item_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_giftcreator(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn item_giftcreator(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_item_stack_count(&mut self, v: i32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_stack_count(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_item_duration(&mut self, v: i32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_duration(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_item_spell_charges(&mut self, v: i32) {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_spell_charges(&self) -> Option<i32> {
        self.values.get(&16).map(|v| *v as i32)
    }

    pub fn set_item_flags(&mut self, v: i32) {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_flags(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_1_1(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_1_1(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_1_3(&mut self, a: u16, b: u16) {
        self.header_set(24, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_1_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&24) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_2_1(&mut self, v: i32) {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_2_1(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_2_3(&mut self, a: u16, b: u16) {
        self.header_set(27, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_2_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&27) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_3_1(&mut self, v: i32) {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_3_1(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_3_3(&mut self, a: u16, b: u16) {
        self.header_set(30, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_3_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&30) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_4_1(&mut self, v: i32) {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_4_1(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_4_3(&mut self, a: u16, b: u16) {
        self.header_set(33, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_4_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&33) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_5_1(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_5_1(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_5_3(&mut self, a: u16, b: u16) {
        self.header_set(36, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_5_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_6_1(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_6_1(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_6_3(&mut self, a: u16, b: u16) {
        self.header_set(39, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_6_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&39) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_7_1(&mut self, v: i32) {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_7_1(&self) -> Option<i32> {
        self.values.get(&40).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_7_3(&mut self, a: u16, b: u16) {
        self.header_set(42, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_7_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&42) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_8_1(&mut self, v: i32) {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_8_1(&self) -> Option<i32> {
        self.values.get(&43).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_8_3(&mut self, a: u16, b: u16) {
        self.header_set(45, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_8_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&45) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_9_1(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_9_1(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_9_3(&mut self, a: u16, b: u16) {
        self.header_set(48, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_9_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&48) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_10_1(&mut self, v: i32) {
        self.header_set(49, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_10_1(&self) -> Option<i32> {
        self.values.get(&49).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_10_3(&mut self, a: u16, b: u16) {
        self.header_set(51, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_10_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&51) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_11_1(&mut self, v: i32) {
        self.header_set(52, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_11_1(&self) -> Option<i32> {
        self.values.get(&52).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_11_3(&mut self, a: u16, b: u16) {
        self.header_set(54, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_11_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&54) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_enchantment_12_1(&mut self, v: i32) {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment_12_1(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_item_enchantment_12_3(&mut self, a: u16, b: u16) {
        self.header_set(57, crate::util::u16s_to_u32(a, b));
    }

    pub fn item_enchantment_12_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&57) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.values.get(&58).map(|v| *v as i32)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.values.get(&60).map(|v| *v as i32)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.header_set(61, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.values.get(&61).map(|v| *v as i32)
    }

    pub fn set_item_create_played_time(&mut self, v: i32) {
        self.header_set(62, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_create_played_time(&self) -> Option<i32> {
        self.values.get(&62).map(|v| *v as i32)
    }

    pub fn set_container_num_slots(&mut self, v: i32) {
        self.header_set(64, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn container_num_slots(&self) -> Option<i32> {
        self.values.get(&64).map(|v| *v as i32)
    }

    pub fn set_container_slot_1(&mut self, v: Guid) {
        self.header_set(66, v.guid() as u32);
        self.header_set(67, (v.guid() >> 32) as u32);
    }

    pub fn container_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&66);
        let upper = self.values.get(&67);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdateUnit {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_type(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_charm(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn unit_charm(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_summon(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn unit_summon(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_critter(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn unit_critter(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_charmedby(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn unit_charmedby(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_summonedby(&mut self, v: Guid) {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
    }

    pub fn unit_summonedby(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_createdby(&mut self, v: Guid) {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
    }

    pub fn unit_createdby(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_target(&mut self, v: Guid) {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
    }

    pub fn unit_target(&self) -> Option<Guid> {
        let lower = self.values.get(&18);
        let upper = self.values.get(&19);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_channel_object(&mut self, v: Guid) {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
    }

    pub fn unit_channel_object(&self) -> Option<Guid> {
        let lower = self.values.get(&20);
        let upper = self.values.get(&21);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_bytes_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.header_set(23, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
    }

    pub fn unit_bytes_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&23) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_health(&mut self, v: i32) {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_health(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_power1(&mut self, v: i32) {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power1(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_power2(&mut self, v: i32) {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power2(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_power3(&mut self, v: i32) {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power3(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_power4(&mut self, v: i32) {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power4(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_power5(&mut self, v: i32) {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power5(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_power6(&mut self, v: i32) {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power6(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_power7(&mut self, v: i32) {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power7(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_maxhealth(&mut self, v: i32) {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxhealth(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower1(&mut self, v: i32) {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower1(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower2(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower2(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower3(&mut self, v: i32) {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower3(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower4(&mut self, v: i32) {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower4(&self) -> Option<i32> {
        self.values.get(&36).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower5(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower5(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower6(&mut self, v: i32) {
        self.header_set(38, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower6(&self) -> Option<i32> {
        self.values.get(&38).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower7(&mut self, v: i32) {
        self.header_set(39, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower7(&self) -> Option<i32> {
        self.values.get(&39).map(|v| *v as i32)
    }

    pub fn set_unit_power_regen_flat_modifier(&mut self, v: f32) {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_regen_flat_modifier(&self) -> Option<f32> {
        self.values.get(&40).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_power_regen_interrupted_flat_modifier(&mut self, v: f32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_regen_interrupted_flat_modifier(&self) -> Option<f32> {
        self.values.get(&47).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_level(&mut self, v: i32) {
        self.header_set(54, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_level(&self) -> Option<i32> {
        self.values.get(&54).map(|v| *v as i32)
    }

    pub fn set_unit_factiontemplate(&mut self, v: i32) {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_factiontemplate(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_unit_virtual_item_slot_id(&mut self, v: i32) {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_virtual_item_slot_id(&self) -> Option<i32> {
        self.values.get(&56).map(|v| *v as i32)
    }

    pub fn set_unit_flags(&mut self, v: i32) {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_flags(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
    }

    pub fn set_unit_flags_2(&mut self, v: i32) {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_flags_2(&self) -> Option<i32> {
        self.values.get(&60).map(|v| *v as i32)
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.header_set(61, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.values.get(&61).map(|v| *v as i32)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.header_set(62, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.values.get(&62).map(|v| *v as i32)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.header_set(64, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.values.get(&64).map(|v| *v as i32)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.header_set(65, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.values.get(&65).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.header_set(66, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.values.get(&66).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.header_set(67, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.values.get(&67).map(|v| *v as i32)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.header_set(68, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.values.get(&68).map(|v| *v as i32)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.header_set(69, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.values.get(&69).map(|v| *v as i32)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.header_set(70, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.values.get(&70).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.header_set(71, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.values.get(&71).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.header_set(72, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.values.get(&72).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.header_set(73, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.values.get(&73).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_bytes_1(&mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) {
        self.header_set(74, u32::from_le_bytes([stand_state.as_int(), unknown1, unknown2, unknown3]));
    }

    pub fn unit_bytes_1(&self) -> Option<(UnitStandState, u8, u8, u8)> {
        if let Some(v) = self.values.get(&74) {
            let v = v.to_le_bytes();
            let (stand_state, unknown1, unknown2, unknown3) = (v[0], v[1], v[2], v[3]);
            Some((stand_state.try_into().unwrap(), unknown1, unknown2, unknown3))
        } else {
            None
        }
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.header_set(75, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.values.get(&75).map(|v| *v as i32)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.header_set(76, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.values.get(&76).map(|v| *v as i32)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.header_set(77, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.values.get(&77).map(|v| *v as i32)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.header_set(78, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.values.get(&78).map(|v| *v as i32)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.header_set(79, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.values.get(&79).map(|v| *v as i32)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.header_set(80, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.values.get(&80).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.header_set(81, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.values.get(&81).map(|v| *v as i32)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.header_set(82, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.values.get(&82).map(|v| *v as i32)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.header_set(83, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.values.get(&83).map(|v| *v as i32)
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.header_set(84, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.values.get(&84).map(|v| *v as i32)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.header_set(85, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.values.get(&85).map(|v| *v as i32)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.header_set(86, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.values.get(&86).map(|v| *v as i32)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.header_set(87, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.values.get(&87).map(|v| *v as i32)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.header_set(88, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.values.get(&88).map(|v| *v as i32)
    }

    pub fn set_unit_posstat0(&mut self, v: i32) {
        self.header_set(89, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat0(&self) -> Option<i32> {
        self.values.get(&89).map(|v| *v as i32)
    }

    pub fn set_unit_posstat1(&mut self, v: i32) {
        self.header_set(90, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat1(&self) -> Option<i32> {
        self.values.get(&90).map(|v| *v as i32)
    }

    pub fn set_unit_posstat2(&mut self, v: i32) {
        self.header_set(91, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat2(&self) -> Option<i32> {
        self.values.get(&91).map(|v| *v as i32)
    }

    pub fn set_unit_posstat3(&mut self, v: i32) {
        self.header_set(92, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat3(&self) -> Option<i32> {
        self.values.get(&92).map(|v| *v as i32)
    }

    pub fn set_unit_posstat4(&mut self, v: i32) {
        self.header_set(93, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat4(&self) -> Option<i32> {
        self.values.get(&93).map(|v| *v as i32)
    }

    pub fn set_unit_negstat0(&mut self, v: i32) {
        self.header_set(94, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat0(&self) -> Option<i32> {
        self.values.get(&94).map(|v| *v as i32)
    }

    pub fn set_unit_negstat1(&mut self, v: i32) {
        self.header_set(95, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat1(&self) -> Option<i32> {
        self.values.get(&95).map(|v| *v as i32)
    }

    pub fn set_unit_negstat2(&mut self, v: i32) {
        self.header_set(96, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat2(&self) -> Option<i32> {
        self.values.get(&96).map(|v| *v as i32)
    }

    pub fn set_unit_negstat3(&mut self, v: i32) {
        self.header_set(97, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat3(&self) -> Option<i32> {
        self.values.get(&97).map(|v| *v as i32)
    }

    pub fn set_unit_negstat4(&mut self, v: i32) {
        self.header_set(98, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat4(&self) -> Option<i32> {
        self.values.get(&98).map(|v| *v as i32)
    }

    pub fn set_unit_resistances(&mut self, v: i32) {
        self.header_set(99, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_resistances(&self) -> Option<i32> {
        self.values.get(&99).map(|v| *v as i32)
    }

    pub fn set_unit_resistancebuffmodspositive(&mut self, v: i32) {
        self.header_set(106, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_resistancebuffmodspositive(&self) -> Option<i32> {
        self.values.get(&106).map(|v| *v as i32)
    }

    pub fn set_unit_resistancebuffmodsnegative(&mut self, v: i32) {
        self.header_set(113, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_resistancebuffmodsnegative(&self) -> Option<i32> {
        self.values.get(&113).map(|v| *v as i32)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.header_set(120, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.values.get(&120).map(|v| *v as i32)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.header_set(121, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.values.get(&121).map(|v| *v as i32)
    }

    pub fn set_unit_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(122, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&122) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.header_set(123, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.values.get(&123).map(|v| *v as i32)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(124, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&124) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&125).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.values.get(&126).map(|v| *v as i32)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(127, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&127) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&128).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.values.get(&129).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.values.get(&130).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.values.get(&131).map(|v| *v as i32)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.header_set(138, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.values.get(&138).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxhealthmodifier(&mut self, v: f32) {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxhealthmodifier(&self) -> Option<f32> {
        self.values.get(&145).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_hoverheight(&mut self, v: f32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_hoverheight(&self) -> Option<f32> {
        self.values.get(&146).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdatePlayer {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_type(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_charm(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn unit_charm(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_summon(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn unit_summon(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_critter(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn unit_critter(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_charmedby(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn unit_charmedby(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_summonedby(&mut self, v: Guid) {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
    }

    pub fn unit_summonedby(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_createdby(&mut self, v: Guid) {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
    }

    pub fn unit_createdby(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_target(&mut self, v: Guid) {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
    }

    pub fn unit_target(&self) -> Option<Guid> {
        let lower = self.values.get(&18);
        let upper = self.values.get(&19);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_channel_object(&mut self, v: Guid) {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
    }

    pub fn unit_channel_object(&self) -> Option<Guid> {
        let lower = self.values.get(&20);
        let upper = self.values.get(&21);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_bytes_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.header_set(23, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
    }

    pub fn unit_bytes_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&23) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_health(&mut self, v: i32) {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_health(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_power1(&mut self, v: i32) {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power1(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_power2(&mut self, v: i32) {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power2(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_power3(&mut self, v: i32) {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power3(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_power4(&mut self, v: i32) {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power4(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_power5(&mut self, v: i32) {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power5(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_power6(&mut self, v: i32) {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power6(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_power7(&mut self, v: i32) {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power7(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_maxhealth(&mut self, v: i32) {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxhealth(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower1(&mut self, v: i32) {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower1(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower2(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower2(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower3(&mut self, v: i32) {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower3(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower4(&mut self, v: i32) {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower4(&self) -> Option<i32> {
        self.values.get(&36).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower5(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower5(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower6(&mut self, v: i32) {
        self.header_set(38, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower6(&self) -> Option<i32> {
        self.values.get(&38).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower7(&mut self, v: i32) {
        self.header_set(39, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower7(&self) -> Option<i32> {
        self.values.get(&39).map(|v| *v as i32)
    }

    pub fn set_unit_power_regen_flat_modifier(&mut self, v: f32) {
        self.header_set(40, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_regen_flat_modifier(&self) -> Option<f32> {
        self.values.get(&40).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_power_regen_interrupted_flat_modifier(&mut self, v: f32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_regen_interrupted_flat_modifier(&self) -> Option<f32> {
        self.values.get(&47).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_level(&mut self, v: i32) {
        self.header_set(54, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_level(&self) -> Option<i32> {
        self.values.get(&54).map(|v| *v as i32)
    }

    pub fn set_unit_factiontemplate(&mut self, v: i32) {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_factiontemplate(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_unit_virtual_item_slot_id(&mut self, v: i32) {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_virtual_item_slot_id(&self) -> Option<i32> {
        self.values.get(&56).map(|v| *v as i32)
    }

    pub fn set_unit_flags(&mut self, v: i32) {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_flags(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
    }

    pub fn set_unit_flags_2(&mut self, v: i32) {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_flags_2(&self) -> Option<i32> {
        self.values.get(&60).map(|v| *v as i32)
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.header_set(61, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.values.get(&61).map(|v| *v as i32)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.header_set(62, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.values.get(&62).map(|v| *v as i32)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.header_set(64, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.values.get(&64).map(|v| *v as i32)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.header_set(65, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.values.get(&65).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.header_set(66, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.values.get(&66).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.header_set(67, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.values.get(&67).map(|v| *v as i32)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.header_set(68, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.values.get(&68).map(|v| *v as i32)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.header_set(69, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.values.get(&69).map(|v| *v as i32)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.header_set(70, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.values.get(&70).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.header_set(71, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.values.get(&71).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.header_set(72, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.values.get(&72).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.header_set(73, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.values.get(&73).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_bytes_1(&mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) {
        self.header_set(74, u32::from_le_bytes([stand_state.as_int(), unknown1, unknown2, unknown3]));
    }

    pub fn unit_bytes_1(&self) -> Option<(UnitStandState, u8, u8, u8)> {
        if let Some(v) = self.values.get(&74) {
            let v = v.to_le_bytes();
            let (stand_state, unknown1, unknown2, unknown3) = (v[0], v[1], v[2], v[3]);
            Some((stand_state.try_into().unwrap(), unknown1, unknown2, unknown3))
        } else {
            None
        }
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.header_set(75, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.values.get(&75).map(|v| *v as i32)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.header_set(76, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.values.get(&76).map(|v| *v as i32)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.header_set(77, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.values.get(&77).map(|v| *v as i32)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.header_set(78, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.values.get(&78).map(|v| *v as i32)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.header_set(79, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.values.get(&79).map(|v| *v as i32)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.header_set(80, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.values.get(&80).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.header_set(81, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.values.get(&81).map(|v| *v as i32)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.header_set(82, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.values.get(&82).map(|v| *v as i32)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.header_set(83, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.values.get(&83).map(|v| *v as i32)
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.header_set(84, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.values.get(&84).map(|v| *v as i32)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.header_set(85, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.values.get(&85).map(|v| *v as i32)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.header_set(86, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.values.get(&86).map(|v| *v as i32)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.header_set(87, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.values.get(&87).map(|v| *v as i32)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.header_set(88, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.values.get(&88).map(|v| *v as i32)
    }

    pub fn set_unit_posstat0(&mut self, v: i32) {
        self.header_set(89, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat0(&self) -> Option<i32> {
        self.values.get(&89).map(|v| *v as i32)
    }

    pub fn set_unit_posstat1(&mut self, v: i32) {
        self.header_set(90, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat1(&self) -> Option<i32> {
        self.values.get(&90).map(|v| *v as i32)
    }

    pub fn set_unit_posstat2(&mut self, v: i32) {
        self.header_set(91, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat2(&self) -> Option<i32> {
        self.values.get(&91).map(|v| *v as i32)
    }

    pub fn set_unit_posstat3(&mut self, v: i32) {
        self.header_set(92, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat3(&self) -> Option<i32> {
        self.values.get(&92).map(|v| *v as i32)
    }

    pub fn set_unit_posstat4(&mut self, v: i32) {
        self.header_set(93, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat4(&self) -> Option<i32> {
        self.values.get(&93).map(|v| *v as i32)
    }

    pub fn set_unit_negstat0(&mut self, v: i32) {
        self.header_set(94, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat0(&self) -> Option<i32> {
        self.values.get(&94).map(|v| *v as i32)
    }

    pub fn set_unit_negstat1(&mut self, v: i32) {
        self.header_set(95, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat1(&self) -> Option<i32> {
        self.values.get(&95).map(|v| *v as i32)
    }

    pub fn set_unit_negstat2(&mut self, v: i32) {
        self.header_set(96, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat2(&self) -> Option<i32> {
        self.values.get(&96).map(|v| *v as i32)
    }

    pub fn set_unit_negstat3(&mut self, v: i32) {
        self.header_set(97, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat3(&self) -> Option<i32> {
        self.values.get(&97).map(|v| *v as i32)
    }

    pub fn set_unit_negstat4(&mut self, v: i32) {
        self.header_set(98, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat4(&self) -> Option<i32> {
        self.values.get(&98).map(|v| *v as i32)
    }

    pub fn set_unit_resistances(&mut self, v: i32) {
        self.header_set(99, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_resistances(&self) -> Option<i32> {
        self.values.get(&99).map(|v| *v as i32)
    }

    pub fn set_unit_resistancebuffmodspositive(&mut self, v: i32) {
        self.header_set(106, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_resistancebuffmodspositive(&self) -> Option<i32> {
        self.values.get(&106).map(|v| *v as i32)
    }

    pub fn set_unit_resistancebuffmodsnegative(&mut self, v: i32) {
        self.header_set(113, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_resistancebuffmodsnegative(&self) -> Option<i32> {
        self.values.get(&113).map(|v| *v as i32)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.header_set(120, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.values.get(&120).map(|v| *v as i32)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.header_set(121, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.values.get(&121).map(|v| *v as i32)
    }

    pub fn set_unit_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(122, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&122) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.header_set(123, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.values.get(&123).map(|v| *v as i32)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(124, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&124) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&125).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.values.get(&126).map(|v| *v as i32)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(127, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&127) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&128).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.values.get(&129).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.values.get(&130).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.values.get(&131).map(|v| *v as i32)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.header_set(138, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.values.get(&138).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxhealthmodifier(&mut self, v: f32) {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxhealthmodifier(&self) -> Option<f32> {
        self.values.get(&145).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_hoverheight(&mut self, v: f32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_hoverheight(&self) -> Option<f32> {
        self.values.get(&146).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_duel_arbiter(&mut self, v: Guid) {
        self.header_set(148, v.guid() as u32);
        self.header_set(149, (v.guid() >> 32) as u32);
    }

    pub fn player_duel_arbiter(&self) -> Option<Guid> {
        let lower = self.values.get(&148);
        let upper = self.values.get(&149);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_flags(&mut self, v: i32) {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_flags(&self) -> Option<i32> {
        self.values.get(&150).map(|v| *v as i32)
    }

    pub fn set_player_guildid(&mut self, v: i32) {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_guildid(&self) -> Option<i32> {
        self.values.get(&151).map(|v| *v as i32)
    }

    pub fn set_player_guildrank(&mut self, v: i32) {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_guildrank(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_player_field_bytes(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(153, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_field_bytes(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&153) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(154, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&154) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_bytes_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(155, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_bytes_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&155) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_duel_team(&mut self, v: i32) {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_duel_team(&self) -> Option<i32> {
        self.values.get(&156).map(|v| *v as i32)
    }

    pub fn set_player_guild_timestamp(&mut self, v: i32) {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_guild_timestamp(&self) -> Option<i32> {
        self.values.get(&157).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_1_1(&mut self, v: i32) {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_1_1(&self) -> Option<i32> {
        self.values.get(&158).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_1_2(&mut self, v: i32) {
        self.header_set(159, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_1_2(&self) -> Option<i32> {
        self.values.get(&159).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_1_3(&mut self, a: u16, b: u16) {
        self.header_set(160, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_1_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&160) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_1_4(&mut self, v: i32) {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_1_4(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_2_1(&mut self, v: i32) {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_2_1(&self) -> Option<i32> {
        self.values.get(&163).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_2_2(&mut self, v: i32) {
        self.header_set(164, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_2_2(&self) -> Option<i32> {
        self.values.get(&164).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_2_3(&mut self, a: u16, b: u16) {
        self.header_set(165, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_2_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&165) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_2_5(&mut self, v: i32) {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_2_5(&self) -> Option<i32> {
        self.values.get(&167).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_3_1(&mut self, v: i32) {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_3_1(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_3_2(&mut self, v: i32) {
        self.header_set(169, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_3_2(&self) -> Option<i32> {
        self.values.get(&169).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_3_3(&mut self, a: u16, b: u16) {
        self.header_set(170, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_3_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&170) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_3_5(&mut self, v: i32) {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_3_5(&self) -> Option<i32> {
        self.values.get(&172).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_4_1(&mut self, v: i32) {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_4_1(&self) -> Option<i32> {
        self.values.get(&173).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_4_2(&mut self, v: i32) {
        self.header_set(174, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_4_2(&self) -> Option<i32> {
        self.values.get(&174).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_4_3(&mut self, a: u16, b: u16) {
        self.header_set(175, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_4_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&175) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_4_5(&mut self, v: i32) {
        self.header_set(177, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_4_5(&self) -> Option<i32> {
        self.values.get(&177).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_5_1(&mut self, v: i32) {
        self.header_set(178, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_5_1(&self) -> Option<i32> {
        self.values.get(&178).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_5_2(&mut self, v: i32) {
        self.header_set(179, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_5_2(&self) -> Option<i32> {
        self.values.get(&179).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_5_3(&mut self, a: u16, b: u16) {
        self.header_set(180, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_5_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&180) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_5_5(&mut self, v: i32) {
        self.header_set(182, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_5_5(&self) -> Option<i32> {
        self.values.get(&182).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_6_1(&mut self, v: i32) {
        self.header_set(183, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_6_1(&self) -> Option<i32> {
        self.values.get(&183).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_6_2(&mut self, v: i32) {
        self.header_set(184, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_6_2(&self) -> Option<i32> {
        self.values.get(&184).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_6_3(&mut self, a: u16, b: u16) {
        self.header_set(185, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_6_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&185) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_6_5(&mut self, v: i32) {
        self.header_set(187, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_6_5(&self) -> Option<i32> {
        self.values.get(&187).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_7_1(&mut self, v: i32) {
        self.header_set(188, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_7_1(&self) -> Option<i32> {
        self.values.get(&188).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_7_2(&mut self, v: i32) {
        self.header_set(189, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_7_2(&self) -> Option<i32> {
        self.values.get(&189).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_7_3(&mut self, a: u16, b: u16) {
        self.header_set(190, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_7_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&190) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_7_5(&mut self, v: i32) {
        self.header_set(192, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_7_5(&self) -> Option<i32> {
        self.values.get(&192).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_8_1(&mut self, v: i32) {
        self.header_set(193, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_8_1(&self) -> Option<i32> {
        self.values.get(&193).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_8_2(&mut self, v: i32) {
        self.header_set(194, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_8_2(&self) -> Option<i32> {
        self.values.get(&194).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_8_3(&mut self, a: u16, b: u16) {
        self.header_set(195, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_8_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&195) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_8_5(&mut self, v: i32) {
        self.header_set(197, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_8_5(&self) -> Option<i32> {
        self.values.get(&197).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_9_1(&mut self, v: i32) {
        self.header_set(198, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_9_1(&self) -> Option<i32> {
        self.values.get(&198).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_9_2(&mut self, v: i32) {
        self.header_set(199, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_9_2(&self) -> Option<i32> {
        self.values.get(&199).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_9_3(&mut self, a: u16, b: u16) {
        self.header_set(200, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_9_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&200) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_9_5(&mut self, v: i32) {
        self.header_set(202, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_9_5(&self) -> Option<i32> {
        self.values.get(&202).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_10_1(&mut self, v: i32) {
        self.header_set(203, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_10_1(&self) -> Option<i32> {
        self.values.get(&203).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_10_2(&mut self, v: i32) {
        self.header_set(204, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_10_2(&self) -> Option<i32> {
        self.values.get(&204).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_10_3(&mut self, a: u16, b: u16) {
        self.header_set(205, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_10_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&205) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_10_5(&mut self, v: i32) {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_10_5(&self) -> Option<i32> {
        self.values.get(&207).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_11_1(&mut self, v: i32) {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_11_1(&self) -> Option<i32> {
        self.values.get(&208).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_11_2(&mut self, v: i32) {
        self.header_set(209, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_11_2(&self) -> Option<i32> {
        self.values.get(&209).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_11_3(&mut self, a: u16, b: u16) {
        self.header_set(210, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_11_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&210) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_11_5(&mut self, v: i32) {
        self.header_set(212, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_11_5(&self) -> Option<i32> {
        self.values.get(&212).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_12_1(&mut self, v: i32) {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_12_1(&self) -> Option<i32> {
        self.values.get(&213).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_12_2(&mut self, v: i32) {
        self.header_set(214, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_12_2(&self) -> Option<i32> {
        self.values.get(&214).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_12_3(&mut self, a: u16, b: u16) {
        self.header_set(215, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_12_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&215) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_12_5(&mut self, v: i32) {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_12_5(&self) -> Option<i32> {
        self.values.get(&217).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_13_1(&mut self, v: i32) {
        self.header_set(218, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_13_1(&self) -> Option<i32> {
        self.values.get(&218).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_13_2(&mut self, v: i32) {
        self.header_set(219, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_13_2(&self) -> Option<i32> {
        self.values.get(&219).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_13_3(&mut self, a: u16, b: u16) {
        self.header_set(220, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_13_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&220) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_13_5(&mut self, v: i32) {
        self.header_set(222, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_13_5(&self) -> Option<i32> {
        self.values.get(&222).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_14_1(&mut self, v: i32) {
        self.header_set(223, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_14_1(&self) -> Option<i32> {
        self.values.get(&223).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_14_2(&mut self, v: i32) {
        self.header_set(224, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_14_2(&self) -> Option<i32> {
        self.values.get(&224).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_14_3(&mut self, a: u16, b: u16) {
        self.header_set(225, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_14_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&225) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_14_5(&mut self, v: i32) {
        self.header_set(227, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_14_5(&self) -> Option<i32> {
        self.values.get(&227).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_15_1(&mut self, v: i32) {
        self.header_set(228, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_15_1(&self) -> Option<i32> {
        self.values.get(&228).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_15_2(&mut self, v: i32) {
        self.header_set(229, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_15_2(&self) -> Option<i32> {
        self.values.get(&229).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_15_3(&mut self, a: u16, b: u16) {
        self.header_set(230, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_15_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&230) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_15_5(&mut self, v: i32) {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_15_5(&self) -> Option<i32> {
        self.values.get(&232).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_16_1(&mut self, v: i32) {
        self.header_set(233, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_16_1(&self) -> Option<i32> {
        self.values.get(&233).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_16_2(&mut self, v: i32) {
        self.header_set(234, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_16_2(&self) -> Option<i32> {
        self.values.get(&234).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_16_3(&mut self, a: u16, b: u16) {
        self.header_set(235, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_16_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&235) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_16_5(&mut self, v: i32) {
        self.header_set(237, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_16_5(&self) -> Option<i32> {
        self.values.get(&237).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_17_1(&mut self, v: i32) {
        self.header_set(238, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_17_1(&self) -> Option<i32> {
        self.values.get(&238).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_17_2(&mut self, v: i32) {
        self.header_set(239, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_17_2(&self) -> Option<i32> {
        self.values.get(&239).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_17_3(&mut self, a: u16, b: u16) {
        self.header_set(240, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_17_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&240) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_17_5(&mut self, v: i32) {
        self.header_set(242, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_17_5(&self) -> Option<i32> {
        self.values.get(&242).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_18_1(&mut self, v: i32) {
        self.header_set(243, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_18_1(&self) -> Option<i32> {
        self.values.get(&243).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_18_2(&mut self, v: i32) {
        self.header_set(244, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_18_2(&self) -> Option<i32> {
        self.values.get(&244).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_18_3(&mut self, a: u16, b: u16) {
        self.header_set(245, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_18_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&245) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_18_5(&mut self, v: i32) {
        self.header_set(247, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_18_5(&self) -> Option<i32> {
        self.values.get(&247).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_19_1(&mut self, v: i32) {
        self.header_set(248, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_19_1(&self) -> Option<i32> {
        self.values.get(&248).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_19_2(&mut self, v: i32) {
        self.header_set(249, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_19_2(&self) -> Option<i32> {
        self.values.get(&249).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_19_3(&mut self, a: u16, b: u16) {
        self.header_set(250, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_19_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&250) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_19_5(&mut self, v: i32) {
        self.header_set(252, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_19_5(&self) -> Option<i32> {
        self.values.get(&252).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_20_1(&mut self, v: i32) {
        self.header_set(253, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_20_1(&self) -> Option<i32> {
        self.values.get(&253).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_20_2(&mut self, v: i32) {
        self.header_set(254, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_20_2(&self) -> Option<i32> {
        self.values.get(&254).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_20_3(&mut self, a: u16, b: u16) {
        self.header_set(255, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_20_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&255) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_20_5(&mut self, v: i32) {
        self.header_set(257, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_20_5(&self) -> Option<i32> {
        self.values.get(&257).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_21_1(&mut self, v: i32) {
        self.header_set(258, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_21_1(&self) -> Option<i32> {
        self.values.get(&258).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_21_2(&mut self, v: i32) {
        self.header_set(259, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_21_2(&self) -> Option<i32> {
        self.values.get(&259).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_21_3(&mut self, a: u16, b: u16) {
        self.header_set(260, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_21_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&260) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_21_5(&mut self, v: i32) {
        self.header_set(262, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_21_5(&self) -> Option<i32> {
        self.values.get(&262).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_22_1(&mut self, v: i32) {
        self.header_set(263, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_22_1(&self) -> Option<i32> {
        self.values.get(&263).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_22_2(&mut self, v: i32) {
        self.header_set(264, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_22_2(&self) -> Option<i32> {
        self.values.get(&264).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_22_3(&mut self, a: u16, b: u16) {
        self.header_set(265, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_22_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&265) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_22_5(&mut self, v: i32) {
        self.header_set(267, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_22_5(&self) -> Option<i32> {
        self.values.get(&267).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_23_1(&mut self, v: i32) {
        self.header_set(268, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_23_1(&self) -> Option<i32> {
        self.values.get(&268).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_23_2(&mut self, v: i32) {
        self.header_set(269, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_23_2(&self) -> Option<i32> {
        self.values.get(&269).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_23_3(&mut self, a: u16, b: u16) {
        self.header_set(270, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_23_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&270) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_23_5(&mut self, v: i32) {
        self.header_set(272, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_23_5(&self) -> Option<i32> {
        self.values.get(&272).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_24_1(&mut self, v: i32) {
        self.header_set(273, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_24_1(&self) -> Option<i32> {
        self.values.get(&273).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_24_2(&mut self, v: i32) {
        self.header_set(274, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_24_2(&self) -> Option<i32> {
        self.values.get(&274).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_24_3(&mut self, a: u16, b: u16) {
        self.header_set(275, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_24_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&275) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_24_5(&mut self, v: i32) {
        self.header_set(277, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_24_5(&self) -> Option<i32> {
        self.values.get(&277).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_25_1(&mut self, v: i32) {
        self.header_set(278, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_25_1(&self) -> Option<i32> {
        self.values.get(&278).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_25_2(&mut self, v: i32) {
        self.header_set(279, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_25_2(&self) -> Option<i32> {
        self.values.get(&279).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_25_3(&mut self, a: u16, b: u16) {
        self.header_set(280, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_quest_log_25_3(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&280) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_25_5(&mut self, v: i32) {
        self.header_set(282, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_25_5(&self) -> Option<i32> {
        self.values.get(&282).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_1_entryid(&mut self, v: i32) {
        self.header_set(283, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_1_entryid(&self) -> Option<i32> {
        self.values.get(&283).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_1_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(284, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_1_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&284) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_2_entryid(&mut self, v: i32) {
        self.header_set(285, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_2_entryid(&self) -> Option<i32> {
        self.values.get(&285).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_2_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(286, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_2_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&286) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_3_entryid(&mut self, v: i32) {
        self.header_set(287, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_3_entryid(&self) -> Option<i32> {
        self.values.get(&287).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_3_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(288, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_3_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&288) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_4_entryid(&mut self, v: i32) {
        self.header_set(289, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_4_entryid(&self) -> Option<i32> {
        self.values.get(&289).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_4_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(290, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_4_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&290) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_5_entryid(&mut self, v: i32) {
        self.header_set(291, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_5_entryid(&self) -> Option<i32> {
        self.values.get(&291).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_5_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(292, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_5_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&292) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_6_entryid(&mut self, v: i32) {
        self.header_set(293, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_6_entryid(&self) -> Option<i32> {
        self.values.get(&293).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_6_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(294, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_6_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&294) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_7_entryid(&mut self, v: i32) {
        self.header_set(295, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_7_entryid(&self) -> Option<i32> {
        self.values.get(&295).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_7_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(296, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_7_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&296) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_8_entryid(&mut self, v: i32) {
        self.header_set(297, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_8_entryid(&self) -> Option<i32> {
        self.values.get(&297).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_8_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(298, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_8_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&298) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_9_entryid(&mut self, v: i32) {
        self.header_set(299, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_9_entryid(&self) -> Option<i32> {
        self.values.get(&299).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_9_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(300, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_9_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&300) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_10_entryid(&mut self, v: i32) {
        self.header_set(301, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_10_entryid(&self) -> Option<i32> {
        self.values.get(&301).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_10_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(302, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_10_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&302) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_11_entryid(&mut self, v: i32) {
        self.header_set(303, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_11_entryid(&self) -> Option<i32> {
        self.values.get(&303).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_11_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(304, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_11_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&304) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_12_entryid(&mut self, v: i32) {
        self.header_set(305, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_12_entryid(&self) -> Option<i32> {
        self.values.get(&305).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_12_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(306, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_12_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&306) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_13_entryid(&mut self, v: i32) {
        self.header_set(307, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_13_entryid(&self) -> Option<i32> {
        self.values.get(&307).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_13_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(308, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_13_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&308) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_14_entryid(&mut self, v: i32) {
        self.header_set(309, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_14_entryid(&self) -> Option<i32> {
        self.values.get(&309).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_14_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(310, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_14_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&310) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_15_entryid(&mut self, v: i32) {
        self.header_set(311, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_15_entryid(&self) -> Option<i32> {
        self.values.get(&311).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_15_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(312, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_15_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&312) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_16_entryid(&mut self, v: i32) {
        self.header_set(313, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_16_entryid(&self) -> Option<i32> {
        self.values.get(&313).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_16_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(314, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_16_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&314) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_17_entryid(&mut self, v: i32) {
        self.header_set(315, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_17_entryid(&self) -> Option<i32> {
        self.values.get(&315).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_17_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(316, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_17_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&316) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_18_entryid(&mut self, v: i32) {
        self.header_set(317, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_18_entryid(&self) -> Option<i32> {
        self.values.get(&317).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_18_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(318, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_18_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&318) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_19_entryid(&mut self, v: i32) {
        self.header_set(319, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_19_entryid(&self) -> Option<i32> {
        self.values.get(&319).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_19_enchantment(&mut self, a: u16, b: u16) {
        self.header_set(320, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_19_enchantment(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&320) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_chosen_title(&mut self, v: i32) {
        self.header_set(321, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_chosen_title(&self) -> Option<i32> {
        self.values.get(&321).map(|v| *v as i32)
    }

    pub fn set_player_fake_inebriation(&mut self, v: i32) {
        self.header_set(322, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_fake_inebriation(&self) -> Option<i32> {
        self.values.get(&322).map(|v| *v as i32)
    }

    pub fn set_player_inv_slot_head1(&mut self, v: Guid) {
        self.header_set(324, v.guid() as u32);
        self.header_set(325, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head1(&self) -> Option<Guid> {
        let lower = self.values.get(&324);
        let upper = self.values.get(&325);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head2(&mut self, v: Guid) {
        self.header_set(326, v.guid() as u32);
        self.header_set(327, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head2(&self) -> Option<Guid> {
        let lower = self.values.get(&326);
        let upper = self.values.get(&327);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head3(&mut self, v: Guid) {
        self.header_set(328, v.guid() as u32);
        self.header_set(329, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head3(&self) -> Option<Guid> {
        let lower = self.values.get(&328);
        let upper = self.values.get(&329);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head4(&mut self, v: Guid) {
        self.header_set(330, v.guid() as u32);
        self.header_set(331, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head4(&self) -> Option<Guid> {
        let lower = self.values.get(&330);
        let upper = self.values.get(&331);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head5(&mut self, v: Guid) {
        self.header_set(332, v.guid() as u32);
        self.header_set(333, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head5(&self) -> Option<Guid> {
        let lower = self.values.get(&332);
        let upper = self.values.get(&333);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head6(&mut self, v: Guid) {
        self.header_set(334, v.guid() as u32);
        self.header_set(335, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head6(&self) -> Option<Guid> {
        let lower = self.values.get(&334);
        let upper = self.values.get(&335);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head7(&mut self, v: Guid) {
        self.header_set(336, v.guid() as u32);
        self.header_set(337, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head7(&self) -> Option<Guid> {
        let lower = self.values.get(&336);
        let upper = self.values.get(&337);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head8(&mut self, v: Guid) {
        self.header_set(338, v.guid() as u32);
        self.header_set(339, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head8(&self) -> Option<Guid> {
        let lower = self.values.get(&338);
        let upper = self.values.get(&339);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head9(&mut self, v: Guid) {
        self.header_set(340, v.guid() as u32);
        self.header_set(341, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head9(&self) -> Option<Guid> {
        let lower = self.values.get(&340);
        let upper = self.values.get(&341);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head10(&mut self, v: Guid) {
        self.header_set(342, v.guid() as u32);
        self.header_set(343, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head10(&self) -> Option<Guid> {
        let lower = self.values.get(&342);
        let upper = self.values.get(&343);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head11(&mut self, v: Guid) {
        self.header_set(344, v.guid() as u32);
        self.header_set(345, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head11(&self) -> Option<Guid> {
        let lower = self.values.get(&344);
        let upper = self.values.get(&345);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head12(&mut self, v: Guid) {
        self.header_set(346, v.guid() as u32);
        self.header_set(347, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head12(&self) -> Option<Guid> {
        let lower = self.values.get(&346);
        let upper = self.values.get(&347);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head13(&mut self, v: Guid) {
        self.header_set(348, v.guid() as u32);
        self.header_set(349, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head13(&self) -> Option<Guid> {
        let lower = self.values.get(&348);
        let upper = self.values.get(&349);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head14(&mut self, v: Guid) {
        self.header_set(350, v.guid() as u32);
        self.header_set(351, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head14(&self) -> Option<Guid> {
        let lower = self.values.get(&350);
        let upper = self.values.get(&351);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head15(&mut self, v: Guid) {
        self.header_set(352, v.guid() as u32);
        self.header_set(353, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head15(&self) -> Option<Guid> {
        let lower = self.values.get(&352);
        let upper = self.values.get(&353);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head16(&mut self, v: Guid) {
        self.header_set(354, v.guid() as u32);
        self.header_set(355, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head16(&self) -> Option<Guid> {
        let lower = self.values.get(&354);
        let upper = self.values.get(&355);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head17(&mut self, v: Guid) {
        self.header_set(356, v.guid() as u32);
        self.header_set(357, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head17(&self) -> Option<Guid> {
        let lower = self.values.get(&356);
        let upper = self.values.get(&357);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head18(&mut self, v: Guid) {
        self.header_set(358, v.guid() as u32);
        self.header_set(359, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head18(&self) -> Option<Guid> {
        let lower = self.values.get(&358);
        let upper = self.values.get(&359);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head19(&mut self, v: Guid) {
        self.header_set(360, v.guid() as u32);
        self.header_set(361, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head19(&self) -> Option<Guid> {
        let lower = self.values.get(&360);
        let upper = self.values.get(&361);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head20(&mut self, v: Guid) {
        self.header_set(362, v.guid() as u32);
        self.header_set(363, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head20(&self) -> Option<Guid> {
        let lower = self.values.get(&362);
        let upper = self.values.get(&363);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head21(&mut self, v: Guid) {
        self.header_set(364, v.guid() as u32);
        self.header_set(365, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head21(&self) -> Option<Guid> {
        let lower = self.values.get(&364);
        let upper = self.values.get(&365);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head22(&mut self, v: Guid) {
        self.header_set(366, v.guid() as u32);
        self.header_set(367, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head22(&self) -> Option<Guid> {
        let lower = self.values.get(&366);
        let upper = self.values.get(&367);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_head23(&mut self, v: Guid) {
        self.header_set(368, v.guid() as u32);
        self.header_set(369, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head23(&self) -> Option<Guid> {
        let lower = self.values.get(&368);
        let upper = self.values.get(&369);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_1(&mut self, v: Guid) {
        self.header_set(370, v.guid() as u32);
        self.header_set(371, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&370);
        let upper = self.values.get(&371);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_2(&mut self, v: Guid) {
        self.header_set(372, v.guid() as u32);
        self.header_set(373, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&372);
        let upper = self.values.get(&373);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_3(&mut self, v: Guid) {
        self.header_set(374, v.guid() as u32);
        self.header_set(375, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&374);
        let upper = self.values.get(&375);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_4(&mut self, v: Guid) {
        self.header_set(376, v.guid() as u32);
        self.header_set(377, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&376);
        let upper = self.values.get(&377);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_5(&mut self, v: Guid) {
        self.header_set(378, v.guid() as u32);
        self.header_set(379, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&378);
        let upper = self.values.get(&379);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_6(&mut self, v: Guid) {
        self.header_set(380, v.guid() as u32);
        self.header_set(381, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&380);
        let upper = self.values.get(&381);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_7(&mut self, v: Guid) {
        self.header_set(382, v.guid() as u32);
        self.header_set(383, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&382);
        let upper = self.values.get(&383);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_8(&mut self, v: Guid) {
        self.header_set(384, v.guid() as u32);
        self.header_set(385, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_8(&self) -> Option<Guid> {
        let lower = self.values.get(&384);
        let upper = self.values.get(&385);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_9(&mut self, v: Guid) {
        self.header_set(386, v.guid() as u32);
        self.header_set(387, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_9(&self) -> Option<Guid> {
        let lower = self.values.get(&386);
        let upper = self.values.get(&387);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_10(&mut self, v: Guid) {
        self.header_set(388, v.guid() as u32);
        self.header_set(389, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_10(&self) -> Option<Guid> {
        let lower = self.values.get(&388);
        let upper = self.values.get(&389);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_11(&mut self, v: Guid) {
        self.header_set(390, v.guid() as u32);
        self.header_set(391, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_11(&self) -> Option<Guid> {
        let lower = self.values.get(&390);
        let upper = self.values.get(&391);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_12(&mut self, v: Guid) {
        self.header_set(392, v.guid() as u32);
        self.header_set(393, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_12(&self) -> Option<Guid> {
        let lower = self.values.get(&392);
        let upper = self.values.get(&393);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_13(&mut self, v: Guid) {
        self.header_set(394, v.guid() as u32);
        self.header_set(395, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_13(&self) -> Option<Guid> {
        let lower = self.values.get(&394);
        let upper = self.values.get(&395);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_14(&mut self, v: Guid) {
        self.header_set(396, v.guid() as u32);
        self.header_set(397, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_14(&self) -> Option<Guid> {
        let lower = self.values.get(&396);
        let upper = self.values.get(&397);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_15(&mut self, v: Guid) {
        self.header_set(398, v.guid() as u32);
        self.header_set(399, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_15(&self) -> Option<Guid> {
        let lower = self.values.get(&398);
        let upper = self.values.get(&399);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_16(&mut self, v: Guid) {
        self.header_set(400, v.guid() as u32);
        self.header_set(401, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_16(&self) -> Option<Guid> {
        let lower = self.values.get(&400);
        let upper = self.values.get(&401);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_1(&mut self, v: Guid) {
        self.header_set(402, v.guid() as u32);
        self.header_set(403, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&402);
        let upper = self.values.get(&403);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_2(&mut self, v: Guid) {
        self.header_set(404, v.guid() as u32);
        self.header_set(405, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&404);
        let upper = self.values.get(&405);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_3(&mut self, v: Guid) {
        self.header_set(406, v.guid() as u32);
        self.header_set(407, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&406);
        let upper = self.values.get(&407);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_4(&mut self, v: Guid) {
        self.header_set(408, v.guid() as u32);
        self.header_set(409, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&408);
        let upper = self.values.get(&409);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_5(&mut self, v: Guid) {
        self.header_set(410, v.guid() as u32);
        self.header_set(411, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&410);
        let upper = self.values.get(&411);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_6(&mut self, v: Guid) {
        self.header_set(412, v.guid() as u32);
        self.header_set(413, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&412);
        let upper = self.values.get(&413);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_7(&mut self, v: Guid) {
        self.header_set(414, v.guid() as u32);
        self.header_set(415, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&414);
        let upper = self.values.get(&415);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_8(&mut self, v: Guid) {
        self.header_set(416, v.guid() as u32);
        self.header_set(417, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_8(&self) -> Option<Guid> {
        let lower = self.values.get(&416);
        let upper = self.values.get(&417);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_9(&mut self, v: Guid) {
        self.header_set(418, v.guid() as u32);
        self.header_set(419, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_9(&self) -> Option<Guid> {
        let lower = self.values.get(&418);
        let upper = self.values.get(&419);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_10(&mut self, v: Guid) {
        self.header_set(420, v.guid() as u32);
        self.header_set(421, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_10(&self) -> Option<Guid> {
        let lower = self.values.get(&420);
        let upper = self.values.get(&421);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_11(&mut self, v: Guid) {
        self.header_set(422, v.guid() as u32);
        self.header_set(423, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_11(&self) -> Option<Guid> {
        let lower = self.values.get(&422);
        let upper = self.values.get(&423);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_12(&mut self, v: Guid) {
        self.header_set(424, v.guid() as u32);
        self.header_set(425, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_12(&self) -> Option<Guid> {
        let lower = self.values.get(&424);
        let upper = self.values.get(&425);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_13(&mut self, v: Guid) {
        self.header_set(426, v.guid() as u32);
        self.header_set(427, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_13(&self) -> Option<Guid> {
        let lower = self.values.get(&426);
        let upper = self.values.get(&427);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_14(&mut self, v: Guid) {
        self.header_set(428, v.guid() as u32);
        self.header_set(429, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_14(&self) -> Option<Guid> {
        let lower = self.values.get(&428);
        let upper = self.values.get(&429);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_15(&mut self, v: Guid) {
        self.header_set(430, v.guid() as u32);
        self.header_set(431, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_15(&self) -> Option<Guid> {
        let lower = self.values.get(&430);
        let upper = self.values.get(&431);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_16(&mut self, v: Guid) {
        self.header_set(432, v.guid() as u32);
        self.header_set(433, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_16(&self) -> Option<Guid> {
        let lower = self.values.get(&432);
        let upper = self.values.get(&433);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_17(&mut self, v: Guid) {
        self.header_set(434, v.guid() as u32);
        self.header_set(435, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_17(&self) -> Option<Guid> {
        let lower = self.values.get(&434);
        let upper = self.values.get(&435);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_18(&mut self, v: Guid) {
        self.header_set(436, v.guid() as u32);
        self.header_set(437, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_18(&self) -> Option<Guid> {
        let lower = self.values.get(&436);
        let upper = self.values.get(&437);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_19(&mut self, v: Guid) {
        self.header_set(438, v.guid() as u32);
        self.header_set(439, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_19(&self) -> Option<Guid> {
        let lower = self.values.get(&438);
        let upper = self.values.get(&439);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_20(&mut self, v: Guid) {
        self.header_set(440, v.guid() as u32);
        self.header_set(441, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_20(&self) -> Option<Guid> {
        let lower = self.values.get(&440);
        let upper = self.values.get(&441);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_21(&mut self, v: Guid) {
        self.header_set(442, v.guid() as u32);
        self.header_set(443, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_21(&self) -> Option<Guid> {
        let lower = self.values.get(&442);
        let upper = self.values.get(&443);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_22(&mut self, v: Guid) {
        self.header_set(444, v.guid() as u32);
        self.header_set(445, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_22(&self) -> Option<Guid> {
        let lower = self.values.get(&444);
        let upper = self.values.get(&445);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_23(&mut self, v: Guid) {
        self.header_set(446, v.guid() as u32);
        self.header_set(447, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_23(&self) -> Option<Guid> {
        let lower = self.values.get(&446);
        let upper = self.values.get(&447);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_24(&mut self, v: Guid) {
        self.header_set(448, v.guid() as u32);
        self.header_set(449, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_24(&self) -> Option<Guid> {
        let lower = self.values.get(&448);
        let upper = self.values.get(&449);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_25(&mut self, v: Guid) {
        self.header_set(450, v.guid() as u32);
        self.header_set(451, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_25(&self) -> Option<Guid> {
        let lower = self.values.get(&450);
        let upper = self.values.get(&451);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_26(&mut self, v: Guid) {
        self.header_set(452, v.guid() as u32);
        self.header_set(453, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_26(&self) -> Option<Guid> {
        let lower = self.values.get(&452);
        let upper = self.values.get(&453);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_27(&mut self, v: Guid) {
        self.header_set(454, v.guid() as u32);
        self.header_set(455, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_27(&self) -> Option<Guid> {
        let lower = self.values.get(&454);
        let upper = self.values.get(&455);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_28(&mut self, v: Guid) {
        self.header_set(456, v.guid() as u32);
        self.header_set(457, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_28(&self) -> Option<Guid> {
        let lower = self.values.get(&456);
        let upper = self.values.get(&457);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_1(&mut self, v: Guid) {
        self.header_set(458, v.guid() as u32);
        self.header_set(459, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&458);
        let upper = self.values.get(&459);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_2(&mut self, v: Guid) {
        self.header_set(460, v.guid() as u32);
        self.header_set(461, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&460);
        let upper = self.values.get(&461);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_3(&mut self, v: Guid) {
        self.header_set(462, v.guid() as u32);
        self.header_set(463, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&462);
        let upper = self.values.get(&463);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_4(&mut self, v: Guid) {
        self.header_set(464, v.guid() as u32);
        self.header_set(465, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&464);
        let upper = self.values.get(&465);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_5(&mut self, v: Guid) {
        self.header_set(466, v.guid() as u32);
        self.header_set(467, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&466);
        let upper = self.values.get(&467);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_6(&mut self, v: Guid) {
        self.header_set(468, v.guid() as u32);
        self.header_set(469, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&468);
        let upper = self.values.get(&469);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_7(&mut self, v: Guid) {
        self.header_set(470, v.guid() as u32);
        self.header_set(471, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&470);
        let upper = self.values.get(&471);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_1(&mut self, v: Guid) {
        self.header_set(472, v.guid() as u32);
        self.header_set(473, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&472);
        let upper = self.values.get(&473);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_2(&mut self, v: Guid) {
        self.header_set(474, v.guid() as u32);
        self.header_set(475, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&474);
        let upper = self.values.get(&475);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_3(&mut self, v: Guid) {
        self.header_set(476, v.guid() as u32);
        self.header_set(477, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&476);
        let upper = self.values.get(&477);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_4(&mut self, v: Guid) {
        self.header_set(478, v.guid() as u32);
        self.header_set(479, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&478);
        let upper = self.values.get(&479);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_5(&mut self, v: Guid) {
        self.header_set(480, v.guid() as u32);
        self.header_set(481, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&480);
        let upper = self.values.get(&481);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_6(&mut self, v: Guid) {
        self.header_set(482, v.guid() as u32);
        self.header_set(483, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&482);
        let upper = self.values.get(&483);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_7(&mut self, v: Guid) {
        self.header_set(484, v.guid() as u32);
        self.header_set(485, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&484);
        let upper = self.values.get(&485);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_8(&mut self, v: Guid) {
        self.header_set(486, v.guid() as u32);
        self.header_set(487, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_8(&self) -> Option<Guid> {
        let lower = self.values.get(&486);
        let upper = self.values.get(&487);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_9(&mut self, v: Guid) {
        self.header_set(488, v.guid() as u32);
        self.header_set(489, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_9(&self) -> Option<Guid> {
        let lower = self.values.get(&488);
        let upper = self.values.get(&489);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_10(&mut self, v: Guid) {
        self.header_set(490, v.guid() as u32);
        self.header_set(491, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_10(&self) -> Option<Guid> {
        let lower = self.values.get(&490);
        let upper = self.values.get(&491);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_11(&mut self, v: Guid) {
        self.header_set(492, v.guid() as u32);
        self.header_set(493, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_11(&self) -> Option<Guid> {
        let lower = self.values.get(&492);
        let upper = self.values.get(&493);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_12(&mut self, v: Guid) {
        self.header_set(494, v.guid() as u32);
        self.header_set(495, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_12(&self) -> Option<Guid> {
        let lower = self.values.get(&494);
        let upper = self.values.get(&495);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_1(&mut self, v: Guid) {
        self.header_set(496, v.guid() as u32);
        self.header_set(497, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&496);
        let upper = self.values.get(&497);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_2(&mut self, v: Guid) {
        self.header_set(498, v.guid() as u32);
        self.header_set(499, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&498);
        let upper = self.values.get(&499);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_3(&mut self, v: Guid) {
        self.header_set(500, v.guid() as u32);
        self.header_set(501, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&500);
        let upper = self.values.get(&501);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_4(&mut self, v: Guid) {
        self.header_set(502, v.guid() as u32);
        self.header_set(503, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&502);
        let upper = self.values.get(&503);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_5(&mut self, v: Guid) {
        self.header_set(504, v.guid() as u32);
        self.header_set(505, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&504);
        let upper = self.values.get(&505);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_6(&mut self, v: Guid) {
        self.header_set(506, v.guid() as u32);
        self.header_set(507, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&506);
        let upper = self.values.get(&507);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_7(&mut self, v: Guid) {
        self.header_set(508, v.guid() as u32);
        self.header_set(509, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&508);
        let upper = self.values.get(&509);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_8(&mut self, v: Guid) {
        self.header_set(510, v.guid() as u32);
        self.header_set(511, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_8(&self) -> Option<Guid> {
        let lower = self.values.get(&510);
        let upper = self.values.get(&511);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_9(&mut self, v: Guid) {
        self.header_set(512, v.guid() as u32);
        self.header_set(513, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_9(&self) -> Option<Guid> {
        let lower = self.values.get(&512);
        let upper = self.values.get(&513);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_10(&mut self, v: Guid) {
        self.header_set(514, v.guid() as u32);
        self.header_set(515, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_10(&self) -> Option<Guid> {
        let lower = self.values.get(&514);
        let upper = self.values.get(&515);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_11(&mut self, v: Guid) {
        self.header_set(516, v.guid() as u32);
        self.header_set(517, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_11(&self) -> Option<Guid> {
        let lower = self.values.get(&516);
        let upper = self.values.get(&517);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_12(&mut self, v: Guid) {
        self.header_set(518, v.guid() as u32);
        self.header_set(519, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_12(&self) -> Option<Guid> {
        let lower = self.values.get(&518);
        let upper = self.values.get(&519);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_13(&mut self, v: Guid) {
        self.header_set(520, v.guid() as u32);
        self.header_set(521, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_13(&self) -> Option<Guid> {
        let lower = self.values.get(&520);
        let upper = self.values.get(&521);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_14(&mut self, v: Guid) {
        self.header_set(522, v.guid() as u32);
        self.header_set(523, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_14(&self) -> Option<Guid> {
        let lower = self.values.get(&522);
        let upper = self.values.get(&523);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_15(&mut self, v: Guid) {
        self.header_set(524, v.guid() as u32);
        self.header_set(525, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_15(&self) -> Option<Guid> {
        let lower = self.values.get(&524);
        let upper = self.values.get(&525);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_16(&mut self, v: Guid) {
        self.header_set(526, v.guid() as u32);
        self.header_set(527, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_16(&self) -> Option<Guid> {
        let lower = self.values.get(&526);
        let upper = self.values.get(&527);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_17(&mut self, v: Guid) {
        self.header_set(528, v.guid() as u32);
        self.header_set(529, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_17(&self) -> Option<Guid> {
        let lower = self.values.get(&528);
        let upper = self.values.get(&529);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_18(&mut self, v: Guid) {
        self.header_set(530, v.guid() as u32);
        self.header_set(531, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_18(&self) -> Option<Guid> {
        let lower = self.values.get(&530);
        let upper = self.values.get(&531);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_19(&mut self, v: Guid) {
        self.header_set(532, v.guid() as u32);
        self.header_set(533, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_19(&self) -> Option<Guid> {
        let lower = self.values.get(&532);
        let upper = self.values.get(&533);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_20(&mut self, v: Guid) {
        self.header_set(534, v.guid() as u32);
        self.header_set(535, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_20(&self) -> Option<Guid> {
        let lower = self.values.get(&534);
        let upper = self.values.get(&535);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_21(&mut self, v: Guid) {
        self.header_set(536, v.guid() as u32);
        self.header_set(537, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_21(&self) -> Option<Guid> {
        let lower = self.values.get(&536);
        let upper = self.values.get(&537);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_22(&mut self, v: Guid) {
        self.header_set(538, v.guid() as u32);
        self.header_set(539, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_22(&self) -> Option<Guid> {
        let lower = self.values.get(&538);
        let upper = self.values.get(&539);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_23(&mut self, v: Guid) {
        self.header_set(540, v.guid() as u32);
        self.header_set(541, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_23(&self) -> Option<Guid> {
        let lower = self.values.get(&540);
        let upper = self.values.get(&541);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_24(&mut self, v: Guid) {
        self.header_set(542, v.guid() as u32);
        self.header_set(543, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_24(&self) -> Option<Guid> {
        let lower = self.values.get(&542);
        let upper = self.values.get(&543);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_25(&mut self, v: Guid) {
        self.header_set(544, v.guid() as u32);
        self.header_set(545, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_25(&self) -> Option<Guid> {
        let lower = self.values.get(&544);
        let upper = self.values.get(&545);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_26(&mut self, v: Guid) {
        self.header_set(546, v.guid() as u32);
        self.header_set(547, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_26(&self) -> Option<Guid> {
        let lower = self.values.get(&546);
        let upper = self.values.get(&547);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_27(&mut self, v: Guid) {
        self.header_set(548, v.guid() as u32);
        self.header_set(549, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_27(&self) -> Option<Guid> {
        let lower = self.values.get(&548);
        let upper = self.values.get(&549);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_28(&mut self, v: Guid) {
        self.header_set(550, v.guid() as u32);
        self.header_set(551, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_28(&self) -> Option<Guid> {
        let lower = self.values.get(&550);
        let upper = self.values.get(&551);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_29(&mut self, v: Guid) {
        self.header_set(552, v.guid() as u32);
        self.header_set(553, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_29(&self) -> Option<Guid> {
        let lower = self.values.get(&552);
        let upper = self.values.get(&553);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_30(&mut self, v: Guid) {
        self.header_set(554, v.guid() as u32);
        self.header_set(555, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_30(&self) -> Option<Guid> {
        let lower = self.values.get(&554);
        let upper = self.values.get(&555);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_31(&mut self, v: Guid) {
        self.header_set(556, v.guid() as u32);
        self.header_set(557, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_31(&self) -> Option<Guid> {
        let lower = self.values.get(&556);
        let upper = self.values.get(&557);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_32(&mut self, v: Guid) {
        self.header_set(558, v.guid() as u32);
        self.header_set(559, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_32(&self) -> Option<Guid> {
        let lower = self.values.get(&558);
        let upper = self.values.get(&559);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_1(&mut self, v: Guid) {
        self.header_set(560, v.guid() as u32);
        self.header_set(561, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&560);
        let upper = self.values.get(&561);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_2(&mut self, v: Guid) {
        self.header_set(562, v.guid() as u32);
        self.header_set(563, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&562);
        let upper = self.values.get(&563);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_3(&mut self, v: Guid) {
        self.header_set(564, v.guid() as u32);
        self.header_set(565, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&564);
        let upper = self.values.get(&565);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_4(&mut self, v: Guid) {
        self.header_set(566, v.guid() as u32);
        self.header_set(567, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&566);
        let upper = self.values.get(&567);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_5(&mut self, v: Guid) {
        self.header_set(568, v.guid() as u32);
        self.header_set(569, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&568);
        let upper = self.values.get(&569);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_6(&mut self, v: Guid) {
        self.header_set(570, v.guid() as u32);
        self.header_set(571, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&570);
        let upper = self.values.get(&571);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_7(&mut self, v: Guid) {
        self.header_set(572, v.guid() as u32);
        self.header_set(573, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&572);
        let upper = self.values.get(&573);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_8(&mut self, v: Guid) {
        self.header_set(574, v.guid() as u32);
        self.header_set(575, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_8(&self) -> Option<Guid> {
        let lower = self.values.get(&574);
        let upper = self.values.get(&575);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_9(&mut self, v: Guid) {
        self.header_set(576, v.guid() as u32);
        self.header_set(577, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_9(&self) -> Option<Guid> {
        let lower = self.values.get(&576);
        let upper = self.values.get(&577);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_10(&mut self, v: Guid) {
        self.header_set(578, v.guid() as u32);
        self.header_set(579, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_10(&self) -> Option<Guid> {
        let lower = self.values.get(&578);
        let upper = self.values.get(&579);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_11(&mut self, v: Guid) {
        self.header_set(580, v.guid() as u32);
        self.header_set(581, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_11(&self) -> Option<Guid> {
        let lower = self.values.get(&580);
        let upper = self.values.get(&581);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_12(&mut self, v: Guid) {
        self.header_set(582, v.guid() as u32);
        self.header_set(583, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_12(&self) -> Option<Guid> {
        let lower = self.values.get(&582);
        let upper = self.values.get(&583);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_13(&mut self, v: Guid) {
        self.header_set(584, v.guid() as u32);
        self.header_set(585, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_13(&self) -> Option<Guid> {
        let lower = self.values.get(&584);
        let upper = self.values.get(&585);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_14(&mut self, v: Guid) {
        self.header_set(586, v.guid() as u32);
        self.header_set(587, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_14(&self) -> Option<Guid> {
        let lower = self.values.get(&586);
        let upper = self.values.get(&587);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_15(&mut self, v: Guid) {
        self.header_set(588, v.guid() as u32);
        self.header_set(589, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_15(&self) -> Option<Guid> {
        let lower = self.values.get(&588);
        let upper = self.values.get(&589);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_16(&mut self, v: Guid) {
        self.header_set(590, v.guid() as u32);
        self.header_set(591, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_16(&self) -> Option<Guid> {
        let lower = self.values.get(&590);
        let upper = self.values.get(&591);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_17(&mut self, v: Guid) {
        self.header_set(592, v.guid() as u32);
        self.header_set(593, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_17(&self) -> Option<Guid> {
        let lower = self.values.get(&592);
        let upper = self.values.get(&593);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_18(&mut self, v: Guid) {
        self.header_set(594, v.guid() as u32);
        self.header_set(595, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_18(&self) -> Option<Guid> {
        let lower = self.values.get(&594);
        let upper = self.values.get(&595);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_19(&mut self, v: Guid) {
        self.header_set(596, v.guid() as u32);
        self.header_set(597, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_19(&self) -> Option<Guid> {
        let lower = self.values.get(&596);
        let upper = self.values.get(&597);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_20(&mut self, v: Guid) {
        self.header_set(598, v.guid() as u32);
        self.header_set(599, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_20(&self) -> Option<Guid> {
        let lower = self.values.get(&598);
        let upper = self.values.get(&599);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_21(&mut self, v: Guid) {
        self.header_set(600, v.guid() as u32);
        self.header_set(601, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_21(&self) -> Option<Guid> {
        let lower = self.values.get(&600);
        let upper = self.values.get(&601);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_22(&mut self, v: Guid) {
        self.header_set(602, v.guid() as u32);
        self.header_set(603, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_22(&self) -> Option<Guid> {
        let lower = self.values.get(&602);
        let upper = self.values.get(&603);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_23(&mut self, v: Guid) {
        self.header_set(604, v.guid() as u32);
        self.header_set(605, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_23(&self) -> Option<Guid> {
        let lower = self.values.get(&604);
        let upper = self.values.get(&605);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_24(&mut self, v: Guid) {
        self.header_set(606, v.guid() as u32);
        self.header_set(607, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_24(&self) -> Option<Guid> {
        let lower = self.values.get(&606);
        let upper = self.values.get(&607);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_25(&mut self, v: Guid) {
        self.header_set(608, v.guid() as u32);
        self.header_set(609, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_25(&self) -> Option<Guid> {
        let lower = self.values.get(&608);
        let upper = self.values.get(&609);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_26(&mut self, v: Guid) {
        self.header_set(610, v.guid() as u32);
        self.header_set(611, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_26(&self) -> Option<Guid> {
        let lower = self.values.get(&610);
        let upper = self.values.get(&611);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_27(&mut self, v: Guid) {
        self.header_set(612, v.guid() as u32);
        self.header_set(613, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_27(&self) -> Option<Guid> {
        let lower = self.values.get(&612);
        let upper = self.values.get(&613);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_28(&mut self, v: Guid) {
        self.header_set(614, v.guid() as u32);
        self.header_set(615, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_28(&self) -> Option<Guid> {
        let lower = self.values.get(&614);
        let upper = self.values.get(&615);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_29(&mut self, v: Guid) {
        self.header_set(616, v.guid() as u32);
        self.header_set(617, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_29(&self) -> Option<Guid> {
        let lower = self.values.get(&616);
        let upper = self.values.get(&617);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_30(&mut self, v: Guid) {
        self.header_set(618, v.guid() as u32);
        self.header_set(619, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_30(&self) -> Option<Guid> {
        let lower = self.values.get(&618);
        let upper = self.values.get(&619);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_31(&mut self, v: Guid) {
        self.header_set(620, v.guid() as u32);
        self.header_set(621, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_31(&self) -> Option<Guid> {
        let lower = self.values.get(&620);
        let upper = self.values.get(&621);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_currencytoken_slot_32(&mut self, v: Guid) {
        self.header_set(622, v.guid() as u32);
        self.header_set(623, (v.guid() >> 32) as u32);
    }

    pub fn player_currencytoken_slot_32(&self) -> Option<Guid> {
        let lower = self.values.get(&622);
        let upper = self.values.get(&623);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_farsight(&mut self, v: Guid) {
        self.header_set(624, v.guid() as u32);
        self.header_set(625, (v.guid() >> 32) as u32);
    }

    pub fn player_farsight(&self) -> Option<Guid> {
        let lower = self.values.get(&624);
        let upper = self.values.get(&625);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_known_titles(&mut self, v: Guid) {
        self.header_set(626, v.guid() as u32);
        self.header_set(627, (v.guid() >> 32) as u32);
    }

    pub fn player_known_titles(&self) -> Option<Guid> {
        let lower = self.values.get(&626);
        let upper = self.values.get(&627);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_known_titles1(&mut self, v: Guid) {
        self.header_set(628, v.guid() as u32);
        self.header_set(629, (v.guid() >> 32) as u32);
    }

    pub fn player_known_titles1(&self) -> Option<Guid> {
        let lower = self.values.get(&628);
        let upper = self.values.get(&629);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_known_titles2(&mut self, v: Guid) {
        self.header_set(630, v.guid() as u32);
        self.header_set(631, (v.guid() >> 32) as u32);
    }

    pub fn player_known_titles2(&self) -> Option<Guid> {
        let lower = self.values.get(&630);
        let upper = self.values.get(&631);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_known_currencies(&mut self, v: Guid) {
        self.header_set(632, v.guid() as u32);
        self.header_set(633, (v.guid() >> 32) as u32);
    }

    pub fn player_known_currencies(&self) -> Option<Guid> {
        let lower = self.values.get(&632);
        let upper = self.values.get(&633);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_xp(&mut self, v: i32) {
        self.header_set(634, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_xp(&self) -> Option<i32> {
        self.values.get(&634).map(|v| *v as i32)
    }

    pub fn set_player_next_level_xp(&mut self, v: i32) {
        self.header_set(635, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_next_level_xp(&self) -> Option<i32> {
        self.values.get(&635).map(|v| *v as i32)
    }

    pub fn set_player_skill_info(&mut self, skill_info: crate::wrath::SkillInfo, index: SkillInfoIndex) {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
    }

    pub fn player_skill_info(&self, index: SkillInfoIndex) -> Option<crate::wrath::SkillInfo> {
        crate::wrath::SkillInfo::from_range(self.values.range(index.first()..=index.last()))
    }

    pub fn set_player_character_points1(&mut self, v: i32) {
        self.header_set(1020, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_character_points1(&self) -> Option<i32> {
        self.values.get(&1020).map(|v| *v as i32)
    }

    pub fn set_player_character_points2(&mut self, v: i32) {
        self.header_set(1021, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_character_points2(&self) -> Option<i32> {
        self.values.get(&1021).map(|v| *v as i32)
    }

    pub fn set_player_track_creatures(&mut self, v: i32) {
        self.header_set(1022, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_track_creatures(&self) -> Option<i32> {
        self.values.get(&1022).map(|v| *v as i32)
    }

    pub fn set_player_track_resources(&mut self, v: i32) {
        self.header_set(1023, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_track_resources(&self) -> Option<i32> {
        self.values.get(&1023).map(|v| *v as i32)
    }

    pub fn set_player_block_percentage(&mut self, v: f32) {
        self.header_set(1024, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_block_percentage(&self) -> Option<f32> {
        self.values.get(&1024).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_dodge_percentage(&mut self, v: f32) {
        self.header_set(1025, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_dodge_percentage(&self) -> Option<f32> {
        self.values.get(&1025).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_parry_percentage(&mut self, v: f32) {
        self.header_set(1026, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_parry_percentage(&self) -> Option<f32> {
        self.values.get(&1026).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_expertise(&mut self, v: i32) {
        self.header_set(1027, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_expertise(&self) -> Option<i32> {
        self.values.get(&1027).map(|v| *v as i32)
    }

    pub fn set_player_offhand_expertise(&mut self, v: i32) {
        self.header_set(1028, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_offhand_expertise(&self) -> Option<i32> {
        self.values.get(&1028).map(|v| *v as i32)
    }

    pub fn set_player_crit_percentage(&mut self, v: f32) {
        self.header_set(1029, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_crit_percentage(&self) -> Option<f32> {
        self.values.get(&1029).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_ranged_crit_percentage(&mut self, v: f32) {
        self.header_set(1030, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_ranged_crit_percentage(&self) -> Option<f32> {
        self.values.get(&1030).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_offhand_crit_percentage(&mut self, v: f32) {
        self.header_set(1031, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_offhand_crit_percentage(&self) -> Option<f32> {
        self.values.get(&1031).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_spell_crit_percentage1(&mut self, v: f32) {
        self.header_set(1032, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_spell_crit_percentage1(&self) -> Option<f32> {
        self.values.get(&1032).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_shield_block(&mut self, v: i32) {
        self.header_set(1039, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_shield_block(&self) -> Option<i32> {
        self.values.get(&1039).map(|v| *v as i32)
    }

    pub fn set_player_shield_block_crit_percentage(&mut self, v: f32) {
        self.header_set(1040, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_shield_block_crit_percentage(&self) -> Option<f32> {
        self.values.get(&1040).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_explored_zones_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1041, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_explored_zones_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1041) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_rest_state_experience(&mut self, v: i32) {
        self.header_set(1169, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_rest_state_experience(&self) -> Option<i32> {
        self.values.get(&1169).map(|v| *v as i32)
    }

    pub fn set_player_coinage(&mut self, v: i32) {
        self.header_set(1170, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_coinage(&self) -> Option<i32> {
        self.values.get(&1170).map(|v| *v as i32)
    }

    pub fn set_player_mod_damage_done_pos(&mut self, v: i32) {
        self.header_set(1171, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_damage_done_pos(&self) -> Option<i32> {
        self.values.get(&1171).map(|v| *v as i32)
    }

    pub fn set_player_mod_damage_done_neg(&mut self, v: i32) {
        self.header_set(1178, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_damage_done_neg(&self) -> Option<i32> {
        self.values.get(&1178).map(|v| *v as i32)
    }

    pub fn set_player_mod_damage_done_pct(&mut self, v: i32) {
        self.header_set(1185, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_damage_done_pct(&self) -> Option<i32> {
        self.values.get(&1185).map(|v| *v as i32)
    }

    pub fn set_player_mod_healing_done_pos(&mut self, v: i32) {
        self.header_set(1192, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_healing_done_pos(&self) -> Option<i32> {
        self.values.get(&1192).map(|v| *v as i32)
    }

    pub fn set_player_mod_healing_pct(&mut self, v: f32) {
        self.header_set(1193, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_healing_pct(&self) -> Option<f32> {
        self.values.get(&1193).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_mod_healing_done_pct(&mut self, v: f32) {
        self.header_set(1194, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_healing_done_pct(&self) -> Option<f32> {
        self.values.get(&1194).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_mod_target_resistance(&mut self, v: i32) {
        self.header_set(1195, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_target_resistance(&self) -> Option<i32> {
        self.values.get(&1195).map(|v| *v as i32)
    }

    pub fn set_player_mod_target_physical_resistance(&mut self, v: i32) {
        self.header_set(1196, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_target_physical_resistance(&self) -> Option<i32> {
        self.values.get(&1196).map(|v| *v as i32)
    }

    pub fn set_player_features(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1197, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_features(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1197) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_ammo_id(&mut self, v: i32) {
        self.header_set(1198, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_ammo_id(&self) -> Option<i32> {
        self.values.get(&1198).map(|v| *v as i32)
    }

    pub fn set_player_self_res_spell(&mut self, v: i32) {
        self.header_set(1199, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_self_res_spell(&self) -> Option<i32> {
        self.values.get(&1199).map(|v| *v as i32)
    }

    pub fn set_player_pvp_medals(&mut self, v: i32) {
        self.header_set(1200, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_pvp_medals(&self) -> Option<i32> {
        self.values.get(&1200).map(|v| *v as i32)
    }

    pub fn set_player_buyback_price_1(&mut self, v: i32) {
        self.header_set(1201, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_buyback_price_1(&self) -> Option<i32> {
        self.values.get(&1201).map(|v| *v as i32)
    }

    pub fn set_player_buyback_timestamp_1(&mut self, v: i32) {
        self.header_set(1213, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_buyback_timestamp_1(&self) -> Option<i32> {
        self.values.get(&1213).map(|v| *v as i32)
    }

    pub fn set_player_kills(&mut self, a: u16, b: u16) {
        self.header_set(1225, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_kills(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1225) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_today_contribution(&mut self, v: i32) {
        self.header_set(1226, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_today_contribution(&self) -> Option<i32> {
        self.values.get(&1226).map(|v| *v as i32)
    }

    pub fn set_player_yesterday_contribution(&mut self, v: i32) {
        self.header_set(1227, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_yesterday_contribution(&self) -> Option<i32> {
        self.values.get(&1227).map(|v| *v as i32)
    }

    pub fn set_player_lifetime_honorbale_kills(&mut self, v: i32) {
        self.header_set(1228, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_lifetime_honorbale_kills(&self) -> Option<i32> {
        self.values.get(&1228).map(|v| *v as i32)
    }

    pub fn set_player_bytes2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1229, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_bytes2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1229) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_watched_faction_index(&mut self, v: i32) {
        self.header_set(1230, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_watched_faction_index(&self) -> Option<i32> {
        self.values.get(&1230).map(|v| *v as i32)
    }

    pub fn set_player_combat_rating_1(&mut self, v: i32) {
        self.header_set(1231, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_combat_rating_1(&self) -> Option<i32> {
        self.values.get(&1231).map(|v| *v as i32)
    }

    pub fn set_player_arena_team_info_1_1(&mut self, v: i32) {
        self.header_set(1256, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_arena_team_info_1_1(&self) -> Option<i32> {
        self.values.get(&1256).map(|v| *v as i32)
    }

    pub fn set_player_honor_currency(&mut self, v: i32) {
        self.header_set(1277, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_honor_currency(&self) -> Option<i32> {
        self.values.get(&1277).map(|v| *v as i32)
    }

    pub fn set_player_arena_currency(&mut self, v: i32) {
        self.header_set(1278, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_arena_currency(&self) -> Option<i32> {
        self.values.get(&1278).map(|v| *v as i32)
    }

    pub fn set_player_max_level(&mut self, v: i32) {
        self.header_set(1279, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_max_level(&self) -> Option<i32> {
        self.values.get(&1279).map(|v| *v as i32)
    }

    pub fn set_player_daily_quests_1(&mut self, v: i32) {
        self.header_set(1280, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_daily_quests_1(&self) -> Option<i32> {
        self.values.get(&1280).map(|v| *v as i32)
    }

    pub fn set_player_rune_regen_1(&mut self, v: f32) {
        self.header_set(1305, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_rune_regen_1(&self) -> Option<f32> {
        self.values.get(&1305).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_no_reagent_cost_1(&mut self, v: i32) {
        self.header_set(1309, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_no_reagent_cost_1(&self) -> Option<i32> {
        self.values.get(&1309).map(|v| *v as i32)
    }

    pub fn set_player_glyph_slots_1(&mut self, v: i32) {
        self.header_set(1312, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_glyph_slots_1(&self) -> Option<i32> {
        self.values.get(&1312).map(|v| *v as i32)
    }

    pub fn set_player_glyphs_1(&mut self, v: i32) {
        self.header_set(1318, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_glyphs_1(&self) -> Option<i32> {
        self.values.get(&1318).map(|v| *v as i32)
    }

    pub fn set_player_glyphs_enabled(&mut self, v: i32) {
        self.header_set(1324, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_glyphs_enabled(&self) -> Option<i32> {
        self.values.get(&1324).map(|v| *v as i32)
    }

    pub fn set_player_pet_spell_power(&mut self, v: i32) {
        self.header_set(1325, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_pet_spell_power(&self) -> Option<i32> {
        self.values.get(&1325).map(|v| *v as i32)
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

}

impl UpdateGameObject {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_type(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_gameobject_displayid(&mut self, v: i32) {
        self.header_set(8, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_displayid(&self) -> Option<i32> {
        self.values.get(&8).map(|v| *v as i32)
    }

    pub fn set_gameobject_flags(&mut self, v: i32) {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_flags(&self) -> Option<i32> {
        self.values.get(&9).map(|v| *v as i32)
    }

    pub fn set_gameobject_parentrotation(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_parentrotation(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_dynamic(&mut self, a: u16, b: u16) {
        self.header_set(14, crate::util::u16s_to_u32(a, b));
    }

    pub fn gameobject_dynamic(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&14) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_gameobject_faction(&mut self, v: i32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_faction(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_gameobject_level(&mut self, v: i32) {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_level(&self) -> Option<i32> {
        self.values.get(&16).map(|v| *v as i32)
    }

    pub fn set_gameobject_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(17, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn gameobject_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
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
    pub fn set_object_guid(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_type(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_dynamicobject_caster(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn dynamicobject_caster(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_dynamicobject_bytes(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(8, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn dynamicobject_bytes(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&8) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_dynamicobject_spellid(&mut self, v: i32) {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_spellid(&self) -> Option<i32> {
        self.values.get(&9).map(|v| *v as i32)
    }

    pub fn set_dynamicobject_radius(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_radius(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_casttime(&mut self, v: i32) {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_casttime(&self) -> Option<i32> {
        self.values.get(&11).map(|v| *v as i32)
    }

}

impl UpdateCorpse {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        let lower = self.values.get(&0);
        let upper = self.values.get(&1);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.header_set(2, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_type(&self) -> Option<i32> {
        self.values.get(&2).map(|v| *v as i32)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.values.get(&3).map(|v| *v as i32)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.values.get(&4).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_corpse_owner(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn corpse_owner(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_corpse_party(&mut self, v: Guid) {
        self.header_set(8, v.guid() as u32);
        self.header_set(9, (v.guid() >> 32) as u32);
    }

    pub fn corpse_party(&self) -> Option<Guid> {
        let lower = self.values.get(&8);
        let upper = self.values.get(&9);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_corpse_display_id(&mut self, v: i32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_display_id(&self) -> Option<i32> {
        self.values.get(&10).map(|v| *v as i32)
    }

    pub fn set_corpse_item(&mut self, v: i32) {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_item(&self) -> Option<i32> {
        self.values.get(&11).map(|v| *v as i32)
    }

    pub fn set_corpse_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(30, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&30) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(31, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&31) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_guild(&mut self, v: i32) {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_guild(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_corpse_flags(&mut self, v: i32) {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_flags(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_corpse_dynamic_flags(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_dynamic_flags(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

}

