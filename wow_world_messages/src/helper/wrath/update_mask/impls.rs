use crate::Guid;
use std::convert::TryInto;
use super::indices::*;

use crate::wrath::{
    Race, Class, Gender, Power, UnitStandState, UpdateItem, UpdateItemBuilder, UpdateContainer, 
    UpdateContainerBuilder, UpdateUnit, UpdateUnitBuilder, UpdatePlayer, UpdatePlayerBuilder, 
    UpdateGameObject, UpdateGameObjectBuilder, UpdateDynamicObject, UpdateDynamicObjectBuilder, 
    UpdateCorpse, UpdateCorpseBuilder
};

impl UpdateItemBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.set_guid(0, v);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.set_int(2, v);
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.set_int(3, v);
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.set_float(4, v);
        self
    }

    pub fn set_item_owner(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

    pub fn set_item_contained(mut self, v: Guid) -> Self {
        self.set_guid(8, v);
        self
    }

    pub fn set_item_creator(mut self, v: Guid) -> Self {
        self.set_guid(10, v);
        self
    }

    pub fn set_item_giftcreator(mut self, v: Guid) -> Self {
        self.set_guid(12, v);
        self
    }

    pub fn set_item_stack_count(mut self, v: i32) -> Self {
        self.set_int(14, v);
        self
    }

    pub fn set_item_duration(mut self, v: i32) -> Self {
        self.set_int(15, v);
        self
    }

    pub fn set_item_spell_charges(mut self, v: i32) -> Self {
        self.set_int(16, v);
        self
    }

    pub fn set_item_flags(mut self, v: i32) -> Self {
        self.set_int(21, v);
        self
    }

    pub fn set_item_enchantment_1_1(mut self, v: i32) -> Self {
        self.set_int(22, v);
        self
    }

    pub fn set_item_enchantment_1_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(24, a, b);
        self
    }

    pub fn set_item_enchantment_2_1(mut self, v: i32) -> Self {
        self.set_int(25, v);
        self
    }

    pub fn set_item_enchantment_2_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(27, a, b);
        self
    }

    pub fn set_item_enchantment_3_1(mut self, v: i32) -> Self {
        self.set_int(28, v);
        self
    }

    pub fn set_item_enchantment_3_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(30, a, b);
        self
    }

    pub fn set_item_enchantment_4_1(mut self, v: i32) -> Self {
        self.set_int(31, v);
        self
    }

    pub fn set_item_enchantment_4_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(33, a, b);
        self
    }

    pub fn set_item_enchantment_5_1(mut self, v: i32) -> Self {
        self.set_int(34, v);
        self
    }

    pub fn set_item_enchantment_5_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(36, a, b);
        self
    }

    pub fn set_item_enchantment_6_1(mut self, v: i32) -> Self {
        self.set_int(37, v);
        self
    }

    pub fn set_item_enchantment_6_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(39, a, b);
        self
    }

    pub fn set_item_enchantment_7_1(mut self, v: i32) -> Self {
        self.set_int(40, v);
        self
    }

    pub fn set_item_enchantment_7_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(42, a, b);
        self
    }

    pub fn set_item_enchantment_8_1(mut self, v: i32) -> Self {
        self.set_int(43, v);
        self
    }

    pub fn set_item_enchantment_8_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(45, a, b);
        self
    }

    pub fn set_item_enchantment_9_1(mut self, v: i32) -> Self {
        self.set_int(46, v);
        self
    }

    pub fn set_item_enchantment_9_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(48, a, b);
        self
    }

    pub fn set_item_enchantment_10_1(mut self, v: i32) -> Self {
        self.set_int(49, v);
        self
    }

    pub fn set_item_enchantment_10_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(51, a, b);
        self
    }

    pub fn set_item_enchantment_11_1(mut self, v: i32) -> Self {
        self.set_int(52, v);
        self
    }

    pub fn set_item_enchantment_11_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(54, a, b);
        self
    }

    pub fn set_item_enchantment_12_1(mut self, v: i32) -> Self {
        self.set_int(55, v);
        self
    }

    pub fn set_item_enchantment_12_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(57, a, b);
        self
    }

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.set_int(58, v);
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.set_int(59, v);
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.set_int(60, v);
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.set_int(61, v);
        self
    }

    pub fn set_item_create_played_time(mut self, v: i32) -> Self {
        self.set_int(62, v);
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

}

impl UpdateContainerBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.set_guid(0, v);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.set_int(2, v);
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.set_int(3, v);
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.set_float(4, v);
        self
    }

    pub fn set_item_owner(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

    pub fn set_item_contained(mut self, v: Guid) -> Self {
        self.set_guid(8, v);
        self
    }

    pub fn set_item_creator(mut self, v: Guid) -> Self {
        self.set_guid(10, v);
        self
    }

    pub fn set_item_giftcreator(mut self, v: Guid) -> Self {
        self.set_guid(12, v);
        self
    }

    pub fn set_item_stack_count(mut self, v: i32) -> Self {
        self.set_int(14, v);
        self
    }

    pub fn set_item_duration(mut self, v: i32) -> Self {
        self.set_int(15, v);
        self
    }

    pub fn set_item_spell_charges(mut self, v: i32) -> Self {
        self.set_int(16, v);
        self
    }

    pub fn set_item_flags(mut self, v: i32) -> Self {
        self.set_int(21, v);
        self
    }

    pub fn set_item_enchantment_1_1(mut self, v: i32) -> Self {
        self.set_int(22, v);
        self
    }

    pub fn set_item_enchantment_1_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(24, a, b);
        self
    }

    pub fn set_item_enchantment_2_1(mut self, v: i32) -> Self {
        self.set_int(25, v);
        self
    }

    pub fn set_item_enchantment_2_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(27, a, b);
        self
    }

    pub fn set_item_enchantment_3_1(mut self, v: i32) -> Self {
        self.set_int(28, v);
        self
    }

    pub fn set_item_enchantment_3_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(30, a, b);
        self
    }

    pub fn set_item_enchantment_4_1(mut self, v: i32) -> Self {
        self.set_int(31, v);
        self
    }

    pub fn set_item_enchantment_4_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(33, a, b);
        self
    }

    pub fn set_item_enchantment_5_1(mut self, v: i32) -> Self {
        self.set_int(34, v);
        self
    }

    pub fn set_item_enchantment_5_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(36, a, b);
        self
    }

    pub fn set_item_enchantment_6_1(mut self, v: i32) -> Self {
        self.set_int(37, v);
        self
    }

    pub fn set_item_enchantment_6_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(39, a, b);
        self
    }

    pub fn set_item_enchantment_7_1(mut self, v: i32) -> Self {
        self.set_int(40, v);
        self
    }

    pub fn set_item_enchantment_7_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(42, a, b);
        self
    }

    pub fn set_item_enchantment_8_1(mut self, v: i32) -> Self {
        self.set_int(43, v);
        self
    }

    pub fn set_item_enchantment_8_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(45, a, b);
        self
    }

    pub fn set_item_enchantment_9_1(mut self, v: i32) -> Self {
        self.set_int(46, v);
        self
    }

    pub fn set_item_enchantment_9_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(48, a, b);
        self
    }

    pub fn set_item_enchantment_10_1(mut self, v: i32) -> Self {
        self.set_int(49, v);
        self
    }

    pub fn set_item_enchantment_10_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(51, a, b);
        self
    }

    pub fn set_item_enchantment_11_1(mut self, v: i32) -> Self {
        self.set_int(52, v);
        self
    }

    pub fn set_item_enchantment_11_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(54, a, b);
        self
    }

    pub fn set_item_enchantment_12_1(mut self, v: i32) -> Self {
        self.set_int(55, v);
        self
    }

    pub fn set_item_enchantment_12_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(57, a, b);
        self
    }

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.set_int(58, v);
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.set_int(59, v);
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.set_int(60, v);
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.set_int(61, v);
        self
    }

    pub fn set_item_create_played_time(mut self, v: i32) -> Self {
        self.set_int(62, v);
        self
    }

    pub fn set_container_num_slots(mut self, v: i32) -> Self {
        self.set_int(64, v);
        self
    }

    pub fn set_container_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(66, v);
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

}

impl UpdateUnitBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.set_guid(0, v);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.set_int(2, v);
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.set_int(3, v);
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.set_float(4, v);
        self
    }

    pub fn set_unit_charm(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

    pub fn set_unit_summon(mut self, v: Guid) -> Self {
        self.set_guid(8, v);
        self
    }

    pub fn set_unit_critter(mut self, v: Guid) -> Self {
        self.set_guid(10, v);
        self
    }

    pub fn set_unit_charmedby(mut self, v: Guid) -> Self {
        self.set_guid(12, v);
        self
    }

    pub fn set_unit_summonedby(mut self, v: Guid) -> Self {
        self.set_guid(14, v);
        self
    }

    pub fn set_unit_createdby(mut self, v: Guid) -> Self {
        self.set_guid(16, v);
        self
    }

    pub fn set_unit_target(mut self, v: Guid) -> Self {
        self.set_guid(18, v);
        self
    }

    pub fn set_unit_channel_object(mut self, v: Guid) -> Self {
        self.set_guid(20, v);
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.set_int(22, v);
        self
    }

    pub fn set_unit_bytes_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.set_bytes(23, race.as_int(), class.as_int(), gender.as_int(), power.as_int());
        self
    }

    pub fn set_unit_health(mut self, v: i32) -> Self {
        self.set_int(24, v);
        self
    }

    pub fn set_unit_power1(mut self, v: i32) -> Self {
        self.set_int(25, v);
        self
    }

    pub fn set_unit_power2(mut self, v: i32) -> Self {
        self.set_int(26, v);
        self
    }

    pub fn set_unit_power3(mut self, v: i32) -> Self {
        self.set_int(27, v);
        self
    }

    pub fn set_unit_power4(mut self, v: i32) -> Self {
        self.set_int(28, v);
        self
    }

    pub fn set_unit_power5(mut self, v: i32) -> Self {
        self.set_int(29, v);
        self
    }

    pub fn set_unit_power6(mut self, v: i32) -> Self {
        self.set_int(30, v);
        self
    }

    pub fn set_unit_power7(mut self, v: i32) -> Self {
        self.set_int(31, v);
        self
    }

    pub fn set_unit_maxhealth(mut self, v: i32) -> Self {
        self.set_int(32, v);
        self
    }

    pub fn set_unit_maxpower1(mut self, v: i32) -> Self {
        self.set_int(33, v);
        self
    }

    pub fn set_unit_maxpower2(mut self, v: i32) -> Self {
        self.set_int(34, v);
        self
    }

    pub fn set_unit_maxpower3(mut self, v: i32) -> Self {
        self.set_int(35, v);
        self
    }

    pub fn set_unit_maxpower4(mut self, v: i32) -> Self {
        self.set_int(36, v);
        self
    }

    pub fn set_unit_maxpower5(mut self, v: i32) -> Self {
        self.set_int(37, v);
        self
    }

    pub fn set_unit_maxpower6(mut self, v: i32) -> Self {
        self.set_int(38, v);
        self
    }

    pub fn set_unit_maxpower7(mut self, v: i32) -> Self {
        self.set_int(39, v);
        self
    }

    pub fn set_unit_power_regen_flat_modifier(mut self, v: f32) -> Self {
        self.set_float(40, v);
        self
    }

    pub fn set_unit_power_regen_interrupted_flat_modifier(mut self, v: f32) -> Self {
        self.set_float(47, v);
        self
    }

    pub fn set_unit_level(mut self, v: i32) -> Self {
        self.set_int(54, v);
        self
    }

    pub fn set_unit_factiontemplate(mut self, v: i32) -> Self {
        self.set_int(55, v);
        self
    }

    pub fn set_unit_virtual_item_slot_id(mut self, v: i32) -> Self {
        self.set_int(56, v);
        self
    }

    pub fn set_unit_flags(mut self, v: i32) -> Self {
        self.set_int(59, v);
        self
    }

    pub fn set_unit_flags_2(mut self, v: i32) -> Self {
        self.set_int(60, v);
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.set_int(61, v);
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.set_int(62, v);
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.set_int(64, v);
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.set_float(65, v);
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.set_float(66, v);
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.set_int(67, v);
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.set_int(68, v);
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.set_int(69, v);
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.set_float(70, v);
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.set_float(71, v);
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(72, v);
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(73, v);
        self
    }

    pub fn set_unit_bytes_1(mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) -> Self {
        self.set_bytes(74, stand_state.as_int(), unknown1, unknown2, unknown3);
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.set_int(75, v);
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.set_int(76, v);
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.set_int(77, v);
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.set_int(78, v);
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.set_int(79, v);
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.set_float(80, v);
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.set_int(81, v);
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.set_int(82, v);
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.set_int(83, v);
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.set_int(84, v);
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.set_int(85, v);
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.set_int(86, v);
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.set_int(87, v);
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.set_int(88, v);
        self
    }

    pub fn set_unit_posstat0(mut self, v: i32) -> Self {
        self.set_int(89, v);
        self
    }

    pub fn set_unit_posstat1(mut self, v: i32) -> Self {
        self.set_int(90, v);
        self
    }

    pub fn set_unit_posstat2(mut self, v: i32) -> Self {
        self.set_int(91, v);
        self
    }

    pub fn set_unit_posstat3(mut self, v: i32) -> Self {
        self.set_int(92, v);
        self
    }

    pub fn set_unit_posstat4(mut self, v: i32) -> Self {
        self.set_int(93, v);
        self
    }

    pub fn set_unit_negstat0(mut self, v: i32) -> Self {
        self.set_int(94, v);
        self
    }

    pub fn set_unit_negstat1(mut self, v: i32) -> Self {
        self.set_int(95, v);
        self
    }

    pub fn set_unit_negstat2(mut self, v: i32) -> Self {
        self.set_int(96, v);
        self
    }

    pub fn set_unit_negstat3(mut self, v: i32) -> Self {
        self.set_int(97, v);
        self
    }

    pub fn set_unit_negstat4(mut self, v: i32) -> Self {
        self.set_int(98, v);
        self
    }

    pub fn set_unit_resistances(mut self, v: i32) -> Self {
        self.set_int(99, v);
        self
    }

    pub fn set_unit_resistancebuffmodspositive(mut self, v: i32) -> Self {
        self.set_int(106, v);
        self
    }

    pub fn set_unit_resistancebuffmodsnegative(mut self, v: i32) -> Self {
        self.set_int(113, v);
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.set_int(120, v);
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.set_int(121, v);
        self
    }

    pub fn set_unit_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(122, a, b, c, d);
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.set_int(123, v);
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(124, a, b);
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(125, v);
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.set_int(126, v);
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(127, a, b);
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(128, v);
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.set_float(129, v);
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.set_float(130, v);
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.set_int(131, v);
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.set_float(138, v);
        self
    }

    pub fn set_unit_maxhealthmodifier(mut self, v: f32) -> Self {
        self.set_float(145, v);
        self
    }

    pub fn set_unit_hoverheight(mut self, v: f32) -> Self {
        self.set_float(146, v);
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

}

impl UpdatePlayerBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.set_guid(0, v);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.set_int(2, v);
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.set_int(3, v);
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.set_float(4, v);
        self
    }

    pub fn set_unit_charm(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

    pub fn set_unit_summon(mut self, v: Guid) -> Self {
        self.set_guid(8, v);
        self
    }

    pub fn set_unit_critter(mut self, v: Guid) -> Self {
        self.set_guid(10, v);
        self
    }

    pub fn set_unit_charmedby(mut self, v: Guid) -> Self {
        self.set_guid(12, v);
        self
    }

    pub fn set_unit_summonedby(mut self, v: Guid) -> Self {
        self.set_guid(14, v);
        self
    }

    pub fn set_unit_createdby(mut self, v: Guid) -> Self {
        self.set_guid(16, v);
        self
    }

    pub fn set_unit_target(mut self, v: Guid) -> Self {
        self.set_guid(18, v);
        self
    }

    pub fn set_unit_channel_object(mut self, v: Guid) -> Self {
        self.set_guid(20, v);
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.set_int(22, v);
        self
    }

    pub fn set_unit_bytes_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.set_bytes(23, race.as_int(), class.as_int(), gender.as_int(), power.as_int());
        self
    }

    pub fn set_unit_health(mut self, v: i32) -> Self {
        self.set_int(24, v);
        self
    }

    pub fn set_unit_power1(mut self, v: i32) -> Self {
        self.set_int(25, v);
        self
    }

    pub fn set_unit_power2(mut self, v: i32) -> Self {
        self.set_int(26, v);
        self
    }

    pub fn set_unit_power3(mut self, v: i32) -> Self {
        self.set_int(27, v);
        self
    }

    pub fn set_unit_power4(mut self, v: i32) -> Self {
        self.set_int(28, v);
        self
    }

    pub fn set_unit_power5(mut self, v: i32) -> Self {
        self.set_int(29, v);
        self
    }

    pub fn set_unit_power6(mut self, v: i32) -> Self {
        self.set_int(30, v);
        self
    }

    pub fn set_unit_power7(mut self, v: i32) -> Self {
        self.set_int(31, v);
        self
    }

    pub fn set_unit_maxhealth(mut self, v: i32) -> Self {
        self.set_int(32, v);
        self
    }

    pub fn set_unit_maxpower1(mut self, v: i32) -> Self {
        self.set_int(33, v);
        self
    }

    pub fn set_unit_maxpower2(mut self, v: i32) -> Self {
        self.set_int(34, v);
        self
    }

    pub fn set_unit_maxpower3(mut self, v: i32) -> Self {
        self.set_int(35, v);
        self
    }

    pub fn set_unit_maxpower4(mut self, v: i32) -> Self {
        self.set_int(36, v);
        self
    }

    pub fn set_unit_maxpower5(mut self, v: i32) -> Self {
        self.set_int(37, v);
        self
    }

    pub fn set_unit_maxpower6(mut self, v: i32) -> Self {
        self.set_int(38, v);
        self
    }

    pub fn set_unit_maxpower7(mut self, v: i32) -> Self {
        self.set_int(39, v);
        self
    }

    pub fn set_unit_power_regen_flat_modifier(mut self, v: f32) -> Self {
        self.set_float(40, v);
        self
    }

    pub fn set_unit_power_regen_interrupted_flat_modifier(mut self, v: f32) -> Self {
        self.set_float(47, v);
        self
    }

    pub fn set_unit_level(mut self, v: i32) -> Self {
        self.set_int(54, v);
        self
    }

    pub fn set_unit_factiontemplate(mut self, v: i32) -> Self {
        self.set_int(55, v);
        self
    }

    pub fn set_unit_virtual_item_slot_id(mut self, v: i32) -> Self {
        self.set_int(56, v);
        self
    }

    pub fn set_unit_flags(mut self, v: i32) -> Self {
        self.set_int(59, v);
        self
    }

    pub fn set_unit_flags_2(mut self, v: i32) -> Self {
        self.set_int(60, v);
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.set_int(61, v);
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.set_int(62, v);
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.set_int(64, v);
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.set_float(65, v);
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.set_float(66, v);
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.set_int(67, v);
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.set_int(68, v);
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.set_int(69, v);
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.set_float(70, v);
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.set_float(71, v);
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(72, v);
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(73, v);
        self
    }

    pub fn set_unit_bytes_1(mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) -> Self {
        self.set_bytes(74, stand_state.as_int(), unknown1, unknown2, unknown3);
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.set_int(75, v);
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.set_int(76, v);
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.set_int(77, v);
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.set_int(78, v);
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.set_int(79, v);
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.set_float(80, v);
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.set_int(81, v);
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.set_int(82, v);
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.set_int(83, v);
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.set_int(84, v);
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.set_int(85, v);
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.set_int(86, v);
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.set_int(87, v);
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.set_int(88, v);
        self
    }

    pub fn set_unit_posstat0(mut self, v: i32) -> Self {
        self.set_int(89, v);
        self
    }

    pub fn set_unit_posstat1(mut self, v: i32) -> Self {
        self.set_int(90, v);
        self
    }

    pub fn set_unit_posstat2(mut self, v: i32) -> Self {
        self.set_int(91, v);
        self
    }

    pub fn set_unit_posstat3(mut self, v: i32) -> Self {
        self.set_int(92, v);
        self
    }

    pub fn set_unit_posstat4(mut self, v: i32) -> Self {
        self.set_int(93, v);
        self
    }

    pub fn set_unit_negstat0(mut self, v: i32) -> Self {
        self.set_int(94, v);
        self
    }

    pub fn set_unit_negstat1(mut self, v: i32) -> Self {
        self.set_int(95, v);
        self
    }

    pub fn set_unit_negstat2(mut self, v: i32) -> Self {
        self.set_int(96, v);
        self
    }

    pub fn set_unit_negstat3(mut self, v: i32) -> Self {
        self.set_int(97, v);
        self
    }

    pub fn set_unit_negstat4(mut self, v: i32) -> Self {
        self.set_int(98, v);
        self
    }

    pub fn set_unit_resistances(mut self, v: i32) -> Self {
        self.set_int(99, v);
        self
    }

    pub fn set_unit_resistancebuffmodspositive(mut self, v: i32) -> Self {
        self.set_int(106, v);
        self
    }

    pub fn set_unit_resistancebuffmodsnegative(mut self, v: i32) -> Self {
        self.set_int(113, v);
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.set_int(120, v);
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.set_int(121, v);
        self
    }

    pub fn set_unit_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(122, a, b, c, d);
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.set_int(123, v);
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(124, a, b);
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(125, v);
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.set_int(126, v);
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(127, a, b);
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(128, v);
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.set_float(129, v);
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.set_float(130, v);
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.set_int(131, v);
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.set_float(138, v);
        self
    }

    pub fn set_unit_maxhealthmodifier(mut self, v: f32) -> Self {
        self.set_float(145, v);
        self
    }

    pub fn set_unit_hoverheight(mut self, v: f32) -> Self {
        self.set_float(146, v);
        self
    }

    pub fn set_player_duel_arbiter(mut self, v: Guid) -> Self {
        self.set_guid(148, v);
        self
    }

    pub fn set_player_flags(mut self, v: i32) -> Self {
        self.set_int(150, v);
        self
    }

    pub fn set_player_guildid(mut self, v: i32) -> Self {
        self.set_int(151, v);
        self
    }

    pub fn set_player_guildrank(mut self, v: i32) -> Self {
        self.set_int(152, v);
        self
    }

    pub fn set_player_field_bytes(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(153, a, b, c, d);
        self
    }

    pub fn set_player_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(154, a, b, c, d);
        self
    }

    pub fn set_player_bytes_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(155, a, b, c, d);
        self
    }

    pub fn set_player_duel_team(mut self, v: i32) -> Self {
        self.set_int(156, v);
        self
    }

    pub fn set_player_guild_timestamp(mut self, v: i32) -> Self {
        self.set_int(157, v);
        self
    }

    pub fn set_player_quest_log_1_1(mut self, v: i32) -> Self {
        self.set_int(158, v);
        self
    }

    pub fn set_player_quest_log_1_2(mut self, v: i32) -> Self {
        self.set_int(159, v);
        self
    }

    pub fn set_player_quest_log_1_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(160, a, b);
        self
    }

    pub fn set_player_quest_log_1_4(mut self, v: i32) -> Self {
        self.set_int(162, v);
        self
    }

    pub fn set_player_quest_log_2_1(mut self, v: i32) -> Self {
        self.set_int(163, v);
        self
    }

    pub fn set_player_quest_log_2_2(mut self, v: i32) -> Self {
        self.set_int(164, v);
        self
    }

    pub fn set_player_quest_log_2_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(165, a, b);
        self
    }

    pub fn set_player_quest_log_2_5(mut self, v: i32) -> Self {
        self.set_int(167, v);
        self
    }

    pub fn set_player_quest_log_3_1(mut self, v: i32) -> Self {
        self.set_int(168, v);
        self
    }

    pub fn set_player_quest_log_3_2(mut self, v: i32) -> Self {
        self.set_int(169, v);
        self
    }

    pub fn set_player_quest_log_3_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(170, a, b);
        self
    }

    pub fn set_player_quest_log_3_5(mut self, v: i32) -> Self {
        self.set_int(172, v);
        self
    }

    pub fn set_player_quest_log_4_1(mut self, v: i32) -> Self {
        self.set_int(173, v);
        self
    }

    pub fn set_player_quest_log_4_2(mut self, v: i32) -> Self {
        self.set_int(174, v);
        self
    }

    pub fn set_player_quest_log_4_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(175, a, b);
        self
    }

    pub fn set_player_quest_log_4_5(mut self, v: i32) -> Self {
        self.set_int(177, v);
        self
    }

    pub fn set_player_quest_log_5_1(mut self, v: i32) -> Self {
        self.set_int(178, v);
        self
    }

    pub fn set_player_quest_log_5_2(mut self, v: i32) -> Self {
        self.set_int(179, v);
        self
    }

    pub fn set_player_quest_log_5_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(180, a, b);
        self
    }

    pub fn set_player_quest_log_5_5(mut self, v: i32) -> Self {
        self.set_int(182, v);
        self
    }

    pub fn set_player_quest_log_6_1(mut self, v: i32) -> Self {
        self.set_int(183, v);
        self
    }

    pub fn set_player_quest_log_6_2(mut self, v: i32) -> Self {
        self.set_int(184, v);
        self
    }

    pub fn set_player_quest_log_6_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(185, a, b);
        self
    }

    pub fn set_player_quest_log_6_5(mut self, v: i32) -> Self {
        self.set_int(187, v);
        self
    }

    pub fn set_player_quest_log_7_1(mut self, v: i32) -> Self {
        self.set_int(188, v);
        self
    }

    pub fn set_player_quest_log_7_2(mut self, v: i32) -> Self {
        self.set_int(189, v);
        self
    }

    pub fn set_player_quest_log_7_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(190, a, b);
        self
    }

    pub fn set_player_quest_log_7_5(mut self, v: i32) -> Self {
        self.set_int(192, v);
        self
    }

    pub fn set_player_quest_log_8_1(mut self, v: i32) -> Self {
        self.set_int(193, v);
        self
    }

    pub fn set_player_quest_log_8_2(mut self, v: i32) -> Self {
        self.set_int(194, v);
        self
    }

    pub fn set_player_quest_log_8_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(195, a, b);
        self
    }

    pub fn set_player_quest_log_8_5(mut self, v: i32) -> Self {
        self.set_int(197, v);
        self
    }

    pub fn set_player_quest_log_9_1(mut self, v: i32) -> Self {
        self.set_int(198, v);
        self
    }

    pub fn set_player_quest_log_9_2(mut self, v: i32) -> Self {
        self.set_int(199, v);
        self
    }

    pub fn set_player_quest_log_9_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(200, a, b);
        self
    }

    pub fn set_player_quest_log_9_5(mut self, v: i32) -> Self {
        self.set_int(202, v);
        self
    }

    pub fn set_player_quest_log_10_1(mut self, v: i32) -> Self {
        self.set_int(203, v);
        self
    }

    pub fn set_player_quest_log_10_2(mut self, v: i32) -> Self {
        self.set_int(204, v);
        self
    }

    pub fn set_player_quest_log_10_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(205, a, b);
        self
    }

    pub fn set_player_quest_log_10_5(mut self, v: i32) -> Self {
        self.set_int(207, v);
        self
    }

    pub fn set_player_quest_log_11_1(mut self, v: i32) -> Self {
        self.set_int(208, v);
        self
    }

    pub fn set_player_quest_log_11_2(mut self, v: i32) -> Self {
        self.set_int(209, v);
        self
    }

    pub fn set_player_quest_log_11_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(210, a, b);
        self
    }

    pub fn set_player_quest_log_11_5(mut self, v: i32) -> Self {
        self.set_int(212, v);
        self
    }

    pub fn set_player_quest_log_12_1(mut self, v: i32) -> Self {
        self.set_int(213, v);
        self
    }

    pub fn set_player_quest_log_12_2(mut self, v: i32) -> Self {
        self.set_int(214, v);
        self
    }

    pub fn set_player_quest_log_12_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(215, a, b);
        self
    }

    pub fn set_player_quest_log_12_5(mut self, v: i32) -> Self {
        self.set_int(217, v);
        self
    }

    pub fn set_player_quest_log_13_1(mut self, v: i32) -> Self {
        self.set_int(218, v);
        self
    }

    pub fn set_player_quest_log_13_2(mut self, v: i32) -> Self {
        self.set_int(219, v);
        self
    }

    pub fn set_player_quest_log_13_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(220, a, b);
        self
    }

    pub fn set_player_quest_log_13_5(mut self, v: i32) -> Self {
        self.set_int(222, v);
        self
    }

    pub fn set_player_quest_log_14_1(mut self, v: i32) -> Self {
        self.set_int(223, v);
        self
    }

    pub fn set_player_quest_log_14_2(mut self, v: i32) -> Self {
        self.set_int(224, v);
        self
    }

    pub fn set_player_quest_log_14_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(225, a, b);
        self
    }

    pub fn set_player_quest_log_14_5(mut self, v: i32) -> Self {
        self.set_int(227, v);
        self
    }

    pub fn set_player_quest_log_15_1(mut self, v: i32) -> Self {
        self.set_int(228, v);
        self
    }

    pub fn set_player_quest_log_15_2(mut self, v: i32) -> Self {
        self.set_int(229, v);
        self
    }

    pub fn set_player_quest_log_15_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(230, a, b);
        self
    }

    pub fn set_player_quest_log_15_5(mut self, v: i32) -> Self {
        self.set_int(232, v);
        self
    }

    pub fn set_player_quest_log_16_1(mut self, v: i32) -> Self {
        self.set_int(233, v);
        self
    }

    pub fn set_player_quest_log_16_2(mut self, v: i32) -> Self {
        self.set_int(234, v);
        self
    }

    pub fn set_player_quest_log_16_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(235, a, b);
        self
    }

    pub fn set_player_quest_log_16_5(mut self, v: i32) -> Self {
        self.set_int(237, v);
        self
    }

    pub fn set_player_quest_log_17_1(mut self, v: i32) -> Self {
        self.set_int(238, v);
        self
    }

    pub fn set_player_quest_log_17_2(mut self, v: i32) -> Self {
        self.set_int(239, v);
        self
    }

    pub fn set_player_quest_log_17_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(240, a, b);
        self
    }

    pub fn set_player_quest_log_17_5(mut self, v: i32) -> Self {
        self.set_int(242, v);
        self
    }

    pub fn set_player_quest_log_18_1(mut self, v: i32) -> Self {
        self.set_int(243, v);
        self
    }

    pub fn set_player_quest_log_18_2(mut self, v: i32) -> Self {
        self.set_int(244, v);
        self
    }

    pub fn set_player_quest_log_18_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(245, a, b);
        self
    }

    pub fn set_player_quest_log_18_5(mut self, v: i32) -> Self {
        self.set_int(247, v);
        self
    }

    pub fn set_player_quest_log_19_1(mut self, v: i32) -> Self {
        self.set_int(248, v);
        self
    }

    pub fn set_player_quest_log_19_2(mut self, v: i32) -> Self {
        self.set_int(249, v);
        self
    }

    pub fn set_player_quest_log_19_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(250, a, b);
        self
    }

    pub fn set_player_quest_log_19_5(mut self, v: i32) -> Self {
        self.set_int(252, v);
        self
    }

    pub fn set_player_quest_log_20_1(mut self, v: i32) -> Self {
        self.set_int(253, v);
        self
    }

    pub fn set_player_quest_log_20_2(mut self, v: i32) -> Self {
        self.set_int(254, v);
        self
    }

    pub fn set_player_quest_log_20_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(255, a, b);
        self
    }

    pub fn set_player_quest_log_20_5(mut self, v: i32) -> Self {
        self.set_int(257, v);
        self
    }

    pub fn set_player_quest_log_21_1(mut self, v: i32) -> Self {
        self.set_int(258, v);
        self
    }

    pub fn set_player_quest_log_21_2(mut self, v: i32) -> Self {
        self.set_int(259, v);
        self
    }

    pub fn set_player_quest_log_21_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(260, a, b);
        self
    }

    pub fn set_player_quest_log_21_5(mut self, v: i32) -> Self {
        self.set_int(262, v);
        self
    }

    pub fn set_player_quest_log_22_1(mut self, v: i32) -> Self {
        self.set_int(263, v);
        self
    }

    pub fn set_player_quest_log_22_2(mut self, v: i32) -> Self {
        self.set_int(264, v);
        self
    }

    pub fn set_player_quest_log_22_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(265, a, b);
        self
    }

    pub fn set_player_quest_log_22_5(mut self, v: i32) -> Self {
        self.set_int(267, v);
        self
    }

    pub fn set_player_quest_log_23_1(mut self, v: i32) -> Self {
        self.set_int(268, v);
        self
    }

    pub fn set_player_quest_log_23_2(mut self, v: i32) -> Self {
        self.set_int(269, v);
        self
    }

    pub fn set_player_quest_log_23_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(270, a, b);
        self
    }

    pub fn set_player_quest_log_23_5(mut self, v: i32) -> Self {
        self.set_int(272, v);
        self
    }

    pub fn set_player_quest_log_24_1(mut self, v: i32) -> Self {
        self.set_int(273, v);
        self
    }

    pub fn set_player_quest_log_24_2(mut self, v: i32) -> Self {
        self.set_int(274, v);
        self
    }

    pub fn set_player_quest_log_24_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(275, a, b);
        self
    }

    pub fn set_player_quest_log_24_5(mut self, v: i32) -> Self {
        self.set_int(277, v);
        self
    }

    pub fn set_player_quest_log_25_1(mut self, v: i32) -> Self {
        self.set_int(278, v);
        self
    }

    pub fn set_player_quest_log_25_2(mut self, v: i32) -> Self {
        self.set_int(279, v);
        self
    }

    pub fn set_player_quest_log_25_3(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(280, a, b);
        self
    }

    pub fn set_player_quest_log_25_5(mut self, v: i32) -> Self {
        self.set_int(282, v);
        self
    }

    pub fn set_player_visible_item_1_entryid(mut self, v: i32) -> Self {
        self.set_int(283, v);
        self
    }

    pub fn set_player_visible_item_1_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(284, a, b);
        self
    }

    pub fn set_player_visible_item_2_entryid(mut self, v: i32) -> Self {
        self.set_int(285, v);
        self
    }

    pub fn set_player_visible_item_2_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(286, a, b);
        self
    }

    pub fn set_player_visible_item_3_entryid(mut self, v: i32) -> Self {
        self.set_int(287, v);
        self
    }

    pub fn set_player_visible_item_3_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(288, a, b);
        self
    }

    pub fn set_player_visible_item_4_entryid(mut self, v: i32) -> Self {
        self.set_int(289, v);
        self
    }

    pub fn set_player_visible_item_4_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(290, a, b);
        self
    }

    pub fn set_player_visible_item_5_entryid(mut self, v: i32) -> Self {
        self.set_int(291, v);
        self
    }

    pub fn set_player_visible_item_5_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(292, a, b);
        self
    }

    pub fn set_player_visible_item_6_entryid(mut self, v: i32) -> Self {
        self.set_int(293, v);
        self
    }

    pub fn set_player_visible_item_6_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(294, a, b);
        self
    }

    pub fn set_player_visible_item_7_entryid(mut self, v: i32) -> Self {
        self.set_int(295, v);
        self
    }

    pub fn set_player_visible_item_7_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(296, a, b);
        self
    }

    pub fn set_player_visible_item_8_entryid(mut self, v: i32) -> Self {
        self.set_int(297, v);
        self
    }

    pub fn set_player_visible_item_8_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(298, a, b);
        self
    }

    pub fn set_player_visible_item_9_entryid(mut self, v: i32) -> Self {
        self.set_int(299, v);
        self
    }

    pub fn set_player_visible_item_9_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(300, a, b);
        self
    }

    pub fn set_player_visible_item_10_entryid(mut self, v: i32) -> Self {
        self.set_int(301, v);
        self
    }

    pub fn set_player_visible_item_10_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(302, a, b);
        self
    }

    pub fn set_player_visible_item_11_entryid(mut self, v: i32) -> Self {
        self.set_int(303, v);
        self
    }

    pub fn set_player_visible_item_11_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(304, a, b);
        self
    }

    pub fn set_player_visible_item_12_entryid(mut self, v: i32) -> Self {
        self.set_int(305, v);
        self
    }

    pub fn set_player_visible_item_12_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(306, a, b);
        self
    }

    pub fn set_player_visible_item_13_entryid(mut self, v: i32) -> Self {
        self.set_int(307, v);
        self
    }

    pub fn set_player_visible_item_13_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(308, a, b);
        self
    }

    pub fn set_player_visible_item_14_entryid(mut self, v: i32) -> Self {
        self.set_int(309, v);
        self
    }

    pub fn set_player_visible_item_14_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(310, a, b);
        self
    }

    pub fn set_player_visible_item_15_entryid(mut self, v: i32) -> Self {
        self.set_int(311, v);
        self
    }

    pub fn set_player_visible_item_15_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(312, a, b);
        self
    }

    pub fn set_player_visible_item_16_entryid(mut self, v: i32) -> Self {
        self.set_int(313, v);
        self
    }

    pub fn set_player_visible_item_16_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(314, a, b);
        self
    }

    pub fn set_player_visible_item_17_entryid(mut self, v: i32) -> Self {
        self.set_int(315, v);
        self
    }

    pub fn set_player_visible_item_17_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(316, a, b);
        self
    }

    pub fn set_player_visible_item_18_entryid(mut self, v: i32) -> Self {
        self.set_int(317, v);
        self
    }

    pub fn set_player_visible_item_18_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(318, a, b);
        self
    }

    pub fn set_player_visible_item_19_entryid(mut self, v: i32) -> Self {
        self.set_int(319, v);
        self
    }

    pub fn set_player_visible_item_19_enchantment(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(320, a, b);
        self
    }

    pub fn set_player_chosen_title(mut self, v: i32) -> Self {
        self.set_int(321, v);
        self
    }

    pub fn set_player_fake_inebriation(mut self, v: i32) -> Self {
        self.set_int(322, v);
        self
    }

    pub fn set_player_inv_slot_head1(mut self, v: Guid) -> Self {
        self.set_guid(324, v);
        self
    }

    pub fn set_player_inv_slot_head2(mut self, v: Guid) -> Self {
        self.set_guid(326, v);
        self
    }

    pub fn set_player_inv_slot_head3(mut self, v: Guid) -> Self {
        self.set_guid(328, v);
        self
    }

    pub fn set_player_inv_slot_head4(mut self, v: Guid) -> Self {
        self.set_guid(330, v);
        self
    }

    pub fn set_player_inv_slot_head5(mut self, v: Guid) -> Self {
        self.set_guid(332, v);
        self
    }

    pub fn set_player_inv_slot_head6(mut self, v: Guid) -> Self {
        self.set_guid(334, v);
        self
    }

    pub fn set_player_inv_slot_head7(mut self, v: Guid) -> Self {
        self.set_guid(336, v);
        self
    }

    pub fn set_player_inv_slot_head8(mut self, v: Guid) -> Self {
        self.set_guid(338, v);
        self
    }

    pub fn set_player_inv_slot_head9(mut self, v: Guid) -> Self {
        self.set_guid(340, v);
        self
    }

    pub fn set_player_inv_slot_head10(mut self, v: Guid) -> Self {
        self.set_guid(342, v);
        self
    }

    pub fn set_player_inv_slot_head11(mut self, v: Guid) -> Self {
        self.set_guid(344, v);
        self
    }

    pub fn set_player_inv_slot_head12(mut self, v: Guid) -> Self {
        self.set_guid(346, v);
        self
    }

    pub fn set_player_inv_slot_head13(mut self, v: Guid) -> Self {
        self.set_guid(348, v);
        self
    }

    pub fn set_player_inv_slot_head14(mut self, v: Guid) -> Self {
        self.set_guid(350, v);
        self
    }

    pub fn set_player_inv_slot_head15(mut self, v: Guid) -> Self {
        self.set_guid(352, v);
        self
    }

    pub fn set_player_inv_slot_head16(mut self, v: Guid) -> Self {
        self.set_guid(354, v);
        self
    }

    pub fn set_player_inv_slot_head17(mut self, v: Guid) -> Self {
        self.set_guid(356, v);
        self
    }

    pub fn set_player_inv_slot_head18(mut self, v: Guid) -> Self {
        self.set_guid(358, v);
        self
    }

    pub fn set_player_inv_slot_head19(mut self, v: Guid) -> Self {
        self.set_guid(360, v);
        self
    }

    pub fn set_player_inv_slot_head20(mut self, v: Guid) -> Self {
        self.set_guid(362, v);
        self
    }

    pub fn set_player_inv_slot_head21(mut self, v: Guid) -> Self {
        self.set_guid(364, v);
        self
    }

    pub fn set_player_inv_slot_head22(mut self, v: Guid) -> Self {
        self.set_guid(366, v);
        self
    }

    pub fn set_player_inv_slot_head23(mut self, v: Guid) -> Self {
        self.set_guid(368, v);
        self
    }

    pub fn set_player_pack_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(370, v);
        self
    }

    pub fn set_player_pack_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(372, v);
        self
    }

    pub fn set_player_pack_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(374, v);
        self
    }

    pub fn set_player_pack_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(376, v);
        self
    }

    pub fn set_player_pack_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(378, v);
        self
    }

    pub fn set_player_pack_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(380, v);
        self
    }

    pub fn set_player_pack_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(382, v);
        self
    }

    pub fn set_player_pack_slot_8(mut self, v: Guid) -> Self {
        self.set_guid(384, v);
        self
    }

    pub fn set_player_pack_slot_9(mut self, v: Guid) -> Self {
        self.set_guid(386, v);
        self
    }

    pub fn set_player_pack_slot_10(mut self, v: Guid) -> Self {
        self.set_guid(388, v);
        self
    }

    pub fn set_player_pack_slot_11(mut self, v: Guid) -> Self {
        self.set_guid(390, v);
        self
    }

    pub fn set_player_pack_slot_12(mut self, v: Guid) -> Self {
        self.set_guid(392, v);
        self
    }

    pub fn set_player_pack_slot_13(mut self, v: Guid) -> Self {
        self.set_guid(394, v);
        self
    }

    pub fn set_player_pack_slot_14(mut self, v: Guid) -> Self {
        self.set_guid(396, v);
        self
    }

    pub fn set_player_pack_slot_15(mut self, v: Guid) -> Self {
        self.set_guid(398, v);
        self
    }

    pub fn set_player_pack_slot_16(mut self, v: Guid) -> Self {
        self.set_guid(400, v);
        self
    }

    pub fn set_player_bank_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(402, v);
        self
    }

    pub fn set_player_bank_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(404, v);
        self
    }

    pub fn set_player_bank_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(406, v);
        self
    }

    pub fn set_player_bank_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(408, v);
        self
    }

    pub fn set_player_bank_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(410, v);
        self
    }

    pub fn set_player_bank_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(412, v);
        self
    }

    pub fn set_player_bank_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(414, v);
        self
    }

    pub fn set_player_bank_slot_8(mut self, v: Guid) -> Self {
        self.set_guid(416, v);
        self
    }

    pub fn set_player_bank_slot_9(mut self, v: Guid) -> Self {
        self.set_guid(418, v);
        self
    }

    pub fn set_player_bank_slot_10(mut self, v: Guid) -> Self {
        self.set_guid(420, v);
        self
    }

    pub fn set_player_bank_slot_11(mut self, v: Guid) -> Self {
        self.set_guid(422, v);
        self
    }

    pub fn set_player_bank_slot_12(mut self, v: Guid) -> Self {
        self.set_guid(424, v);
        self
    }

    pub fn set_player_bank_slot_13(mut self, v: Guid) -> Self {
        self.set_guid(426, v);
        self
    }

    pub fn set_player_bank_slot_14(mut self, v: Guid) -> Self {
        self.set_guid(428, v);
        self
    }

    pub fn set_player_bank_slot_15(mut self, v: Guid) -> Self {
        self.set_guid(430, v);
        self
    }

    pub fn set_player_bank_slot_16(mut self, v: Guid) -> Self {
        self.set_guid(432, v);
        self
    }

    pub fn set_player_bank_slot_17(mut self, v: Guid) -> Self {
        self.set_guid(434, v);
        self
    }

    pub fn set_player_bank_slot_18(mut self, v: Guid) -> Self {
        self.set_guid(436, v);
        self
    }

    pub fn set_player_bank_slot_19(mut self, v: Guid) -> Self {
        self.set_guid(438, v);
        self
    }

    pub fn set_player_bank_slot_20(mut self, v: Guid) -> Self {
        self.set_guid(440, v);
        self
    }

    pub fn set_player_bank_slot_21(mut self, v: Guid) -> Self {
        self.set_guid(442, v);
        self
    }

    pub fn set_player_bank_slot_22(mut self, v: Guid) -> Self {
        self.set_guid(444, v);
        self
    }

    pub fn set_player_bank_slot_23(mut self, v: Guid) -> Self {
        self.set_guid(446, v);
        self
    }

    pub fn set_player_bank_slot_24(mut self, v: Guid) -> Self {
        self.set_guid(448, v);
        self
    }

    pub fn set_player_bank_slot_25(mut self, v: Guid) -> Self {
        self.set_guid(450, v);
        self
    }

    pub fn set_player_bank_slot_26(mut self, v: Guid) -> Self {
        self.set_guid(452, v);
        self
    }

    pub fn set_player_bank_slot_27(mut self, v: Guid) -> Self {
        self.set_guid(454, v);
        self
    }

    pub fn set_player_bank_slot_28(mut self, v: Guid) -> Self {
        self.set_guid(456, v);
        self
    }

    pub fn set_player_bankbag_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(458, v);
        self
    }

    pub fn set_player_bankbag_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(460, v);
        self
    }

    pub fn set_player_bankbag_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(462, v);
        self
    }

    pub fn set_player_bankbag_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(464, v);
        self
    }

    pub fn set_player_bankbag_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(466, v);
        self
    }

    pub fn set_player_bankbag_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(468, v);
        self
    }

    pub fn set_player_bankbag_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(470, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(472, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(474, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(476, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(478, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(480, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(482, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(484, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_8(mut self, v: Guid) -> Self {
        self.set_guid(486, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_9(mut self, v: Guid) -> Self {
        self.set_guid(488, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_10(mut self, v: Guid) -> Self {
        self.set_guid(490, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_11(mut self, v: Guid) -> Self {
        self.set_guid(492, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_12(mut self, v: Guid) -> Self {
        self.set_guid(494, v);
        self
    }

    pub fn set_player_keyring_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(496, v);
        self
    }

    pub fn set_player_keyring_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(498, v);
        self
    }

    pub fn set_player_keyring_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(500, v);
        self
    }

    pub fn set_player_keyring_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(502, v);
        self
    }

    pub fn set_player_keyring_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(504, v);
        self
    }

    pub fn set_player_keyring_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(506, v);
        self
    }

    pub fn set_player_keyring_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(508, v);
        self
    }

    pub fn set_player_keyring_slot_8(mut self, v: Guid) -> Self {
        self.set_guid(510, v);
        self
    }

    pub fn set_player_keyring_slot_9(mut self, v: Guid) -> Self {
        self.set_guid(512, v);
        self
    }

    pub fn set_player_keyring_slot_10(mut self, v: Guid) -> Self {
        self.set_guid(514, v);
        self
    }

    pub fn set_player_keyring_slot_11(mut self, v: Guid) -> Self {
        self.set_guid(516, v);
        self
    }

    pub fn set_player_keyring_slot_12(mut self, v: Guid) -> Self {
        self.set_guid(518, v);
        self
    }

    pub fn set_player_keyring_slot_13(mut self, v: Guid) -> Self {
        self.set_guid(520, v);
        self
    }

    pub fn set_player_keyring_slot_14(mut self, v: Guid) -> Self {
        self.set_guid(522, v);
        self
    }

    pub fn set_player_keyring_slot_15(mut self, v: Guid) -> Self {
        self.set_guid(524, v);
        self
    }

    pub fn set_player_keyring_slot_16(mut self, v: Guid) -> Self {
        self.set_guid(526, v);
        self
    }

    pub fn set_player_keyring_slot_17(mut self, v: Guid) -> Self {
        self.set_guid(528, v);
        self
    }

    pub fn set_player_keyring_slot_18(mut self, v: Guid) -> Self {
        self.set_guid(530, v);
        self
    }

    pub fn set_player_keyring_slot_19(mut self, v: Guid) -> Self {
        self.set_guid(532, v);
        self
    }

    pub fn set_player_keyring_slot_20(mut self, v: Guid) -> Self {
        self.set_guid(534, v);
        self
    }

    pub fn set_player_keyring_slot_21(mut self, v: Guid) -> Self {
        self.set_guid(536, v);
        self
    }

    pub fn set_player_keyring_slot_22(mut self, v: Guid) -> Self {
        self.set_guid(538, v);
        self
    }

    pub fn set_player_keyring_slot_23(mut self, v: Guid) -> Self {
        self.set_guid(540, v);
        self
    }

    pub fn set_player_keyring_slot_24(mut self, v: Guid) -> Self {
        self.set_guid(542, v);
        self
    }

    pub fn set_player_keyring_slot_25(mut self, v: Guid) -> Self {
        self.set_guid(544, v);
        self
    }

    pub fn set_player_keyring_slot_26(mut self, v: Guid) -> Self {
        self.set_guid(546, v);
        self
    }

    pub fn set_player_keyring_slot_27(mut self, v: Guid) -> Self {
        self.set_guid(548, v);
        self
    }

    pub fn set_player_keyring_slot_28(mut self, v: Guid) -> Self {
        self.set_guid(550, v);
        self
    }

    pub fn set_player_keyring_slot_29(mut self, v: Guid) -> Self {
        self.set_guid(552, v);
        self
    }

    pub fn set_player_keyring_slot_30(mut self, v: Guid) -> Self {
        self.set_guid(554, v);
        self
    }

    pub fn set_player_keyring_slot_31(mut self, v: Guid) -> Self {
        self.set_guid(556, v);
        self
    }

    pub fn set_player_keyring_slot_32(mut self, v: Guid) -> Self {
        self.set_guid(558, v);
        self
    }

    pub fn set_player_currencytoken_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(560, v);
        self
    }

    pub fn set_player_currencytoken_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(562, v);
        self
    }

    pub fn set_player_currencytoken_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(564, v);
        self
    }

    pub fn set_player_currencytoken_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(566, v);
        self
    }

    pub fn set_player_currencytoken_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(568, v);
        self
    }

    pub fn set_player_currencytoken_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(570, v);
        self
    }

    pub fn set_player_currencytoken_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(572, v);
        self
    }

    pub fn set_player_currencytoken_slot_8(mut self, v: Guid) -> Self {
        self.set_guid(574, v);
        self
    }

    pub fn set_player_currencytoken_slot_9(mut self, v: Guid) -> Self {
        self.set_guid(576, v);
        self
    }

    pub fn set_player_currencytoken_slot_10(mut self, v: Guid) -> Self {
        self.set_guid(578, v);
        self
    }

    pub fn set_player_currencytoken_slot_11(mut self, v: Guid) -> Self {
        self.set_guid(580, v);
        self
    }

    pub fn set_player_currencytoken_slot_12(mut self, v: Guid) -> Self {
        self.set_guid(582, v);
        self
    }

    pub fn set_player_currencytoken_slot_13(mut self, v: Guid) -> Self {
        self.set_guid(584, v);
        self
    }

    pub fn set_player_currencytoken_slot_14(mut self, v: Guid) -> Self {
        self.set_guid(586, v);
        self
    }

    pub fn set_player_currencytoken_slot_15(mut self, v: Guid) -> Self {
        self.set_guid(588, v);
        self
    }

    pub fn set_player_currencytoken_slot_16(mut self, v: Guid) -> Self {
        self.set_guid(590, v);
        self
    }

    pub fn set_player_currencytoken_slot_17(mut self, v: Guid) -> Self {
        self.set_guid(592, v);
        self
    }

    pub fn set_player_currencytoken_slot_18(mut self, v: Guid) -> Self {
        self.set_guid(594, v);
        self
    }

    pub fn set_player_currencytoken_slot_19(mut self, v: Guid) -> Self {
        self.set_guid(596, v);
        self
    }

    pub fn set_player_currencytoken_slot_20(mut self, v: Guid) -> Self {
        self.set_guid(598, v);
        self
    }

    pub fn set_player_currencytoken_slot_21(mut self, v: Guid) -> Self {
        self.set_guid(600, v);
        self
    }

    pub fn set_player_currencytoken_slot_22(mut self, v: Guid) -> Self {
        self.set_guid(602, v);
        self
    }

    pub fn set_player_currencytoken_slot_23(mut self, v: Guid) -> Self {
        self.set_guid(604, v);
        self
    }

    pub fn set_player_currencytoken_slot_24(mut self, v: Guid) -> Self {
        self.set_guid(606, v);
        self
    }

    pub fn set_player_currencytoken_slot_25(mut self, v: Guid) -> Self {
        self.set_guid(608, v);
        self
    }

    pub fn set_player_currencytoken_slot_26(mut self, v: Guid) -> Self {
        self.set_guid(610, v);
        self
    }

    pub fn set_player_currencytoken_slot_27(mut self, v: Guid) -> Self {
        self.set_guid(612, v);
        self
    }

    pub fn set_player_currencytoken_slot_28(mut self, v: Guid) -> Self {
        self.set_guid(614, v);
        self
    }

    pub fn set_player_currencytoken_slot_29(mut self, v: Guid) -> Self {
        self.set_guid(616, v);
        self
    }

    pub fn set_player_currencytoken_slot_30(mut self, v: Guid) -> Self {
        self.set_guid(618, v);
        self
    }

    pub fn set_player_currencytoken_slot_31(mut self, v: Guid) -> Self {
        self.set_guid(620, v);
        self
    }

    pub fn set_player_currencytoken_slot_32(mut self, v: Guid) -> Self {
        self.set_guid(622, v);
        self
    }

    pub fn set_player_farsight(mut self, v: Guid) -> Self {
        self.set_guid(624, v);
        self
    }

    pub fn set_player_known_titles(mut self, v: Guid) -> Self {
        self.set_guid(626, v);
        self
    }

    pub fn set_player_known_titles1(mut self, v: Guid) -> Self {
        self.set_guid(628, v);
        self
    }

    pub fn set_player_known_titles2(mut self, v: Guid) -> Self {
        self.set_guid(630, v);
        self
    }

    pub fn set_player_known_currencies(mut self, v: Guid) -> Self {
        self.set_guid(632, v);
        self
    }

    pub fn set_player_xp(mut self, v: i32) -> Self {
        self.set_int(634, v);
        self
    }

    pub fn set_player_next_level_xp(mut self, v: i32) -> Self {
        self.set_int(635, v);
        self
    }

    pub fn set_player_skill_info(mut self, skill_info: crate::wrath::SkillInfo, index: SkillInfoIndex) -> Self {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
        self
    }

    pub fn set_player_character_points1(mut self, v: i32) -> Self {
        self.set_int(1020, v);
        self
    }

    pub fn set_player_character_points2(mut self, v: i32) -> Self {
        self.set_int(1021, v);
        self
    }

    pub fn set_player_track_creatures(mut self, v: i32) -> Self {
        self.set_int(1022, v);
        self
    }

    pub fn set_player_track_resources(mut self, v: i32) -> Self {
        self.set_int(1023, v);
        self
    }

    pub fn set_player_block_percentage(mut self, v: f32) -> Self {
        self.set_float(1024, v);
        self
    }

    pub fn set_player_dodge_percentage(mut self, v: f32) -> Self {
        self.set_float(1025, v);
        self
    }

    pub fn set_player_parry_percentage(mut self, v: f32) -> Self {
        self.set_float(1026, v);
        self
    }

    pub fn set_player_expertise(mut self, v: i32) -> Self {
        self.set_int(1027, v);
        self
    }

    pub fn set_player_offhand_expertise(mut self, v: i32) -> Self {
        self.set_int(1028, v);
        self
    }

    pub fn set_player_crit_percentage(mut self, v: f32) -> Self {
        self.set_float(1029, v);
        self
    }

    pub fn set_player_ranged_crit_percentage(mut self, v: f32) -> Self {
        self.set_float(1030, v);
        self
    }

    pub fn set_player_offhand_crit_percentage(mut self, v: f32) -> Self {
        self.set_float(1031, v);
        self
    }

    pub fn set_player_spell_crit_percentage1(mut self, v: f32) -> Self {
        self.set_float(1032, v);
        self
    }

    pub fn set_player_shield_block(mut self, v: i32) -> Self {
        self.set_int(1039, v);
        self
    }

    pub fn set_player_shield_block_crit_percentage(mut self, v: f32) -> Self {
        self.set_float(1040, v);
        self
    }

    pub fn set_player_explored_zones_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(1041, a, b, c, d);
        self
    }

    pub fn set_player_rest_state_experience(mut self, v: i32) -> Self {
        self.set_int(1169, v);
        self
    }

    pub fn set_player_coinage(mut self, v: i32) -> Self {
        self.set_int(1170, v);
        self
    }

    pub fn set_player_mod_damage_done_pos(mut self, v: i32) -> Self {
        self.set_int(1171, v);
        self
    }

    pub fn set_player_mod_damage_done_neg(mut self, v: i32) -> Self {
        self.set_int(1178, v);
        self
    }

    pub fn set_player_mod_damage_done_pct(mut self, v: i32) -> Self {
        self.set_int(1185, v);
        self
    }

    pub fn set_player_mod_healing_done_pos(mut self, v: i32) -> Self {
        self.set_int(1192, v);
        self
    }

    pub fn set_player_mod_healing_pct(mut self, v: f32) -> Self {
        self.set_float(1193, v);
        self
    }

    pub fn set_player_mod_healing_done_pct(mut self, v: f32) -> Self {
        self.set_float(1194, v);
        self
    }

    pub fn set_player_mod_target_resistance(mut self, v: i32) -> Self {
        self.set_int(1195, v);
        self
    }

    pub fn set_player_mod_target_physical_resistance(mut self, v: i32) -> Self {
        self.set_int(1196, v);
        self
    }

    pub fn set_player_features(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(1197, a, b, c, d);
        self
    }

    pub fn set_player_ammo_id(mut self, v: i32) -> Self {
        self.set_int(1198, v);
        self
    }

    pub fn set_player_self_res_spell(mut self, v: i32) -> Self {
        self.set_int(1199, v);
        self
    }

    pub fn set_player_pvp_medals(mut self, v: i32) -> Self {
        self.set_int(1200, v);
        self
    }

    pub fn set_player_buyback_price_1(mut self, v: i32) -> Self {
        self.set_int(1201, v);
        self
    }

    pub fn set_player_buyback_timestamp_1(mut self, v: i32) -> Self {
        self.set_int(1213, v);
        self
    }

    pub fn set_player_kills(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(1225, a, b);
        self
    }

    pub fn set_player_today_contribution(mut self, v: i32) -> Self {
        self.set_int(1226, v);
        self
    }

    pub fn set_player_yesterday_contribution(mut self, v: i32) -> Self {
        self.set_int(1227, v);
        self
    }

    pub fn set_player_lifetime_honorbale_kills(mut self, v: i32) -> Self {
        self.set_int(1228, v);
        self
    }

    pub fn set_player_bytes2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(1229, a, b, c, d);
        self
    }

    pub fn set_player_watched_faction_index(mut self, v: i32) -> Self {
        self.set_int(1230, v);
        self
    }

    pub fn set_player_combat_rating_1(mut self, v: i32) -> Self {
        self.set_int(1231, v);
        self
    }

    pub fn set_player_arena_team_info_1_1(mut self, v: i32) -> Self {
        self.set_int(1256, v);
        self
    }

    pub fn set_player_honor_currency(mut self, v: i32) -> Self {
        self.set_int(1277, v);
        self
    }

    pub fn set_player_arena_currency(mut self, v: i32) -> Self {
        self.set_int(1278, v);
        self
    }

    pub fn set_player_max_level(mut self, v: i32) -> Self {
        self.set_int(1279, v);
        self
    }

    pub fn set_player_daily_quests_1(mut self, v: i32) -> Self {
        self.set_int(1280, v);
        self
    }

    pub fn set_player_rune_regen_1(mut self, v: f32) -> Self {
        self.set_float(1305, v);
        self
    }

    pub fn set_player_no_reagent_cost_1(mut self, v: i32) -> Self {
        self.set_int(1309, v);
        self
    }

    pub fn set_player_glyph_slots_1(mut self, v: i32) -> Self {
        self.set_int(1312, v);
        self
    }

    pub fn set_player_glyphs_1(mut self, v: i32) -> Self {
        self.set_int(1318, v);
        self
    }

    pub fn set_player_glyphs_enabled(mut self, v: i32) -> Self {
        self.set_int(1324, v);
        self
    }

    pub fn set_player_pet_spell_power(mut self, v: i32) -> Self {
        self.set_int(1325, v);
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

}

impl UpdateGameObjectBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.set_guid(0, v);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.set_int(2, v);
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.set_int(3, v);
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.set_float(4, v);
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

    pub fn set_gameobject_displayid(mut self, v: i32) -> Self {
        self.set_int(8, v);
        self
    }

    pub fn set_gameobject_flags(mut self, v: i32) -> Self {
        self.set_int(9, v);
        self
    }

    pub fn set_gameobject_parentrotation(mut self, v: f32) -> Self {
        self.set_float(10, v);
        self
    }

    pub fn set_gameobject_dynamic(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(14, a, b);
        self
    }

    pub fn set_gameobject_faction(mut self, v: i32) -> Self {
        self.set_int(15, v);
        self
    }

    pub fn set_gameobject_level(mut self, v: i32) -> Self {
        self.set_int(16, v);
        self
    }

    pub fn set_gameobject_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(17, a, b, c, d);
        self
    }

}

impl UpdateDynamicObjectBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.set_guid(0, v);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.set_int(2, v);
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.set_int(3, v);
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.set_float(4, v);
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

    pub fn set_dynamicobject_caster(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

    pub fn set_dynamicobject_bytes(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(8, a, b, c, d);
        self
    }

    pub fn set_dynamicobject_spellid(mut self, v: i32) -> Self {
        self.set_int(9, v);
        self
    }

    pub fn set_dynamicobject_radius(mut self, v: f32) -> Self {
        self.set_float(10, v);
        self
    }

    pub fn set_dynamicobject_casttime(mut self, v: i32) -> Self {
        self.set_int(11, v);
        self
    }

}

impl UpdateCorpseBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.set_guid(0, v);
        self
    }

    pub fn set_object_type(mut self, v: i32) -> Self {
        self.set_int(2, v);
        self
    }

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.set_int(3, v);
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.set_float(4, v);
        self
    }

    pub fn set_object_created_by(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

    pub fn set_corpse_owner(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

    pub fn set_corpse_party(mut self, v: Guid) -> Self {
        self.set_guid(8, v);
        self
    }

    pub fn set_corpse_display_id(mut self, v: i32) -> Self {
        self.set_int(10, v);
        self
    }

    pub fn set_corpse_item(mut self, v: i32) -> Self {
        self.set_int(11, v);
        self
    }

    pub fn set_corpse_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(30, a, b, c, d);
        self
    }

    pub fn set_corpse_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(31, a, b, c, d);
        self
    }

    pub fn set_corpse_guild(mut self, v: i32) -> Self {
        self.set_int(32, v);
        self
    }

    pub fn set_corpse_flags(mut self, v: i32) -> Self {
        self.set_int(33, v);
        self
    }

    pub fn set_corpse_dynamic_flags(mut self, v: i32) -> Self {
        self.set_int(34, v);
        self
    }

}

impl UpdateItem {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.set_guid(0, v);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        self.get_guid(0)
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.set_int(2, v);
    }

    pub fn object_type(&self) -> Option<i32> {
        self.get_int(2)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.set_int(3, v);
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.get_int(3)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.set_float(4, v);
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.get_float(4)
    }

    pub fn set_item_owner(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn item_owner(&self) -> Option<Guid> {
        self.get_guid(6)
    }

    pub fn set_item_contained(&mut self, v: Guid) {
        self.set_guid(8, v);
    }

    pub fn item_contained(&self) -> Option<Guid> {
        self.get_guid(8)
    }

    pub fn set_item_creator(&mut self, v: Guid) {
        self.set_guid(10, v);
    }

    pub fn item_creator(&self) -> Option<Guid> {
        self.get_guid(10)
    }

    pub fn set_item_giftcreator(&mut self, v: Guid) {
        self.set_guid(12, v);
    }

    pub fn item_giftcreator(&self) -> Option<Guid> {
        self.get_guid(12)
    }

    pub fn set_item_stack_count(&mut self, v: i32) {
        self.set_int(14, v);
    }

    pub fn item_stack_count(&self) -> Option<i32> {
        self.get_int(14)
    }

    pub fn set_item_duration(&mut self, v: i32) {
        self.set_int(15, v);
    }

    pub fn item_duration(&self) -> Option<i32> {
        self.get_int(15)
    }

    pub fn set_item_spell_charges(&mut self, v: i32) {
        self.set_int(16, v);
    }

    pub fn item_spell_charges(&self) -> Option<i32> {
        self.get_int(16)
    }

    pub fn set_item_flags(&mut self, v: i32) {
        self.set_int(21, v);
    }

    pub fn item_flags(&self) -> Option<i32> {
        self.get_int(21)
    }

    pub fn set_item_enchantment_1_1(&mut self, v: i32) {
        self.set_int(22, v);
    }

    pub fn item_enchantment_1_1(&self) -> Option<i32> {
        self.get_int(22)
    }

    pub fn set_item_enchantment_1_3(&mut self, a: u16, b: u16) {
        self.set_shorts(24, a, b);
    }

    pub fn item_enchantment_1_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(24)
    }

    pub fn set_item_enchantment_2_1(&mut self, v: i32) {
        self.set_int(25, v);
    }

    pub fn item_enchantment_2_1(&self) -> Option<i32> {
        self.get_int(25)
    }

    pub fn set_item_enchantment_2_3(&mut self, a: u16, b: u16) {
        self.set_shorts(27, a, b);
    }

    pub fn item_enchantment_2_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(27)
    }

    pub fn set_item_enchantment_3_1(&mut self, v: i32) {
        self.set_int(28, v);
    }

    pub fn item_enchantment_3_1(&self) -> Option<i32> {
        self.get_int(28)
    }

    pub fn set_item_enchantment_3_3(&mut self, a: u16, b: u16) {
        self.set_shorts(30, a, b);
    }

    pub fn item_enchantment_3_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(30)
    }

    pub fn set_item_enchantment_4_1(&mut self, v: i32) {
        self.set_int(31, v);
    }

    pub fn item_enchantment_4_1(&self) -> Option<i32> {
        self.get_int(31)
    }

    pub fn set_item_enchantment_4_3(&mut self, a: u16, b: u16) {
        self.set_shorts(33, a, b);
    }

    pub fn item_enchantment_4_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(33)
    }

    pub fn set_item_enchantment_5_1(&mut self, v: i32) {
        self.set_int(34, v);
    }

    pub fn item_enchantment_5_1(&self) -> Option<i32> {
        self.get_int(34)
    }

    pub fn set_item_enchantment_5_3(&mut self, a: u16, b: u16) {
        self.set_shorts(36, a, b);
    }

    pub fn item_enchantment_5_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(36)
    }

    pub fn set_item_enchantment_6_1(&mut self, v: i32) {
        self.set_int(37, v);
    }

    pub fn item_enchantment_6_1(&self) -> Option<i32> {
        self.get_int(37)
    }

    pub fn set_item_enchantment_6_3(&mut self, a: u16, b: u16) {
        self.set_shorts(39, a, b);
    }

    pub fn item_enchantment_6_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(39)
    }

    pub fn set_item_enchantment_7_1(&mut self, v: i32) {
        self.set_int(40, v);
    }

    pub fn item_enchantment_7_1(&self) -> Option<i32> {
        self.get_int(40)
    }

    pub fn set_item_enchantment_7_3(&mut self, a: u16, b: u16) {
        self.set_shorts(42, a, b);
    }

    pub fn item_enchantment_7_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(42)
    }

    pub fn set_item_enchantment_8_1(&mut self, v: i32) {
        self.set_int(43, v);
    }

    pub fn item_enchantment_8_1(&self) -> Option<i32> {
        self.get_int(43)
    }

    pub fn set_item_enchantment_8_3(&mut self, a: u16, b: u16) {
        self.set_shorts(45, a, b);
    }

    pub fn item_enchantment_8_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(45)
    }

    pub fn set_item_enchantment_9_1(&mut self, v: i32) {
        self.set_int(46, v);
    }

    pub fn item_enchantment_9_1(&self) -> Option<i32> {
        self.get_int(46)
    }

    pub fn set_item_enchantment_9_3(&mut self, a: u16, b: u16) {
        self.set_shorts(48, a, b);
    }

    pub fn item_enchantment_9_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(48)
    }

    pub fn set_item_enchantment_10_1(&mut self, v: i32) {
        self.set_int(49, v);
    }

    pub fn item_enchantment_10_1(&self) -> Option<i32> {
        self.get_int(49)
    }

    pub fn set_item_enchantment_10_3(&mut self, a: u16, b: u16) {
        self.set_shorts(51, a, b);
    }

    pub fn item_enchantment_10_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(51)
    }

    pub fn set_item_enchantment_11_1(&mut self, v: i32) {
        self.set_int(52, v);
    }

    pub fn item_enchantment_11_1(&self) -> Option<i32> {
        self.get_int(52)
    }

    pub fn set_item_enchantment_11_3(&mut self, a: u16, b: u16) {
        self.set_shorts(54, a, b);
    }

    pub fn item_enchantment_11_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(54)
    }

    pub fn set_item_enchantment_12_1(&mut self, v: i32) {
        self.set_int(55, v);
    }

    pub fn item_enchantment_12_1(&self) -> Option<i32> {
        self.get_int(55)
    }

    pub fn set_item_enchantment_12_3(&mut self, a: u16, b: u16) {
        self.set_shorts(57, a, b);
    }

    pub fn item_enchantment_12_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(57)
    }

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.set_int(58, v);
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.get_int(58)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.set_int(59, v);
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.get_int(59)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.set_int(60, v);
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.get_int(60)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.set_int(61, v);
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.get_int(61)
    }

    pub fn set_item_create_played_time(&mut self, v: i32) {
        self.set_int(62, v);
    }

    pub fn item_create_played_time(&self) -> Option<i32> {
        self.get_int(62)
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        self.get_guid(6)
    }

}

impl UpdateContainer {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.set_guid(0, v);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        self.get_guid(0)
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.set_int(2, v);
    }

    pub fn object_type(&self) -> Option<i32> {
        self.get_int(2)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.set_int(3, v);
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.get_int(3)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.set_float(4, v);
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.get_float(4)
    }

    pub fn set_item_owner(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn item_owner(&self) -> Option<Guid> {
        self.get_guid(6)
    }

    pub fn set_item_contained(&mut self, v: Guid) {
        self.set_guid(8, v);
    }

    pub fn item_contained(&self) -> Option<Guid> {
        self.get_guid(8)
    }

    pub fn set_item_creator(&mut self, v: Guid) {
        self.set_guid(10, v);
    }

    pub fn item_creator(&self) -> Option<Guid> {
        self.get_guid(10)
    }

    pub fn set_item_giftcreator(&mut self, v: Guid) {
        self.set_guid(12, v);
    }

    pub fn item_giftcreator(&self) -> Option<Guid> {
        self.get_guid(12)
    }

    pub fn set_item_stack_count(&mut self, v: i32) {
        self.set_int(14, v);
    }

    pub fn item_stack_count(&self) -> Option<i32> {
        self.get_int(14)
    }

    pub fn set_item_duration(&mut self, v: i32) {
        self.set_int(15, v);
    }

    pub fn item_duration(&self) -> Option<i32> {
        self.get_int(15)
    }

    pub fn set_item_spell_charges(&mut self, v: i32) {
        self.set_int(16, v);
    }

    pub fn item_spell_charges(&self) -> Option<i32> {
        self.get_int(16)
    }

    pub fn set_item_flags(&mut self, v: i32) {
        self.set_int(21, v);
    }

    pub fn item_flags(&self) -> Option<i32> {
        self.get_int(21)
    }

    pub fn set_item_enchantment_1_1(&mut self, v: i32) {
        self.set_int(22, v);
    }

    pub fn item_enchantment_1_1(&self) -> Option<i32> {
        self.get_int(22)
    }

    pub fn set_item_enchantment_1_3(&mut self, a: u16, b: u16) {
        self.set_shorts(24, a, b);
    }

    pub fn item_enchantment_1_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(24)
    }

    pub fn set_item_enchantment_2_1(&mut self, v: i32) {
        self.set_int(25, v);
    }

    pub fn item_enchantment_2_1(&self) -> Option<i32> {
        self.get_int(25)
    }

    pub fn set_item_enchantment_2_3(&mut self, a: u16, b: u16) {
        self.set_shorts(27, a, b);
    }

    pub fn item_enchantment_2_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(27)
    }

    pub fn set_item_enchantment_3_1(&mut self, v: i32) {
        self.set_int(28, v);
    }

    pub fn item_enchantment_3_1(&self) -> Option<i32> {
        self.get_int(28)
    }

    pub fn set_item_enchantment_3_3(&mut self, a: u16, b: u16) {
        self.set_shorts(30, a, b);
    }

    pub fn item_enchantment_3_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(30)
    }

    pub fn set_item_enchantment_4_1(&mut self, v: i32) {
        self.set_int(31, v);
    }

    pub fn item_enchantment_4_1(&self) -> Option<i32> {
        self.get_int(31)
    }

    pub fn set_item_enchantment_4_3(&mut self, a: u16, b: u16) {
        self.set_shorts(33, a, b);
    }

    pub fn item_enchantment_4_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(33)
    }

    pub fn set_item_enchantment_5_1(&mut self, v: i32) {
        self.set_int(34, v);
    }

    pub fn item_enchantment_5_1(&self) -> Option<i32> {
        self.get_int(34)
    }

    pub fn set_item_enchantment_5_3(&mut self, a: u16, b: u16) {
        self.set_shorts(36, a, b);
    }

    pub fn item_enchantment_5_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(36)
    }

    pub fn set_item_enchantment_6_1(&mut self, v: i32) {
        self.set_int(37, v);
    }

    pub fn item_enchantment_6_1(&self) -> Option<i32> {
        self.get_int(37)
    }

    pub fn set_item_enchantment_6_3(&mut self, a: u16, b: u16) {
        self.set_shorts(39, a, b);
    }

    pub fn item_enchantment_6_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(39)
    }

    pub fn set_item_enchantment_7_1(&mut self, v: i32) {
        self.set_int(40, v);
    }

    pub fn item_enchantment_7_1(&self) -> Option<i32> {
        self.get_int(40)
    }

    pub fn set_item_enchantment_7_3(&mut self, a: u16, b: u16) {
        self.set_shorts(42, a, b);
    }

    pub fn item_enchantment_7_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(42)
    }

    pub fn set_item_enchantment_8_1(&mut self, v: i32) {
        self.set_int(43, v);
    }

    pub fn item_enchantment_8_1(&self) -> Option<i32> {
        self.get_int(43)
    }

    pub fn set_item_enchantment_8_3(&mut self, a: u16, b: u16) {
        self.set_shorts(45, a, b);
    }

    pub fn item_enchantment_8_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(45)
    }

    pub fn set_item_enchantment_9_1(&mut self, v: i32) {
        self.set_int(46, v);
    }

    pub fn item_enchantment_9_1(&self) -> Option<i32> {
        self.get_int(46)
    }

    pub fn set_item_enchantment_9_3(&mut self, a: u16, b: u16) {
        self.set_shorts(48, a, b);
    }

    pub fn item_enchantment_9_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(48)
    }

    pub fn set_item_enchantment_10_1(&mut self, v: i32) {
        self.set_int(49, v);
    }

    pub fn item_enchantment_10_1(&self) -> Option<i32> {
        self.get_int(49)
    }

    pub fn set_item_enchantment_10_3(&mut self, a: u16, b: u16) {
        self.set_shorts(51, a, b);
    }

    pub fn item_enchantment_10_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(51)
    }

    pub fn set_item_enchantment_11_1(&mut self, v: i32) {
        self.set_int(52, v);
    }

    pub fn item_enchantment_11_1(&self) -> Option<i32> {
        self.get_int(52)
    }

    pub fn set_item_enchantment_11_3(&mut self, a: u16, b: u16) {
        self.set_shorts(54, a, b);
    }

    pub fn item_enchantment_11_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(54)
    }

    pub fn set_item_enchantment_12_1(&mut self, v: i32) {
        self.set_int(55, v);
    }

    pub fn item_enchantment_12_1(&self) -> Option<i32> {
        self.get_int(55)
    }

    pub fn set_item_enchantment_12_3(&mut self, a: u16, b: u16) {
        self.set_shorts(57, a, b);
    }

    pub fn item_enchantment_12_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(57)
    }

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.set_int(58, v);
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.get_int(58)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.set_int(59, v);
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.get_int(59)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.set_int(60, v);
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.get_int(60)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.set_int(61, v);
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.get_int(61)
    }

    pub fn set_item_create_played_time(&mut self, v: i32) {
        self.set_int(62, v);
    }

    pub fn item_create_played_time(&self) -> Option<i32> {
        self.get_int(62)
    }

    pub fn set_container_num_slots(&mut self, v: i32) {
        self.set_int(64, v);
    }

    pub fn container_num_slots(&self) -> Option<i32> {
        self.get_int(64)
    }

    pub fn set_container_slot_1(&mut self, v: Guid) {
        self.set_guid(66, v);
    }

    pub fn container_slot_1(&self) -> Option<Guid> {
        self.get_guid(66)
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        self.get_guid(6)
    }

}

impl UpdateUnit {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.set_guid(0, v);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        self.get_guid(0)
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.set_int(2, v);
    }

    pub fn object_type(&self) -> Option<i32> {
        self.get_int(2)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.set_int(3, v);
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.get_int(3)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.set_float(4, v);
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.get_float(4)
    }

    pub fn set_unit_charm(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn unit_charm(&self) -> Option<Guid> {
        self.get_guid(6)
    }

    pub fn set_unit_summon(&mut self, v: Guid) {
        self.set_guid(8, v);
    }

    pub fn unit_summon(&self) -> Option<Guid> {
        self.get_guid(8)
    }

    pub fn set_unit_critter(&mut self, v: Guid) {
        self.set_guid(10, v);
    }

    pub fn unit_critter(&self) -> Option<Guid> {
        self.get_guid(10)
    }

    pub fn set_unit_charmedby(&mut self, v: Guid) {
        self.set_guid(12, v);
    }

    pub fn unit_charmedby(&self) -> Option<Guid> {
        self.get_guid(12)
    }

    pub fn set_unit_summonedby(&mut self, v: Guid) {
        self.set_guid(14, v);
    }

    pub fn unit_summonedby(&self) -> Option<Guid> {
        self.get_guid(14)
    }

    pub fn set_unit_createdby(&mut self, v: Guid) {
        self.set_guid(16, v);
    }

    pub fn unit_createdby(&self) -> Option<Guid> {
        self.get_guid(16)
    }

    pub fn set_unit_target(&mut self, v: Guid) {
        self.set_guid(18, v);
    }

    pub fn unit_target(&self) -> Option<Guid> {
        self.get_guid(18)
    }

    pub fn set_unit_channel_object(&mut self, v: Guid) {
        self.set_guid(20, v);
    }

    pub fn unit_channel_object(&self) -> Option<Guid> {
        self.get_guid(20)
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.set_int(22, v);
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.get_int(22)
    }

    pub fn set_unit_bytes_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.set_bytes(23, race.as_int(), class.as_int(), gender.as_int(), power.as_int());
    }

    pub fn unit_bytes_0(&self) -> Option<(Race, Class, Gender, Power)> {
        self.get_bytes(23).map(|(race, class, gender, power)| {
            (race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap())
        })
    }

    pub fn set_unit_health(&mut self, v: i32) {
        self.set_int(24, v);
    }

    pub fn unit_health(&self) -> Option<i32> {
        self.get_int(24)
    }

    pub fn set_unit_power1(&mut self, v: i32) {
        self.set_int(25, v);
    }

    pub fn unit_power1(&self) -> Option<i32> {
        self.get_int(25)
    }

    pub fn set_unit_power2(&mut self, v: i32) {
        self.set_int(26, v);
    }

    pub fn unit_power2(&self) -> Option<i32> {
        self.get_int(26)
    }

    pub fn set_unit_power3(&mut self, v: i32) {
        self.set_int(27, v);
    }

    pub fn unit_power3(&self) -> Option<i32> {
        self.get_int(27)
    }

    pub fn set_unit_power4(&mut self, v: i32) {
        self.set_int(28, v);
    }

    pub fn unit_power4(&self) -> Option<i32> {
        self.get_int(28)
    }

    pub fn set_unit_power5(&mut self, v: i32) {
        self.set_int(29, v);
    }

    pub fn unit_power5(&self) -> Option<i32> {
        self.get_int(29)
    }

    pub fn set_unit_power6(&mut self, v: i32) {
        self.set_int(30, v);
    }

    pub fn unit_power6(&self) -> Option<i32> {
        self.get_int(30)
    }

    pub fn set_unit_power7(&mut self, v: i32) {
        self.set_int(31, v);
    }

    pub fn unit_power7(&self) -> Option<i32> {
        self.get_int(31)
    }

    pub fn set_unit_maxhealth(&mut self, v: i32) {
        self.set_int(32, v);
    }

    pub fn unit_maxhealth(&self) -> Option<i32> {
        self.get_int(32)
    }

    pub fn set_unit_maxpower1(&mut self, v: i32) {
        self.set_int(33, v);
    }

    pub fn unit_maxpower1(&self) -> Option<i32> {
        self.get_int(33)
    }

    pub fn set_unit_maxpower2(&mut self, v: i32) {
        self.set_int(34, v);
    }

    pub fn unit_maxpower2(&self) -> Option<i32> {
        self.get_int(34)
    }

    pub fn set_unit_maxpower3(&mut self, v: i32) {
        self.set_int(35, v);
    }

    pub fn unit_maxpower3(&self) -> Option<i32> {
        self.get_int(35)
    }

    pub fn set_unit_maxpower4(&mut self, v: i32) {
        self.set_int(36, v);
    }

    pub fn unit_maxpower4(&self) -> Option<i32> {
        self.get_int(36)
    }

    pub fn set_unit_maxpower5(&mut self, v: i32) {
        self.set_int(37, v);
    }

    pub fn unit_maxpower5(&self) -> Option<i32> {
        self.get_int(37)
    }

    pub fn set_unit_maxpower6(&mut self, v: i32) {
        self.set_int(38, v);
    }

    pub fn unit_maxpower6(&self) -> Option<i32> {
        self.get_int(38)
    }

    pub fn set_unit_maxpower7(&mut self, v: i32) {
        self.set_int(39, v);
    }

    pub fn unit_maxpower7(&self) -> Option<i32> {
        self.get_int(39)
    }

    pub fn set_unit_power_regen_flat_modifier(&mut self, v: f32) {
        self.set_float(40, v);
    }

    pub fn unit_power_regen_flat_modifier(&self) -> Option<f32> {
        self.get_float(40)
    }

    pub fn set_unit_power_regen_interrupted_flat_modifier(&mut self, v: f32) {
        self.set_float(47, v);
    }

    pub fn unit_power_regen_interrupted_flat_modifier(&self) -> Option<f32> {
        self.get_float(47)
    }

    pub fn set_unit_level(&mut self, v: i32) {
        self.set_int(54, v);
    }

    pub fn unit_level(&self) -> Option<i32> {
        self.get_int(54)
    }

    pub fn set_unit_factiontemplate(&mut self, v: i32) {
        self.set_int(55, v);
    }

    pub fn unit_factiontemplate(&self) -> Option<i32> {
        self.get_int(55)
    }

    pub fn set_unit_virtual_item_slot_id(&mut self, v: i32) {
        self.set_int(56, v);
    }

    pub fn unit_virtual_item_slot_id(&self) -> Option<i32> {
        self.get_int(56)
    }

    pub fn set_unit_flags(&mut self, v: i32) {
        self.set_int(59, v);
    }

    pub fn unit_flags(&self) -> Option<i32> {
        self.get_int(59)
    }

    pub fn set_unit_flags_2(&mut self, v: i32) {
        self.set_int(60, v);
    }

    pub fn unit_flags_2(&self) -> Option<i32> {
        self.get_int(60)
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.set_int(61, v);
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.get_int(61)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.set_int(62, v);
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.get_int(62)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.set_int(64, v);
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.get_int(64)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.set_float(65, v);
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.get_float(65)
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.set_float(66, v);
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.get_float(66)
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.set_int(67, v);
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.get_int(67)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.set_int(68, v);
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.get_int(68)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.set_int(69, v);
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.get_int(69)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.set_float(70, v);
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.get_float(70)
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.set_float(71, v);
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.get_float(71)
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.set_float(72, v);
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.get_float(72)
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.set_float(73, v);
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.get_float(73)
    }

    pub fn set_unit_bytes_1(&mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) {
        self.set_bytes(74, stand_state.as_int(), unknown1, unknown2, unknown3);
    }

    pub fn unit_bytes_1(&self) -> Option<(UnitStandState, u8, u8, u8)> {
        self.get_bytes(74).map(|(stand_state, unknown1, unknown2, unknown3)| {
            (stand_state.try_into().unwrap(), unknown1, unknown2, unknown3)
        })
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.set_int(75, v);
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.get_int(75)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.set_int(76, v);
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.get_int(76)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.set_int(77, v);
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.get_int(77)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.set_int(78, v);
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.get_int(78)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.set_int(79, v);
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.get_int(79)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.set_float(80, v);
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.get_float(80)
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.set_int(81, v);
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.get_int(81)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.set_int(82, v);
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.get_int(82)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.set_int(83, v);
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.get_int(83)
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.set_int(84, v);
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.get_int(84)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.set_int(85, v);
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.get_int(85)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.set_int(86, v);
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.get_int(86)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.set_int(87, v);
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.get_int(87)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.set_int(88, v);
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.get_int(88)
    }

    pub fn set_unit_posstat0(&mut self, v: i32) {
        self.set_int(89, v);
    }

    pub fn unit_posstat0(&self) -> Option<i32> {
        self.get_int(89)
    }

    pub fn set_unit_posstat1(&mut self, v: i32) {
        self.set_int(90, v);
    }

    pub fn unit_posstat1(&self) -> Option<i32> {
        self.get_int(90)
    }

    pub fn set_unit_posstat2(&mut self, v: i32) {
        self.set_int(91, v);
    }

    pub fn unit_posstat2(&self) -> Option<i32> {
        self.get_int(91)
    }

    pub fn set_unit_posstat3(&mut self, v: i32) {
        self.set_int(92, v);
    }

    pub fn unit_posstat3(&self) -> Option<i32> {
        self.get_int(92)
    }

    pub fn set_unit_posstat4(&mut self, v: i32) {
        self.set_int(93, v);
    }

    pub fn unit_posstat4(&self) -> Option<i32> {
        self.get_int(93)
    }

    pub fn set_unit_negstat0(&mut self, v: i32) {
        self.set_int(94, v);
    }

    pub fn unit_negstat0(&self) -> Option<i32> {
        self.get_int(94)
    }

    pub fn set_unit_negstat1(&mut self, v: i32) {
        self.set_int(95, v);
    }

    pub fn unit_negstat1(&self) -> Option<i32> {
        self.get_int(95)
    }

    pub fn set_unit_negstat2(&mut self, v: i32) {
        self.set_int(96, v);
    }

    pub fn unit_negstat2(&self) -> Option<i32> {
        self.get_int(96)
    }

    pub fn set_unit_negstat3(&mut self, v: i32) {
        self.set_int(97, v);
    }

    pub fn unit_negstat3(&self) -> Option<i32> {
        self.get_int(97)
    }

    pub fn set_unit_negstat4(&mut self, v: i32) {
        self.set_int(98, v);
    }

    pub fn unit_negstat4(&self) -> Option<i32> {
        self.get_int(98)
    }

    pub fn set_unit_resistances(&mut self, v: i32) {
        self.set_int(99, v);
    }

    pub fn unit_resistances(&self) -> Option<i32> {
        self.get_int(99)
    }

    pub fn set_unit_resistancebuffmodspositive(&mut self, v: i32) {
        self.set_int(106, v);
    }

    pub fn unit_resistancebuffmodspositive(&self) -> Option<i32> {
        self.get_int(106)
    }

    pub fn set_unit_resistancebuffmodsnegative(&mut self, v: i32) {
        self.set_int(113, v);
    }

    pub fn unit_resistancebuffmodsnegative(&self) -> Option<i32> {
        self.get_int(113)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.set_int(120, v);
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.get_int(120)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.set_int(121, v);
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.get_int(121)
    }

    pub fn set_unit_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(122, a, b, c, d);
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(122)
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.set_int(123, v);
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.get_int(123)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(124, a, b);
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(124)
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(125, v);
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(125)
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.set_int(126, v);
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.get_int(126)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(127, a, b);
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(127)
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(128, v);
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(128)
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.set_float(129, v);
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.get_float(129)
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.set_float(130, v);
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.get_float(130)
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.set_int(131, v);
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.get_int(131)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.set_float(138, v);
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.get_float(138)
    }

    pub fn set_unit_maxhealthmodifier(&mut self, v: f32) {
        self.set_float(145, v);
    }

    pub fn unit_maxhealthmodifier(&self) -> Option<f32> {
        self.get_float(145)
    }

    pub fn set_unit_hoverheight(&mut self, v: f32) {
        self.set_float(146, v);
    }

    pub fn unit_hoverheight(&self) -> Option<f32> {
        self.get_float(146)
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        self.get_guid(6)
    }

}

impl UpdatePlayer {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.set_guid(0, v);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        self.get_guid(0)
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.set_int(2, v);
    }

    pub fn object_type(&self) -> Option<i32> {
        self.get_int(2)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.set_int(3, v);
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.get_int(3)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.set_float(4, v);
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.get_float(4)
    }

    pub fn set_unit_charm(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn unit_charm(&self) -> Option<Guid> {
        self.get_guid(6)
    }

    pub fn set_unit_summon(&mut self, v: Guid) {
        self.set_guid(8, v);
    }

    pub fn unit_summon(&self) -> Option<Guid> {
        self.get_guid(8)
    }

    pub fn set_unit_critter(&mut self, v: Guid) {
        self.set_guid(10, v);
    }

    pub fn unit_critter(&self) -> Option<Guid> {
        self.get_guid(10)
    }

    pub fn set_unit_charmedby(&mut self, v: Guid) {
        self.set_guid(12, v);
    }

    pub fn unit_charmedby(&self) -> Option<Guid> {
        self.get_guid(12)
    }

    pub fn set_unit_summonedby(&mut self, v: Guid) {
        self.set_guid(14, v);
    }

    pub fn unit_summonedby(&self) -> Option<Guid> {
        self.get_guid(14)
    }

    pub fn set_unit_createdby(&mut self, v: Guid) {
        self.set_guid(16, v);
    }

    pub fn unit_createdby(&self) -> Option<Guid> {
        self.get_guid(16)
    }

    pub fn set_unit_target(&mut self, v: Guid) {
        self.set_guid(18, v);
    }

    pub fn unit_target(&self) -> Option<Guid> {
        self.get_guid(18)
    }

    pub fn set_unit_channel_object(&mut self, v: Guid) {
        self.set_guid(20, v);
    }

    pub fn unit_channel_object(&self) -> Option<Guid> {
        self.get_guid(20)
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.set_int(22, v);
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.get_int(22)
    }

    pub fn set_unit_bytes_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.set_bytes(23, race.as_int(), class.as_int(), gender.as_int(), power.as_int());
    }

    pub fn unit_bytes_0(&self) -> Option<(Race, Class, Gender, Power)> {
        self.get_bytes(23).map(|(race, class, gender, power)| {
            (race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap())
        })
    }

    pub fn set_unit_health(&mut self, v: i32) {
        self.set_int(24, v);
    }

    pub fn unit_health(&self) -> Option<i32> {
        self.get_int(24)
    }

    pub fn set_unit_power1(&mut self, v: i32) {
        self.set_int(25, v);
    }

    pub fn unit_power1(&self) -> Option<i32> {
        self.get_int(25)
    }

    pub fn set_unit_power2(&mut self, v: i32) {
        self.set_int(26, v);
    }

    pub fn unit_power2(&self) -> Option<i32> {
        self.get_int(26)
    }

    pub fn set_unit_power3(&mut self, v: i32) {
        self.set_int(27, v);
    }

    pub fn unit_power3(&self) -> Option<i32> {
        self.get_int(27)
    }

    pub fn set_unit_power4(&mut self, v: i32) {
        self.set_int(28, v);
    }

    pub fn unit_power4(&self) -> Option<i32> {
        self.get_int(28)
    }

    pub fn set_unit_power5(&mut self, v: i32) {
        self.set_int(29, v);
    }

    pub fn unit_power5(&self) -> Option<i32> {
        self.get_int(29)
    }

    pub fn set_unit_power6(&mut self, v: i32) {
        self.set_int(30, v);
    }

    pub fn unit_power6(&self) -> Option<i32> {
        self.get_int(30)
    }

    pub fn set_unit_power7(&mut self, v: i32) {
        self.set_int(31, v);
    }

    pub fn unit_power7(&self) -> Option<i32> {
        self.get_int(31)
    }

    pub fn set_unit_maxhealth(&mut self, v: i32) {
        self.set_int(32, v);
    }

    pub fn unit_maxhealth(&self) -> Option<i32> {
        self.get_int(32)
    }

    pub fn set_unit_maxpower1(&mut self, v: i32) {
        self.set_int(33, v);
    }

    pub fn unit_maxpower1(&self) -> Option<i32> {
        self.get_int(33)
    }

    pub fn set_unit_maxpower2(&mut self, v: i32) {
        self.set_int(34, v);
    }

    pub fn unit_maxpower2(&self) -> Option<i32> {
        self.get_int(34)
    }

    pub fn set_unit_maxpower3(&mut self, v: i32) {
        self.set_int(35, v);
    }

    pub fn unit_maxpower3(&self) -> Option<i32> {
        self.get_int(35)
    }

    pub fn set_unit_maxpower4(&mut self, v: i32) {
        self.set_int(36, v);
    }

    pub fn unit_maxpower4(&self) -> Option<i32> {
        self.get_int(36)
    }

    pub fn set_unit_maxpower5(&mut self, v: i32) {
        self.set_int(37, v);
    }

    pub fn unit_maxpower5(&self) -> Option<i32> {
        self.get_int(37)
    }

    pub fn set_unit_maxpower6(&mut self, v: i32) {
        self.set_int(38, v);
    }

    pub fn unit_maxpower6(&self) -> Option<i32> {
        self.get_int(38)
    }

    pub fn set_unit_maxpower7(&mut self, v: i32) {
        self.set_int(39, v);
    }

    pub fn unit_maxpower7(&self) -> Option<i32> {
        self.get_int(39)
    }

    pub fn set_unit_power_regen_flat_modifier(&mut self, v: f32) {
        self.set_float(40, v);
    }

    pub fn unit_power_regen_flat_modifier(&self) -> Option<f32> {
        self.get_float(40)
    }

    pub fn set_unit_power_regen_interrupted_flat_modifier(&mut self, v: f32) {
        self.set_float(47, v);
    }

    pub fn unit_power_regen_interrupted_flat_modifier(&self) -> Option<f32> {
        self.get_float(47)
    }

    pub fn set_unit_level(&mut self, v: i32) {
        self.set_int(54, v);
    }

    pub fn unit_level(&self) -> Option<i32> {
        self.get_int(54)
    }

    pub fn set_unit_factiontemplate(&mut self, v: i32) {
        self.set_int(55, v);
    }

    pub fn unit_factiontemplate(&self) -> Option<i32> {
        self.get_int(55)
    }

    pub fn set_unit_virtual_item_slot_id(&mut self, v: i32) {
        self.set_int(56, v);
    }

    pub fn unit_virtual_item_slot_id(&self) -> Option<i32> {
        self.get_int(56)
    }

    pub fn set_unit_flags(&mut self, v: i32) {
        self.set_int(59, v);
    }

    pub fn unit_flags(&self) -> Option<i32> {
        self.get_int(59)
    }

    pub fn set_unit_flags_2(&mut self, v: i32) {
        self.set_int(60, v);
    }

    pub fn unit_flags_2(&self) -> Option<i32> {
        self.get_int(60)
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.set_int(61, v);
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.get_int(61)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.set_int(62, v);
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.get_int(62)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.set_int(64, v);
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.get_int(64)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.set_float(65, v);
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.get_float(65)
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.set_float(66, v);
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.get_float(66)
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.set_int(67, v);
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.get_int(67)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.set_int(68, v);
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.get_int(68)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.set_int(69, v);
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.get_int(69)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.set_float(70, v);
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.get_float(70)
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.set_float(71, v);
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.get_float(71)
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.set_float(72, v);
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.get_float(72)
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.set_float(73, v);
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.get_float(73)
    }

    pub fn set_unit_bytes_1(&mut self, stand_state: UnitStandState, unknown1: u8, unknown2: u8, unknown3: u8) {
        self.set_bytes(74, stand_state.as_int(), unknown1, unknown2, unknown3);
    }

    pub fn unit_bytes_1(&self) -> Option<(UnitStandState, u8, u8, u8)> {
        self.get_bytes(74).map(|(stand_state, unknown1, unknown2, unknown3)| {
            (stand_state.try_into().unwrap(), unknown1, unknown2, unknown3)
        })
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.set_int(75, v);
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.get_int(75)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.set_int(76, v);
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.get_int(76)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.set_int(77, v);
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.get_int(77)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.set_int(78, v);
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.get_int(78)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.set_int(79, v);
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.get_int(79)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.set_float(80, v);
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.get_float(80)
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.set_int(81, v);
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.get_int(81)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.set_int(82, v);
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.get_int(82)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.set_int(83, v);
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.get_int(83)
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.set_int(84, v);
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.get_int(84)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.set_int(85, v);
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.get_int(85)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.set_int(86, v);
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.get_int(86)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.set_int(87, v);
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.get_int(87)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.set_int(88, v);
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.get_int(88)
    }

    pub fn set_unit_posstat0(&mut self, v: i32) {
        self.set_int(89, v);
    }

    pub fn unit_posstat0(&self) -> Option<i32> {
        self.get_int(89)
    }

    pub fn set_unit_posstat1(&mut self, v: i32) {
        self.set_int(90, v);
    }

    pub fn unit_posstat1(&self) -> Option<i32> {
        self.get_int(90)
    }

    pub fn set_unit_posstat2(&mut self, v: i32) {
        self.set_int(91, v);
    }

    pub fn unit_posstat2(&self) -> Option<i32> {
        self.get_int(91)
    }

    pub fn set_unit_posstat3(&mut self, v: i32) {
        self.set_int(92, v);
    }

    pub fn unit_posstat3(&self) -> Option<i32> {
        self.get_int(92)
    }

    pub fn set_unit_posstat4(&mut self, v: i32) {
        self.set_int(93, v);
    }

    pub fn unit_posstat4(&self) -> Option<i32> {
        self.get_int(93)
    }

    pub fn set_unit_negstat0(&mut self, v: i32) {
        self.set_int(94, v);
    }

    pub fn unit_negstat0(&self) -> Option<i32> {
        self.get_int(94)
    }

    pub fn set_unit_negstat1(&mut self, v: i32) {
        self.set_int(95, v);
    }

    pub fn unit_negstat1(&self) -> Option<i32> {
        self.get_int(95)
    }

    pub fn set_unit_negstat2(&mut self, v: i32) {
        self.set_int(96, v);
    }

    pub fn unit_negstat2(&self) -> Option<i32> {
        self.get_int(96)
    }

    pub fn set_unit_negstat3(&mut self, v: i32) {
        self.set_int(97, v);
    }

    pub fn unit_negstat3(&self) -> Option<i32> {
        self.get_int(97)
    }

    pub fn set_unit_negstat4(&mut self, v: i32) {
        self.set_int(98, v);
    }

    pub fn unit_negstat4(&self) -> Option<i32> {
        self.get_int(98)
    }

    pub fn set_unit_resistances(&mut self, v: i32) {
        self.set_int(99, v);
    }

    pub fn unit_resistances(&self) -> Option<i32> {
        self.get_int(99)
    }

    pub fn set_unit_resistancebuffmodspositive(&mut self, v: i32) {
        self.set_int(106, v);
    }

    pub fn unit_resistancebuffmodspositive(&self) -> Option<i32> {
        self.get_int(106)
    }

    pub fn set_unit_resistancebuffmodsnegative(&mut self, v: i32) {
        self.set_int(113, v);
    }

    pub fn unit_resistancebuffmodsnegative(&self) -> Option<i32> {
        self.get_int(113)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.set_int(120, v);
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.get_int(120)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.set_int(121, v);
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.get_int(121)
    }

    pub fn set_unit_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(122, a, b, c, d);
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(122)
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.set_int(123, v);
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.get_int(123)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(124, a, b);
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(124)
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(125, v);
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(125)
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.set_int(126, v);
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.get_int(126)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(127, a, b);
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(127)
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(128, v);
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(128)
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.set_float(129, v);
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.get_float(129)
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.set_float(130, v);
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.get_float(130)
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.set_int(131, v);
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.get_int(131)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.set_float(138, v);
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.get_float(138)
    }

    pub fn set_unit_maxhealthmodifier(&mut self, v: f32) {
        self.set_float(145, v);
    }

    pub fn unit_maxhealthmodifier(&self) -> Option<f32> {
        self.get_float(145)
    }

    pub fn set_unit_hoverheight(&mut self, v: f32) {
        self.set_float(146, v);
    }

    pub fn unit_hoverheight(&self) -> Option<f32> {
        self.get_float(146)
    }

    pub fn set_player_duel_arbiter(&mut self, v: Guid) {
        self.set_guid(148, v);
    }

    pub fn player_duel_arbiter(&self) -> Option<Guid> {
        self.get_guid(148)
    }

    pub fn set_player_flags(&mut self, v: i32) {
        self.set_int(150, v);
    }

    pub fn player_flags(&self) -> Option<i32> {
        self.get_int(150)
    }

    pub fn set_player_guildid(&mut self, v: i32) {
        self.set_int(151, v);
    }

    pub fn player_guildid(&self) -> Option<i32> {
        self.get_int(151)
    }

    pub fn set_player_guildrank(&mut self, v: i32) {
        self.set_int(152, v);
    }

    pub fn player_guildrank(&self) -> Option<i32> {
        self.get_int(152)
    }

    pub fn set_player_field_bytes(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(153, a, b, c, d);
    }

    pub fn player_field_bytes(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(153)
    }

    pub fn set_player_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(154, a, b, c, d);
    }

    pub fn player_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(154)
    }

    pub fn set_player_bytes_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(155, a, b, c, d);
    }

    pub fn player_bytes_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(155)
    }

    pub fn set_player_duel_team(&mut self, v: i32) {
        self.set_int(156, v);
    }

    pub fn player_duel_team(&self) -> Option<i32> {
        self.get_int(156)
    }

    pub fn set_player_guild_timestamp(&mut self, v: i32) {
        self.set_int(157, v);
    }

    pub fn player_guild_timestamp(&self) -> Option<i32> {
        self.get_int(157)
    }

    pub fn set_player_quest_log_1_1(&mut self, v: i32) {
        self.set_int(158, v);
    }

    pub fn player_quest_log_1_1(&self) -> Option<i32> {
        self.get_int(158)
    }

    pub fn set_player_quest_log_1_2(&mut self, v: i32) {
        self.set_int(159, v);
    }

    pub fn player_quest_log_1_2(&self) -> Option<i32> {
        self.get_int(159)
    }

    pub fn set_player_quest_log_1_3(&mut self, a: u16, b: u16) {
        self.set_shorts(160, a, b);
    }

    pub fn player_quest_log_1_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(160)
    }

    pub fn set_player_quest_log_1_4(&mut self, v: i32) {
        self.set_int(162, v);
    }

    pub fn player_quest_log_1_4(&self) -> Option<i32> {
        self.get_int(162)
    }

    pub fn set_player_quest_log_2_1(&mut self, v: i32) {
        self.set_int(163, v);
    }

    pub fn player_quest_log_2_1(&self) -> Option<i32> {
        self.get_int(163)
    }

    pub fn set_player_quest_log_2_2(&mut self, v: i32) {
        self.set_int(164, v);
    }

    pub fn player_quest_log_2_2(&self) -> Option<i32> {
        self.get_int(164)
    }

    pub fn set_player_quest_log_2_3(&mut self, a: u16, b: u16) {
        self.set_shorts(165, a, b);
    }

    pub fn player_quest_log_2_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(165)
    }

    pub fn set_player_quest_log_2_5(&mut self, v: i32) {
        self.set_int(167, v);
    }

    pub fn player_quest_log_2_5(&self) -> Option<i32> {
        self.get_int(167)
    }

    pub fn set_player_quest_log_3_1(&mut self, v: i32) {
        self.set_int(168, v);
    }

    pub fn player_quest_log_3_1(&self) -> Option<i32> {
        self.get_int(168)
    }

    pub fn set_player_quest_log_3_2(&mut self, v: i32) {
        self.set_int(169, v);
    }

    pub fn player_quest_log_3_2(&self) -> Option<i32> {
        self.get_int(169)
    }

    pub fn set_player_quest_log_3_3(&mut self, a: u16, b: u16) {
        self.set_shorts(170, a, b);
    }

    pub fn player_quest_log_3_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(170)
    }

    pub fn set_player_quest_log_3_5(&mut self, v: i32) {
        self.set_int(172, v);
    }

    pub fn player_quest_log_3_5(&self) -> Option<i32> {
        self.get_int(172)
    }

    pub fn set_player_quest_log_4_1(&mut self, v: i32) {
        self.set_int(173, v);
    }

    pub fn player_quest_log_4_1(&self) -> Option<i32> {
        self.get_int(173)
    }

    pub fn set_player_quest_log_4_2(&mut self, v: i32) {
        self.set_int(174, v);
    }

    pub fn player_quest_log_4_2(&self) -> Option<i32> {
        self.get_int(174)
    }

    pub fn set_player_quest_log_4_3(&mut self, a: u16, b: u16) {
        self.set_shorts(175, a, b);
    }

    pub fn player_quest_log_4_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(175)
    }

    pub fn set_player_quest_log_4_5(&mut self, v: i32) {
        self.set_int(177, v);
    }

    pub fn player_quest_log_4_5(&self) -> Option<i32> {
        self.get_int(177)
    }

    pub fn set_player_quest_log_5_1(&mut self, v: i32) {
        self.set_int(178, v);
    }

    pub fn player_quest_log_5_1(&self) -> Option<i32> {
        self.get_int(178)
    }

    pub fn set_player_quest_log_5_2(&mut self, v: i32) {
        self.set_int(179, v);
    }

    pub fn player_quest_log_5_2(&self) -> Option<i32> {
        self.get_int(179)
    }

    pub fn set_player_quest_log_5_3(&mut self, a: u16, b: u16) {
        self.set_shorts(180, a, b);
    }

    pub fn player_quest_log_5_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(180)
    }

    pub fn set_player_quest_log_5_5(&mut self, v: i32) {
        self.set_int(182, v);
    }

    pub fn player_quest_log_5_5(&self) -> Option<i32> {
        self.get_int(182)
    }

    pub fn set_player_quest_log_6_1(&mut self, v: i32) {
        self.set_int(183, v);
    }

    pub fn player_quest_log_6_1(&self) -> Option<i32> {
        self.get_int(183)
    }

    pub fn set_player_quest_log_6_2(&mut self, v: i32) {
        self.set_int(184, v);
    }

    pub fn player_quest_log_6_2(&self) -> Option<i32> {
        self.get_int(184)
    }

    pub fn set_player_quest_log_6_3(&mut self, a: u16, b: u16) {
        self.set_shorts(185, a, b);
    }

    pub fn player_quest_log_6_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(185)
    }

    pub fn set_player_quest_log_6_5(&mut self, v: i32) {
        self.set_int(187, v);
    }

    pub fn player_quest_log_6_5(&self) -> Option<i32> {
        self.get_int(187)
    }

    pub fn set_player_quest_log_7_1(&mut self, v: i32) {
        self.set_int(188, v);
    }

    pub fn player_quest_log_7_1(&self) -> Option<i32> {
        self.get_int(188)
    }

    pub fn set_player_quest_log_7_2(&mut self, v: i32) {
        self.set_int(189, v);
    }

    pub fn player_quest_log_7_2(&self) -> Option<i32> {
        self.get_int(189)
    }

    pub fn set_player_quest_log_7_3(&mut self, a: u16, b: u16) {
        self.set_shorts(190, a, b);
    }

    pub fn player_quest_log_7_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(190)
    }

    pub fn set_player_quest_log_7_5(&mut self, v: i32) {
        self.set_int(192, v);
    }

    pub fn player_quest_log_7_5(&self) -> Option<i32> {
        self.get_int(192)
    }

    pub fn set_player_quest_log_8_1(&mut self, v: i32) {
        self.set_int(193, v);
    }

    pub fn player_quest_log_8_1(&self) -> Option<i32> {
        self.get_int(193)
    }

    pub fn set_player_quest_log_8_2(&mut self, v: i32) {
        self.set_int(194, v);
    }

    pub fn player_quest_log_8_2(&self) -> Option<i32> {
        self.get_int(194)
    }

    pub fn set_player_quest_log_8_3(&mut self, a: u16, b: u16) {
        self.set_shorts(195, a, b);
    }

    pub fn player_quest_log_8_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(195)
    }

    pub fn set_player_quest_log_8_5(&mut self, v: i32) {
        self.set_int(197, v);
    }

    pub fn player_quest_log_8_5(&self) -> Option<i32> {
        self.get_int(197)
    }

    pub fn set_player_quest_log_9_1(&mut self, v: i32) {
        self.set_int(198, v);
    }

    pub fn player_quest_log_9_1(&self) -> Option<i32> {
        self.get_int(198)
    }

    pub fn set_player_quest_log_9_2(&mut self, v: i32) {
        self.set_int(199, v);
    }

    pub fn player_quest_log_9_2(&self) -> Option<i32> {
        self.get_int(199)
    }

    pub fn set_player_quest_log_9_3(&mut self, a: u16, b: u16) {
        self.set_shorts(200, a, b);
    }

    pub fn player_quest_log_9_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(200)
    }

    pub fn set_player_quest_log_9_5(&mut self, v: i32) {
        self.set_int(202, v);
    }

    pub fn player_quest_log_9_5(&self) -> Option<i32> {
        self.get_int(202)
    }

    pub fn set_player_quest_log_10_1(&mut self, v: i32) {
        self.set_int(203, v);
    }

    pub fn player_quest_log_10_1(&self) -> Option<i32> {
        self.get_int(203)
    }

    pub fn set_player_quest_log_10_2(&mut self, v: i32) {
        self.set_int(204, v);
    }

    pub fn player_quest_log_10_2(&self) -> Option<i32> {
        self.get_int(204)
    }

    pub fn set_player_quest_log_10_3(&mut self, a: u16, b: u16) {
        self.set_shorts(205, a, b);
    }

    pub fn player_quest_log_10_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(205)
    }

    pub fn set_player_quest_log_10_5(&mut self, v: i32) {
        self.set_int(207, v);
    }

    pub fn player_quest_log_10_5(&self) -> Option<i32> {
        self.get_int(207)
    }

    pub fn set_player_quest_log_11_1(&mut self, v: i32) {
        self.set_int(208, v);
    }

    pub fn player_quest_log_11_1(&self) -> Option<i32> {
        self.get_int(208)
    }

    pub fn set_player_quest_log_11_2(&mut self, v: i32) {
        self.set_int(209, v);
    }

    pub fn player_quest_log_11_2(&self) -> Option<i32> {
        self.get_int(209)
    }

    pub fn set_player_quest_log_11_3(&mut self, a: u16, b: u16) {
        self.set_shorts(210, a, b);
    }

    pub fn player_quest_log_11_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(210)
    }

    pub fn set_player_quest_log_11_5(&mut self, v: i32) {
        self.set_int(212, v);
    }

    pub fn player_quest_log_11_5(&self) -> Option<i32> {
        self.get_int(212)
    }

    pub fn set_player_quest_log_12_1(&mut self, v: i32) {
        self.set_int(213, v);
    }

    pub fn player_quest_log_12_1(&self) -> Option<i32> {
        self.get_int(213)
    }

    pub fn set_player_quest_log_12_2(&mut self, v: i32) {
        self.set_int(214, v);
    }

    pub fn player_quest_log_12_2(&self) -> Option<i32> {
        self.get_int(214)
    }

    pub fn set_player_quest_log_12_3(&mut self, a: u16, b: u16) {
        self.set_shorts(215, a, b);
    }

    pub fn player_quest_log_12_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(215)
    }

    pub fn set_player_quest_log_12_5(&mut self, v: i32) {
        self.set_int(217, v);
    }

    pub fn player_quest_log_12_5(&self) -> Option<i32> {
        self.get_int(217)
    }

    pub fn set_player_quest_log_13_1(&mut self, v: i32) {
        self.set_int(218, v);
    }

    pub fn player_quest_log_13_1(&self) -> Option<i32> {
        self.get_int(218)
    }

    pub fn set_player_quest_log_13_2(&mut self, v: i32) {
        self.set_int(219, v);
    }

    pub fn player_quest_log_13_2(&self) -> Option<i32> {
        self.get_int(219)
    }

    pub fn set_player_quest_log_13_3(&mut self, a: u16, b: u16) {
        self.set_shorts(220, a, b);
    }

    pub fn player_quest_log_13_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(220)
    }

    pub fn set_player_quest_log_13_5(&mut self, v: i32) {
        self.set_int(222, v);
    }

    pub fn player_quest_log_13_5(&self) -> Option<i32> {
        self.get_int(222)
    }

    pub fn set_player_quest_log_14_1(&mut self, v: i32) {
        self.set_int(223, v);
    }

    pub fn player_quest_log_14_1(&self) -> Option<i32> {
        self.get_int(223)
    }

    pub fn set_player_quest_log_14_2(&mut self, v: i32) {
        self.set_int(224, v);
    }

    pub fn player_quest_log_14_2(&self) -> Option<i32> {
        self.get_int(224)
    }

    pub fn set_player_quest_log_14_3(&mut self, a: u16, b: u16) {
        self.set_shorts(225, a, b);
    }

    pub fn player_quest_log_14_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(225)
    }

    pub fn set_player_quest_log_14_5(&mut self, v: i32) {
        self.set_int(227, v);
    }

    pub fn player_quest_log_14_5(&self) -> Option<i32> {
        self.get_int(227)
    }

    pub fn set_player_quest_log_15_1(&mut self, v: i32) {
        self.set_int(228, v);
    }

    pub fn player_quest_log_15_1(&self) -> Option<i32> {
        self.get_int(228)
    }

    pub fn set_player_quest_log_15_2(&mut self, v: i32) {
        self.set_int(229, v);
    }

    pub fn player_quest_log_15_2(&self) -> Option<i32> {
        self.get_int(229)
    }

    pub fn set_player_quest_log_15_3(&mut self, a: u16, b: u16) {
        self.set_shorts(230, a, b);
    }

    pub fn player_quest_log_15_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(230)
    }

    pub fn set_player_quest_log_15_5(&mut self, v: i32) {
        self.set_int(232, v);
    }

    pub fn player_quest_log_15_5(&self) -> Option<i32> {
        self.get_int(232)
    }

    pub fn set_player_quest_log_16_1(&mut self, v: i32) {
        self.set_int(233, v);
    }

    pub fn player_quest_log_16_1(&self) -> Option<i32> {
        self.get_int(233)
    }

    pub fn set_player_quest_log_16_2(&mut self, v: i32) {
        self.set_int(234, v);
    }

    pub fn player_quest_log_16_2(&self) -> Option<i32> {
        self.get_int(234)
    }

    pub fn set_player_quest_log_16_3(&mut self, a: u16, b: u16) {
        self.set_shorts(235, a, b);
    }

    pub fn player_quest_log_16_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(235)
    }

    pub fn set_player_quest_log_16_5(&mut self, v: i32) {
        self.set_int(237, v);
    }

    pub fn player_quest_log_16_5(&self) -> Option<i32> {
        self.get_int(237)
    }

    pub fn set_player_quest_log_17_1(&mut self, v: i32) {
        self.set_int(238, v);
    }

    pub fn player_quest_log_17_1(&self) -> Option<i32> {
        self.get_int(238)
    }

    pub fn set_player_quest_log_17_2(&mut self, v: i32) {
        self.set_int(239, v);
    }

    pub fn player_quest_log_17_2(&self) -> Option<i32> {
        self.get_int(239)
    }

    pub fn set_player_quest_log_17_3(&mut self, a: u16, b: u16) {
        self.set_shorts(240, a, b);
    }

    pub fn player_quest_log_17_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(240)
    }

    pub fn set_player_quest_log_17_5(&mut self, v: i32) {
        self.set_int(242, v);
    }

    pub fn player_quest_log_17_5(&self) -> Option<i32> {
        self.get_int(242)
    }

    pub fn set_player_quest_log_18_1(&mut self, v: i32) {
        self.set_int(243, v);
    }

    pub fn player_quest_log_18_1(&self) -> Option<i32> {
        self.get_int(243)
    }

    pub fn set_player_quest_log_18_2(&mut self, v: i32) {
        self.set_int(244, v);
    }

    pub fn player_quest_log_18_2(&self) -> Option<i32> {
        self.get_int(244)
    }

    pub fn set_player_quest_log_18_3(&mut self, a: u16, b: u16) {
        self.set_shorts(245, a, b);
    }

    pub fn player_quest_log_18_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(245)
    }

    pub fn set_player_quest_log_18_5(&mut self, v: i32) {
        self.set_int(247, v);
    }

    pub fn player_quest_log_18_5(&self) -> Option<i32> {
        self.get_int(247)
    }

    pub fn set_player_quest_log_19_1(&mut self, v: i32) {
        self.set_int(248, v);
    }

    pub fn player_quest_log_19_1(&self) -> Option<i32> {
        self.get_int(248)
    }

    pub fn set_player_quest_log_19_2(&mut self, v: i32) {
        self.set_int(249, v);
    }

    pub fn player_quest_log_19_2(&self) -> Option<i32> {
        self.get_int(249)
    }

    pub fn set_player_quest_log_19_3(&mut self, a: u16, b: u16) {
        self.set_shorts(250, a, b);
    }

    pub fn player_quest_log_19_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(250)
    }

    pub fn set_player_quest_log_19_5(&mut self, v: i32) {
        self.set_int(252, v);
    }

    pub fn player_quest_log_19_5(&self) -> Option<i32> {
        self.get_int(252)
    }

    pub fn set_player_quest_log_20_1(&mut self, v: i32) {
        self.set_int(253, v);
    }

    pub fn player_quest_log_20_1(&self) -> Option<i32> {
        self.get_int(253)
    }

    pub fn set_player_quest_log_20_2(&mut self, v: i32) {
        self.set_int(254, v);
    }

    pub fn player_quest_log_20_2(&self) -> Option<i32> {
        self.get_int(254)
    }

    pub fn set_player_quest_log_20_3(&mut self, a: u16, b: u16) {
        self.set_shorts(255, a, b);
    }

    pub fn player_quest_log_20_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(255)
    }

    pub fn set_player_quest_log_20_5(&mut self, v: i32) {
        self.set_int(257, v);
    }

    pub fn player_quest_log_20_5(&self) -> Option<i32> {
        self.get_int(257)
    }

    pub fn set_player_quest_log_21_1(&mut self, v: i32) {
        self.set_int(258, v);
    }

    pub fn player_quest_log_21_1(&self) -> Option<i32> {
        self.get_int(258)
    }

    pub fn set_player_quest_log_21_2(&mut self, v: i32) {
        self.set_int(259, v);
    }

    pub fn player_quest_log_21_2(&self) -> Option<i32> {
        self.get_int(259)
    }

    pub fn set_player_quest_log_21_3(&mut self, a: u16, b: u16) {
        self.set_shorts(260, a, b);
    }

    pub fn player_quest_log_21_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(260)
    }

    pub fn set_player_quest_log_21_5(&mut self, v: i32) {
        self.set_int(262, v);
    }

    pub fn player_quest_log_21_5(&self) -> Option<i32> {
        self.get_int(262)
    }

    pub fn set_player_quest_log_22_1(&mut self, v: i32) {
        self.set_int(263, v);
    }

    pub fn player_quest_log_22_1(&self) -> Option<i32> {
        self.get_int(263)
    }

    pub fn set_player_quest_log_22_2(&mut self, v: i32) {
        self.set_int(264, v);
    }

    pub fn player_quest_log_22_2(&self) -> Option<i32> {
        self.get_int(264)
    }

    pub fn set_player_quest_log_22_3(&mut self, a: u16, b: u16) {
        self.set_shorts(265, a, b);
    }

    pub fn player_quest_log_22_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(265)
    }

    pub fn set_player_quest_log_22_5(&mut self, v: i32) {
        self.set_int(267, v);
    }

    pub fn player_quest_log_22_5(&self) -> Option<i32> {
        self.get_int(267)
    }

    pub fn set_player_quest_log_23_1(&mut self, v: i32) {
        self.set_int(268, v);
    }

    pub fn player_quest_log_23_1(&self) -> Option<i32> {
        self.get_int(268)
    }

    pub fn set_player_quest_log_23_2(&mut self, v: i32) {
        self.set_int(269, v);
    }

    pub fn player_quest_log_23_2(&self) -> Option<i32> {
        self.get_int(269)
    }

    pub fn set_player_quest_log_23_3(&mut self, a: u16, b: u16) {
        self.set_shorts(270, a, b);
    }

    pub fn player_quest_log_23_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(270)
    }

    pub fn set_player_quest_log_23_5(&mut self, v: i32) {
        self.set_int(272, v);
    }

    pub fn player_quest_log_23_5(&self) -> Option<i32> {
        self.get_int(272)
    }

    pub fn set_player_quest_log_24_1(&mut self, v: i32) {
        self.set_int(273, v);
    }

    pub fn player_quest_log_24_1(&self) -> Option<i32> {
        self.get_int(273)
    }

    pub fn set_player_quest_log_24_2(&mut self, v: i32) {
        self.set_int(274, v);
    }

    pub fn player_quest_log_24_2(&self) -> Option<i32> {
        self.get_int(274)
    }

    pub fn set_player_quest_log_24_3(&mut self, a: u16, b: u16) {
        self.set_shorts(275, a, b);
    }

    pub fn player_quest_log_24_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(275)
    }

    pub fn set_player_quest_log_24_5(&mut self, v: i32) {
        self.set_int(277, v);
    }

    pub fn player_quest_log_24_5(&self) -> Option<i32> {
        self.get_int(277)
    }

    pub fn set_player_quest_log_25_1(&mut self, v: i32) {
        self.set_int(278, v);
    }

    pub fn player_quest_log_25_1(&self) -> Option<i32> {
        self.get_int(278)
    }

    pub fn set_player_quest_log_25_2(&mut self, v: i32) {
        self.set_int(279, v);
    }

    pub fn player_quest_log_25_2(&self) -> Option<i32> {
        self.get_int(279)
    }

    pub fn set_player_quest_log_25_3(&mut self, a: u16, b: u16) {
        self.set_shorts(280, a, b);
    }

    pub fn player_quest_log_25_3(&self) -> Option<(u16, u16)> {
        self.get_shorts(280)
    }

    pub fn set_player_quest_log_25_5(&mut self, v: i32) {
        self.set_int(282, v);
    }

    pub fn player_quest_log_25_5(&self) -> Option<i32> {
        self.get_int(282)
    }

    pub fn set_player_visible_item_1_entryid(&mut self, v: i32) {
        self.set_int(283, v);
    }

    pub fn player_visible_item_1_entryid(&self) -> Option<i32> {
        self.get_int(283)
    }

    pub fn set_player_visible_item_1_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(284, a, b);
    }

    pub fn player_visible_item_1_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(284)
    }

    pub fn set_player_visible_item_2_entryid(&mut self, v: i32) {
        self.set_int(285, v);
    }

    pub fn player_visible_item_2_entryid(&self) -> Option<i32> {
        self.get_int(285)
    }

    pub fn set_player_visible_item_2_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(286, a, b);
    }

    pub fn player_visible_item_2_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(286)
    }

    pub fn set_player_visible_item_3_entryid(&mut self, v: i32) {
        self.set_int(287, v);
    }

    pub fn player_visible_item_3_entryid(&self) -> Option<i32> {
        self.get_int(287)
    }

    pub fn set_player_visible_item_3_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(288, a, b);
    }

    pub fn player_visible_item_3_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(288)
    }

    pub fn set_player_visible_item_4_entryid(&mut self, v: i32) {
        self.set_int(289, v);
    }

    pub fn player_visible_item_4_entryid(&self) -> Option<i32> {
        self.get_int(289)
    }

    pub fn set_player_visible_item_4_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(290, a, b);
    }

    pub fn player_visible_item_4_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(290)
    }

    pub fn set_player_visible_item_5_entryid(&mut self, v: i32) {
        self.set_int(291, v);
    }

    pub fn player_visible_item_5_entryid(&self) -> Option<i32> {
        self.get_int(291)
    }

    pub fn set_player_visible_item_5_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(292, a, b);
    }

    pub fn player_visible_item_5_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(292)
    }

    pub fn set_player_visible_item_6_entryid(&mut self, v: i32) {
        self.set_int(293, v);
    }

    pub fn player_visible_item_6_entryid(&self) -> Option<i32> {
        self.get_int(293)
    }

    pub fn set_player_visible_item_6_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(294, a, b);
    }

    pub fn player_visible_item_6_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(294)
    }

    pub fn set_player_visible_item_7_entryid(&mut self, v: i32) {
        self.set_int(295, v);
    }

    pub fn player_visible_item_7_entryid(&self) -> Option<i32> {
        self.get_int(295)
    }

    pub fn set_player_visible_item_7_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(296, a, b);
    }

    pub fn player_visible_item_7_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(296)
    }

    pub fn set_player_visible_item_8_entryid(&mut self, v: i32) {
        self.set_int(297, v);
    }

    pub fn player_visible_item_8_entryid(&self) -> Option<i32> {
        self.get_int(297)
    }

    pub fn set_player_visible_item_8_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(298, a, b);
    }

    pub fn player_visible_item_8_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(298)
    }

    pub fn set_player_visible_item_9_entryid(&mut self, v: i32) {
        self.set_int(299, v);
    }

    pub fn player_visible_item_9_entryid(&self) -> Option<i32> {
        self.get_int(299)
    }

    pub fn set_player_visible_item_9_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(300, a, b);
    }

    pub fn player_visible_item_9_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(300)
    }

    pub fn set_player_visible_item_10_entryid(&mut self, v: i32) {
        self.set_int(301, v);
    }

    pub fn player_visible_item_10_entryid(&self) -> Option<i32> {
        self.get_int(301)
    }

    pub fn set_player_visible_item_10_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(302, a, b);
    }

    pub fn player_visible_item_10_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(302)
    }

    pub fn set_player_visible_item_11_entryid(&mut self, v: i32) {
        self.set_int(303, v);
    }

    pub fn player_visible_item_11_entryid(&self) -> Option<i32> {
        self.get_int(303)
    }

    pub fn set_player_visible_item_11_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(304, a, b);
    }

    pub fn player_visible_item_11_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(304)
    }

    pub fn set_player_visible_item_12_entryid(&mut self, v: i32) {
        self.set_int(305, v);
    }

    pub fn player_visible_item_12_entryid(&self) -> Option<i32> {
        self.get_int(305)
    }

    pub fn set_player_visible_item_12_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(306, a, b);
    }

    pub fn player_visible_item_12_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(306)
    }

    pub fn set_player_visible_item_13_entryid(&mut self, v: i32) {
        self.set_int(307, v);
    }

    pub fn player_visible_item_13_entryid(&self) -> Option<i32> {
        self.get_int(307)
    }

    pub fn set_player_visible_item_13_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(308, a, b);
    }

    pub fn player_visible_item_13_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(308)
    }

    pub fn set_player_visible_item_14_entryid(&mut self, v: i32) {
        self.set_int(309, v);
    }

    pub fn player_visible_item_14_entryid(&self) -> Option<i32> {
        self.get_int(309)
    }

    pub fn set_player_visible_item_14_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(310, a, b);
    }

    pub fn player_visible_item_14_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(310)
    }

    pub fn set_player_visible_item_15_entryid(&mut self, v: i32) {
        self.set_int(311, v);
    }

    pub fn player_visible_item_15_entryid(&self) -> Option<i32> {
        self.get_int(311)
    }

    pub fn set_player_visible_item_15_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(312, a, b);
    }

    pub fn player_visible_item_15_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(312)
    }

    pub fn set_player_visible_item_16_entryid(&mut self, v: i32) {
        self.set_int(313, v);
    }

    pub fn player_visible_item_16_entryid(&self) -> Option<i32> {
        self.get_int(313)
    }

    pub fn set_player_visible_item_16_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(314, a, b);
    }

    pub fn player_visible_item_16_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(314)
    }

    pub fn set_player_visible_item_17_entryid(&mut self, v: i32) {
        self.set_int(315, v);
    }

    pub fn player_visible_item_17_entryid(&self) -> Option<i32> {
        self.get_int(315)
    }

    pub fn set_player_visible_item_17_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(316, a, b);
    }

    pub fn player_visible_item_17_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(316)
    }

    pub fn set_player_visible_item_18_entryid(&mut self, v: i32) {
        self.set_int(317, v);
    }

    pub fn player_visible_item_18_entryid(&self) -> Option<i32> {
        self.get_int(317)
    }

    pub fn set_player_visible_item_18_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(318, a, b);
    }

    pub fn player_visible_item_18_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(318)
    }

    pub fn set_player_visible_item_19_entryid(&mut self, v: i32) {
        self.set_int(319, v);
    }

    pub fn player_visible_item_19_entryid(&self) -> Option<i32> {
        self.get_int(319)
    }

    pub fn set_player_visible_item_19_enchantment(&mut self, a: u16, b: u16) {
        self.set_shorts(320, a, b);
    }

    pub fn player_visible_item_19_enchantment(&self) -> Option<(u16, u16)> {
        self.get_shorts(320)
    }

    pub fn set_player_chosen_title(&mut self, v: i32) {
        self.set_int(321, v);
    }

    pub fn player_chosen_title(&self) -> Option<i32> {
        self.get_int(321)
    }

    pub fn set_player_fake_inebriation(&mut self, v: i32) {
        self.set_int(322, v);
    }

    pub fn player_fake_inebriation(&self) -> Option<i32> {
        self.get_int(322)
    }

    pub fn set_player_inv_slot_head1(&mut self, v: Guid) {
        self.set_guid(324, v);
    }

    pub fn player_inv_slot_head1(&self) -> Option<Guid> {
        self.get_guid(324)
    }

    pub fn set_player_inv_slot_head2(&mut self, v: Guid) {
        self.set_guid(326, v);
    }

    pub fn player_inv_slot_head2(&self) -> Option<Guid> {
        self.get_guid(326)
    }

    pub fn set_player_inv_slot_head3(&mut self, v: Guid) {
        self.set_guid(328, v);
    }

    pub fn player_inv_slot_head3(&self) -> Option<Guid> {
        self.get_guid(328)
    }

    pub fn set_player_inv_slot_head4(&mut self, v: Guid) {
        self.set_guid(330, v);
    }

    pub fn player_inv_slot_head4(&self) -> Option<Guid> {
        self.get_guid(330)
    }

    pub fn set_player_inv_slot_head5(&mut self, v: Guid) {
        self.set_guid(332, v);
    }

    pub fn player_inv_slot_head5(&self) -> Option<Guid> {
        self.get_guid(332)
    }

    pub fn set_player_inv_slot_head6(&mut self, v: Guid) {
        self.set_guid(334, v);
    }

    pub fn player_inv_slot_head6(&self) -> Option<Guid> {
        self.get_guid(334)
    }

    pub fn set_player_inv_slot_head7(&mut self, v: Guid) {
        self.set_guid(336, v);
    }

    pub fn player_inv_slot_head7(&self) -> Option<Guid> {
        self.get_guid(336)
    }

    pub fn set_player_inv_slot_head8(&mut self, v: Guid) {
        self.set_guid(338, v);
    }

    pub fn player_inv_slot_head8(&self) -> Option<Guid> {
        self.get_guid(338)
    }

    pub fn set_player_inv_slot_head9(&mut self, v: Guid) {
        self.set_guid(340, v);
    }

    pub fn player_inv_slot_head9(&self) -> Option<Guid> {
        self.get_guid(340)
    }

    pub fn set_player_inv_slot_head10(&mut self, v: Guid) {
        self.set_guid(342, v);
    }

    pub fn player_inv_slot_head10(&self) -> Option<Guid> {
        self.get_guid(342)
    }

    pub fn set_player_inv_slot_head11(&mut self, v: Guid) {
        self.set_guid(344, v);
    }

    pub fn player_inv_slot_head11(&self) -> Option<Guid> {
        self.get_guid(344)
    }

    pub fn set_player_inv_slot_head12(&mut self, v: Guid) {
        self.set_guid(346, v);
    }

    pub fn player_inv_slot_head12(&self) -> Option<Guid> {
        self.get_guid(346)
    }

    pub fn set_player_inv_slot_head13(&mut self, v: Guid) {
        self.set_guid(348, v);
    }

    pub fn player_inv_slot_head13(&self) -> Option<Guid> {
        self.get_guid(348)
    }

    pub fn set_player_inv_slot_head14(&mut self, v: Guid) {
        self.set_guid(350, v);
    }

    pub fn player_inv_slot_head14(&self) -> Option<Guid> {
        self.get_guid(350)
    }

    pub fn set_player_inv_slot_head15(&mut self, v: Guid) {
        self.set_guid(352, v);
    }

    pub fn player_inv_slot_head15(&self) -> Option<Guid> {
        self.get_guid(352)
    }

    pub fn set_player_inv_slot_head16(&mut self, v: Guid) {
        self.set_guid(354, v);
    }

    pub fn player_inv_slot_head16(&self) -> Option<Guid> {
        self.get_guid(354)
    }

    pub fn set_player_inv_slot_head17(&mut self, v: Guid) {
        self.set_guid(356, v);
    }

    pub fn player_inv_slot_head17(&self) -> Option<Guid> {
        self.get_guid(356)
    }

    pub fn set_player_inv_slot_head18(&mut self, v: Guid) {
        self.set_guid(358, v);
    }

    pub fn player_inv_slot_head18(&self) -> Option<Guid> {
        self.get_guid(358)
    }

    pub fn set_player_inv_slot_head19(&mut self, v: Guid) {
        self.set_guid(360, v);
    }

    pub fn player_inv_slot_head19(&self) -> Option<Guid> {
        self.get_guid(360)
    }

    pub fn set_player_inv_slot_head20(&mut self, v: Guid) {
        self.set_guid(362, v);
    }

    pub fn player_inv_slot_head20(&self) -> Option<Guid> {
        self.get_guid(362)
    }

    pub fn set_player_inv_slot_head21(&mut self, v: Guid) {
        self.set_guid(364, v);
    }

    pub fn player_inv_slot_head21(&self) -> Option<Guid> {
        self.get_guid(364)
    }

    pub fn set_player_inv_slot_head22(&mut self, v: Guid) {
        self.set_guid(366, v);
    }

    pub fn player_inv_slot_head22(&self) -> Option<Guid> {
        self.get_guid(366)
    }

    pub fn set_player_inv_slot_head23(&mut self, v: Guid) {
        self.set_guid(368, v);
    }

    pub fn player_inv_slot_head23(&self) -> Option<Guid> {
        self.get_guid(368)
    }

    pub fn set_player_pack_slot_1(&mut self, v: Guid) {
        self.set_guid(370, v);
    }

    pub fn player_pack_slot_1(&self) -> Option<Guid> {
        self.get_guid(370)
    }

    pub fn set_player_pack_slot_2(&mut self, v: Guid) {
        self.set_guid(372, v);
    }

    pub fn player_pack_slot_2(&self) -> Option<Guid> {
        self.get_guid(372)
    }

    pub fn set_player_pack_slot_3(&mut self, v: Guid) {
        self.set_guid(374, v);
    }

    pub fn player_pack_slot_3(&self) -> Option<Guid> {
        self.get_guid(374)
    }

    pub fn set_player_pack_slot_4(&mut self, v: Guid) {
        self.set_guid(376, v);
    }

    pub fn player_pack_slot_4(&self) -> Option<Guid> {
        self.get_guid(376)
    }

    pub fn set_player_pack_slot_5(&mut self, v: Guid) {
        self.set_guid(378, v);
    }

    pub fn player_pack_slot_5(&self) -> Option<Guid> {
        self.get_guid(378)
    }

    pub fn set_player_pack_slot_6(&mut self, v: Guid) {
        self.set_guid(380, v);
    }

    pub fn player_pack_slot_6(&self) -> Option<Guid> {
        self.get_guid(380)
    }

    pub fn set_player_pack_slot_7(&mut self, v: Guid) {
        self.set_guid(382, v);
    }

    pub fn player_pack_slot_7(&self) -> Option<Guid> {
        self.get_guid(382)
    }

    pub fn set_player_pack_slot_8(&mut self, v: Guid) {
        self.set_guid(384, v);
    }

    pub fn player_pack_slot_8(&self) -> Option<Guid> {
        self.get_guid(384)
    }

    pub fn set_player_pack_slot_9(&mut self, v: Guid) {
        self.set_guid(386, v);
    }

    pub fn player_pack_slot_9(&self) -> Option<Guid> {
        self.get_guid(386)
    }

    pub fn set_player_pack_slot_10(&mut self, v: Guid) {
        self.set_guid(388, v);
    }

    pub fn player_pack_slot_10(&self) -> Option<Guid> {
        self.get_guid(388)
    }

    pub fn set_player_pack_slot_11(&mut self, v: Guid) {
        self.set_guid(390, v);
    }

    pub fn player_pack_slot_11(&self) -> Option<Guid> {
        self.get_guid(390)
    }

    pub fn set_player_pack_slot_12(&mut self, v: Guid) {
        self.set_guid(392, v);
    }

    pub fn player_pack_slot_12(&self) -> Option<Guid> {
        self.get_guid(392)
    }

    pub fn set_player_pack_slot_13(&mut self, v: Guid) {
        self.set_guid(394, v);
    }

    pub fn player_pack_slot_13(&self) -> Option<Guid> {
        self.get_guid(394)
    }

    pub fn set_player_pack_slot_14(&mut self, v: Guid) {
        self.set_guid(396, v);
    }

    pub fn player_pack_slot_14(&self) -> Option<Guid> {
        self.get_guid(396)
    }

    pub fn set_player_pack_slot_15(&mut self, v: Guid) {
        self.set_guid(398, v);
    }

    pub fn player_pack_slot_15(&self) -> Option<Guid> {
        self.get_guid(398)
    }

    pub fn set_player_pack_slot_16(&mut self, v: Guid) {
        self.set_guid(400, v);
    }

    pub fn player_pack_slot_16(&self) -> Option<Guid> {
        self.get_guid(400)
    }

    pub fn set_player_bank_slot_1(&mut self, v: Guid) {
        self.set_guid(402, v);
    }

    pub fn player_bank_slot_1(&self) -> Option<Guid> {
        self.get_guid(402)
    }

    pub fn set_player_bank_slot_2(&mut self, v: Guid) {
        self.set_guid(404, v);
    }

    pub fn player_bank_slot_2(&self) -> Option<Guid> {
        self.get_guid(404)
    }

    pub fn set_player_bank_slot_3(&mut self, v: Guid) {
        self.set_guid(406, v);
    }

    pub fn player_bank_slot_3(&self) -> Option<Guid> {
        self.get_guid(406)
    }

    pub fn set_player_bank_slot_4(&mut self, v: Guid) {
        self.set_guid(408, v);
    }

    pub fn player_bank_slot_4(&self) -> Option<Guid> {
        self.get_guid(408)
    }

    pub fn set_player_bank_slot_5(&mut self, v: Guid) {
        self.set_guid(410, v);
    }

    pub fn player_bank_slot_5(&self) -> Option<Guid> {
        self.get_guid(410)
    }

    pub fn set_player_bank_slot_6(&mut self, v: Guid) {
        self.set_guid(412, v);
    }

    pub fn player_bank_slot_6(&self) -> Option<Guid> {
        self.get_guid(412)
    }

    pub fn set_player_bank_slot_7(&mut self, v: Guid) {
        self.set_guid(414, v);
    }

    pub fn player_bank_slot_7(&self) -> Option<Guid> {
        self.get_guid(414)
    }

    pub fn set_player_bank_slot_8(&mut self, v: Guid) {
        self.set_guid(416, v);
    }

    pub fn player_bank_slot_8(&self) -> Option<Guid> {
        self.get_guid(416)
    }

    pub fn set_player_bank_slot_9(&mut self, v: Guid) {
        self.set_guid(418, v);
    }

    pub fn player_bank_slot_9(&self) -> Option<Guid> {
        self.get_guid(418)
    }

    pub fn set_player_bank_slot_10(&mut self, v: Guid) {
        self.set_guid(420, v);
    }

    pub fn player_bank_slot_10(&self) -> Option<Guid> {
        self.get_guid(420)
    }

    pub fn set_player_bank_slot_11(&mut self, v: Guid) {
        self.set_guid(422, v);
    }

    pub fn player_bank_slot_11(&self) -> Option<Guid> {
        self.get_guid(422)
    }

    pub fn set_player_bank_slot_12(&mut self, v: Guid) {
        self.set_guid(424, v);
    }

    pub fn player_bank_slot_12(&self) -> Option<Guid> {
        self.get_guid(424)
    }

    pub fn set_player_bank_slot_13(&mut self, v: Guid) {
        self.set_guid(426, v);
    }

    pub fn player_bank_slot_13(&self) -> Option<Guid> {
        self.get_guid(426)
    }

    pub fn set_player_bank_slot_14(&mut self, v: Guid) {
        self.set_guid(428, v);
    }

    pub fn player_bank_slot_14(&self) -> Option<Guid> {
        self.get_guid(428)
    }

    pub fn set_player_bank_slot_15(&mut self, v: Guid) {
        self.set_guid(430, v);
    }

    pub fn player_bank_slot_15(&self) -> Option<Guid> {
        self.get_guid(430)
    }

    pub fn set_player_bank_slot_16(&mut self, v: Guid) {
        self.set_guid(432, v);
    }

    pub fn player_bank_slot_16(&self) -> Option<Guid> {
        self.get_guid(432)
    }

    pub fn set_player_bank_slot_17(&mut self, v: Guid) {
        self.set_guid(434, v);
    }

    pub fn player_bank_slot_17(&self) -> Option<Guid> {
        self.get_guid(434)
    }

    pub fn set_player_bank_slot_18(&mut self, v: Guid) {
        self.set_guid(436, v);
    }

    pub fn player_bank_slot_18(&self) -> Option<Guid> {
        self.get_guid(436)
    }

    pub fn set_player_bank_slot_19(&mut self, v: Guid) {
        self.set_guid(438, v);
    }

    pub fn player_bank_slot_19(&self) -> Option<Guid> {
        self.get_guid(438)
    }

    pub fn set_player_bank_slot_20(&mut self, v: Guid) {
        self.set_guid(440, v);
    }

    pub fn player_bank_slot_20(&self) -> Option<Guid> {
        self.get_guid(440)
    }

    pub fn set_player_bank_slot_21(&mut self, v: Guid) {
        self.set_guid(442, v);
    }

    pub fn player_bank_slot_21(&self) -> Option<Guid> {
        self.get_guid(442)
    }

    pub fn set_player_bank_slot_22(&mut self, v: Guid) {
        self.set_guid(444, v);
    }

    pub fn player_bank_slot_22(&self) -> Option<Guid> {
        self.get_guid(444)
    }

    pub fn set_player_bank_slot_23(&mut self, v: Guid) {
        self.set_guid(446, v);
    }

    pub fn player_bank_slot_23(&self) -> Option<Guid> {
        self.get_guid(446)
    }

    pub fn set_player_bank_slot_24(&mut self, v: Guid) {
        self.set_guid(448, v);
    }

    pub fn player_bank_slot_24(&self) -> Option<Guid> {
        self.get_guid(448)
    }

    pub fn set_player_bank_slot_25(&mut self, v: Guid) {
        self.set_guid(450, v);
    }

    pub fn player_bank_slot_25(&self) -> Option<Guid> {
        self.get_guid(450)
    }

    pub fn set_player_bank_slot_26(&mut self, v: Guid) {
        self.set_guid(452, v);
    }

    pub fn player_bank_slot_26(&self) -> Option<Guid> {
        self.get_guid(452)
    }

    pub fn set_player_bank_slot_27(&mut self, v: Guid) {
        self.set_guid(454, v);
    }

    pub fn player_bank_slot_27(&self) -> Option<Guid> {
        self.get_guid(454)
    }

    pub fn set_player_bank_slot_28(&mut self, v: Guid) {
        self.set_guid(456, v);
    }

    pub fn player_bank_slot_28(&self) -> Option<Guid> {
        self.get_guid(456)
    }

    pub fn set_player_bankbag_slot_1(&mut self, v: Guid) {
        self.set_guid(458, v);
    }

    pub fn player_bankbag_slot_1(&self) -> Option<Guid> {
        self.get_guid(458)
    }

    pub fn set_player_bankbag_slot_2(&mut self, v: Guid) {
        self.set_guid(460, v);
    }

    pub fn player_bankbag_slot_2(&self) -> Option<Guid> {
        self.get_guid(460)
    }

    pub fn set_player_bankbag_slot_3(&mut self, v: Guid) {
        self.set_guid(462, v);
    }

    pub fn player_bankbag_slot_3(&self) -> Option<Guid> {
        self.get_guid(462)
    }

    pub fn set_player_bankbag_slot_4(&mut self, v: Guid) {
        self.set_guid(464, v);
    }

    pub fn player_bankbag_slot_4(&self) -> Option<Guid> {
        self.get_guid(464)
    }

    pub fn set_player_bankbag_slot_5(&mut self, v: Guid) {
        self.set_guid(466, v);
    }

    pub fn player_bankbag_slot_5(&self) -> Option<Guid> {
        self.get_guid(466)
    }

    pub fn set_player_bankbag_slot_6(&mut self, v: Guid) {
        self.set_guid(468, v);
    }

    pub fn player_bankbag_slot_6(&self) -> Option<Guid> {
        self.get_guid(468)
    }

    pub fn set_player_bankbag_slot_7(&mut self, v: Guid) {
        self.set_guid(470, v);
    }

    pub fn player_bankbag_slot_7(&self) -> Option<Guid> {
        self.get_guid(470)
    }

    pub fn set_player_vendorbuyback_slot_1(&mut self, v: Guid) {
        self.set_guid(472, v);
    }

    pub fn player_vendorbuyback_slot_1(&self) -> Option<Guid> {
        self.get_guid(472)
    }

    pub fn set_player_vendorbuyback_slot_2(&mut self, v: Guid) {
        self.set_guid(474, v);
    }

    pub fn player_vendorbuyback_slot_2(&self) -> Option<Guid> {
        self.get_guid(474)
    }

    pub fn set_player_vendorbuyback_slot_3(&mut self, v: Guid) {
        self.set_guid(476, v);
    }

    pub fn player_vendorbuyback_slot_3(&self) -> Option<Guid> {
        self.get_guid(476)
    }

    pub fn set_player_vendorbuyback_slot_4(&mut self, v: Guid) {
        self.set_guid(478, v);
    }

    pub fn player_vendorbuyback_slot_4(&self) -> Option<Guid> {
        self.get_guid(478)
    }

    pub fn set_player_vendorbuyback_slot_5(&mut self, v: Guid) {
        self.set_guid(480, v);
    }

    pub fn player_vendorbuyback_slot_5(&self) -> Option<Guid> {
        self.get_guid(480)
    }

    pub fn set_player_vendorbuyback_slot_6(&mut self, v: Guid) {
        self.set_guid(482, v);
    }

    pub fn player_vendorbuyback_slot_6(&self) -> Option<Guid> {
        self.get_guid(482)
    }

    pub fn set_player_vendorbuyback_slot_7(&mut self, v: Guid) {
        self.set_guid(484, v);
    }

    pub fn player_vendorbuyback_slot_7(&self) -> Option<Guid> {
        self.get_guid(484)
    }

    pub fn set_player_vendorbuyback_slot_8(&mut self, v: Guid) {
        self.set_guid(486, v);
    }

    pub fn player_vendorbuyback_slot_8(&self) -> Option<Guid> {
        self.get_guid(486)
    }

    pub fn set_player_vendorbuyback_slot_9(&mut self, v: Guid) {
        self.set_guid(488, v);
    }

    pub fn player_vendorbuyback_slot_9(&self) -> Option<Guid> {
        self.get_guid(488)
    }

    pub fn set_player_vendorbuyback_slot_10(&mut self, v: Guid) {
        self.set_guid(490, v);
    }

    pub fn player_vendorbuyback_slot_10(&self) -> Option<Guid> {
        self.get_guid(490)
    }

    pub fn set_player_vendorbuyback_slot_11(&mut self, v: Guid) {
        self.set_guid(492, v);
    }

    pub fn player_vendorbuyback_slot_11(&self) -> Option<Guid> {
        self.get_guid(492)
    }

    pub fn set_player_vendorbuyback_slot_12(&mut self, v: Guid) {
        self.set_guid(494, v);
    }

    pub fn player_vendorbuyback_slot_12(&self) -> Option<Guid> {
        self.get_guid(494)
    }

    pub fn set_player_keyring_slot_1(&mut self, v: Guid) {
        self.set_guid(496, v);
    }

    pub fn player_keyring_slot_1(&self) -> Option<Guid> {
        self.get_guid(496)
    }

    pub fn set_player_keyring_slot_2(&mut self, v: Guid) {
        self.set_guid(498, v);
    }

    pub fn player_keyring_slot_2(&self) -> Option<Guid> {
        self.get_guid(498)
    }

    pub fn set_player_keyring_slot_3(&mut self, v: Guid) {
        self.set_guid(500, v);
    }

    pub fn player_keyring_slot_3(&self) -> Option<Guid> {
        self.get_guid(500)
    }

    pub fn set_player_keyring_slot_4(&mut self, v: Guid) {
        self.set_guid(502, v);
    }

    pub fn player_keyring_slot_4(&self) -> Option<Guid> {
        self.get_guid(502)
    }

    pub fn set_player_keyring_slot_5(&mut self, v: Guid) {
        self.set_guid(504, v);
    }

    pub fn player_keyring_slot_5(&self) -> Option<Guid> {
        self.get_guid(504)
    }

    pub fn set_player_keyring_slot_6(&mut self, v: Guid) {
        self.set_guid(506, v);
    }

    pub fn player_keyring_slot_6(&self) -> Option<Guid> {
        self.get_guid(506)
    }

    pub fn set_player_keyring_slot_7(&mut self, v: Guid) {
        self.set_guid(508, v);
    }

    pub fn player_keyring_slot_7(&self) -> Option<Guid> {
        self.get_guid(508)
    }

    pub fn set_player_keyring_slot_8(&mut self, v: Guid) {
        self.set_guid(510, v);
    }

    pub fn player_keyring_slot_8(&self) -> Option<Guid> {
        self.get_guid(510)
    }

    pub fn set_player_keyring_slot_9(&mut self, v: Guid) {
        self.set_guid(512, v);
    }

    pub fn player_keyring_slot_9(&self) -> Option<Guid> {
        self.get_guid(512)
    }

    pub fn set_player_keyring_slot_10(&mut self, v: Guid) {
        self.set_guid(514, v);
    }

    pub fn player_keyring_slot_10(&self) -> Option<Guid> {
        self.get_guid(514)
    }

    pub fn set_player_keyring_slot_11(&mut self, v: Guid) {
        self.set_guid(516, v);
    }

    pub fn player_keyring_slot_11(&self) -> Option<Guid> {
        self.get_guid(516)
    }

    pub fn set_player_keyring_slot_12(&mut self, v: Guid) {
        self.set_guid(518, v);
    }

    pub fn player_keyring_slot_12(&self) -> Option<Guid> {
        self.get_guid(518)
    }

    pub fn set_player_keyring_slot_13(&mut self, v: Guid) {
        self.set_guid(520, v);
    }

    pub fn player_keyring_slot_13(&self) -> Option<Guid> {
        self.get_guid(520)
    }

    pub fn set_player_keyring_slot_14(&mut self, v: Guid) {
        self.set_guid(522, v);
    }

    pub fn player_keyring_slot_14(&self) -> Option<Guid> {
        self.get_guid(522)
    }

    pub fn set_player_keyring_slot_15(&mut self, v: Guid) {
        self.set_guid(524, v);
    }

    pub fn player_keyring_slot_15(&self) -> Option<Guid> {
        self.get_guid(524)
    }

    pub fn set_player_keyring_slot_16(&mut self, v: Guid) {
        self.set_guid(526, v);
    }

    pub fn player_keyring_slot_16(&self) -> Option<Guid> {
        self.get_guid(526)
    }

    pub fn set_player_keyring_slot_17(&mut self, v: Guid) {
        self.set_guid(528, v);
    }

    pub fn player_keyring_slot_17(&self) -> Option<Guid> {
        self.get_guid(528)
    }

    pub fn set_player_keyring_slot_18(&mut self, v: Guid) {
        self.set_guid(530, v);
    }

    pub fn player_keyring_slot_18(&self) -> Option<Guid> {
        self.get_guid(530)
    }

    pub fn set_player_keyring_slot_19(&mut self, v: Guid) {
        self.set_guid(532, v);
    }

    pub fn player_keyring_slot_19(&self) -> Option<Guid> {
        self.get_guid(532)
    }

    pub fn set_player_keyring_slot_20(&mut self, v: Guid) {
        self.set_guid(534, v);
    }

    pub fn player_keyring_slot_20(&self) -> Option<Guid> {
        self.get_guid(534)
    }

    pub fn set_player_keyring_slot_21(&mut self, v: Guid) {
        self.set_guid(536, v);
    }

    pub fn player_keyring_slot_21(&self) -> Option<Guid> {
        self.get_guid(536)
    }

    pub fn set_player_keyring_slot_22(&mut self, v: Guid) {
        self.set_guid(538, v);
    }

    pub fn player_keyring_slot_22(&self) -> Option<Guid> {
        self.get_guid(538)
    }

    pub fn set_player_keyring_slot_23(&mut self, v: Guid) {
        self.set_guid(540, v);
    }

    pub fn player_keyring_slot_23(&self) -> Option<Guid> {
        self.get_guid(540)
    }

    pub fn set_player_keyring_slot_24(&mut self, v: Guid) {
        self.set_guid(542, v);
    }

    pub fn player_keyring_slot_24(&self) -> Option<Guid> {
        self.get_guid(542)
    }

    pub fn set_player_keyring_slot_25(&mut self, v: Guid) {
        self.set_guid(544, v);
    }

    pub fn player_keyring_slot_25(&self) -> Option<Guid> {
        self.get_guid(544)
    }

    pub fn set_player_keyring_slot_26(&mut self, v: Guid) {
        self.set_guid(546, v);
    }

    pub fn player_keyring_slot_26(&self) -> Option<Guid> {
        self.get_guid(546)
    }

    pub fn set_player_keyring_slot_27(&mut self, v: Guid) {
        self.set_guid(548, v);
    }

    pub fn player_keyring_slot_27(&self) -> Option<Guid> {
        self.get_guid(548)
    }

    pub fn set_player_keyring_slot_28(&mut self, v: Guid) {
        self.set_guid(550, v);
    }

    pub fn player_keyring_slot_28(&self) -> Option<Guid> {
        self.get_guid(550)
    }

    pub fn set_player_keyring_slot_29(&mut self, v: Guid) {
        self.set_guid(552, v);
    }

    pub fn player_keyring_slot_29(&self) -> Option<Guid> {
        self.get_guid(552)
    }

    pub fn set_player_keyring_slot_30(&mut self, v: Guid) {
        self.set_guid(554, v);
    }

    pub fn player_keyring_slot_30(&self) -> Option<Guid> {
        self.get_guid(554)
    }

    pub fn set_player_keyring_slot_31(&mut self, v: Guid) {
        self.set_guid(556, v);
    }

    pub fn player_keyring_slot_31(&self) -> Option<Guid> {
        self.get_guid(556)
    }

    pub fn set_player_keyring_slot_32(&mut self, v: Guid) {
        self.set_guid(558, v);
    }

    pub fn player_keyring_slot_32(&self) -> Option<Guid> {
        self.get_guid(558)
    }

    pub fn set_player_currencytoken_slot_1(&mut self, v: Guid) {
        self.set_guid(560, v);
    }

    pub fn player_currencytoken_slot_1(&self) -> Option<Guid> {
        self.get_guid(560)
    }

    pub fn set_player_currencytoken_slot_2(&mut self, v: Guid) {
        self.set_guid(562, v);
    }

    pub fn player_currencytoken_slot_2(&self) -> Option<Guid> {
        self.get_guid(562)
    }

    pub fn set_player_currencytoken_slot_3(&mut self, v: Guid) {
        self.set_guid(564, v);
    }

    pub fn player_currencytoken_slot_3(&self) -> Option<Guid> {
        self.get_guid(564)
    }

    pub fn set_player_currencytoken_slot_4(&mut self, v: Guid) {
        self.set_guid(566, v);
    }

    pub fn player_currencytoken_slot_4(&self) -> Option<Guid> {
        self.get_guid(566)
    }

    pub fn set_player_currencytoken_slot_5(&mut self, v: Guid) {
        self.set_guid(568, v);
    }

    pub fn player_currencytoken_slot_5(&self) -> Option<Guid> {
        self.get_guid(568)
    }

    pub fn set_player_currencytoken_slot_6(&mut self, v: Guid) {
        self.set_guid(570, v);
    }

    pub fn player_currencytoken_slot_6(&self) -> Option<Guid> {
        self.get_guid(570)
    }

    pub fn set_player_currencytoken_slot_7(&mut self, v: Guid) {
        self.set_guid(572, v);
    }

    pub fn player_currencytoken_slot_7(&self) -> Option<Guid> {
        self.get_guid(572)
    }

    pub fn set_player_currencytoken_slot_8(&mut self, v: Guid) {
        self.set_guid(574, v);
    }

    pub fn player_currencytoken_slot_8(&self) -> Option<Guid> {
        self.get_guid(574)
    }

    pub fn set_player_currencytoken_slot_9(&mut self, v: Guid) {
        self.set_guid(576, v);
    }

    pub fn player_currencytoken_slot_9(&self) -> Option<Guid> {
        self.get_guid(576)
    }

    pub fn set_player_currencytoken_slot_10(&mut self, v: Guid) {
        self.set_guid(578, v);
    }

    pub fn player_currencytoken_slot_10(&self) -> Option<Guid> {
        self.get_guid(578)
    }

    pub fn set_player_currencytoken_slot_11(&mut self, v: Guid) {
        self.set_guid(580, v);
    }

    pub fn player_currencytoken_slot_11(&self) -> Option<Guid> {
        self.get_guid(580)
    }

    pub fn set_player_currencytoken_slot_12(&mut self, v: Guid) {
        self.set_guid(582, v);
    }

    pub fn player_currencytoken_slot_12(&self) -> Option<Guid> {
        self.get_guid(582)
    }

    pub fn set_player_currencytoken_slot_13(&mut self, v: Guid) {
        self.set_guid(584, v);
    }

    pub fn player_currencytoken_slot_13(&self) -> Option<Guid> {
        self.get_guid(584)
    }

    pub fn set_player_currencytoken_slot_14(&mut self, v: Guid) {
        self.set_guid(586, v);
    }

    pub fn player_currencytoken_slot_14(&self) -> Option<Guid> {
        self.get_guid(586)
    }

    pub fn set_player_currencytoken_slot_15(&mut self, v: Guid) {
        self.set_guid(588, v);
    }

    pub fn player_currencytoken_slot_15(&self) -> Option<Guid> {
        self.get_guid(588)
    }

    pub fn set_player_currencytoken_slot_16(&mut self, v: Guid) {
        self.set_guid(590, v);
    }

    pub fn player_currencytoken_slot_16(&self) -> Option<Guid> {
        self.get_guid(590)
    }

    pub fn set_player_currencytoken_slot_17(&mut self, v: Guid) {
        self.set_guid(592, v);
    }

    pub fn player_currencytoken_slot_17(&self) -> Option<Guid> {
        self.get_guid(592)
    }

    pub fn set_player_currencytoken_slot_18(&mut self, v: Guid) {
        self.set_guid(594, v);
    }

    pub fn player_currencytoken_slot_18(&self) -> Option<Guid> {
        self.get_guid(594)
    }

    pub fn set_player_currencytoken_slot_19(&mut self, v: Guid) {
        self.set_guid(596, v);
    }

    pub fn player_currencytoken_slot_19(&self) -> Option<Guid> {
        self.get_guid(596)
    }

    pub fn set_player_currencytoken_slot_20(&mut self, v: Guid) {
        self.set_guid(598, v);
    }

    pub fn player_currencytoken_slot_20(&self) -> Option<Guid> {
        self.get_guid(598)
    }

    pub fn set_player_currencytoken_slot_21(&mut self, v: Guid) {
        self.set_guid(600, v);
    }

    pub fn player_currencytoken_slot_21(&self) -> Option<Guid> {
        self.get_guid(600)
    }

    pub fn set_player_currencytoken_slot_22(&mut self, v: Guid) {
        self.set_guid(602, v);
    }

    pub fn player_currencytoken_slot_22(&self) -> Option<Guid> {
        self.get_guid(602)
    }

    pub fn set_player_currencytoken_slot_23(&mut self, v: Guid) {
        self.set_guid(604, v);
    }

    pub fn player_currencytoken_slot_23(&self) -> Option<Guid> {
        self.get_guid(604)
    }

    pub fn set_player_currencytoken_slot_24(&mut self, v: Guid) {
        self.set_guid(606, v);
    }

    pub fn player_currencytoken_slot_24(&self) -> Option<Guid> {
        self.get_guid(606)
    }

    pub fn set_player_currencytoken_slot_25(&mut self, v: Guid) {
        self.set_guid(608, v);
    }

    pub fn player_currencytoken_slot_25(&self) -> Option<Guid> {
        self.get_guid(608)
    }

    pub fn set_player_currencytoken_slot_26(&mut self, v: Guid) {
        self.set_guid(610, v);
    }

    pub fn player_currencytoken_slot_26(&self) -> Option<Guid> {
        self.get_guid(610)
    }

    pub fn set_player_currencytoken_slot_27(&mut self, v: Guid) {
        self.set_guid(612, v);
    }

    pub fn player_currencytoken_slot_27(&self) -> Option<Guid> {
        self.get_guid(612)
    }

    pub fn set_player_currencytoken_slot_28(&mut self, v: Guid) {
        self.set_guid(614, v);
    }

    pub fn player_currencytoken_slot_28(&self) -> Option<Guid> {
        self.get_guid(614)
    }

    pub fn set_player_currencytoken_slot_29(&mut self, v: Guid) {
        self.set_guid(616, v);
    }

    pub fn player_currencytoken_slot_29(&self) -> Option<Guid> {
        self.get_guid(616)
    }

    pub fn set_player_currencytoken_slot_30(&mut self, v: Guid) {
        self.set_guid(618, v);
    }

    pub fn player_currencytoken_slot_30(&self) -> Option<Guid> {
        self.get_guid(618)
    }

    pub fn set_player_currencytoken_slot_31(&mut self, v: Guid) {
        self.set_guid(620, v);
    }

    pub fn player_currencytoken_slot_31(&self) -> Option<Guid> {
        self.get_guid(620)
    }

    pub fn set_player_currencytoken_slot_32(&mut self, v: Guid) {
        self.set_guid(622, v);
    }

    pub fn player_currencytoken_slot_32(&self) -> Option<Guid> {
        self.get_guid(622)
    }

    pub fn set_player_farsight(&mut self, v: Guid) {
        self.set_guid(624, v);
    }

    pub fn player_farsight(&self) -> Option<Guid> {
        self.get_guid(624)
    }

    pub fn set_player_known_titles(&mut self, v: Guid) {
        self.set_guid(626, v);
    }

    pub fn player_known_titles(&self) -> Option<Guid> {
        self.get_guid(626)
    }

    pub fn set_player_known_titles1(&mut self, v: Guid) {
        self.set_guid(628, v);
    }

    pub fn player_known_titles1(&self) -> Option<Guid> {
        self.get_guid(628)
    }

    pub fn set_player_known_titles2(&mut self, v: Guid) {
        self.set_guid(630, v);
    }

    pub fn player_known_titles2(&self) -> Option<Guid> {
        self.get_guid(630)
    }

    pub fn set_player_known_currencies(&mut self, v: Guid) {
        self.set_guid(632, v);
    }

    pub fn player_known_currencies(&self) -> Option<Guid> {
        self.get_guid(632)
    }

    pub fn set_player_xp(&mut self, v: i32) {
        self.set_int(634, v);
    }

    pub fn player_xp(&self) -> Option<i32> {
        self.get_int(634)
    }

    pub fn set_player_next_level_xp(&mut self, v: i32) {
        self.set_int(635, v);
    }

    pub fn player_next_level_xp(&self) -> Option<i32> {
        self.get_int(635)
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
        self.set_int(1020, v);
    }

    pub fn player_character_points1(&self) -> Option<i32> {
        self.get_int(1020)
    }

    pub fn set_player_character_points2(&mut self, v: i32) {
        self.set_int(1021, v);
    }

    pub fn player_character_points2(&self) -> Option<i32> {
        self.get_int(1021)
    }

    pub fn set_player_track_creatures(&mut self, v: i32) {
        self.set_int(1022, v);
    }

    pub fn player_track_creatures(&self) -> Option<i32> {
        self.get_int(1022)
    }

    pub fn set_player_track_resources(&mut self, v: i32) {
        self.set_int(1023, v);
    }

    pub fn player_track_resources(&self) -> Option<i32> {
        self.get_int(1023)
    }

    pub fn set_player_block_percentage(&mut self, v: f32) {
        self.set_float(1024, v);
    }

    pub fn player_block_percentage(&self) -> Option<f32> {
        self.get_float(1024)
    }

    pub fn set_player_dodge_percentage(&mut self, v: f32) {
        self.set_float(1025, v);
    }

    pub fn player_dodge_percentage(&self) -> Option<f32> {
        self.get_float(1025)
    }

    pub fn set_player_parry_percentage(&mut self, v: f32) {
        self.set_float(1026, v);
    }

    pub fn player_parry_percentage(&self) -> Option<f32> {
        self.get_float(1026)
    }

    pub fn set_player_expertise(&mut self, v: i32) {
        self.set_int(1027, v);
    }

    pub fn player_expertise(&self) -> Option<i32> {
        self.get_int(1027)
    }

    pub fn set_player_offhand_expertise(&mut self, v: i32) {
        self.set_int(1028, v);
    }

    pub fn player_offhand_expertise(&self) -> Option<i32> {
        self.get_int(1028)
    }

    pub fn set_player_crit_percentage(&mut self, v: f32) {
        self.set_float(1029, v);
    }

    pub fn player_crit_percentage(&self) -> Option<f32> {
        self.get_float(1029)
    }

    pub fn set_player_ranged_crit_percentage(&mut self, v: f32) {
        self.set_float(1030, v);
    }

    pub fn player_ranged_crit_percentage(&self) -> Option<f32> {
        self.get_float(1030)
    }

    pub fn set_player_offhand_crit_percentage(&mut self, v: f32) {
        self.set_float(1031, v);
    }

    pub fn player_offhand_crit_percentage(&self) -> Option<f32> {
        self.get_float(1031)
    }

    pub fn set_player_spell_crit_percentage1(&mut self, v: f32) {
        self.set_float(1032, v);
    }

    pub fn player_spell_crit_percentage1(&self) -> Option<f32> {
        self.get_float(1032)
    }

    pub fn set_player_shield_block(&mut self, v: i32) {
        self.set_int(1039, v);
    }

    pub fn player_shield_block(&self) -> Option<i32> {
        self.get_int(1039)
    }

    pub fn set_player_shield_block_crit_percentage(&mut self, v: f32) {
        self.set_float(1040, v);
    }

    pub fn player_shield_block_crit_percentage(&self) -> Option<f32> {
        self.get_float(1040)
    }

    pub fn set_player_explored_zones_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(1041, a, b, c, d);
    }

    pub fn player_explored_zones_1(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(1041)
    }

    pub fn set_player_rest_state_experience(&mut self, v: i32) {
        self.set_int(1169, v);
    }

    pub fn player_rest_state_experience(&self) -> Option<i32> {
        self.get_int(1169)
    }

    pub fn set_player_coinage(&mut self, v: i32) {
        self.set_int(1170, v);
    }

    pub fn player_coinage(&self) -> Option<i32> {
        self.get_int(1170)
    }

    pub fn set_player_mod_damage_done_pos(&mut self, v: i32) {
        self.set_int(1171, v);
    }

    pub fn player_mod_damage_done_pos(&self) -> Option<i32> {
        self.get_int(1171)
    }

    pub fn set_player_mod_damage_done_neg(&mut self, v: i32) {
        self.set_int(1178, v);
    }

    pub fn player_mod_damage_done_neg(&self) -> Option<i32> {
        self.get_int(1178)
    }

    pub fn set_player_mod_damage_done_pct(&mut self, v: i32) {
        self.set_int(1185, v);
    }

    pub fn player_mod_damage_done_pct(&self) -> Option<i32> {
        self.get_int(1185)
    }

    pub fn set_player_mod_healing_done_pos(&mut self, v: i32) {
        self.set_int(1192, v);
    }

    pub fn player_mod_healing_done_pos(&self) -> Option<i32> {
        self.get_int(1192)
    }

    pub fn set_player_mod_healing_pct(&mut self, v: f32) {
        self.set_float(1193, v);
    }

    pub fn player_mod_healing_pct(&self) -> Option<f32> {
        self.get_float(1193)
    }

    pub fn set_player_mod_healing_done_pct(&mut self, v: f32) {
        self.set_float(1194, v);
    }

    pub fn player_mod_healing_done_pct(&self) -> Option<f32> {
        self.get_float(1194)
    }

    pub fn set_player_mod_target_resistance(&mut self, v: i32) {
        self.set_int(1195, v);
    }

    pub fn player_mod_target_resistance(&self) -> Option<i32> {
        self.get_int(1195)
    }

    pub fn set_player_mod_target_physical_resistance(&mut self, v: i32) {
        self.set_int(1196, v);
    }

    pub fn player_mod_target_physical_resistance(&self) -> Option<i32> {
        self.get_int(1196)
    }

    pub fn set_player_features(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(1197, a, b, c, d);
    }

    pub fn player_features(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(1197)
    }

    pub fn set_player_ammo_id(&mut self, v: i32) {
        self.set_int(1198, v);
    }

    pub fn player_ammo_id(&self) -> Option<i32> {
        self.get_int(1198)
    }

    pub fn set_player_self_res_spell(&mut self, v: i32) {
        self.set_int(1199, v);
    }

    pub fn player_self_res_spell(&self) -> Option<i32> {
        self.get_int(1199)
    }

    pub fn set_player_pvp_medals(&mut self, v: i32) {
        self.set_int(1200, v);
    }

    pub fn player_pvp_medals(&self) -> Option<i32> {
        self.get_int(1200)
    }

    pub fn set_player_buyback_price_1(&mut self, v: i32) {
        self.set_int(1201, v);
    }

    pub fn player_buyback_price_1(&self) -> Option<i32> {
        self.get_int(1201)
    }

    pub fn set_player_buyback_timestamp_1(&mut self, v: i32) {
        self.set_int(1213, v);
    }

    pub fn player_buyback_timestamp_1(&self) -> Option<i32> {
        self.get_int(1213)
    }

    pub fn set_player_kills(&mut self, a: u16, b: u16) {
        self.set_shorts(1225, a, b);
    }

    pub fn player_kills(&self) -> Option<(u16, u16)> {
        self.get_shorts(1225)
    }

    pub fn set_player_today_contribution(&mut self, v: i32) {
        self.set_int(1226, v);
    }

    pub fn player_today_contribution(&self) -> Option<i32> {
        self.get_int(1226)
    }

    pub fn set_player_yesterday_contribution(&mut self, v: i32) {
        self.set_int(1227, v);
    }

    pub fn player_yesterday_contribution(&self) -> Option<i32> {
        self.get_int(1227)
    }

    pub fn set_player_lifetime_honorbale_kills(&mut self, v: i32) {
        self.set_int(1228, v);
    }

    pub fn player_lifetime_honorbale_kills(&self) -> Option<i32> {
        self.get_int(1228)
    }

    pub fn set_player_bytes2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(1229, a, b, c, d);
    }

    pub fn player_bytes2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(1229)
    }

    pub fn set_player_watched_faction_index(&mut self, v: i32) {
        self.set_int(1230, v);
    }

    pub fn player_watched_faction_index(&self) -> Option<i32> {
        self.get_int(1230)
    }

    pub fn set_player_combat_rating_1(&mut self, v: i32) {
        self.set_int(1231, v);
    }

    pub fn player_combat_rating_1(&self) -> Option<i32> {
        self.get_int(1231)
    }

    pub fn set_player_arena_team_info_1_1(&mut self, v: i32) {
        self.set_int(1256, v);
    }

    pub fn player_arena_team_info_1_1(&self) -> Option<i32> {
        self.get_int(1256)
    }

    pub fn set_player_honor_currency(&mut self, v: i32) {
        self.set_int(1277, v);
    }

    pub fn player_honor_currency(&self) -> Option<i32> {
        self.get_int(1277)
    }

    pub fn set_player_arena_currency(&mut self, v: i32) {
        self.set_int(1278, v);
    }

    pub fn player_arena_currency(&self) -> Option<i32> {
        self.get_int(1278)
    }

    pub fn set_player_max_level(&mut self, v: i32) {
        self.set_int(1279, v);
    }

    pub fn player_max_level(&self) -> Option<i32> {
        self.get_int(1279)
    }

    pub fn set_player_daily_quests_1(&mut self, v: i32) {
        self.set_int(1280, v);
    }

    pub fn player_daily_quests_1(&self) -> Option<i32> {
        self.get_int(1280)
    }

    pub fn set_player_rune_regen_1(&mut self, v: f32) {
        self.set_float(1305, v);
    }

    pub fn player_rune_regen_1(&self) -> Option<f32> {
        self.get_float(1305)
    }

    pub fn set_player_no_reagent_cost_1(&mut self, v: i32) {
        self.set_int(1309, v);
    }

    pub fn player_no_reagent_cost_1(&self) -> Option<i32> {
        self.get_int(1309)
    }

    pub fn set_player_glyph_slots_1(&mut self, v: i32) {
        self.set_int(1312, v);
    }

    pub fn player_glyph_slots_1(&self) -> Option<i32> {
        self.get_int(1312)
    }

    pub fn set_player_glyphs_1(&mut self, v: i32) {
        self.set_int(1318, v);
    }

    pub fn player_glyphs_1(&self) -> Option<i32> {
        self.get_int(1318)
    }

    pub fn set_player_glyphs_enabled(&mut self, v: i32) {
        self.set_int(1324, v);
    }

    pub fn player_glyphs_enabled(&self) -> Option<i32> {
        self.get_int(1324)
    }

    pub fn set_player_pet_spell_power(&mut self, v: i32) {
        self.set_int(1325, v);
    }

    pub fn player_pet_spell_power(&self) -> Option<i32> {
        self.get_int(1325)
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        self.get_guid(6)
    }

}

impl UpdateGameObject {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.set_guid(0, v);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        self.get_guid(0)
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.set_int(2, v);
    }

    pub fn object_type(&self) -> Option<i32> {
        self.get_int(2)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.set_int(3, v);
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.get_int(3)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.set_float(4, v);
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.get_float(4)
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        self.get_guid(6)
    }

    pub fn set_gameobject_displayid(&mut self, v: i32) {
        self.set_int(8, v);
    }

    pub fn gameobject_displayid(&self) -> Option<i32> {
        self.get_int(8)
    }

    pub fn set_gameobject_flags(&mut self, v: i32) {
        self.set_int(9, v);
    }

    pub fn gameobject_flags(&self) -> Option<i32> {
        self.get_int(9)
    }

    pub fn set_gameobject_parentrotation(&mut self, v: f32) {
        self.set_float(10, v);
    }

    pub fn gameobject_parentrotation(&self) -> Option<f32> {
        self.get_float(10)
    }

    pub fn set_gameobject_dynamic(&mut self, a: u16, b: u16) {
        self.set_shorts(14, a, b);
    }

    pub fn gameobject_dynamic(&self) -> Option<(u16, u16)> {
        self.get_shorts(14)
    }

    pub fn set_gameobject_faction(&mut self, v: i32) {
        self.set_int(15, v);
    }

    pub fn gameobject_faction(&self) -> Option<i32> {
        self.get_int(15)
    }

    pub fn set_gameobject_level(&mut self, v: i32) {
        self.set_int(16, v);
    }

    pub fn gameobject_level(&self) -> Option<i32> {
        self.get_int(16)
    }

    pub fn set_gameobject_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(17, a, b, c, d);
    }

    pub fn gameobject_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(17)
    }

}

impl UpdateDynamicObject {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.set_guid(0, v);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        self.get_guid(0)
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.set_int(2, v);
    }

    pub fn object_type(&self) -> Option<i32> {
        self.get_int(2)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.set_int(3, v);
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.get_int(3)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.set_float(4, v);
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.get_float(4)
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        self.get_guid(6)
    }

    pub fn set_dynamicobject_caster(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn dynamicobject_caster(&self) -> Option<Guid> {
        self.get_guid(6)
    }

    pub fn set_dynamicobject_bytes(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(8, a, b, c, d);
    }

    pub fn dynamicobject_bytes(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(8)
    }

    pub fn set_dynamicobject_spellid(&mut self, v: i32) {
        self.set_int(9, v);
    }

    pub fn dynamicobject_spellid(&self) -> Option<i32> {
        self.get_int(9)
    }

    pub fn set_dynamicobject_radius(&mut self, v: f32) {
        self.set_float(10, v);
    }

    pub fn dynamicobject_radius(&self) -> Option<f32> {
        self.get_float(10)
    }

    pub fn set_dynamicobject_casttime(&mut self, v: i32) {
        self.set_int(11, v);
    }

    pub fn dynamicobject_casttime(&self) -> Option<i32> {
        self.get_int(11)
    }

}

impl UpdateCorpse {
    pub fn set_object_guid(&mut self, v: Guid) {
        self.set_guid(0, v);
    }

    pub fn object_guid(&self) -> Option<Guid> {
        self.get_guid(0)
    }

    pub fn set_object_type(&mut self, v: i32) {
        self.set_int(2, v);
    }

    pub fn object_type(&self) -> Option<i32> {
        self.get_int(2)
    }

    pub fn set_object_entry(&mut self, v: i32) {
        self.set_int(3, v);
    }

    pub fn object_entry(&self) -> Option<i32> {
        self.get_int(3)
    }

    pub fn set_object_scale_x(&mut self, v: f32) {
        self.set_float(4, v);
    }

    pub fn object_scale_x(&self) -> Option<f32> {
        self.get_float(4)
    }

    pub fn set_object_created_by(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        self.get_guid(6)
    }

    pub fn set_corpse_owner(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn corpse_owner(&self) -> Option<Guid> {
        self.get_guid(6)
    }

    pub fn set_corpse_party(&mut self, v: Guid) {
        self.set_guid(8, v);
    }

    pub fn corpse_party(&self) -> Option<Guid> {
        self.get_guid(8)
    }

    pub fn set_corpse_display_id(&mut self, v: i32) {
        self.set_int(10, v);
    }

    pub fn corpse_display_id(&self) -> Option<i32> {
        self.get_int(10)
    }

    pub fn set_corpse_item(&mut self, v: i32) {
        self.set_int(11, v);
    }

    pub fn corpse_item(&self) -> Option<i32> {
        self.get_int(11)
    }

    pub fn set_corpse_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(30, a, b, c, d);
    }

    pub fn corpse_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(30)
    }

    pub fn set_corpse_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(31, a, b, c, d);
    }

    pub fn corpse_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(31)
    }

    pub fn set_corpse_guild(&mut self, v: i32) {
        self.set_int(32, v);
    }

    pub fn corpse_guild(&self) -> Option<i32> {
        self.get_int(32)
    }

    pub fn set_corpse_flags(&mut self, v: i32) {
        self.set_int(33, v);
    }

    pub fn corpse_flags(&self) -> Option<i32> {
        self.get_int(33)
    }

    pub fn set_corpse_dynamic_flags(&mut self, v: i32) {
        self.set_int(34, v);
    }

    pub fn corpse_dynamic_flags(&self) -> Option<i32> {
        self.get_int(34)
    }

}

