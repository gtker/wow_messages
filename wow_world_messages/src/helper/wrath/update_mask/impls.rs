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

    pub fn set_player_visible_item(mut self, visible_item: crate::wrath::VisibleItem, index: VisibleItemIndex) -> Self {
        for (index, value) in visible_item.mask_values(index) {
            self.header_set(index, value);
        }
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

    pub fn set_player_field_inv(mut self, item_slot: crate::wrath::ItemSlot, item: Guid) -> Self {
        let offset = 324 + item_slot.as_int() as u16 * 2;
        self.set_guid(offset, item);
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

    pub fn set_player_bytes2_glow(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
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

    pub fn set_player_visible_item(&mut self, visible_item: crate::wrath::VisibleItem, index: VisibleItemIndex) {
        for (index, value) in visible_item.mask_values(index) {
            self.header_set(index, value);
        }
    }

    pub fn player_visible_item(&self, index: VisibleItemIndex) -> Option<crate::wrath::VisibleItem> {
        crate::wrath::VisibleItem::from_range(self.values.range(index.first()..=index.last()))
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

    pub fn set_player_field_inv(&mut self, item_slot: crate::wrath::ItemSlot, item: Guid) {
        let offset = 324 + item_slot.as_int() as u16 * 2;
        self.set_guid(offset, item);
    }

    pub fn player_field_inv(&self, item_slot: crate::wrath::ItemSlot) -> Option<Guid> {
        let offset = 324 + item_slot.as_int() as u16 * 2;
        self.get_guid(offset)
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

    pub fn set_player_bytes2_glow(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(1229, a, b, c, d);
    }

    pub fn player_bytes2_glow(&self) -> Option<(u8, u8, u8, u8)> {
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

