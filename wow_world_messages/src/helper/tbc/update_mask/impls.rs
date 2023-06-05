use crate::Guid;
use std::convert::TryInto;
use super::indices::*;
use crate::tbc::{Race};
use crate::tbc::{Class};
use crate::tbc::{Gender};
use crate::tbc::{Power};
use crate::tbc::{UpdateContainer, UpdateContainerBuilder, UpdateCorpse, UpdateCorpseBuilder, UpdateDynamicObject, UpdateDynamicObjectBuilder, UpdateGameObject, UpdateGameObjectBuilder, UpdateItem, UpdateItemBuilder, UpdatePlayer, UpdatePlayerBuilder, UpdateUnit, UpdateUnitBuilder};

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

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.set_int(55, v);
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.set_int(56, v);
        self
    }

    pub fn set_item_item_text_id(mut self, v: i32) -> Self {
        self.set_int(57, v);
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.set_int(58, v);
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.set_int(59, v);
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

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.set_int(55, v);
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.set_int(56, v);
        self
    }

    pub fn set_item_item_text_id(mut self, v: i32) -> Self {
        self.set_int(57, v);
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.set_int(58, v);
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.set_int(59, v);
        self
    }

    pub fn set_container_num_slots(mut self, v: i32) -> Self {
        self.set_int(60, v);
        self
    }

    pub fn set_container_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(62, v);
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

    pub fn set_unit_charmedby(mut self, v: Guid) -> Self {
        self.set_guid(10, v);
        self
    }

    pub fn set_unit_summonedby(mut self, v: Guid) -> Self {
        self.set_guid(12, v);
        self
    }

    pub fn set_unit_createdby(mut self, v: Guid) -> Self {
        self.set_guid(14, v);
        self
    }

    pub fn set_unit_target(mut self, v: Guid) -> Self {
        self.set_guid(16, v);
        self
    }

    pub fn set_unit_persuaded(mut self, v: Guid) -> Self {
        self.set_guid(18, v);
        self
    }

    pub fn set_unit_channel_object(mut self, v: Guid) -> Self {
        self.set_guid(20, v);
        self
    }

    pub fn set_unit_health(mut self, v: i32) -> Self {
        self.set_int(22, v);
        self
    }

    pub fn set_unit_power1(mut self, v: i32) -> Self {
        self.set_int(23, v);
        self
    }

    pub fn set_unit_power2(mut self, v: i32) -> Self {
        self.set_int(24, v);
        self
    }

    pub fn set_unit_power3(mut self, v: i32) -> Self {
        self.set_int(25, v);
        self
    }

    pub fn set_unit_power4(mut self, v: i32) -> Self {
        self.set_int(26, v);
        self
    }

    pub fn set_unit_power5(mut self, v: i32) -> Self {
        self.set_int(27, v);
        self
    }

    pub fn set_unit_maxhealth(mut self, v: i32) -> Self {
        self.set_int(28, v);
        self
    }

    pub fn set_unit_maxpower1(mut self, v: i32) -> Self {
        self.set_int(29, v);
        self
    }

    pub fn set_unit_maxpower2(mut self, v: i32) -> Self {
        self.set_int(30, v);
        self
    }

    pub fn set_unit_maxpower3(mut self, v: i32) -> Self {
        self.set_int(31, v);
        self
    }

    pub fn set_unit_maxpower4(mut self, v: i32) -> Self {
        self.set_int(32, v);
        self
    }

    pub fn set_unit_maxpower5(mut self, v: i32) -> Self {
        self.set_int(33, v);
        self
    }

    pub fn set_unit_level(mut self, v: i32) -> Self {
        self.set_int(34, v);
        self
    }

    pub fn set_unit_factiontemplate(mut self, v: i32) -> Self {
        self.set_int(35, v);
        self
    }

    pub fn set_unit_bytes_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.set_bytes(36, race.as_int(), class.as_int(), gender.as_int(), power.as_int());
        self
    }

    pub fn set_unit_virtual_item_slot_display(mut self, v: i32) -> Self {
        self.set_int(37, v);
        self
    }

    pub fn set_unit_virtual_item_info(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(40, a, b, c, d);
        self
    }

    pub fn set_unit_flags(mut self, v: i32) -> Self {
        self.set_int(46, v);
        self
    }

    pub fn set_unit_flags_2(mut self, v: i32) -> Self {
        self.set_int(47, v);
        self
    }

    pub fn set_unit_aura(mut self, v: i32) -> Self {
        self.set_int(48, v);
        self
    }

    pub fn set_unit_auraflags(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(104, a, b, c, d);
        self
    }

    pub fn set_unit_auralevels(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(118, a, b, c, d);
        self
    }

    pub fn set_unit_auraapplications(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(132, a, b, c, d);
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.set_int(146, v);
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.set_int(147, v);
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.set_int(149, v);
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.set_float(150, v);
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.set_float(151, v);
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.set_int(152, v);
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.set_int(153, v);
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.set_int(154, v);
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.set_float(155, v);
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.set_float(156, v);
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(157, v);
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(158, v);
        self
    }

    pub fn set_unit_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(159, a, b, c, d);
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.set_int(160, v);
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.set_int(161, v);
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.set_int(162, v);
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.set_int(163, v);
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.set_int(164, v);
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.set_int(165, v);
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.set_float(166, v);
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.set_int(167, v);
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.set_int(168, v);
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.set_int(169, v);
        self
    }

    pub fn set_unit_training_points(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(170, a, b);
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.set_int(171, v);
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.set_int(172, v);
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.set_int(173, v);
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.set_int(174, v);
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.set_int(175, v);
        self
    }

    pub fn set_unit_posstat1(mut self, v: i32) -> Self {
        self.set_int(177, v);
        self
    }

    pub fn set_unit_posstat2(mut self, v: i32) -> Self {
        self.set_int(178, v);
        self
    }

    pub fn set_unit_posstat3(mut self, v: i32) -> Self {
        self.set_int(179, v);
        self
    }

    pub fn set_unit_negstat1(mut self, v: i32) -> Self {
        self.set_int(182, v);
        self
    }

    pub fn set_unit_negstat2(mut self, v: i32) -> Self {
        self.set_int(183, v);
        self
    }

    pub fn set_unit_negstat3(mut self, v: i32) -> Self {
        self.set_int(184, v);
        self
    }

    pub fn set_unit_resistances(mut self, v: i32) -> Self {
        self.set_int(186, v);
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.set_int(207, v);
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.set_int(208, v);
        self
    }

    pub fn set_unit_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(209, a, b, c, d);
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.set_int(210, v);
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(211, a, b);
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(212, v);
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.set_int(213, v);
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(214, a, b);
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(215, v);
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.set_float(216, v);
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.set_float(217, v);
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.set_int(218, v);
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.set_float(225, v);
        self
    }

    pub fn set_unit_maxhealthmodifier(mut self, v: f32) -> Self {
        self.set_float(232, v);
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

    pub fn set_unit_charmedby(mut self, v: Guid) -> Self {
        self.set_guid(10, v);
        self
    }

    pub fn set_unit_summonedby(mut self, v: Guid) -> Self {
        self.set_guid(12, v);
        self
    }

    pub fn set_unit_createdby(mut self, v: Guid) -> Self {
        self.set_guid(14, v);
        self
    }

    pub fn set_unit_target(mut self, v: Guid) -> Self {
        self.set_guid(16, v);
        self
    }

    pub fn set_unit_persuaded(mut self, v: Guid) -> Self {
        self.set_guid(18, v);
        self
    }

    pub fn set_unit_channel_object(mut self, v: Guid) -> Self {
        self.set_guid(20, v);
        self
    }

    pub fn set_unit_health(mut self, v: i32) -> Self {
        self.set_int(22, v);
        self
    }

    pub fn set_unit_power1(mut self, v: i32) -> Self {
        self.set_int(23, v);
        self
    }

    pub fn set_unit_power2(mut self, v: i32) -> Self {
        self.set_int(24, v);
        self
    }

    pub fn set_unit_power3(mut self, v: i32) -> Self {
        self.set_int(25, v);
        self
    }

    pub fn set_unit_power4(mut self, v: i32) -> Self {
        self.set_int(26, v);
        self
    }

    pub fn set_unit_power5(mut self, v: i32) -> Self {
        self.set_int(27, v);
        self
    }

    pub fn set_unit_maxhealth(mut self, v: i32) -> Self {
        self.set_int(28, v);
        self
    }

    pub fn set_unit_maxpower1(mut self, v: i32) -> Self {
        self.set_int(29, v);
        self
    }

    pub fn set_unit_maxpower2(mut self, v: i32) -> Self {
        self.set_int(30, v);
        self
    }

    pub fn set_unit_maxpower3(mut self, v: i32) -> Self {
        self.set_int(31, v);
        self
    }

    pub fn set_unit_maxpower4(mut self, v: i32) -> Self {
        self.set_int(32, v);
        self
    }

    pub fn set_unit_maxpower5(mut self, v: i32) -> Self {
        self.set_int(33, v);
        self
    }

    pub fn set_unit_level(mut self, v: i32) -> Self {
        self.set_int(34, v);
        self
    }

    pub fn set_unit_factiontemplate(mut self, v: i32) -> Self {
        self.set_int(35, v);
        self
    }

    pub fn set_unit_bytes_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.set_bytes(36, race.as_int(), class.as_int(), gender.as_int(), power.as_int());
        self
    }

    pub fn set_unit_virtual_item_slot_display(mut self, v: i32) -> Self {
        self.set_int(37, v);
        self
    }

    pub fn set_unit_virtual_item_info(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(40, a, b, c, d);
        self
    }

    pub fn set_unit_flags(mut self, v: i32) -> Self {
        self.set_int(46, v);
        self
    }

    pub fn set_unit_flags_2(mut self, v: i32) -> Self {
        self.set_int(47, v);
        self
    }

    pub fn set_unit_aura(mut self, v: i32) -> Self {
        self.set_int(48, v);
        self
    }

    pub fn set_unit_auraflags(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(104, a, b, c, d);
        self
    }

    pub fn set_unit_auralevels(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(118, a, b, c, d);
        self
    }

    pub fn set_unit_auraapplications(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(132, a, b, c, d);
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.set_int(146, v);
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.set_int(147, v);
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.set_int(149, v);
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.set_float(150, v);
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.set_float(151, v);
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.set_int(152, v);
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.set_int(153, v);
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.set_int(154, v);
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.set_float(155, v);
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.set_float(156, v);
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(157, v);
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(158, v);
        self
    }

    pub fn set_unit_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(159, a, b, c, d);
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.set_int(160, v);
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.set_int(161, v);
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.set_int(162, v);
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.set_int(163, v);
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.set_int(164, v);
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.set_int(165, v);
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.set_float(166, v);
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.set_int(167, v);
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.set_int(168, v);
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.set_int(169, v);
        self
    }

    pub fn set_unit_training_points(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(170, a, b);
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.set_int(171, v);
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.set_int(172, v);
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.set_int(173, v);
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.set_int(174, v);
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.set_int(175, v);
        self
    }

    pub fn set_player_posstat0(mut self, v: i32) -> Self {
        self.set_int(176, v);
        self
    }

    pub fn set_unit_posstat1(mut self, v: i32) -> Self {
        self.set_int(177, v);
        self
    }

    pub fn set_unit_posstat2(mut self, v: i32) -> Self {
        self.set_int(178, v);
        self
    }

    pub fn set_unit_posstat3(mut self, v: i32) -> Self {
        self.set_int(179, v);
        self
    }

    pub fn set_player_posstat4(mut self, v: i32) -> Self {
        self.set_int(180, v);
        self
    }

    pub fn set_player_negstat0(mut self, v: i32) -> Self {
        self.set_int(181, v);
        self
    }

    pub fn set_unit_negstat1(mut self, v: i32) -> Self {
        self.set_int(182, v);
        self
    }

    pub fn set_unit_negstat2(mut self, v: i32) -> Self {
        self.set_int(183, v);
        self
    }

    pub fn set_unit_negstat3(mut self, v: i32) -> Self {
        self.set_int(184, v);
        self
    }

    pub fn set_player_negstat4(mut self, v: i32) -> Self {
        self.set_int(185, v);
        self
    }

    pub fn set_unit_resistances(mut self, v: i32) -> Self {
        self.set_int(186, v);
        self
    }

    pub fn set_player_resistancebuffmodspositive(mut self, v: i32) -> Self {
        self.set_int(193, v);
        self
    }

    pub fn set_player_resistancebuffmodsnegative(mut self, v: i32) -> Self {
        self.set_int(200, v);
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.set_int(207, v);
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.set_int(208, v);
        self
    }

    pub fn set_unit_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(209, a, b, c, d);
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.set_int(210, v);
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(211, a, b);
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(212, v);
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.set_int(213, v);
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(214, a, b);
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(215, v);
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.set_float(216, v);
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.set_float(217, v);
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.set_int(218, v);
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.set_float(225, v);
        self
    }

    pub fn set_unit_maxhealthmodifier(mut self, v: f32) -> Self {
        self.set_float(232, v);
        self
    }

    pub fn set_player_duel_arbiter(mut self, v: Guid) -> Self {
        self.set_guid(234, v);
        self
    }

    pub fn set_player_flags(mut self, v: i32) -> Self {
        self.set_int(236, v);
        self
    }

    pub fn set_player_guildid(mut self, v: i32) -> Self {
        self.set_int(237, v);
        self
    }

    pub fn set_player_guildrank(mut self, v: i32) -> Self {
        self.set_int(238, v);
        self
    }

    pub fn set_player_field_bytes(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(239, a, b, c, d);
        self
    }

    pub fn set_player_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(240, a, b, c, d);
        self
    }

    pub fn set_player_bytes_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(241, a, b, c, d);
        self
    }

    pub fn set_player_duel_team(mut self, v: i32) -> Self {
        self.set_int(242, v);
        self
    }

    pub fn set_player_guild_timestamp(mut self, v: i32) -> Self {
        self.set_int(243, v);
        self
    }

    pub fn set_player_quest_log_1_1(mut self, v: i32) -> Self {
        self.set_int(244, v);
        self
    }

    pub fn set_player_quest_log_1_2(mut self, v: i32) -> Self {
        self.set_int(245, v);
        self
    }

    pub fn set_player_quest_log_1_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(246, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_1_4(mut self, v: i32) -> Self {
        self.set_int(247, v);
        self
    }

    pub fn set_player_quest_log_2_1(mut self, v: i32) -> Self {
        self.set_int(248, v);
        self
    }

    pub fn set_player_quest_log_2_2(mut self, v: i32) -> Self {
        self.set_int(249, v);
        self
    }

    pub fn set_player_quest_log_2_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(250, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_2_4(mut self, v: i32) -> Self {
        self.set_int(251, v);
        self
    }

    pub fn set_player_quest_log_3_1(mut self, v: i32) -> Self {
        self.set_int(252, v);
        self
    }

    pub fn set_player_quest_log_3_2(mut self, v: i32) -> Self {
        self.set_int(253, v);
        self
    }

    pub fn set_player_quest_log_3_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(254, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_3_4(mut self, v: i32) -> Self {
        self.set_int(255, v);
        self
    }

    pub fn set_player_quest_log_4_1(mut self, v: i32) -> Self {
        self.set_int(256, v);
        self
    }

    pub fn set_player_quest_log_4_2(mut self, v: i32) -> Self {
        self.set_int(257, v);
        self
    }

    pub fn set_player_quest_log_4_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(258, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_4_4(mut self, v: i32) -> Self {
        self.set_int(259, v);
        self
    }

    pub fn set_player_quest_log_5_1(mut self, v: i32) -> Self {
        self.set_int(260, v);
        self
    }

    pub fn set_player_quest_log_5_2(mut self, v: i32) -> Self {
        self.set_int(261, v);
        self
    }

    pub fn set_player_quest_log_5_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(262, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_5_4(mut self, v: i32) -> Self {
        self.set_int(263, v);
        self
    }

    pub fn set_player_quest_log_6_1(mut self, v: i32) -> Self {
        self.set_int(264, v);
        self
    }

    pub fn set_player_quest_log_6_2(mut self, v: i32) -> Self {
        self.set_int(265, v);
        self
    }

    pub fn set_player_quest_log_6_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(266, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_6_4(mut self, v: i32) -> Self {
        self.set_int(267, v);
        self
    }

    pub fn set_player_quest_log_7_1(mut self, v: i32) -> Self {
        self.set_int(268, v);
        self
    }

    pub fn set_player_quest_log_7_2(mut self, v: i32) -> Self {
        self.set_int(269, v);
        self
    }

    pub fn set_player_quest_log_7_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(270, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_7_4(mut self, v: i32) -> Self {
        self.set_int(271, v);
        self
    }

    pub fn set_player_quest_log_8_1(mut self, v: i32) -> Self {
        self.set_int(272, v);
        self
    }

    pub fn set_player_quest_log_8_2(mut self, v: i32) -> Self {
        self.set_int(273, v);
        self
    }

    pub fn set_player_quest_log_8_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(274, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_8_4(mut self, v: i32) -> Self {
        self.set_int(275, v);
        self
    }

    pub fn set_player_quest_log_9_1(mut self, v: i32) -> Self {
        self.set_int(276, v);
        self
    }

    pub fn set_player_quest_log_9_2(mut self, v: i32) -> Self {
        self.set_int(277, v);
        self
    }

    pub fn set_player_quest_log_9_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(278, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_9_4(mut self, v: i32) -> Self {
        self.set_int(279, v);
        self
    }

    pub fn set_player_quest_log_10_1(mut self, v: i32) -> Self {
        self.set_int(280, v);
        self
    }

    pub fn set_player_quest_log_10_2(mut self, v: i32) -> Self {
        self.set_int(281, v);
        self
    }

    pub fn set_player_quest_log_10_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(282, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_10_4(mut self, v: i32) -> Self {
        self.set_int(283, v);
        self
    }

    pub fn set_player_quest_log_11_1(mut self, v: i32) -> Self {
        self.set_int(284, v);
        self
    }

    pub fn set_player_quest_log_11_2(mut self, v: i32) -> Self {
        self.set_int(285, v);
        self
    }

    pub fn set_player_quest_log_11_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(286, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_11_4(mut self, v: i32) -> Self {
        self.set_int(287, v);
        self
    }

    pub fn set_player_quest_log_12_1(mut self, v: i32) -> Self {
        self.set_int(288, v);
        self
    }

    pub fn set_player_quest_log_12_2(mut self, v: i32) -> Self {
        self.set_int(289, v);
        self
    }

    pub fn set_player_quest_log_12_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(290, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_12_4(mut self, v: i32) -> Self {
        self.set_int(291, v);
        self
    }

    pub fn set_player_quest_log_13_1(mut self, v: i32) -> Self {
        self.set_int(292, v);
        self
    }

    pub fn set_player_quest_log_13_2(mut self, v: i32) -> Self {
        self.set_int(293, v);
        self
    }

    pub fn set_player_quest_log_13_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(294, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_13_4(mut self, v: i32) -> Self {
        self.set_int(295, v);
        self
    }

    pub fn set_player_quest_log_14_1(mut self, v: i32) -> Self {
        self.set_int(296, v);
        self
    }

    pub fn set_player_quest_log_14_2(mut self, v: i32) -> Self {
        self.set_int(297, v);
        self
    }

    pub fn set_player_quest_log_14_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(298, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_14_4(mut self, v: i32) -> Self {
        self.set_int(299, v);
        self
    }

    pub fn set_player_quest_log_15_1(mut self, v: i32) -> Self {
        self.set_int(300, v);
        self
    }

    pub fn set_player_quest_log_15_2(mut self, v: i32) -> Self {
        self.set_int(301, v);
        self
    }

    pub fn set_player_quest_log_15_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(302, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_15_4(mut self, v: i32) -> Self {
        self.set_int(303, v);
        self
    }

    pub fn set_player_quest_log_16_1(mut self, v: i32) -> Self {
        self.set_int(304, v);
        self
    }

    pub fn set_player_quest_log_16_2(mut self, v: i32) -> Self {
        self.set_int(305, v);
        self
    }

    pub fn set_player_quest_log_16_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(306, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_16_4(mut self, v: i32) -> Self {
        self.set_int(307, v);
        self
    }

    pub fn set_player_quest_log_17_1(mut self, v: i32) -> Self {
        self.set_int(308, v);
        self
    }

    pub fn set_player_quest_log_17_2(mut self, v: i32) -> Self {
        self.set_int(309, v);
        self
    }

    pub fn set_player_quest_log_17_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(310, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_17_4(mut self, v: i32) -> Self {
        self.set_int(311, v);
        self
    }

    pub fn set_player_quest_log_18_1(mut self, v: i32) -> Self {
        self.set_int(312, v);
        self
    }

    pub fn set_player_quest_log_18_2(mut self, v: i32) -> Self {
        self.set_int(313, v);
        self
    }

    pub fn set_player_quest_log_18_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(314, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_18_4(mut self, v: i32) -> Self {
        self.set_int(315, v);
        self
    }

    pub fn set_player_quest_log_19_1(mut self, v: i32) -> Self {
        self.set_int(316, v);
        self
    }

    pub fn set_player_quest_log_19_2(mut self, v: i32) -> Self {
        self.set_int(317, v);
        self
    }

    pub fn set_player_quest_log_19_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(318, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_19_4(mut self, v: i32) -> Self {
        self.set_int(319, v);
        self
    }

    pub fn set_player_quest_log_20_1(mut self, v: i32) -> Self {
        self.set_int(320, v);
        self
    }

    pub fn set_player_quest_log_20_2(mut self, v: i32) -> Self {
        self.set_int(321, v);
        self
    }

    pub fn set_player_quest_log_20_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(322, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_20_4(mut self, v: i32) -> Self {
        self.set_int(323, v);
        self
    }

    pub fn set_player_quest_log_21_1(mut self, v: i32) -> Self {
        self.set_int(324, v);
        self
    }

    pub fn set_player_quest_log_21_2(mut self, v: i32) -> Self {
        self.set_int(325, v);
        self
    }

    pub fn set_player_quest_log_21_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(326, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_21_4(mut self, v: i32) -> Self {
        self.set_int(327, v);
        self
    }

    pub fn set_player_quest_log_22_1(mut self, v: i32) -> Self {
        self.set_int(328, v);
        self
    }

    pub fn set_player_quest_log_22_2(mut self, v: i32) -> Self {
        self.set_int(329, v);
        self
    }

    pub fn set_player_quest_log_22_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(330, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_22_4(mut self, v: i32) -> Self {
        self.set_int(331, v);
        self
    }

    pub fn set_player_quest_log_23_1(mut self, v: i32) -> Self {
        self.set_int(332, v);
        self
    }

    pub fn set_player_quest_log_23_2(mut self, v: i32) -> Self {
        self.set_int(333, v);
        self
    }

    pub fn set_player_quest_log_23_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(334, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_23_4(mut self, v: i32) -> Self {
        self.set_int(335, v);
        self
    }

    pub fn set_player_quest_log_24_1(mut self, v: i32) -> Self {
        self.set_int(336, v);
        self
    }

    pub fn set_player_quest_log_24_2(mut self, v: i32) -> Self {
        self.set_int(337, v);
        self
    }

    pub fn set_player_quest_log_24_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(338, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_24_4(mut self, v: i32) -> Self {
        self.set_int(339, v);
        self
    }

    pub fn set_player_quest_log_25_1(mut self, v: i32) -> Self {
        self.set_int(340, v);
        self
    }

    pub fn set_player_quest_log_25_2(mut self, v: i32) -> Self {
        self.set_int(341, v);
        self
    }

    pub fn set_player_quest_log_25_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(342, a, b, c, d);
        self
    }

    pub fn set_player_quest_log_25_4(mut self, v: i32) -> Self {
        self.set_int(343, v);
        self
    }

    pub fn set_player_visible_item_1_creator(mut self, v: Guid) -> Self {
        self.set_guid(344, v);
        self
    }

    pub fn set_player_visible_item_1_0(mut self, v: i32) -> Self {
        self.set_int(346, v);
        self
    }

    pub fn set_player_visible_item_1_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(358, a, b);
        self
    }

    pub fn set_player_visible_item_2_creator(mut self, v: Guid) -> Self {
        self.set_guid(360, v);
        self
    }

    pub fn set_player_visible_item_2_0(mut self, v: i32) -> Self {
        self.set_int(362, v);
        self
    }

    pub fn set_player_visible_item_2_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(374, a, b);
        self
    }

    pub fn set_player_visible_item_3_creator(mut self, v: Guid) -> Self {
        self.set_guid(376, v);
        self
    }

    pub fn set_player_visible_item_3_0(mut self, v: i32) -> Self {
        self.set_int(378, v);
        self
    }

    pub fn set_player_visible_item_3_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(390, a, b);
        self
    }

    pub fn set_player_visible_item_4_creator(mut self, v: Guid) -> Self {
        self.set_guid(392, v);
        self
    }

    pub fn set_player_visible_item_4_0(mut self, v: i32) -> Self {
        self.set_int(394, v);
        self
    }

    pub fn set_player_visible_item_4_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(406, a, b);
        self
    }

    pub fn set_player_visible_item_5_creator(mut self, v: Guid) -> Self {
        self.set_guid(408, v);
        self
    }

    pub fn set_player_visible_item_5_0(mut self, v: i32) -> Self {
        self.set_int(410, v);
        self
    }

    pub fn set_player_visible_item_5_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(422, a, b);
        self
    }

    pub fn set_player_visible_item_6_creator(mut self, v: Guid) -> Self {
        self.set_guid(424, v);
        self
    }

    pub fn set_player_visible_item_6_0(mut self, v: i32) -> Self {
        self.set_int(426, v);
        self
    }

    pub fn set_player_visible_item_6_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(438, a, b);
        self
    }

    pub fn set_player_visible_item_7_creator(mut self, v: Guid) -> Self {
        self.set_guid(440, v);
        self
    }

    pub fn set_player_visible_item_7_0(mut self, v: i32) -> Self {
        self.set_int(442, v);
        self
    }

    pub fn set_player_visible_item_7_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(454, a, b);
        self
    }

    pub fn set_player_visible_item_8_creator(mut self, v: Guid) -> Self {
        self.set_guid(456, v);
        self
    }

    pub fn set_player_visible_item_8_0(mut self, v: i32) -> Self {
        self.set_int(458, v);
        self
    }

    pub fn set_player_visible_item_8_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(470, a, b);
        self
    }

    pub fn set_player_visible_item_9_creator(mut self, v: Guid) -> Self {
        self.set_guid(472, v);
        self
    }

    pub fn set_player_visible_item_9_0(mut self, v: i32) -> Self {
        self.set_int(474, v);
        self
    }

    pub fn set_player_visible_item_9_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(486, a, b);
        self
    }

    pub fn set_player_visible_item_10_creator(mut self, v: Guid) -> Self {
        self.set_guid(488, v);
        self
    }

    pub fn set_player_visible_item_10_0(mut self, v: i32) -> Self {
        self.set_int(490, v);
        self
    }

    pub fn set_player_visible_item_10_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(502, a, b);
        self
    }

    pub fn set_player_visible_item_11_creator(mut self, v: Guid) -> Self {
        self.set_guid(504, v);
        self
    }

    pub fn set_player_visible_item_11_0(mut self, v: i32) -> Self {
        self.set_int(506, v);
        self
    }

    pub fn set_player_visible_item_11_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(518, a, b);
        self
    }

    pub fn set_player_visible_item_12_creator(mut self, v: Guid) -> Self {
        self.set_guid(520, v);
        self
    }

    pub fn set_player_visible_item_12_0(mut self, v: i32) -> Self {
        self.set_int(522, v);
        self
    }

    pub fn set_player_visible_item_12_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(534, a, b);
        self
    }

    pub fn set_player_visible_item_13_creator(mut self, v: Guid) -> Self {
        self.set_guid(536, v);
        self
    }

    pub fn set_player_visible_item_13_0(mut self, v: i32) -> Self {
        self.set_int(538, v);
        self
    }

    pub fn set_player_visible_item_13_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(550, a, b);
        self
    }

    pub fn set_player_visible_item_14_creator(mut self, v: Guid) -> Self {
        self.set_guid(552, v);
        self
    }

    pub fn set_player_visible_item_14_0(mut self, v: i32) -> Self {
        self.set_int(554, v);
        self
    }

    pub fn set_player_visible_item_14_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(566, a, b);
        self
    }

    pub fn set_player_visible_item_15_creator(mut self, v: Guid) -> Self {
        self.set_guid(568, v);
        self
    }

    pub fn set_player_visible_item_15_0(mut self, v: i32) -> Self {
        self.set_int(570, v);
        self
    }

    pub fn set_player_visible_item_15_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(582, a, b);
        self
    }

    pub fn set_player_visible_item_16_creator(mut self, v: Guid) -> Self {
        self.set_guid(584, v);
        self
    }

    pub fn set_player_visible_item_16_0(mut self, v: i32) -> Self {
        self.set_int(586, v);
        self
    }

    pub fn set_player_visible_item_16_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(598, a, b);
        self
    }

    pub fn set_player_visible_item_17_creator(mut self, v: Guid) -> Self {
        self.set_guid(600, v);
        self
    }

    pub fn set_player_visible_item_17_0(mut self, v: i32) -> Self {
        self.set_int(602, v);
        self
    }

    pub fn set_player_visible_item_17_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(614, a, b);
        self
    }

    pub fn set_player_visible_item_18_creator(mut self, v: Guid) -> Self {
        self.set_guid(616, v);
        self
    }

    pub fn set_player_visible_item_18_0(mut self, v: i32) -> Self {
        self.set_int(618, v);
        self
    }

    pub fn set_player_visible_item_18_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(630, a, b);
        self
    }

    pub fn set_player_visible_item_19_creator(mut self, v: Guid) -> Self {
        self.set_guid(632, v);
        self
    }

    pub fn set_player_visible_item_19_0(mut self, v: i32) -> Self {
        self.set_int(634, v);
        self
    }

    pub fn set_player_visible_item_19_properties(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(646, a, b);
        self
    }

    pub fn set_player_chosen_title(mut self, v: i32) -> Self {
        self.set_int(648, v);
        self
    }

    pub fn set_player_inv_slot_head(mut self, v: Guid) -> Self {
        self.set_guid(650, v);
        self
    }

    pub fn set_player_inv_slot_neck(mut self, v: Guid) -> Self {
        self.set_guid(652, v);
        self
    }

    pub fn set_player_inv_slot_shoulders(mut self, v: Guid) -> Self {
        self.set_guid(654, v);
        self
    }

    pub fn set_player_inv_slot_body(mut self, v: Guid) -> Self {
        self.set_guid(656, v);
        self
    }

    pub fn set_player_inv_slot_chest(mut self, v: Guid) -> Self {
        self.set_guid(658, v);
        self
    }

    pub fn set_player_inv_slot_waist(mut self, v: Guid) -> Self {
        self.set_guid(660, v);
        self
    }

    pub fn set_player_inv_slot_legs(mut self, v: Guid) -> Self {
        self.set_guid(662, v);
        self
    }

    pub fn set_player_inv_slot_feet(mut self, v: Guid) -> Self {
        self.set_guid(664, v);
        self
    }

    pub fn set_player_inv_slot_wrists(mut self, v: Guid) -> Self {
        self.set_guid(666, v);
        self
    }

    pub fn set_player_inv_slot_hands(mut self, v: Guid) -> Self {
        self.set_guid(668, v);
        self
    }

    pub fn set_player_inv_slot_finger1(mut self, v: Guid) -> Self {
        self.set_guid(670, v);
        self
    }

    pub fn set_player_inv_slot_finger2(mut self, v: Guid) -> Self {
        self.set_guid(672, v);
        self
    }

    pub fn set_player_inv_slot_trinket1(mut self, v: Guid) -> Self {
        self.set_guid(674, v);
        self
    }

    pub fn set_player_inv_slot_trinket2(mut self, v: Guid) -> Self {
        self.set_guid(676, v);
        self
    }

    pub fn set_player_inv_slot_back(mut self, v: Guid) -> Self {
        self.set_guid(678, v);
        self
    }

    pub fn set_player_inv_slot_main_hand(mut self, v: Guid) -> Self {
        self.set_guid(680, v);
        self
    }

    pub fn set_player_inv_slot_off_hand(mut self, v: Guid) -> Self {
        self.set_guid(682, v);
        self
    }

    pub fn set_player_inv_slot_ranged(mut self, v: Guid) -> Self {
        self.set_guid(684, v);
        self
    }

    pub fn set_player_inv_slot_tabard(mut self, v: Guid) -> Self {
        self.set_guid(686, v);
        self
    }

    pub fn set_player_inv_slot_bag1(mut self, v: Guid) -> Self {
        self.set_guid(688, v);
        self
    }

    pub fn set_player_inv_slot_bag2(mut self, v: Guid) -> Self {
        self.set_guid(690, v);
        self
    }

    pub fn set_player_inv_slot_bag3(mut self, v: Guid) -> Self {
        self.set_guid(692, v);
        self
    }

    pub fn set_player_inv_slot_bag4(mut self, v: Guid) -> Self {
        self.set_guid(694, v);
        self
    }

    pub fn set_player_pack_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(696, v);
        self
    }

    pub fn set_player_pack_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(698, v);
        self
    }

    pub fn set_player_pack_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(700, v);
        self
    }

    pub fn set_player_pack_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(702, v);
        self
    }

    pub fn set_player_pack_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(704, v);
        self
    }

    pub fn set_player_pack_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(706, v);
        self
    }

    pub fn set_player_pack_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(708, v);
        self
    }

    pub fn set_player_pack_slot_8(mut self, v: Guid) -> Self {
        self.set_guid(710, v);
        self
    }

    pub fn set_player_pack_slot_9(mut self, v: Guid) -> Self {
        self.set_guid(712, v);
        self
    }

    pub fn set_player_pack_slot_10(mut self, v: Guid) -> Self {
        self.set_guid(714, v);
        self
    }

    pub fn set_player_pack_slot_11(mut self, v: Guid) -> Self {
        self.set_guid(716, v);
        self
    }

    pub fn set_player_pack_slot_12(mut self, v: Guid) -> Self {
        self.set_guid(718, v);
        self
    }

    pub fn set_player_pack_slot_13(mut self, v: Guid) -> Self {
        self.set_guid(720, v);
        self
    }

    pub fn set_player_pack_slot_14(mut self, v: Guid) -> Self {
        self.set_guid(722, v);
        self
    }

    pub fn set_player_pack_slot_15(mut self, v: Guid) -> Self {
        self.set_guid(724, v);
        self
    }

    pub fn set_player_pack_slot_16(mut self, v: Guid) -> Self {
        self.set_guid(726, v);
        self
    }

    pub fn set_player_bank_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(728, v);
        self
    }

    pub fn set_player_bank_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(730, v);
        self
    }

    pub fn set_player_bank_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(732, v);
        self
    }

    pub fn set_player_bank_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(734, v);
        self
    }

    pub fn set_player_bank_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(736, v);
        self
    }

    pub fn set_player_bank_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(738, v);
        self
    }

    pub fn set_player_bank_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(740, v);
        self
    }

    pub fn set_player_bank_slot_8(mut self, v: Guid) -> Self {
        self.set_guid(742, v);
        self
    }

    pub fn set_player_bank_slot_9(mut self, v: Guid) -> Self {
        self.set_guid(744, v);
        self
    }

    pub fn set_player_bank_slot_10(mut self, v: Guid) -> Self {
        self.set_guid(746, v);
        self
    }

    pub fn set_player_bank_slot_11(mut self, v: Guid) -> Self {
        self.set_guid(748, v);
        self
    }

    pub fn set_player_bank_slot_12(mut self, v: Guid) -> Self {
        self.set_guid(750, v);
        self
    }

    pub fn set_player_bank_slot_13(mut self, v: Guid) -> Self {
        self.set_guid(752, v);
        self
    }

    pub fn set_player_bank_slot_14(mut self, v: Guid) -> Self {
        self.set_guid(754, v);
        self
    }

    pub fn set_player_bank_slot_15(mut self, v: Guid) -> Self {
        self.set_guid(756, v);
        self
    }

    pub fn set_player_bank_slot_16(mut self, v: Guid) -> Self {
        self.set_guid(758, v);
        self
    }

    pub fn set_player_bank_slot_17(mut self, v: Guid) -> Self {
        self.set_guid(760, v);
        self
    }

    pub fn set_player_bank_slot_18(mut self, v: Guid) -> Self {
        self.set_guid(762, v);
        self
    }

    pub fn set_player_bank_slot_19(mut self, v: Guid) -> Self {
        self.set_guid(764, v);
        self
    }

    pub fn set_player_bank_slot_20(mut self, v: Guid) -> Self {
        self.set_guid(766, v);
        self
    }

    pub fn set_player_bank_slot_21(mut self, v: Guid) -> Self {
        self.set_guid(768, v);
        self
    }

    pub fn set_player_bank_slot_22(mut self, v: Guid) -> Self {
        self.set_guid(770, v);
        self
    }

    pub fn set_player_bank_slot_23(mut self, v: Guid) -> Self {
        self.set_guid(772, v);
        self
    }

    pub fn set_player_bank_slot_24(mut self, v: Guid) -> Self {
        self.set_guid(774, v);
        self
    }

    pub fn set_player_bank_slot_25(mut self, v: Guid) -> Self {
        self.set_guid(776, v);
        self
    }

    pub fn set_player_bank_slot_26(mut self, v: Guid) -> Self {
        self.set_guid(778, v);
        self
    }

    pub fn set_player_bank_slot_27(mut self, v: Guid) -> Self {
        self.set_guid(780, v);
        self
    }

    pub fn set_player_bank_slot_28(mut self, v: Guid) -> Self {
        self.set_guid(782, v);
        self
    }

    pub fn set_player_bankbag_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(784, v);
        self
    }

    pub fn set_player_bankbag_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(786, v);
        self
    }

    pub fn set_player_bankbag_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(788, v);
        self
    }

    pub fn set_player_bankbag_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(790, v);
        self
    }

    pub fn set_player_bankbag_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(792, v);
        self
    }

    pub fn set_player_bankbag_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(794, v);
        self
    }

    pub fn set_player_bankbag_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(796, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(798, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(800, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(802, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(804, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(806, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(808, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(810, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_8(mut self, v: Guid) -> Self {
        self.set_guid(812, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_9(mut self, v: Guid) -> Self {
        self.set_guid(814, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_10(mut self, v: Guid) -> Self {
        self.set_guid(816, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_11(mut self, v: Guid) -> Self {
        self.set_guid(818, v);
        self
    }

    pub fn set_player_vendorbuyback_slot_12(mut self, v: Guid) -> Self {
        self.set_guid(820, v);
        self
    }

    pub fn set_player_keyring_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(822, v);
        self
    }

    pub fn set_player_keyring_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(824, v);
        self
    }

    pub fn set_player_keyring_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(826, v);
        self
    }

    pub fn set_player_keyring_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(828, v);
        self
    }

    pub fn set_player_keyring_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(830, v);
        self
    }

    pub fn set_player_keyring_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(832, v);
        self
    }

    pub fn set_player_keyring_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(834, v);
        self
    }

    pub fn set_player_keyring_slot_8(mut self, v: Guid) -> Self {
        self.set_guid(836, v);
        self
    }

    pub fn set_player_keyring_slot_9(mut self, v: Guid) -> Self {
        self.set_guid(838, v);
        self
    }

    pub fn set_player_keyring_slot_10(mut self, v: Guid) -> Self {
        self.set_guid(840, v);
        self
    }

    pub fn set_player_keyring_slot_11(mut self, v: Guid) -> Self {
        self.set_guid(842, v);
        self
    }

    pub fn set_player_keyring_slot_12(mut self, v: Guid) -> Self {
        self.set_guid(844, v);
        self
    }

    pub fn set_player_keyring_slot_13(mut self, v: Guid) -> Self {
        self.set_guid(846, v);
        self
    }

    pub fn set_player_keyring_slot_14(mut self, v: Guid) -> Self {
        self.set_guid(848, v);
        self
    }

    pub fn set_player_keyring_slot_15(mut self, v: Guid) -> Self {
        self.set_guid(850, v);
        self
    }

    pub fn set_player_keyring_slot_16(mut self, v: Guid) -> Self {
        self.set_guid(852, v);
        self
    }

    pub fn set_player_keyring_slot_17(mut self, v: Guid) -> Self {
        self.set_guid(854, v);
        self
    }

    pub fn set_player_keyring_slot_18(mut self, v: Guid) -> Self {
        self.set_guid(856, v);
        self
    }

    pub fn set_player_keyring_slot_19(mut self, v: Guid) -> Self {
        self.set_guid(858, v);
        self
    }

    pub fn set_player_keyring_slot_20(mut self, v: Guid) -> Self {
        self.set_guid(860, v);
        self
    }

    pub fn set_player_keyring_slot_21(mut self, v: Guid) -> Self {
        self.set_guid(862, v);
        self
    }

    pub fn set_player_keyring_slot_22(mut self, v: Guid) -> Self {
        self.set_guid(864, v);
        self
    }

    pub fn set_player_keyring_slot_23(mut self, v: Guid) -> Self {
        self.set_guid(866, v);
        self
    }

    pub fn set_player_keyring_slot_24(mut self, v: Guid) -> Self {
        self.set_guid(868, v);
        self
    }

    pub fn set_player_keyring_slot_25(mut self, v: Guid) -> Self {
        self.set_guid(870, v);
        self
    }

    pub fn set_player_keyring_slot_26(mut self, v: Guid) -> Self {
        self.set_guid(872, v);
        self
    }

    pub fn set_player_keyring_slot_27(mut self, v: Guid) -> Self {
        self.set_guid(874, v);
        self
    }

    pub fn set_player_keyring_slot_28(mut self, v: Guid) -> Self {
        self.set_guid(876, v);
        self
    }

    pub fn set_player_keyring_slot_29(mut self, v: Guid) -> Self {
        self.set_guid(878, v);
        self
    }

    pub fn set_player_keyring_slot_30(mut self, v: Guid) -> Self {
        self.set_guid(880, v);
        self
    }

    pub fn set_player_keyring_slot_31(mut self, v: Guid) -> Self {
        self.set_guid(882, v);
        self
    }

    pub fn set_player_keyring_slot_32(mut self, v: Guid) -> Self {
        self.set_guid(884, v);
        self
    }

    pub fn set_player_vanitypet_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(886, v);
        self
    }

    pub fn set_player_vanitypet_slot_2(mut self, v: Guid) -> Self {
        self.set_guid(888, v);
        self
    }

    pub fn set_player_vanitypet_slot_3(mut self, v: Guid) -> Self {
        self.set_guid(890, v);
        self
    }

    pub fn set_player_vanitypet_slot_4(mut self, v: Guid) -> Self {
        self.set_guid(892, v);
        self
    }

    pub fn set_player_vanitypet_slot_5(mut self, v: Guid) -> Self {
        self.set_guid(894, v);
        self
    }

    pub fn set_player_vanitypet_slot_6(mut self, v: Guid) -> Self {
        self.set_guid(896, v);
        self
    }

    pub fn set_player_vanitypet_slot_7(mut self, v: Guid) -> Self {
        self.set_guid(898, v);
        self
    }

    pub fn set_player_vanitypet_slot_8(mut self, v: Guid) -> Self {
        self.set_guid(900, v);
        self
    }

    pub fn set_player_vanitypet_slot_9(mut self, v: Guid) -> Self {
        self.set_guid(902, v);
        self
    }

    pub fn set_player_vanitypet_slot_10(mut self, v: Guid) -> Self {
        self.set_guid(904, v);
        self
    }

    pub fn set_player_vanitypet_slot_11(mut self, v: Guid) -> Self {
        self.set_guid(906, v);
        self
    }

    pub fn set_player_vanitypet_slot_12(mut self, v: Guid) -> Self {
        self.set_guid(908, v);
        self
    }

    pub fn set_player_vanitypet_slot_13(mut self, v: Guid) -> Self {
        self.set_guid(910, v);
        self
    }

    pub fn set_player_vanitypet_slot_14(mut self, v: Guid) -> Self {
        self.set_guid(912, v);
        self
    }

    pub fn set_player_vanitypet_slot_15(mut self, v: Guid) -> Self {
        self.set_guid(914, v);
        self
    }

    pub fn set_player_vanitypet_slot_16(mut self, v: Guid) -> Self {
        self.set_guid(916, v);
        self
    }

    pub fn set_player_vanitypet_slot_17(mut self, v: Guid) -> Self {
        self.set_guid(918, v);
        self
    }

    pub fn set_player_vanitypet_slot_18(mut self, v: Guid) -> Self {
        self.set_guid(920, v);
        self
    }

    pub fn set_player_farsight(mut self, v: Guid) -> Self {
        self.set_guid(922, v);
        self
    }

    pub fn set_player_known_titles(mut self, v: Guid) -> Self {
        self.set_guid(924, v);
        self
    }

    pub fn set_player_xp(mut self, v: i32) -> Self {
        self.set_int(926, v);
        self
    }

    pub fn set_player_next_level_xp(mut self, v: i32) -> Self {
        self.set_int(927, v);
        self
    }

    pub fn set_player_skill_info(mut self, skill_info: crate::tbc::SkillInfo, index: SkillInfoIndex) -> Self {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
        self
    }

    pub fn set_player_character_points1(mut self, v: i32) -> Self {
        self.set_int(1312, v);
        self
    }

    pub fn set_player_character_points2(mut self, v: i32) -> Self {
        self.set_int(1313, v);
        self
    }

    pub fn set_player_track_creatures(mut self, v: i32) -> Self {
        self.set_int(1314, v);
        self
    }

    pub fn set_player_track_resources(mut self, v: i32) -> Self {
        self.set_int(1315, v);
        self
    }

    pub fn set_player_block_percentage(mut self, v: f32) -> Self {
        self.set_float(1316, v);
        self
    }

    pub fn set_player_dodge_percentage(mut self, v: f32) -> Self {
        self.set_float(1317, v);
        self
    }

    pub fn set_player_parry_percentage(mut self, v: f32) -> Self {
        self.set_float(1318, v);
        self
    }

    pub fn set_player_expertise(mut self, v: i32) -> Self {
        self.set_int(1319, v);
        self
    }

    pub fn set_player_offhand_expertise(mut self, v: i32) -> Self {
        self.set_int(1320, v);
        self
    }

    pub fn set_player_crit_percentage(mut self, v: f32) -> Self {
        self.set_float(1321, v);
        self
    }

    pub fn set_player_ranged_crit_percentage(mut self, v: f32) -> Self {
        self.set_float(1322, v);
        self
    }

    pub fn set_player_offhand_crit_percentage(mut self, v: f32) -> Self {
        self.set_float(1323, v);
        self
    }

    pub fn set_player_spell_crit_percentage1(mut self, v: f32) -> Self {
        self.set_float(1324, v);
        self
    }

    pub fn set_player_shield_block(mut self, v: i32) -> Self {
        self.set_int(1331, v);
        self
    }

    pub fn set_player_explored_zones_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(1332, a, b, c, d);
        self
    }

    pub fn set_player_rest_state_experience(mut self, v: i32) -> Self {
        self.set_int(1460, v);
        self
    }

    pub fn set_player_coinage(mut self, v: i32) -> Self {
        self.set_int(1461, v);
        self
    }

    pub fn set_player_mod_damage_done_pos(mut self, v: i32) -> Self {
        self.set_int(1462, v);
        self
    }

    pub fn set_player_mod_damage_done_neg(mut self, v: i32) -> Self {
        self.set_int(1469, v);
        self
    }

    pub fn set_player_mod_damage_done_pct(mut self, v: i32) -> Self {
        self.set_int(1476, v);
        self
    }

    pub fn set_player_mod_healing_done_pos(mut self, v: i32) -> Self {
        self.set_int(1483, v);
        self
    }

    pub fn set_player_mod_target_resistance(mut self, v: i32) -> Self {
        self.set_int(1484, v);
        self
    }

    pub fn set_player_mod_target_physical_resistance(mut self, v: i32) -> Self {
        self.set_int(1485, v);
        self
    }

    pub fn set_player_features(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(1486, a, b, c, d);
        self
    }

    pub fn set_player_ammo_id(mut self, v: i32) -> Self {
        self.set_int(1487, v);
        self
    }

    pub fn set_player_self_res_spell(mut self, v: i32) -> Self {
        self.set_int(1488, v);
        self
    }

    pub fn set_player_pvp_medals(mut self, v: i32) -> Self {
        self.set_int(1489, v);
        self
    }

    pub fn set_player_buyback_price_1(mut self, v: i32) -> Self {
        self.set_int(1490, v);
        self
    }

    pub fn set_player_buyback_timestamp_1(mut self, v: i32) -> Self {
        self.set_int(1502, v);
        self
    }

    pub fn set_player_kills(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(1514, a, b);
        self
    }

    pub fn set_player_today_contribution(mut self, v: i32) -> Self {
        self.set_int(1515, v);
        self
    }

    pub fn set_player_yesterday_contribution(mut self, v: i32) -> Self {
        self.set_int(1516, v);
        self
    }

    pub fn set_player_lifetime_honorable_kills(mut self, v: i32) -> Self {
        self.set_int(1517, v);
        self
    }

    pub fn set_player_bytes2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(1518, a, b, c, d);
        self
    }

    pub fn set_player_watched_faction_index(mut self, v: i32) -> Self {
        self.set_int(1519, v);
        self
    }

    pub fn set_player_combat_rating_1(mut self, v: i32) -> Self {
        self.set_int(1520, v);
        self
    }

    pub fn set_player_arena_team_info_1_1(mut self, v: i32) -> Self {
        self.set_int(1544, v);
        self
    }

    pub fn set_player_honor_currency(mut self, v: i32) -> Self {
        self.set_int(1562, v);
        self
    }

    pub fn set_player_arena_currency(mut self, v: i32) -> Self {
        self.set_int(1563, v);
        self
    }

    pub fn set_player_mod_mana_regen(mut self, v: f32) -> Self {
        self.set_float(1564, v);
        self
    }

    pub fn set_player_mod_mana_regen_interrupt(mut self, v: f32) -> Self {
        self.set_float(1565, v);
        self
    }

    pub fn set_player_max_level(mut self, v: i32) -> Self {
        self.set_int(1566, v);
        self
    }

    pub fn set_player_daily_quests_1(mut self, v: i32) -> Self {
        self.set_int(1567, v);
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

    pub fn set_gameobject_rotation(mut self, v: f32) -> Self {
        self.set_float(10, v);
        self
    }

    pub fn set_gameobject_state(mut self, v: i32) -> Self {
        self.set_int(14, v);
        self
    }

    pub fn set_gameobject_pos_x(mut self, v: f32) -> Self {
        self.set_float(15, v);
        self
    }

    pub fn set_gameobject_pos_y(mut self, v: f32) -> Self {
        self.set_float(16, v);
        self
    }

    pub fn set_gameobject_pos_z(mut self, v: f32) -> Self {
        self.set_float(17, v);
        self
    }

    pub fn set_gameobject_facing(mut self, v: f32) -> Self {
        self.set_float(18, v);
        self
    }

    pub fn set_gameobject_dyn_flags(mut self, v: i32) -> Self {
        self.set_int(19, v);
        self
    }

    pub fn set_gameobject_faction(mut self, v: i32) -> Self {
        self.set_int(20, v);
        self
    }

    pub fn set_gameobject_type_id(mut self, v: i32) -> Self {
        self.set_int(21, v);
        self
    }

    pub fn set_gameobject_level(mut self, v: i32) -> Self {
        self.set_int(22, v);
        self
    }

    pub fn set_gameobject_artkit(mut self, v: i32) -> Self {
        self.set_int(23, v);
        self
    }

    pub fn set_gameobject_animprogress(mut self, v: i32) -> Self {
        self.set_int(24, v);
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

    pub fn set_dynamicobject_pos_x(mut self, v: f32) -> Self {
        self.set_float(11, v);
        self
    }

    pub fn set_dynamicobject_pos_y(mut self, v: f32) -> Self {
        self.set_float(12, v);
        self
    }

    pub fn set_dynamicobject_pos_z(mut self, v: f32) -> Self {
        self.set_float(13, v);
        self
    }

    pub fn set_dynamicobject_facing(mut self, v: f32) -> Self {
        self.set_float(14, v);
        self
    }

    pub fn set_dynamicobject_casttime(mut self, v: i32) -> Self {
        self.set_int(15, v);
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

    pub fn set_corpse_facing(mut self, v: f32) -> Self {
        self.set_float(10, v);
        self
    }

    pub fn set_corpse_pos_x(mut self, v: f32) -> Self {
        self.set_float(11, v);
        self
    }

    pub fn set_corpse_pos_y(mut self, v: f32) -> Self {
        self.set_float(12, v);
        self
    }

    pub fn set_corpse_pos_z(mut self, v: f32) -> Self {
        self.set_float(13, v);
        self
    }

    pub fn set_corpse_display_id(mut self, v: i32) -> Self {
        self.set_int(14, v);
        self
    }

    pub fn set_corpse_item(mut self, v: i32) -> Self {
        self.set_int(15, v);
        self
    }

    pub fn set_corpse_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(34, a, b, c, d);
        self
    }

    pub fn set_corpse_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(35, a, b, c, d);
        self
    }

    pub fn set_corpse_guild(mut self, v: i32) -> Self {
        self.set_int(36, v);
        self
    }

    pub fn set_corpse_flags(mut self, v: i32) -> Self {
        self.set_int(37, v);
        self
    }

    pub fn set_corpse_dynamic_flags(mut self, v: i32) -> Self {
        self.set_int(38, v);
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

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.set_int(55, v);
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.get_int(55)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.set_int(56, v);
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.get_int(56)
    }

    pub fn set_item_item_text_id(&mut self, v: i32) {
        self.set_int(57, v);
    }

    pub fn item_item_text_id(&self) -> Option<i32> {
        self.get_int(57)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.set_int(58, v);
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.get_int(58)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.set_int(59, v);
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.get_int(59)
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

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.set_int(55, v);
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.get_int(55)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.set_int(56, v);
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.get_int(56)
    }

    pub fn set_item_item_text_id(&mut self, v: i32) {
        self.set_int(57, v);
    }

    pub fn item_item_text_id(&self) -> Option<i32> {
        self.get_int(57)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.set_int(58, v);
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.get_int(58)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.set_int(59, v);
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.get_int(59)
    }

    pub fn set_container_num_slots(&mut self, v: i32) {
        self.set_int(60, v);
    }

    pub fn container_num_slots(&self) -> Option<i32> {
        self.get_int(60)
    }

    pub fn set_container_slot_1(&mut self, v: Guid) {
        self.set_guid(62, v);
    }

    pub fn container_slot_1(&self) -> Option<Guid> {
        self.get_guid(62)
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

    pub fn set_unit_charmedby(&mut self, v: Guid) {
        self.set_guid(10, v);
    }

    pub fn unit_charmedby(&self) -> Option<Guid> {
        self.get_guid(10)
    }

    pub fn set_unit_summonedby(&mut self, v: Guid) {
        self.set_guid(12, v);
    }

    pub fn unit_summonedby(&self) -> Option<Guid> {
        self.get_guid(12)
    }

    pub fn set_unit_createdby(&mut self, v: Guid) {
        self.set_guid(14, v);
    }

    pub fn unit_createdby(&self) -> Option<Guid> {
        self.get_guid(14)
    }

    pub fn set_unit_target(&mut self, v: Guid) {
        self.set_guid(16, v);
    }

    pub fn unit_target(&self) -> Option<Guid> {
        self.get_guid(16)
    }

    pub fn set_unit_persuaded(&mut self, v: Guid) {
        self.set_guid(18, v);
    }

    pub fn unit_persuaded(&self) -> Option<Guid> {
        self.get_guid(18)
    }

    pub fn set_unit_channel_object(&mut self, v: Guid) {
        self.set_guid(20, v);
    }

    pub fn unit_channel_object(&self) -> Option<Guid> {
        self.get_guid(20)
    }

    pub fn set_unit_health(&mut self, v: i32) {
        self.set_int(22, v);
    }

    pub fn unit_health(&self) -> Option<i32> {
        self.get_int(22)
    }

    pub fn set_unit_power1(&mut self, v: i32) {
        self.set_int(23, v);
    }

    pub fn unit_power1(&self) -> Option<i32> {
        self.get_int(23)
    }

    pub fn set_unit_power2(&mut self, v: i32) {
        self.set_int(24, v);
    }

    pub fn unit_power2(&self) -> Option<i32> {
        self.get_int(24)
    }

    pub fn set_unit_power3(&mut self, v: i32) {
        self.set_int(25, v);
    }

    pub fn unit_power3(&self) -> Option<i32> {
        self.get_int(25)
    }

    pub fn set_unit_power4(&mut self, v: i32) {
        self.set_int(26, v);
    }

    pub fn unit_power4(&self) -> Option<i32> {
        self.get_int(26)
    }

    pub fn set_unit_power5(&mut self, v: i32) {
        self.set_int(27, v);
    }

    pub fn unit_power5(&self) -> Option<i32> {
        self.get_int(27)
    }

    pub fn set_unit_maxhealth(&mut self, v: i32) {
        self.set_int(28, v);
    }

    pub fn unit_maxhealth(&self) -> Option<i32> {
        self.get_int(28)
    }

    pub fn set_unit_maxpower1(&mut self, v: i32) {
        self.set_int(29, v);
    }

    pub fn unit_maxpower1(&self) -> Option<i32> {
        self.get_int(29)
    }

    pub fn set_unit_maxpower2(&mut self, v: i32) {
        self.set_int(30, v);
    }

    pub fn unit_maxpower2(&self) -> Option<i32> {
        self.get_int(30)
    }

    pub fn set_unit_maxpower3(&mut self, v: i32) {
        self.set_int(31, v);
    }

    pub fn unit_maxpower3(&self) -> Option<i32> {
        self.get_int(31)
    }

    pub fn set_unit_maxpower4(&mut self, v: i32) {
        self.set_int(32, v);
    }

    pub fn unit_maxpower4(&self) -> Option<i32> {
        self.get_int(32)
    }

    pub fn set_unit_maxpower5(&mut self, v: i32) {
        self.set_int(33, v);
    }

    pub fn unit_maxpower5(&self) -> Option<i32> {
        self.get_int(33)
    }

    pub fn set_unit_level(&mut self, v: i32) {
        self.set_int(34, v);
    }

    pub fn unit_level(&self) -> Option<i32> {
        self.get_int(34)
    }

    pub fn set_unit_factiontemplate(&mut self, v: i32) {
        self.set_int(35, v);
    }

    pub fn unit_factiontemplate(&self) -> Option<i32> {
        self.get_int(35)
    }

    pub fn set_unit_bytes_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.set_bytes(36, race.as_int(), class.as_int(), gender.as_int(), power.as_int());
    }

    pub fn unit_bytes_0(&self) -> Option<(Race, Class, Gender, Power)> {
        self.get_bytes(36).map(|(race, class, gender, power)| {
            (race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap())
        })
    }

    pub fn set_unit_virtual_item_slot_display(&mut self, v: i32) {
        self.set_int(37, v);
    }

    pub fn unit_virtual_item_slot_display(&self) -> Option<i32> {
        self.get_int(37)
    }

    pub fn set_unit_virtual_item_info(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(40, a, b, c, d);
    }

    pub fn unit_virtual_item_info(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(40)
    }

    pub fn set_unit_flags(&mut self, v: i32) {
        self.set_int(46, v);
    }

    pub fn unit_flags(&self) -> Option<i32> {
        self.get_int(46)
    }

    pub fn set_unit_flags_2(&mut self, v: i32) {
        self.set_int(47, v);
    }

    pub fn unit_flags_2(&self) -> Option<i32> {
        self.get_int(47)
    }

    pub fn set_unit_aura(&mut self, v: i32) {
        self.set_int(48, v);
    }

    pub fn unit_aura(&self) -> Option<i32> {
        self.get_int(48)
    }

    pub fn set_unit_auraflags(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(104, a, b, c, d);
    }

    pub fn unit_auraflags(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(104)
    }

    pub fn set_unit_auralevels(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(118, a, b, c, d);
    }

    pub fn unit_auralevels(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(118)
    }

    pub fn set_unit_auraapplications(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(132, a, b, c, d);
    }

    pub fn unit_auraapplications(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(132)
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.set_int(146, v);
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.get_int(146)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.set_int(147, v);
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.get_int(147)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.set_int(149, v);
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.get_int(149)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.set_float(150, v);
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.get_float(150)
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.set_float(151, v);
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.get_float(151)
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.set_int(152, v);
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.get_int(152)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.set_int(153, v);
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.get_int(153)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.set_int(154, v);
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.get_int(154)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.set_float(155, v);
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.get_float(155)
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.set_float(156, v);
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.get_float(156)
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.set_float(157, v);
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.get_float(157)
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.set_float(158, v);
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.get_float(158)
    }

    pub fn set_unit_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(159, a, b, c, d);
    }

    pub fn unit_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(159)
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.set_int(160, v);
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.get_int(160)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.set_int(161, v);
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.get_int(161)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.set_int(162, v);
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.get_int(162)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.set_int(163, v);
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.get_int(163)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.set_int(164, v);
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.get_int(164)
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.set_int(165, v);
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.get_int(165)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.set_float(166, v);
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.get_float(166)
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.set_int(167, v);
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.get_int(167)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.set_int(168, v);
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.get_int(168)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.set_int(169, v);
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.get_int(169)
    }

    pub fn set_unit_training_points(&mut self, a: u16, b: u16) {
        self.set_shorts(170, a, b);
    }

    pub fn unit_training_points(&self) -> Option<(u16, u16)> {
        self.get_shorts(170)
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.set_int(171, v);
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.get_int(171)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.set_int(172, v);
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.get_int(172)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.set_int(173, v);
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.get_int(173)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.set_int(174, v);
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.get_int(174)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.set_int(175, v);
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.get_int(175)
    }

    pub fn set_unit_posstat1(&mut self, v: i32) {
        self.set_int(177, v);
    }

    pub fn unit_posstat1(&self) -> Option<i32> {
        self.get_int(177)
    }

    pub fn set_unit_posstat2(&mut self, v: i32) {
        self.set_int(178, v);
    }

    pub fn unit_posstat2(&self) -> Option<i32> {
        self.get_int(178)
    }

    pub fn set_unit_posstat3(&mut self, v: i32) {
        self.set_int(179, v);
    }

    pub fn unit_posstat3(&self) -> Option<i32> {
        self.get_int(179)
    }

    pub fn set_unit_negstat1(&mut self, v: i32) {
        self.set_int(182, v);
    }

    pub fn unit_negstat1(&self) -> Option<i32> {
        self.get_int(182)
    }

    pub fn set_unit_negstat2(&mut self, v: i32) {
        self.set_int(183, v);
    }

    pub fn unit_negstat2(&self) -> Option<i32> {
        self.get_int(183)
    }

    pub fn set_unit_negstat3(&mut self, v: i32) {
        self.set_int(184, v);
    }

    pub fn unit_negstat3(&self) -> Option<i32> {
        self.get_int(184)
    }

    pub fn set_unit_resistances(&mut self, v: i32) {
        self.set_int(186, v);
    }

    pub fn unit_resistances(&self) -> Option<i32> {
        self.get_int(186)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.set_int(207, v);
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.get_int(207)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.set_int(208, v);
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.get_int(208)
    }

    pub fn set_unit_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(209, a, b, c, d);
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(209)
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.set_int(210, v);
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.get_int(210)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(211, a, b);
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(211)
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(212, v);
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(212)
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.set_int(213, v);
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.get_int(213)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(214, a, b);
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(214)
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(215, v);
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(215)
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.set_float(216, v);
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.get_float(216)
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.set_float(217, v);
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.get_float(217)
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.set_int(218, v);
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.get_int(218)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.set_float(225, v);
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.get_float(225)
    }

    pub fn set_unit_maxhealthmodifier(&mut self, v: f32) {
        self.set_float(232, v);
    }

    pub fn unit_maxhealthmodifier(&self) -> Option<f32> {
        self.get_float(232)
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

    pub fn set_unit_charmedby(&mut self, v: Guid) {
        self.set_guid(10, v);
    }

    pub fn unit_charmedby(&self) -> Option<Guid> {
        self.get_guid(10)
    }

    pub fn set_unit_summonedby(&mut self, v: Guid) {
        self.set_guid(12, v);
    }

    pub fn unit_summonedby(&self) -> Option<Guid> {
        self.get_guid(12)
    }

    pub fn set_unit_createdby(&mut self, v: Guid) {
        self.set_guid(14, v);
    }

    pub fn unit_createdby(&self) -> Option<Guid> {
        self.get_guid(14)
    }

    pub fn set_unit_target(&mut self, v: Guid) {
        self.set_guid(16, v);
    }

    pub fn unit_target(&self) -> Option<Guid> {
        self.get_guid(16)
    }

    pub fn set_unit_persuaded(&mut self, v: Guid) {
        self.set_guid(18, v);
    }

    pub fn unit_persuaded(&self) -> Option<Guid> {
        self.get_guid(18)
    }

    pub fn set_unit_channel_object(&mut self, v: Guid) {
        self.set_guid(20, v);
    }

    pub fn unit_channel_object(&self) -> Option<Guid> {
        self.get_guid(20)
    }

    pub fn set_unit_health(&mut self, v: i32) {
        self.set_int(22, v);
    }

    pub fn unit_health(&self) -> Option<i32> {
        self.get_int(22)
    }

    pub fn set_unit_power1(&mut self, v: i32) {
        self.set_int(23, v);
    }

    pub fn unit_power1(&self) -> Option<i32> {
        self.get_int(23)
    }

    pub fn set_unit_power2(&mut self, v: i32) {
        self.set_int(24, v);
    }

    pub fn unit_power2(&self) -> Option<i32> {
        self.get_int(24)
    }

    pub fn set_unit_power3(&mut self, v: i32) {
        self.set_int(25, v);
    }

    pub fn unit_power3(&self) -> Option<i32> {
        self.get_int(25)
    }

    pub fn set_unit_power4(&mut self, v: i32) {
        self.set_int(26, v);
    }

    pub fn unit_power4(&self) -> Option<i32> {
        self.get_int(26)
    }

    pub fn set_unit_power5(&mut self, v: i32) {
        self.set_int(27, v);
    }

    pub fn unit_power5(&self) -> Option<i32> {
        self.get_int(27)
    }

    pub fn set_unit_maxhealth(&mut self, v: i32) {
        self.set_int(28, v);
    }

    pub fn unit_maxhealth(&self) -> Option<i32> {
        self.get_int(28)
    }

    pub fn set_unit_maxpower1(&mut self, v: i32) {
        self.set_int(29, v);
    }

    pub fn unit_maxpower1(&self) -> Option<i32> {
        self.get_int(29)
    }

    pub fn set_unit_maxpower2(&mut self, v: i32) {
        self.set_int(30, v);
    }

    pub fn unit_maxpower2(&self) -> Option<i32> {
        self.get_int(30)
    }

    pub fn set_unit_maxpower3(&mut self, v: i32) {
        self.set_int(31, v);
    }

    pub fn unit_maxpower3(&self) -> Option<i32> {
        self.get_int(31)
    }

    pub fn set_unit_maxpower4(&mut self, v: i32) {
        self.set_int(32, v);
    }

    pub fn unit_maxpower4(&self) -> Option<i32> {
        self.get_int(32)
    }

    pub fn set_unit_maxpower5(&mut self, v: i32) {
        self.set_int(33, v);
    }

    pub fn unit_maxpower5(&self) -> Option<i32> {
        self.get_int(33)
    }

    pub fn set_unit_level(&mut self, v: i32) {
        self.set_int(34, v);
    }

    pub fn unit_level(&self) -> Option<i32> {
        self.get_int(34)
    }

    pub fn set_unit_factiontemplate(&mut self, v: i32) {
        self.set_int(35, v);
    }

    pub fn unit_factiontemplate(&self) -> Option<i32> {
        self.get_int(35)
    }

    pub fn set_unit_bytes_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.set_bytes(36, race.as_int(), class.as_int(), gender.as_int(), power.as_int());
    }

    pub fn unit_bytes_0(&self) -> Option<(Race, Class, Gender, Power)> {
        self.get_bytes(36).map(|(race, class, gender, power)| {
            (race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap())
        })
    }

    pub fn set_unit_virtual_item_slot_display(&mut self, v: i32) {
        self.set_int(37, v);
    }

    pub fn unit_virtual_item_slot_display(&self) -> Option<i32> {
        self.get_int(37)
    }

    pub fn set_unit_virtual_item_info(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(40, a, b, c, d);
    }

    pub fn unit_virtual_item_info(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(40)
    }

    pub fn set_unit_flags(&mut self, v: i32) {
        self.set_int(46, v);
    }

    pub fn unit_flags(&self) -> Option<i32> {
        self.get_int(46)
    }

    pub fn set_unit_flags_2(&mut self, v: i32) {
        self.set_int(47, v);
    }

    pub fn unit_flags_2(&self) -> Option<i32> {
        self.get_int(47)
    }

    pub fn set_unit_aura(&mut self, v: i32) {
        self.set_int(48, v);
    }

    pub fn unit_aura(&self) -> Option<i32> {
        self.get_int(48)
    }

    pub fn set_unit_auraflags(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(104, a, b, c, d);
    }

    pub fn unit_auraflags(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(104)
    }

    pub fn set_unit_auralevels(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(118, a, b, c, d);
    }

    pub fn unit_auralevels(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(118)
    }

    pub fn set_unit_auraapplications(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(132, a, b, c, d);
    }

    pub fn unit_auraapplications(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(132)
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.set_int(146, v);
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.get_int(146)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.set_int(147, v);
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.get_int(147)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.set_int(149, v);
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.get_int(149)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.set_float(150, v);
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.get_float(150)
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.set_float(151, v);
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.get_float(151)
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.set_int(152, v);
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.get_int(152)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.set_int(153, v);
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.get_int(153)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.set_int(154, v);
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.get_int(154)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.set_float(155, v);
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.get_float(155)
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.set_float(156, v);
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.get_float(156)
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.set_float(157, v);
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.get_float(157)
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.set_float(158, v);
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.get_float(158)
    }

    pub fn set_unit_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(159, a, b, c, d);
    }

    pub fn unit_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(159)
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.set_int(160, v);
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.get_int(160)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.set_int(161, v);
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.get_int(161)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.set_int(162, v);
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.get_int(162)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.set_int(163, v);
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.get_int(163)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.set_int(164, v);
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.get_int(164)
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.set_int(165, v);
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.get_int(165)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.set_float(166, v);
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.get_float(166)
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.set_int(167, v);
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.get_int(167)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.set_int(168, v);
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.get_int(168)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.set_int(169, v);
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.get_int(169)
    }

    pub fn set_unit_training_points(&mut self, a: u16, b: u16) {
        self.set_shorts(170, a, b);
    }

    pub fn unit_training_points(&self) -> Option<(u16, u16)> {
        self.get_shorts(170)
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.set_int(171, v);
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.get_int(171)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.set_int(172, v);
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.get_int(172)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.set_int(173, v);
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.get_int(173)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.set_int(174, v);
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.get_int(174)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.set_int(175, v);
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.get_int(175)
    }

    pub fn set_player_posstat0(&mut self, v: i32) {
        self.set_int(176, v);
    }

    pub fn player_posstat0(&self) -> Option<i32> {
        self.get_int(176)
    }

    pub fn set_unit_posstat1(&mut self, v: i32) {
        self.set_int(177, v);
    }

    pub fn unit_posstat1(&self) -> Option<i32> {
        self.get_int(177)
    }

    pub fn set_unit_posstat2(&mut self, v: i32) {
        self.set_int(178, v);
    }

    pub fn unit_posstat2(&self) -> Option<i32> {
        self.get_int(178)
    }

    pub fn set_unit_posstat3(&mut self, v: i32) {
        self.set_int(179, v);
    }

    pub fn unit_posstat3(&self) -> Option<i32> {
        self.get_int(179)
    }

    pub fn set_player_posstat4(&mut self, v: i32) {
        self.set_int(180, v);
    }

    pub fn player_posstat4(&self) -> Option<i32> {
        self.get_int(180)
    }

    pub fn set_player_negstat0(&mut self, v: i32) {
        self.set_int(181, v);
    }

    pub fn player_negstat0(&self) -> Option<i32> {
        self.get_int(181)
    }

    pub fn set_unit_negstat1(&mut self, v: i32) {
        self.set_int(182, v);
    }

    pub fn unit_negstat1(&self) -> Option<i32> {
        self.get_int(182)
    }

    pub fn set_unit_negstat2(&mut self, v: i32) {
        self.set_int(183, v);
    }

    pub fn unit_negstat2(&self) -> Option<i32> {
        self.get_int(183)
    }

    pub fn set_unit_negstat3(&mut self, v: i32) {
        self.set_int(184, v);
    }

    pub fn unit_negstat3(&self) -> Option<i32> {
        self.get_int(184)
    }

    pub fn set_player_negstat4(&mut self, v: i32) {
        self.set_int(185, v);
    }

    pub fn player_negstat4(&self) -> Option<i32> {
        self.get_int(185)
    }

    pub fn set_unit_resistances(&mut self, v: i32) {
        self.set_int(186, v);
    }

    pub fn unit_resistances(&self) -> Option<i32> {
        self.get_int(186)
    }

    pub fn set_player_resistancebuffmodspositive(&mut self, v: i32) {
        self.set_int(193, v);
    }

    pub fn player_resistancebuffmodspositive(&self) -> Option<i32> {
        self.get_int(193)
    }

    pub fn set_player_resistancebuffmodsnegative(&mut self, v: i32) {
        self.set_int(200, v);
    }

    pub fn player_resistancebuffmodsnegative(&self) -> Option<i32> {
        self.get_int(200)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.set_int(207, v);
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.get_int(207)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.set_int(208, v);
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.get_int(208)
    }

    pub fn set_unit_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(209, a, b, c, d);
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(209)
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.set_int(210, v);
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.get_int(210)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(211, a, b);
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(211)
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(212, v);
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(212)
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.set_int(213, v);
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.get_int(213)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(214, a, b);
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(214)
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(215, v);
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(215)
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.set_float(216, v);
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.get_float(216)
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.set_float(217, v);
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.get_float(217)
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.set_int(218, v);
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.get_int(218)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.set_float(225, v);
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.get_float(225)
    }

    pub fn set_unit_maxhealthmodifier(&mut self, v: f32) {
        self.set_float(232, v);
    }

    pub fn unit_maxhealthmodifier(&self) -> Option<f32> {
        self.get_float(232)
    }

    pub fn set_player_duel_arbiter(&mut self, v: Guid) {
        self.set_guid(234, v);
    }

    pub fn player_duel_arbiter(&self) -> Option<Guid> {
        self.get_guid(234)
    }

    pub fn set_player_flags(&mut self, v: i32) {
        self.set_int(236, v);
    }

    pub fn player_flags(&self) -> Option<i32> {
        self.get_int(236)
    }

    pub fn set_player_guildid(&mut self, v: i32) {
        self.set_int(237, v);
    }

    pub fn player_guildid(&self) -> Option<i32> {
        self.get_int(237)
    }

    pub fn set_player_guildrank(&mut self, v: i32) {
        self.set_int(238, v);
    }

    pub fn player_guildrank(&self) -> Option<i32> {
        self.get_int(238)
    }

    pub fn set_player_field_bytes(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(239, a, b, c, d);
    }

    pub fn player_field_bytes(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(239)
    }

    pub fn set_player_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(240, a, b, c, d);
    }

    pub fn player_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(240)
    }

    pub fn set_player_bytes_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(241, a, b, c, d);
    }

    pub fn player_bytes_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(241)
    }

    pub fn set_player_duel_team(&mut self, v: i32) {
        self.set_int(242, v);
    }

    pub fn player_duel_team(&self) -> Option<i32> {
        self.get_int(242)
    }

    pub fn set_player_guild_timestamp(&mut self, v: i32) {
        self.set_int(243, v);
    }

    pub fn player_guild_timestamp(&self) -> Option<i32> {
        self.get_int(243)
    }

    pub fn set_player_quest_log_1_1(&mut self, v: i32) {
        self.set_int(244, v);
    }

    pub fn player_quest_log_1_1(&self) -> Option<i32> {
        self.get_int(244)
    }

    pub fn set_player_quest_log_1_2(&mut self, v: i32) {
        self.set_int(245, v);
    }

    pub fn player_quest_log_1_2(&self) -> Option<i32> {
        self.get_int(245)
    }

    pub fn set_player_quest_log_1_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(246, a, b, c, d);
    }

    pub fn player_quest_log_1_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(246)
    }

    pub fn set_player_quest_log_1_4(&mut self, v: i32) {
        self.set_int(247, v);
    }

    pub fn player_quest_log_1_4(&self) -> Option<i32> {
        self.get_int(247)
    }

    pub fn set_player_quest_log_2_1(&mut self, v: i32) {
        self.set_int(248, v);
    }

    pub fn player_quest_log_2_1(&self) -> Option<i32> {
        self.get_int(248)
    }

    pub fn set_player_quest_log_2_2(&mut self, v: i32) {
        self.set_int(249, v);
    }

    pub fn player_quest_log_2_2(&self) -> Option<i32> {
        self.get_int(249)
    }

    pub fn set_player_quest_log_2_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(250, a, b, c, d);
    }

    pub fn player_quest_log_2_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(250)
    }

    pub fn set_player_quest_log_2_4(&mut self, v: i32) {
        self.set_int(251, v);
    }

    pub fn player_quest_log_2_4(&self) -> Option<i32> {
        self.get_int(251)
    }

    pub fn set_player_quest_log_3_1(&mut self, v: i32) {
        self.set_int(252, v);
    }

    pub fn player_quest_log_3_1(&self) -> Option<i32> {
        self.get_int(252)
    }

    pub fn set_player_quest_log_3_2(&mut self, v: i32) {
        self.set_int(253, v);
    }

    pub fn player_quest_log_3_2(&self) -> Option<i32> {
        self.get_int(253)
    }

    pub fn set_player_quest_log_3_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(254, a, b, c, d);
    }

    pub fn player_quest_log_3_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(254)
    }

    pub fn set_player_quest_log_3_4(&mut self, v: i32) {
        self.set_int(255, v);
    }

    pub fn player_quest_log_3_4(&self) -> Option<i32> {
        self.get_int(255)
    }

    pub fn set_player_quest_log_4_1(&mut self, v: i32) {
        self.set_int(256, v);
    }

    pub fn player_quest_log_4_1(&self) -> Option<i32> {
        self.get_int(256)
    }

    pub fn set_player_quest_log_4_2(&mut self, v: i32) {
        self.set_int(257, v);
    }

    pub fn player_quest_log_4_2(&self) -> Option<i32> {
        self.get_int(257)
    }

    pub fn set_player_quest_log_4_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(258, a, b, c, d);
    }

    pub fn player_quest_log_4_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(258)
    }

    pub fn set_player_quest_log_4_4(&mut self, v: i32) {
        self.set_int(259, v);
    }

    pub fn player_quest_log_4_4(&self) -> Option<i32> {
        self.get_int(259)
    }

    pub fn set_player_quest_log_5_1(&mut self, v: i32) {
        self.set_int(260, v);
    }

    pub fn player_quest_log_5_1(&self) -> Option<i32> {
        self.get_int(260)
    }

    pub fn set_player_quest_log_5_2(&mut self, v: i32) {
        self.set_int(261, v);
    }

    pub fn player_quest_log_5_2(&self) -> Option<i32> {
        self.get_int(261)
    }

    pub fn set_player_quest_log_5_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(262, a, b, c, d);
    }

    pub fn player_quest_log_5_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(262)
    }

    pub fn set_player_quest_log_5_4(&mut self, v: i32) {
        self.set_int(263, v);
    }

    pub fn player_quest_log_5_4(&self) -> Option<i32> {
        self.get_int(263)
    }

    pub fn set_player_quest_log_6_1(&mut self, v: i32) {
        self.set_int(264, v);
    }

    pub fn player_quest_log_6_1(&self) -> Option<i32> {
        self.get_int(264)
    }

    pub fn set_player_quest_log_6_2(&mut self, v: i32) {
        self.set_int(265, v);
    }

    pub fn player_quest_log_6_2(&self) -> Option<i32> {
        self.get_int(265)
    }

    pub fn set_player_quest_log_6_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(266, a, b, c, d);
    }

    pub fn player_quest_log_6_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(266)
    }

    pub fn set_player_quest_log_6_4(&mut self, v: i32) {
        self.set_int(267, v);
    }

    pub fn player_quest_log_6_4(&self) -> Option<i32> {
        self.get_int(267)
    }

    pub fn set_player_quest_log_7_1(&mut self, v: i32) {
        self.set_int(268, v);
    }

    pub fn player_quest_log_7_1(&self) -> Option<i32> {
        self.get_int(268)
    }

    pub fn set_player_quest_log_7_2(&mut self, v: i32) {
        self.set_int(269, v);
    }

    pub fn player_quest_log_7_2(&self) -> Option<i32> {
        self.get_int(269)
    }

    pub fn set_player_quest_log_7_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(270, a, b, c, d);
    }

    pub fn player_quest_log_7_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(270)
    }

    pub fn set_player_quest_log_7_4(&mut self, v: i32) {
        self.set_int(271, v);
    }

    pub fn player_quest_log_7_4(&self) -> Option<i32> {
        self.get_int(271)
    }

    pub fn set_player_quest_log_8_1(&mut self, v: i32) {
        self.set_int(272, v);
    }

    pub fn player_quest_log_8_1(&self) -> Option<i32> {
        self.get_int(272)
    }

    pub fn set_player_quest_log_8_2(&mut self, v: i32) {
        self.set_int(273, v);
    }

    pub fn player_quest_log_8_2(&self) -> Option<i32> {
        self.get_int(273)
    }

    pub fn set_player_quest_log_8_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(274, a, b, c, d);
    }

    pub fn player_quest_log_8_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(274)
    }

    pub fn set_player_quest_log_8_4(&mut self, v: i32) {
        self.set_int(275, v);
    }

    pub fn player_quest_log_8_4(&self) -> Option<i32> {
        self.get_int(275)
    }

    pub fn set_player_quest_log_9_1(&mut self, v: i32) {
        self.set_int(276, v);
    }

    pub fn player_quest_log_9_1(&self) -> Option<i32> {
        self.get_int(276)
    }

    pub fn set_player_quest_log_9_2(&mut self, v: i32) {
        self.set_int(277, v);
    }

    pub fn player_quest_log_9_2(&self) -> Option<i32> {
        self.get_int(277)
    }

    pub fn set_player_quest_log_9_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(278, a, b, c, d);
    }

    pub fn player_quest_log_9_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(278)
    }

    pub fn set_player_quest_log_9_4(&mut self, v: i32) {
        self.set_int(279, v);
    }

    pub fn player_quest_log_9_4(&self) -> Option<i32> {
        self.get_int(279)
    }

    pub fn set_player_quest_log_10_1(&mut self, v: i32) {
        self.set_int(280, v);
    }

    pub fn player_quest_log_10_1(&self) -> Option<i32> {
        self.get_int(280)
    }

    pub fn set_player_quest_log_10_2(&mut self, v: i32) {
        self.set_int(281, v);
    }

    pub fn player_quest_log_10_2(&self) -> Option<i32> {
        self.get_int(281)
    }

    pub fn set_player_quest_log_10_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(282, a, b, c, d);
    }

    pub fn player_quest_log_10_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(282)
    }

    pub fn set_player_quest_log_10_4(&mut self, v: i32) {
        self.set_int(283, v);
    }

    pub fn player_quest_log_10_4(&self) -> Option<i32> {
        self.get_int(283)
    }

    pub fn set_player_quest_log_11_1(&mut self, v: i32) {
        self.set_int(284, v);
    }

    pub fn player_quest_log_11_1(&self) -> Option<i32> {
        self.get_int(284)
    }

    pub fn set_player_quest_log_11_2(&mut self, v: i32) {
        self.set_int(285, v);
    }

    pub fn player_quest_log_11_2(&self) -> Option<i32> {
        self.get_int(285)
    }

    pub fn set_player_quest_log_11_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(286, a, b, c, d);
    }

    pub fn player_quest_log_11_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(286)
    }

    pub fn set_player_quest_log_11_4(&mut self, v: i32) {
        self.set_int(287, v);
    }

    pub fn player_quest_log_11_4(&self) -> Option<i32> {
        self.get_int(287)
    }

    pub fn set_player_quest_log_12_1(&mut self, v: i32) {
        self.set_int(288, v);
    }

    pub fn player_quest_log_12_1(&self) -> Option<i32> {
        self.get_int(288)
    }

    pub fn set_player_quest_log_12_2(&mut self, v: i32) {
        self.set_int(289, v);
    }

    pub fn player_quest_log_12_2(&self) -> Option<i32> {
        self.get_int(289)
    }

    pub fn set_player_quest_log_12_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(290, a, b, c, d);
    }

    pub fn player_quest_log_12_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(290)
    }

    pub fn set_player_quest_log_12_4(&mut self, v: i32) {
        self.set_int(291, v);
    }

    pub fn player_quest_log_12_4(&self) -> Option<i32> {
        self.get_int(291)
    }

    pub fn set_player_quest_log_13_1(&mut self, v: i32) {
        self.set_int(292, v);
    }

    pub fn player_quest_log_13_1(&self) -> Option<i32> {
        self.get_int(292)
    }

    pub fn set_player_quest_log_13_2(&mut self, v: i32) {
        self.set_int(293, v);
    }

    pub fn player_quest_log_13_2(&self) -> Option<i32> {
        self.get_int(293)
    }

    pub fn set_player_quest_log_13_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(294, a, b, c, d);
    }

    pub fn player_quest_log_13_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(294)
    }

    pub fn set_player_quest_log_13_4(&mut self, v: i32) {
        self.set_int(295, v);
    }

    pub fn player_quest_log_13_4(&self) -> Option<i32> {
        self.get_int(295)
    }

    pub fn set_player_quest_log_14_1(&mut self, v: i32) {
        self.set_int(296, v);
    }

    pub fn player_quest_log_14_1(&self) -> Option<i32> {
        self.get_int(296)
    }

    pub fn set_player_quest_log_14_2(&mut self, v: i32) {
        self.set_int(297, v);
    }

    pub fn player_quest_log_14_2(&self) -> Option<i32> {
        self.get_int(297)
    }

    pub fn set_player_quest_log_14_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(298, a, b, c, d);
    }

    pub fn player_quest_log_14_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(298)
    }

    pub fn set_player_quest_log_14_4(&mut self, v: i32) {
        self.set_int(299, v);
    }

    pub fn player_quest_log_14_4(&self) -> Option<i32> {
        self.get_int(299)
    }

    pub fn set_player_quest_log_15_1(&mut self, v: i32) {
        self.set_int(300, v);
    }

    pub fn player_quest_log_15_1(&self) -> Option<i32> {
        self.get_int(300)
    }

    pub fn set_player_quest_log_15_2(&mut self, v: i32) {
        self.set_int(301, v);
    }

    pub fn player_quest_log_15_2(&self) -> Option<i32> {
        self.get_int(301)
    }

    pub fn set_player_quest_log_15_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(302, a, b, c, d);
    }

    pub fn player_quest_log_15_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(302)
    }

    pub fn set_player_quest_log_15_4(&mut self, v: i32) {
        self.set_int(303, v);
    }

    pub fn player_quest_log_15_4(&self) -> Option<i32> {
        self.get_int(303)
    }

    pub fn set_player_quest_log_16_1(&mut self, v: i32) {
        self.set_int(304, v);
    }

    pub fn player_quest_log_16_1(&self) -> Option<i32> {
        self.get_int(304)
    }

    pub fn set_player_quest_log_16_2(&mut self, v: i32) {
        self.set_int(305, v);
    }

    pub fn player_quest_log_16_2(&self) -> Option<i32> {
        self.get_int(305)
    }

    pub fn set_player_quest_log_16_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(306, a, b, c, d);
    }

    pub fn player_quest_log_16_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(306)
    }

    pub fn set_player_quest_log_16_4(&mut self, v: i32) {
        self.set_int(307, v);
    }

    pub fn player_quest_log_16_4(&self) -> Option<i32> {
        self.get_int(307)
    }

    pub fn set_player_quest_log_17_1(&mut self, v: i32) {
        self.set_int(308, v);
    }

    pub fn player_quest_log_17_1(&self) -> Option<i32> {
        self.get_int(308)
    }

    pub fn set_player_quest_log_17_2(&mut self, v: i32) {
        self.set_int(309, v);
    }

    pub fn player_quest_log_17_2(&self) -> Option<i32> {
        self.get_int(309)
    }

    pub fn set_player_quest_log_17_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(310, a, b, c, d);
    }

    pub fn player_quest_log_17_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(310)
    }

    pub fn set_player_quest_log_17_4(&mut self, v: i32) {
        self.set_int(311, v);
    }

    pub fn player_quest_log_17_4(&self) -> Option<i32> {
        self.get_int(311)
    }

    pub fn set_player_quest_log_18_1(&mut self, v: i32) {
        self.set_int(312, v);
    }

    pub fn player_quest_log_18_1(&self) -> Option<i32> {
        self.get_int(312)
    }

    pub fn set_player_quest_log_18_2(&mut self, v: i32) {
        self.set_int(313, v);
    }

    pub fn player_quest_log_18_2(&self) -> Option<i32> {
        self.get_int(313)
    }

    pub fn set_player_quest_log_18_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(314, a, b, c, d);
    }

    pub fn player_quest_log_18_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(314)
    }

    pub fn set_player_quest_log_18_4(&mut self, v: i32) {
        self.set_int(315, v);
    }

    pub fn player_quest_log_18_4(&self) -> Option<i32> {
        self.get_int(315)
    }

    pub fn set_player_quest_log_19_1(&mut self, v: i32) {
        self.set_int(316, v);
    }

    pub fn player_quest_log_19_1(&self) -> Option<i32> {
        self.get_int(316)
    }

    pub fn set_player_quest_log_19_2(&mut self, v: i32) {
        self.set_int(317, v);
    }

    pub fn player_quest_log_19_2(&self) -> Option<i32> {
        self.get_int(317)
    }

    pub fn set_player_quest_log_19_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(318, a, b, c, d);
    }

    pub fn player_quest_log_19_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(318)
    }

    pub fn set_player_quest_log_19_4(&mut self, v: i32) {
        self.set_int(319, v);
    }

    pub fn player_quest_log_19_4(&self) -> Option<i32> {
        self.get_int(319)
    }

    pub fn set_player_quest_log_20_1(&mut self, v: i32) {
        self.set_int(320, v);
    }

    pub fn player_quest_log_20_1(&self) -> Option<i32> {
        self.get_int(320)
    }

    pub fn set_player_quest_log_20_2(&mut self, v: i32) {
        self.set_int(321, v);
    }

    pub fn player_quest_log_20_2(&self) -> Option<i32> {
        self.get_int(321)
    }

    pub fn set_player_quest_log_20_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(322, a, b, c, d);
    }

    pub fn player_quest_log_20_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(322)
    }

    pub fn set_player_quest_log_20_4(&mut self, v: i32) {
        self.set_int(323, v);
    }

    pub fn player_quest_log_20_4(&self) -> Option<i32> {
        self.get_int(323)
    }

    pub fn set_player_quest_log_21_1(&mut self, v: i32) {
        self.set_int(324, v);
    }

    pub fn player_quest_log_21_1(&self) -> Option<i32> {
        self.get_int(324)
    }

    pub fn set_player_quest_log_21_2(&mut self, v: i32) {
        self.set_int(325, v);
    }

    pub fn player_quest_log_21_2(&self) -> Option<i32> {
        self.get_int(325)
    }

    pub fn set_player_quest_log_21_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(326, a, b, c, d);
    }

    pub fn player_quest_log_21_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(326)
    }

    pub fn set_player_quest_log_21_4(&mut self, v: i32) {
        self.set_int(327, v);
    }

    pub fn player_quest_log_21_4(&self) -> Option<i32> {
        self.get_int(327)
    }

    pub fn set_player_quest_log_22_1(&mut self, v: i32) {
        self.set_int(328, v);
    }

    pub fn player_quest_log_22_1(&self) -> Option<i32> {
        self.get_int(328)
    }

    pub fn set_player_quest_log_22_2(&mut self, v: i32) {
        self.set_int(329, v);
    }

    pub fn player_quest_log_22_2(&self) -> Option<i32> {
        self.get_int(329)
    }

    pub fn set_player_quest_log_22_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(330, a, b, c, d);
    }

    pub fn player_quest_log_22_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(330)
    }

    pub fn set_player_quest_log_22_4(&mut self, v: i32) {
        self.set_int(331, v);
    }

    pub fn player_quest_log_22_4(&self) -> Option<i32> {
        self.get_int(331)
    }

    pub fn set_player_quest_log_23_1(&mut self, v: i32) {
        self.set_int(332, v);
    }

    pub fn player_quest_log_23_1(&self) -> Option<i32> {
        self.get_int(332)
    }

    pub fn set_player_quest_log_23_2(&mut self, v: i32) {
        self.set_int(333, v);
    }

    pub fn player_quest_log_23_2(&self) -> Option<i32> {
        self.get_int(333)
    }

    pub fn set_player_quest_log_23_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(334, a, b, c, d);
    }

    pub fn player_quest_log_23_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(334)
    }

    pub fn set_player_quest_log_23_4(&mut self, v: i32) {
        self.set_int(335, v);
    }

    pub fn player_quest_log_23_4(&self) -> Option<i32> {
        self.get_int(335)
    }

    pub fn set_player_quest_log_24_1(&mut self, v: i32) {
        self.set_int(336, v);
    }

    pub fn player_quest_log_24_1(&self) -> Option<i32> {
        self.get_int(336)
    }

    pub fn set_player_quest_log_24_2(&mut self, v: i32) {
        self.set_int(337, v);
    }

    pub fn player_quest_log_24_2(&self) -> Option<i32> {
        self.get_int(337)
    }

    pub fn set_player_quest_log_24_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(338, a, b, c, d);
    }

    pub fn player_quest_log_24_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(338)
    }

    pub fn set_player_quest_log_24_4(&mut self, v: i32) {
        self.set_int(339, v);
    }

    pub fn player_quest_log_24_4(&self) -> Option<i32> {
        self.get_int(339)
    }

    pub fn set_player_quest_log_25_1(&mut self, v: i32) {
        self.set_int(340, v);
    }

    pub fn player_quest_log_25_1(&self) -> Option<i32> {
        self.get_int(340)
    }

    pub fn set_player_quest_log_25_2(&mut self, v: i32) {
        self.set_int(341, v);
    }

    pub fn player_quest_log_25_2(&self) -> Option<i32> {
        self.get_int(341)
    }

    pub fn set_player_quest_log_25_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(342, a, b, c, d);
    }

    pub fn player_quest_log_25_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(342)
    }

    pub fn set_player_quest_log_25_4(&mut self, v: i32) {
        self.set_int(343, v);
    }

    pub fn player_quest_log_25_4(&self) -> Option<i32> {
        self.get_int(343)
    }

    pub fn set_player_visible_item_1_creator(&mut self, v: Guid) {
        self.set_guid(344, v);
    }

    pub fn player_visible_item_1_creator(&self) -> Option<Guid> {
        self.get_guid(344)
    }

    pub fn set_player_visible_item_1_0(&mut self, v: i32) {
        self.set_int(346, v);
    }

    pub fn player_visible_item_1_0(&self) -> Option<i32> {
        self.get_int(346)
    }

    pub fn set_player_visible_item_1_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(358, a, b);
    }

    pub fn player_visible_item_1_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(358)
    }

    pub fn set_player_visible_item_2_creator(&mut self, v: Guid) {
        self.set_guid(360, v);
    }

    pub fn player_visible_item_2_creator(&self) -> Option<Guid> {
        self.get_guid(360)
    }

    pub fn set_player_visible_item_2_0(&mut self, v: i32) {
        self.set_int(362, v);
    }

    pub fn player_visible_item_2_0(&self) -> Option<i32> {
        self.get_int(362)
    }

    pub fn set_player_visible_item_2_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(374, a, b);
    }

    pub fn player_visible_item_2_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(374)
    }

    pub fn set_player_visible_item_3_creator(&mut self, v: Guid) {
        self.set_guid(376, v);
    }

    pub fn player_visible_item_3_creator(&self) -> Option<Guid> {
        self.get_guid(376)
    }

    pub fn set_player_visible_item_3_0(&mut self, v: i32) {
        self.set_int(378, v);
    }

    pub fn player_visible_item_3_0(&self) -> Option<i32> {
        self.get_int(378)
    }

    pub fn set_player_visible_item_3_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(390, a, b);
    }

    pub fn player_visible_item_3_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(390)
    }

    pub fn set_player_visible_item_4_creator(&mut self, v: Guid) {
        self.set_guid(392, v);
    }

    pub fn player_visible_item_4_creator(&self) -> Option<Guid> {
        self.get_guid(392)
    }

    pub fn set_player_visible_item_4_0(&mut self, v: i32) {
        self.set_int(394, v);
    }

    pub fn player_visible_item_4_0(&self) -> Option<i32> {
        self.get_int(394)
    }

    pub fn set_player_visible_item_4_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(406, a, b);
    }

    pub fn player_visible_item_4_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(406)
    }

    pub fn set_player_visible_item_5_creator(&mut self, v: Guid) {
        self.set_guid(408, v);
    }

    pub fn player_visible_item_5_creator(&self) -> Option<Guid> {
        self.get_guid(408)
    }

    pub fn set_player_visible_item_5_0(&mut self, v: i32) {
        self.set_int(410, v);
    }

    pub fn player_visible_item_5_0(&self) -> Option<i32> {
        self.get_int(410)
    }

    pub fn set_player_visible_item_5_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(422, a, b);
    }

    pub fn player_visible_item_5_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(422)
    }

    pub fn set_player_visible_item_6_creator(&mut self, v: Guid) {
        self.set_guid(424, v);
    }

    pub fn player_visible_item_6_creator(&self) -> Option<Guid> {
        self.get_guid(424)
    }

    pub fn set_player_visible_item_6_0(&mut self, v: i32) {
        self.set_int(426, v);
    }

    pub fn player_visible_item_6_0(&self) -> Option<i32> {
        self.get_int(426)
    }

    pub fn set_player_visible_item_6_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(438, a, b);
    }

    pub fn player_visible_item_6_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(438)
    }

    pub fn set_player_visible_item_7_creator(&mut self, v: Guid) {
        self.set_guid(440, v);
    }

    pub fn player_visible_item_7_creator(&self) -> Option<Guid> {
        self.get_guid(440)
    }

    pub fn set_player_visible_item_7_0(&mut self, v: i32) {
        self.set_int(442, v);
    }

    pub fn player_visible_item_7_0(&self) -> Option<i32> {
        self.get_int(442)
    }

    pub fn set_player_visible_item_7_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(454, a, b);
    }

    pub fn player_visible_item_7_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(454)
    }

    pub fn set_player_visible_item_8_creator(&mut self, v: Guid) {
        self.set_guid(456, v);
    }

    pub fn player_visible_item_8_creator(&self) -> Option<Guid> {
        self.get_guid(456)
    }

    pub fn set_player_visible_item_8_0(&mut self, v: i32) {
        self.set_int(458, v);
    }

    pub fn player_visible_item_8_0(&self) -> Option<i32> {
        self.get_int(458)
    }

    pub fn set_player_visible_item_8_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(470, a, b);
    }

    pub fn player_visible_item_8_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(470)
    }

    pub fn set_player_visible_item_9_creator(&mut self, v: Guid) {
        self.set_guid(472, v);
    }

    pub fn player_visible_item_9_creator(&self) -> Option<Guid> {
        self.get_guid(472)
    }

    pub fn set_player_visible_item_9_0(&mut self, v: i32) {
        self.set_int(474, v);
    }

    pub fn player_visible_item_9_0(&self) -> Option<i32> {
        self.get_int(474)
    }

    pub fn set_player_visible_item_9_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(486, a, b);
    }

    pub fn player_visible_item_9_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(486)
    }

    pub fn set_player_visible_item_10_creator(&mut self, v: Guid) {
        self.set_guid(488, v);
    }

    pub fn player_visible_item_10_creator(&self) -> Option<Guid> {
        self.get_guid(488)
    }

    pub fn set_player_visible_item_10_0(&mut self, v: i32) {
        self.set_int(490, v);
    }

    pub fn player_visible_item_10_0(&self) -> Option<i32> {
        self.get_int(490)
    }

    pub fn set_player_visible_item_10_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(502, a, b);
    }

    pub fn player_visible_item_10_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(502)
    }

    pub fn set_player_visible_item_11_creator(&mut self, v: Guid) {
        self.set_guid(504, v);
    }

    pub fn player_visible_item_11_creator(&self) -> Option<Guid> {
        self.get_guid(504)
    }

    pub fn set_player_visible_item_11_0(&mut self, v: i32) {
        self.set_int(506, v);
    }

    pub fn player_visible_item_11_0(&self) -> Option<i32> {
        self.get_int(506)
    }

    pub fn set_player_visible_item_11_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(518, a, b);
    }

    pub fn player_visible_item_11_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(518)
    }

    pub fn set_player_visible_item_12_creator(&mut self, v: Guid) {
        self.set_guid(520, v);
    }

    pub fn player_visible_item_12_creator(&self) -> Option<Guid> {
        self.get_guid(520)
    }

    pub fn set_player_visible_item_12_0(&mut self, v: i32) {
        self.set_int(522, v);
    }

    pub fn player_visible_item_12_0(&self) -> Option<i32> {
        self.get_int(522)
    }

    pub fn set_player_visible_item_12_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(534, a, b);
    }

    pub fn player_visible_item_12_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(534)
    }

    pub fn set_player_visible_item_13_creator(&mut self, v: Guid) {
        self.set_guid(536, v);
    }

    pub fn player_visible_item_13_creator(&self) -> Option<Guid> {
        self.get_guid(536)
    }

    pub fn set_player_visible_item_13_0(&mut self, v: i32) {
        self.set_int(538, v);
    }

    pub fn player_visible_item_13_0(&self) -> Option<i32> {
        self.get_int(538)
    }

    pub fn set_player_visible_item_13_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(550, a, b);
    }

    pub fn player_visible_item_13_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(550)
    }

    pub fn set_player_visible_item_14_creator(&mut self, v: Guid) {
        self.set_guid(552, v);
    }

    pub fn player_visible_item_14_creator(&self) -> Option<Guid> {
        self.get_guid(552)
    }

    pub fn set_player_visible_item_14_0(&mut self, v: i32) {
        self.set_int(554, v);
    }

    pub fn player_visible_item_14_0(&self) -> Option<i32> {
        self.get_int(554)
    }

    pub fn set_player_visible_item_14_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(566, a, b);
    }

    pub fn player_visible_item_14_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(566)
    }

    pub fn set_player_visible_item_15_creator(&mut self, v: Guid) {
        self.set_guid(568, v);
    }

    pub fn player_visible_item_15_creator(&self) -> Option<Guid> {
        self.get_guid(568)
    }

    pub fn set_player_visible_item_15_0(&mut self, v: i32) {
        self.set_int(570, v);
    }

    pub fn player_visible_item_15_0(&self) -> Option<i32> {
        self.get_int(570)
    }

    pub fn set_player_visible_item_15_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(582, a, b);
    }

    pub fn player_visible_item_15_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(582)
    }

    pub fn set_player_visible_item_16_creator(&mut self, v: Guid) {
        self.set_guid(584, v);
    }

    pub fn player_visible_item_16_creator(&self) -> Option<Guid> {
        self.get_guid(584)
    }

    pub fn set_player_visible_item_16_0(&mut self, v: i32) {
        self.set_int(586, v);
    }

    pub fn player_visible_item_16_0(&self) -> Option<i32> {
        self.get_int(586)
    }

    pub fn set_player_visible_item_16_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(598, a, b);
    }

    pub fn player_visible_item_16_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(598)
    }

    pub fn set_player_visible_item_17_creator(&mut self, v: Guid) {
        self.set_guid(600, v);
    }

    pub fn player_visible_item_17_creator(&self) -> Option<Guid> {
        self.get_guid(600)
    }

    pub fn set_player_visible_item_17_0(&mut self, v: i32) {
        self.set_int(602, v);
    }

    pub fn player_visible_item_17_0(&self) -> Option<i32> {
        self.get_int(602)
    }

    pub fn set_player_visible_item_17_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(614, a, b);
    }

    pub fn player_visible_item_17_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(614)
    }

    pub fn set_player_visible_item_18_creator(&mut self, v: Guid) {
        self.set_guid(616, v);
    }

    pub fn player_visible_item_18_creator(&self) -> Option<Guid> {
        self.get_guid(616)
    }

    pub fn set_player_visible_item_18_0(&mut self, v: i32) {
        self.set_int(618, v);
    }

    pub fn player_visible_item_18_0(&self) -> Option<i32> {
        self.get_int(618)
    }

    pub fn set_player_visible_item_18_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(630, a, b);
    }

    pub fn player_visible_item_18_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(630)
    }

    pub fn set_player_visible_item_19_creator(&mut self, v: Guid) {
        self.set_guid(632, v);
    }

    pub fn player_visible_item_19_creator(&self) -> Option<Guid> {
        self.get_guid(632)
    }

    pub fn set_player_visible_item_19_0(&mut self, v: i32) {
        self.set_int(634, v);
    }

    pub fn player_visible_item_19_0(&self) -> Option<i32> {
        self.get_int(634)
    }

    pub fn set_player_visible_item_19_properties(&mut self, a: u16, b: u16) {
        self.set_shorts(646, a, b);
    }

    pub fn player_visible_item_19_properties(&self) -> Option<(u16, u16)> {
        self.get_shorts(646)
    }

    pub fn set_player_chosen_title(&mut self, v: i32) {
        self.set_int(648, v);
    }

    pub fn player_chosen_title(&self) -> Option<i32> {
        self.get_int(648)
    }

    pub fn set_player_inv_slot_head(&mut self, v: Guid) {
        self.set_guid(650, v);
    }

    pub fn player_inv_slot_head(&self) -> Option<Guid> {
        self.get_guid(650)
    }

    pub fn set_player_inv_slot_neck(&mut self, v: Guid) {
        self.set_guid(652, v);
    }

    pub fn player_inv_slot_neck(&self) -> Option<Guid> {
        self.get_guid(652)
    }

    pub fn set_player_inv_slot_shoulders(&mut self, v: Guid) {
        self.set_guid(654, v);
    }

    pub fn player_inv_slot_shoulders(&self) -> Option<Guid> {
        self.get_guid(654)
    }

    pub fn set_player_inv_slot_body(&mut self, v: Guid) {
        self.set_guid(656, v);
    }

    pub fn player_inv_slot_body(&self) -> Option<Guid> {
        self.get_guid(656)
    }

    pub fn set_player_inv_slot_chest(&mut self, v: Guid) {
        self.set_guid(658, v);
    }

    pub fn player_inv_slot_chest(&self) -> Option<Guid> {
        self.get_guid(658)
    }

    pub fn set_player_inv_slot_waist(&mut self, v: Guid) {
        self.set_guid(660, v);
    }

    pub fn player_inv_slot_waist(&self) -> Option<Guid> {
        self.get_guid(660)
    }

    pub fn set_player_inv_slot_legs(&mut self, v: Guid) {
        self.set_guid(662, v);
    }

    pub fn player_inv_slot_legs(&self) -> Option<Guid> {
        self.get_guid(662)
    }

    pub fn set_player_inv_slot_feet(&mut self, v: Guid) {
        self.set_guid(664, v);
    }

    pub fn player_inv_slot_feet(&self) -> Option<Guid> {
        self.get_guid(664)
    }

    pub fn set_player_inv_slot_wrists(&mut self, v: Guid) {
        self.set_guid(666, v);
    }

    pub fn player_inv_slot_wrists(&self) -> Option<Guid> {
        self.get_guid(666)
    }

    pub fn set_player_inv_slot_hands(&mut self, v: Guid) {
        self.set_guid(668, v);
    }

    pub fn player_inv_slot_hands(&self) -> Option<Guid> {
        self.get_guid(668)
    }

    pub fn set_player_inv_slot_finger1(&mut self, v: Guid) {
        self.set_guid(670, v);
    }

    pub fn player_inv_slot_finger1(&self) -> Option<Guid> {
        self.get_guid(670)
    }

    pub fn set_player_inv_slot_finger2(&mut self, v: Guid) {
        self.set_guid(672, v);
    }

    pub fn player_inv_slot_finger2(&self) -> Option<Guid> {
        self.get_guid(672)
    }

    pub fn set_player_inv_slot_trinket1(&mut self, v: Guid) {
        self.set_guid(674, v);
    }

    pub fn player_inv_slot_trinket1(&self) -> Option<Guid> {
        self.get_guid(674)
    }

    pub fn set_player_inv_slot_trinket2(&mut self, v: Guid) {
        self.set_guid(676, v);
    }

    pub fn player_inv_slot_trinket2(&self) -> Option<Guid> {
        self.get_guid(676)
    }

    pub fn set_player_inv_slot_back(&mut self, v: Guid) {
        self.set_guid(678, v);
    }

    pub fn player_inv_slot_back(&self) -> Option<Guid> {
        self.get_guid(678)
    }

    pub fn set_player_inv_slot_main_hand(&mut self, v: Guid) {
        self.set_guid(680, v);
    }

    pub fn player_inv_slot_main_hand(&self) -> Option<Guid> {
        self.get_guid(680)
    }

    pub fn set_player_inv_slot_off_hand(&mut self, v: Guid) {
        self.set_guid(682, v);
    }

    pub fn player_inv_slot_off_hand(&self) -> Option<Guid> {
        self.get_guid(682)
    }

    pub fn set_player_inv_slot_ranged(&mut self, v: Guid) {
        self.set_guid(684, v);
    }

    pub fn player_inv_slot_ranged(&self) -> Option<Guid> {
        self.get_guid(684)
    }

    pub fn set_player_inv_slot_tabard(&mut self, v: Guid) {
        self.set_guid(686, v);
    }

    pub fn player_inv_slot_tabard(&self) -> Option<Guid> {
        self.get_guid(686)
    }

    pub fn set_player_inv_slot_bag1(&mut self, v: Guid) {
        self.set_guid(688, v);
    }

    pub fn player_inv_slot_bag1(&self) -> Option<Guid> {
        self.get_guid(688)
    }

    pub fn set_player_inv_slot_bag2(&mut self, v: Guid) {
        self.set_guid(690, v);
    }

    pub fn player_inv_slot_bag2(&self) -> Option<Guid> {
        self.get_guid(690)
    }

    pub fn set_player_inv_slot_bag3(&mut self, v: Guid) {
        self.set_guid(692, v);
    }

    pub fn player_inv_slot_bag3(&self) -> Option<Guid> {
        self.get_guid(692)
    }

    pub fn set_player_inv_slot_bag4(&mut self, v: Guid) {
        self.set_guid(694, v);
    }

    pub fn player_inv_slot_bag4(&self) -> Option<Guid> {
        self.get_guid(694)
    }

    pub fn set_player_pack_slot_1(&mut self, v: Guid) {
        self.set_guid(696, v);
    }

    pub fn player_pack_slot_1(&self) -> Option<Guid> {
        self.get_guid(696)
    }

    pub fn set_player_pack_slot_2(&mut self, v: Guid) {
        self.set_guid(698, v);
    }

    pub fn player_pack_slot_2(&self) -> Option<Guid> {
        self.get_guid(698)
    }

    pub fn set_player_pack_slot_3(&mut self, v: Guid) {
        self.set_guid(700, v);
    }

    pub fn player_pack_slot_3(&self) -> Option<Guid> {
        self.get_guid(700)
    }

    pub fn set_player_pack_slot_4(&mut self, v: Guid) {
        self.set_guid(702, v);
    }

    pub fn player_pack_slot_4(&self) -> Option<Guid> {
        self.get_guid(702)
    }

    pub fn set_player_pack_slot_5(&mut self, v: Guid) {
        self.set_guid(704, v);
    }

    pub fn player_pack_slot_5(&self) -> Option<Guid> {
        self.get_guid(704)
    }

    pub fn set_player_pack_slot_6(&mut self, v: Guid) {
        self.set_guid(706, v);
    }

    pub fn player_pack_slot_6(&self) -> Option<Guid> {
        self.get_guid(706)
    }

    pub fn set_player_pack_slot_7(&mut self, v: Guid) {
        self.set_guid(708, v);
    }

    pub fn player_pack_slot_7(&self) -> Option<Guid> {
        self.get_guid(708)
    }

    pub fn set_player_pack_slot_8(&mut self, v: Guid) {
        self.set_guid(710, v);
    }

    pub fn player_pack_slot_8(&self) -> Option<Guid> {
        self.get_guid(710)
    }

    pub fn set_player_pack_slot_9(&mut self, v: Guid) {
        self.set_guid(712, v);
    }

    pub fn player_pack_slot_9(&self) -> Option<Guid> {
        self.get_guid(712)
    }

    pub fn set_player_pack_slot_10(&mut self, v: Guid) {
        self.set_guid(714, v);
    }

    pub fn player_pack_slot_10(&self) -> Option<Guid> {
        self.get_guid(714)
    }

    pub fn set_player_pack_slot_11(&mut self, v: Guid) {
        self.set_guid(716, v);
    }

    pub fn player_pack_slot_11(&self) -> Option<Guid> {
        self.get_guid(716)
    }

    pub fn set_player_pack_slot_12(&mut self, v: Guid) {
        self.set_guid(718, v);
    }

    pub fn player_pack_slot_12(&self) -> Option<Guid> {
        self.get_guid(718)
    }

    pub fn set_player_pack_slot_13(&mut self, v: Guid) {
        self.set_guid(720, v);
    }

    pub fn player_pack_slot_13(&self) -> Option<Guid> {
        self.get_guid(720)
    }

    pub fn set_player_pack_slot_14(&mut self, v: Guid) {
        self.set_guid(722, v);
    }

    pub fn player_pack_slot_14(&self) -> Option<Guid> {
        self.get_guid(722)
    }

    pub fn set_player_pack_slot_15(&mut self, v: Guid) {
        self.set_guid(724, v);
    }

    pub fn player_pack_slot_15(&self) -> Option<Guid> {
        self.get_guid(724)
    }

    pub fn set_player_pack_slot_16(&mut self, v: Guid) {
        self.set_guid(726, v);
    }

    pub fn player_pack_slot_16(&self) -> Option<Guid> {
        self.get_guid(726)
    }

    pub fn set_player_bank_slot_1(&mut self, v: Guid) {
        self.set_guid(728, v);
    }

    pub fn player_bank_slot_1(&self) -> Option<Guid> {
        self.get_guid(728)
    }

    pub fn set_player_bank_slot_2(&mut self, v: Guid) {
        self.set_guid(730, v);
    }

    pub fn player_bank_slot_2(&self) -> Option<Guid> {
        self.get_guid(730)
    }

    pub fn set_player_bank_slot_3(&mut self, v: Guid) {
        self.set_guid(732, v);
    }

    pub fn player_bank_slot_3(&self) -> Option<Guid> {
        self.get_guid(732)
    }

    pub fn set_player_bank_slot_4(&mut self, v: Guid) {
        self.set_guid(734, v);
    }

    pub fn player_bank_slot_4(&self) -> Option<Guid> {
        self.get_guid(734)
    }

    pub fn set_player_bank_slot_5(&mut self, v: Guid) {
        self.set_guid(736, v);
    }

    pub fn player_bank_slot_5(&self) -> Option<Guid> {
        self.get_guid(736)
    }

    pub fn set_player_bank_slot_6(&mut self, v: Guid) {
        self.set_guid(738, v);
    }

    pub fn player_bank_slot_6(&self) -> Option<Guid> {
        self.get_guid(738)
    }

    pub fn set_player_bank_slot_7(&mut self, v: Guid) {
        self.set_guid(740, v);
    }

    pub fn player_bank_slot_7(&self) -> Option<Guid> {
        self.get_guid(740)
    }

    pub fn set_player_bank_slot_8(&mut self, v: Guid) {
        self.set_guid(742, v);
    }

    pub fn player_bank_slot_8(&self) -> Option<Guid> {
        self.get_guid(742)
    }

    pub fn set_player_bank_slot_9(&mut self, v: Guid) {
        self.set_guid(744, v);
    }

    pub fn player_bank_slot_9(&self) -> Option<Guid> {
        self.get_guid(744)
    }

    pub fn set_player_bank_slot_10(&mut self, v: Guid) {
        self.set_guid(746, v);
    }

    pub fn player_bank_slot_10(&self) -> Option<Guid> {
        self.get_guid(746)
    }

    pub fn set_player_bank_slot_11(&mut self, v: Guid) {
        self.set_guid(748, v);
    }

    pub fn player_bank_slot_11(&self) -> Option<Guid> {
        self.get_guid(748)
    }

    pub fn set_player_bank_slot_12(&mut self, v: Guid) {
        self.set_guid(750, v);
    }

    pub fn player_bank_slot_12(&self) -> Option<Guid> {
        self.get_guid(750)
    }

    pub fn set_player_bank_slot_13(&mut self, v: Guid) {
        self.set_guid(752, v);
    }

    pub fn player_bank_slot_13(&self) -> Option<Guid> {
        self.get_guid(752)
    }

    pub fn set_player_bank_slot_14(&mut self, v: Guid) {
        self.set_guid(754, v);
    }

    pub fn player_bank_slot_14(&self) -> Option<Guid> {
        self.get_guid(754)
    }

    pub fn set_player_bank_slot_15(&mut self, v: Guid) {
        self.set_guid(756, v);
    }

    pub fn player_bank_slot_15(&self) -> Option<Guid> {
        self.get_guid(756)
    }

    pub fn set_player_bank_slot_16(&mut self, v: Guid) {
        self.set_guid(758, v);
    }

    pub fn player_bank_slot_16(&self) -> Option<Guid> {
        self.get_guid(758)
    }

    pub fn set_player_bank_slot_17(&mut self, v: Guid) {
        self.set_guid(760, v);
    }

    pub fn player_bank_slot_17(&self) -> Option<Guid> {
        self.get_guid(760)
    }

    pub fn set_player_bank_slot_18(&mut self, v: Guid) {
        self.set_guid(762, v);
    }

    pub fn player_bank_slot_18(&self) -> Option<Guid> {
        self.get_guid(762)
    }

    pub fn set_player_bank_slot_19(&mut self, v: Guid) {
        self.set_guid(764, v);
    }

    pub fn player_bank_slot_19(&self) -> Option<Guid> {
        self.get_guid(764)
    }

    pub fn set_player_bank_slot_20(&mut self, v: Guid) {
        self.set_guid(766, v);
    }

    pub fn player_bank_slot_20(&self) -> Option<Guid> {
        self.get_guid(766)
    }

    pub fn set_player_bank_slot_21(&mut self, v: Guid) {
        self.set_guid(768, v);
    }

    pub fn player_bank_slot_21(&self) -> Option<Guid> {
        self.get_guid(768)
    }

    pub fn set_player_bank_slot_22(&mut self, v: Guid) {
        self.set_guid(770, v);
    }

    pub fn player_bank_slot_22(&self) -> Option<Guid> {
        self.get_guid(770)
    }

    pub fn set_player_bank_slot_23(&mut self, v: Guid) {
        self.set_guid(772, v);
    }

    pub fn player_bank_slot_23(&self) -> Option<Guid> {
        self.get_guid(772)
    }

    pub fn set_player_bank_slot_24(&mut self, v: Guid) {
        self.set_guid(774, v);
    }

    pub fn player_bank_slot_24(&self) -> Option<Guid> {
        self.get_guid(774)
    }

    pub fn set_player_bank_slot_25(&mut self, v: Guid) {
        self.set_guid(776, v);
    }

    pub fn player_bank_slot_25(&self) -> Option<Guid> {
        self.get_guid(776)
    }

    pub fn set_player_bank_slot_26(&mut self, v: Guid) {
        self.set_guid(778, v);
    }

    pub fn player_bank_slot_26(&self) -> Option<Guid> {
        self.get_guid(778)
    }

    pub fn set_player_bank_slot_27(&mut self, v: Guid) {
        self.set_guid(780, v);
    }

    pub fn player_bank_slot_27(&self) -> Option<Guid> {
        self.get_guid(780)
    }

    pub fn set_player_bank_slot_28(&mut self, v: Guid) {
        self.set_guid(782, v);
    }

    pub fn player_bank_slot_28(&self) -> Option<Guid> {
        self.get_guid(782)
    }

    pub fn set_player_bankbag_slot_1(&mut self, v: Guid) {
        self.set_guid(784, v);
    }

    pub fn player_bankbag_slot_1(&self) -> Option<Guid> {
        self.get_guid(784)
    }

    pub fn set_player_bankbag_slot_2(&mut self, v: Guid) {
        self.set_guid(786, v);
    }

    pub fn player_bankbag_slot_2(&self) -> Option<Guid> {
        self.get_guid(786)
    }

    pub fn set_player_bankbag_slot_3(&mut self, v: Guid) {
        self.set_guid(788, v);
    }

    pub fn player_bankbag_slot_3(&self) -> Option<Guid> {
        self.get_guid(788)
    }

    pub fn set_player_bankbag_slot_4(&mut self, v: Guid) {
        self.set_guid(790, v);
    }

    pub fn player_bankbag_slot_4(&self) -> Option<Guid> {
        self.get_guid(790)
    }

    pub fn set_player_bankbag_slot_5(&mut self, v: Guid) {
        self.set_guid(792, v);
    }

    pub fn player_bankbag_slot_5(&self) -> Option<Guid> {
        self.get_guid(792)
    }

    pub fn set_player_bankbag_slot_6(&mut self, v: Guid) {
        self.set_guid(794, v);
    }

    pub fn player_bankbag_slot_6(&self) -> Option<Guid> {
        self.get_guid(794)
    }

    pub fn set_player_bankbag_slot_7(&mut self, v: Guid) {
        self.set_guid(796, v);
    }

    pub fn player_bankbag_slot_7(&self) -> Option<Guid> {
        self.get_guid(796)
    }

    pub fn set_player_vendorbuyback_slot_1(&mut self, v: Guid) {
        self.set_guid(798, v);
    }

    pub fn player_vendorbuyback_slot_1(&self) -> Option<Guid> {
        self.get_guid(798)
    }

    pub fn set_player_vendorbuyback_slot_2(&mut self, v: Guid) {
        self.set_guid(800, v);
    }

    pub fn player_vendorbuyback_slot_2(&self) -> Option<Guid> {
        self.get_guid(800)
    }

    pub fn set_player_vendorbuyback_slot_3(&mut self, v: Guid) {
        self.set_guid(802, v);
    }

    pub fn player_vendorbuyback_slot_3(&self) -> Option<Guid> {
        self.get_guid(802)
    }

    pub fn set_player_vendorbuyback_slot_4(&mut self, v: Guid) {
        self.set_guid(804, v);
    }

    pub fn player_vendorbuyback_slot_4(&self) -> Option<Guid> {
        self.get_guid(804)
    }

    pub fn set_player_vendorbuyback_slot_5(&mut self, v: Guid) {
        self.set_guid(806, v);
    }

    pub fn player_vendorbuyback_slot_5(&self) -> Option<Guid> {
        self.get_guid(806)
    }

    pub fn set_player_vendorbuyback_slot_6(&mut self, v: Guid) {
        self.set_guid(808, v);
    }

    pub fn player_vendorbuyback_slot_6(&self) -> Option<Guid> {
        self.get_guid(808)
    }

    pub fn set_player_vendorbuyback_slot_7(&mut self, v: Guid) {
        self.set_guid(810, v);
    }

    pub fn player_vendorbuyback_slot_7(&self) -> Option<Guid> {
        self.get_guid(810)
    }

    pub fn set_player_vendorbuyback_slot_8(&mut self, v: Guid) {
        self.set_guid(812, v);
    }

    pub fn player_vendorbuyback_slot_8(&self) -> Option<Guid> {
        self.get_guid(812)
    }

    pub fn set_player_vendorbuyback_slot_9(&mut self, v: Guid) {
        self.set_guid(814, v);
    }

    pub fn player_vendorbuyback_slot_9(&self) -> Option<Guid> {
        self.get_guid(814)
    }

    pub fn set_player_vendorbuyback_slot_10(&mut self, v: Guid) {
        self.set_guid(816, v);
    }

    pub fn player_vendorbuyback_slot_10(&self) -> Option<Guid> {
        self.get_guid(816)
    }

    pub fn set_player_vendorbuyback_slot_11(&mut self, v: Guid) {
        self.set_guid(818, v);
    }

    pub fn player_vendorbuyback_slot_11(&self) -> Option<Guid> {
        self.get_guid(818)
    }

    pub fn set_player_vendorbuyback_slot_12(&mut self, v: Guid) {
        self.set_guid(820, v);
    }

    pub fn player_vendorbuyback_slot_12(&self) -> Option<Guid> {
        self.get_guid(820)
    }

    pub fn set_player_keyring_slot_1(&mut self, v: Guid) {
        self.set_guid(822, v);
    }

    pub fn player_keyring_slot_1(&self) -> Option<Guid> {
        self.get_guid(822)
    }

    pub fn set_player_keyring_slot_2(&mut self, v: Guid) {
        self.set_guid(824, v);
    }

    pub fn player_keyring_slot_2(&self) -> Option<Guid> {
        self.get_guid(824)
    }

    pub fn set_player_keyring_slot_3(&mut self, v: Guid) {
        self.set_guid(826, v);
    }

    pub fn player_keyring_slot_3(&self) -> Option<Guid> {
        self.get_guid(826)
    }

    pub fn set_player_keyring_slot_4(&mut self, v: Guid) {
        self.set_guid(828, v);
    }

    pub fn player_keyring_slot_4(&self) -> Option<Guid> {
        self.get_guid(828)
    }

    pub fn set_player_keyring_slot_5(&mut self, v: Guid) {
        self.set_guid(830, v);
    }

    pub fn player_keyring_slot_5(&self) -> Option<Guid> {
        self.get_guid(830)
    }

    pub fn set_player_keyring_slot_6(&mut self, v: Guid) {
        self.set_guid(832, v);
    }

    pub fn player_keyring_slot_6(&self) -> Option<Guid> {
        self.get_guid(832)
    }

    pub fn set_player_keyring_slot_7(&mut self, v: Guid) {
        self.set_guid(834, v);
    }

    pub fn player_keyring_slot_7(&self) -> Option<Guid> {
        self.get_guid(834)
    }

    pub fn set_player_keyring_slot_8(&mut self, v: Guid) {
        self.set_guid(836, v);
    }

    pub fn player_keyring_slot_8(&self) -> Option<Guid> {
        self.get_guid(836)
    }

    pub fn set_player_keyring_slot_9(&mut self, v: Guid) {
        self.set_guid(838, v);
    }

    pub fn player_keyring_slot_9(&self) -> Option<Guid> {
        self.get_guid(838)
    }

    pub fn set_player_keyring_slot_10(&mut self, v: Guid) {
        self.set_guid(840, v);
    }

    pub fn player_keyring_slot_10(&self) -> Option<Guid> {
        self.get_guid(840)
    }

    pub fn set_player_keyring_slot_11(&mut self, v: Guid) {
        self.set_guid(842, v);
    }

    pub fn player_keyring_slot_11(&self) -> Option<Guid> {
        self.get_guid(842)
    }

    pub fn set_player_keyring_slot_12(&mut self, v: Guid) {
        self.set_guid(844, v);
    }

    pub fn player_keyring_slot_12(&self) -> Option<Guid> {
        self.get_guid(844)
    }

    pub fn set_player_keyring_slot_13(&mut self, v: Guid) {
        self.set_guid(846, v);
    }

    pub fn player_keyring_slot_13(&self) -> Option<Guid> {
        self.get_guid(846)
    }

    pub fn set_player_keyring_slot_14(&mut self, v: Guid) {
        self.set_guid(848, v);
    }

    pub fn player_keyring_slot_14(&self) -> Option<Guid> {
        self.get_guid(848)
    }

    pub fn set_player_keyring_slot_15(&mut self, v: Guid) {
        self.set_guid(850, v);
    }

    pub fn player_keyring_slot_15(&self) -> Option<Guid> {
        self.get_guid(850)
    }

    pub fn set_player_keyring_slot_16(&mut self, v: Guid) {
        self.set_guid(852, v);
    }

    pub fn player_keyring_slot_16(&self) -> Option<Guid> {
        self.get_guid(852)
    }

    pub fn set_player_keyring_slot_17(&mut self, v: Guid) {
        self.set_guid(854, v);
    }

    pub fn player_keyring_slot_17(&self) -> Option<Guid> {
        self.get_guid(854)
    }

    pub fn set_player_keyring_slot_18(&mut self, v: Guid) {
        self.set_guid(856, v);
    }

    pub fn player_keyring_slot_18(&self) -> Option<Guid> {
        self.get_guid(856)
    }

    pub fn set_player_keyring_slot_19(&mut self, v: Guid) {
        self.set_guid(858, v);
    }

    pub fn player_keyring_slot_19(&self) -> Option<Guid> {
        self.get_guid(858)
    }

    pub fn set_player_keyring_slot_20(&mut self, v: Guid) {
        self.set_guid(860, v);
    }

    pub fn player_keyring_slot_20(&self) -> Option<Guid> {
        self.get_guid(860)
    }

    pub fn set_player_keyring_slot_21(&mut self, v: Guid) {
        self.set_guid(862, v);
    }

    pub fn player_keyring_slot_21(&self) -> Option<Guid> {
        self.get_guid(862)
    }

    pub fn set_player_keyring_slot_22(&mut self, v: Guid) {
        self.set_guid(864, v);
    }

    pub fn player_keyring_slot_22(&self) -> Option<Guid> {
        self.get_guid(864)
    }

    pub fn set_player_keyring_slot_23(&mut self, v: Guid) {
        self.set_guid(866, v);
    }

    pub fn player_keyring_slot_23(&self) -> Option<Guid> {
        self.get_guid(866)
    }

    pub fn set_player_keyring_slot_24(&mut self, v: Guid) {
        self.set_guid(868, v);
    }

    pub fn player_keyring_slot_24(&self) -> Option<Guid> {
        self.get_guid(868)
    }

    pub fn set_player_keyring_slot_25(&mut self, v: Guid) {
        self.set_guid(870, v);
    }

    pub fn player_keyring_slot_25(&self) -> Option<Guid> {
        self.get_guid(870)
    }

    pub fn set_player_keyring_slot_26(&mut self, v: Guid) {
        self.set_guid(872, v);
    }

    pub fn player_keyring_slot_26(&self) -> Option<Guid> {
        self.get_guid(872)
    }

    pub fn set_player_keyring_slot_27(&mut self, v: Guid) {
        self.set_guid(874, v);
    }

    pub fn player_keyring_slot_27(&self) -> Option<Guid> {
        self.get_guid(874)
    }

    pub fn set_player_keyring_slot_28(&mut self, v: Guid) {
        self.set_guid(876, v);
    }

    pub fn player_keyring_slot_28(&self) -> Option<Guid> {
        self.get_guid(876)
    }

    pub fn set_player_keyring_slot_29(&mut self, v: Guid) {
        self.set_guid(878, v);
    }

    pub fn player_keyring_slot_29(&self) -> Option<Guid> {
        self.get_guid(878)
    }

    pub fn set_player_keyring_slot_30(&mut self, v: Guid) {
        self.set_guid(880, v);
    }

    pub fn player_keyring_slot_30(&self) -> Option<Guid> {
        self.get_guid(880)
    }

    pub fn set_player_keyring_slot_31(&mut self, v: Guid) {
        self.set_guid(882, v);
    }

    pub fn player_keyring_slot_31(&self) -> Option<Guid> {
        self.get_guid(882)
    }

    pub fn set_player_keyring_slot_32(&mut self, v: Guid) {
        self.set_guid(884, v);
    }

    pub fn player_keyring_slot_32(&self) -> Option<Guid> {
        self.get_guid(884)
    }

    pub fn set_player_vanitypet_slot_1(&mut self, v: Guid) {
        self.set_guid(886, v);
    }

    pub fn player_vanitypet_slot_1(&self) -> Option<Guid> {
        self.get_guid(886)
    }

    pub fn set_player_vanitypet_slot_2(&mut self, v: Guid) {
        self.set_guid(888, v);
    }

    pub fn player_vanitypet_slot_2(&self) -> Option<Guid> {
        self.get_guid(888)
    }

    pub fn set_player_vanitypet_slot_3(&mut self, v: Guid) {
        self.set_guid(890, v);
    }

    pub fn player_vanitypet_slot_3(&self) -> Option<Guid> {
        self.get_guid(890)
    }

    pub fn set_player_vanitypet_slot_4(&mut self, v: Guid) {
        self.set_guid(892, v);
    }

    pub fn player_vanitypet_slot_4(&self) -> Option<Guid> {
        self.get_guid(892)
    }

    pub fn set_player_vanitypet_slot_5(&mut self, v: Guid) {
        self.set_guid(894, v);
    }

    pub fn player_vanitypet_slot_5(&self) -> Option<Guid> {
        self.get_guid(894)
    }

    pub fn set_player_vanitypet_slot_6(&mut self, v: Guid) {
        self.set_guid(896, v);
    }

    pub fn player_vanitypet_slot_6(&self) -> Option<Guid> {
        self.get_guid(896)
    }

    pub fn set_player_vanitypet_slot_7(&mut self, v: Guid) {
        self.set_guid(898, v);
    }

    pub fn player_vanitypet_slot_7(&self) -> Option<Guid> {
        self.get_guid(898)
    }

    pub fn set_player_vanitypet_slot_8(&mut self, v: Guid) {
        self.set_guid(900, v);
    }

    pub fn player_vanitypet_slot_8(&self) -> Option<Guid> {
        self.get_guid(900)
    }

    pub fn set_player_vanitypet_slot_9(&mut self, v: Guid) {
        self.set_guid(902, v);
    }

    pub fn player_vanitypet_slot_9(&self) -> Option<Guid> {
        self.get_guid(902)
    }

    pub fn set_player_vanitypet_slot_10(&mut self, v: Guid) {
        self.set_guid(904, v);
    }

    pub fn player_vanitypet_slot_10(&self) -> Option<Guid> {
        self.get_guid(904)
    }

    pub fn set_player_vanitypet_slot_11(&mut self, v: Guid) {
        self.set_guid(906, v);
    }

    pub fn player_vanitypet_slot_11(&self) -> Option<Guid> {
        self.get_guid(906)
    }

    pub fn set_player_vanitypet_slot_12(&mut self, v: Guid) {
        self.set_guid(908, v);
    }

    pub fn player_vanitypet_slot_12(&self) -> Option<Guid> {
        self.get_guid(908)
    }

    pub fn set_player_vanitypet_slot_13(&mut self, v: Guid) {
        self.set_guid(910, v);
    }

    pub fn player_vanitypet_slot_13(&self) -> Option<Guid> {
        self.get_guid(910)
    }

    pub fn set_player_vanitypet_slot_14(&mut self, v: Guid) {
        self.set_guid(912, v);
    }

    pub fn player_vanitypet_slot_14(&self) -> Option<Guid> {
        self.get_guid(912)
    }

    pub fn set_player_vanitypet_slot_15(&mut self, v: Guid) {
        self.set_guid(914, v);
    }

    pub fn player_vanitypet_slot_15(&self) -> Option<Guid> {
        self.get_guid(914)
    }

    pub fn set_player_vanitypet_slot_16(&mut self, v: Guid) {
        self.set_guid(916, v);
    }

    pub fn player_vanitypet_slot_16(&self) -> Option<Guid> {
        self.get_guid(916)
    }

    pub fn set_player_vanitypet_slot_17(&mut self, v: Guid) {
        self.set_guid(918, v);
    }

    pub fn player_vanitypet_slot_17(&self) -> Option<Guid> {
        self.get_guid(918)
    }

    pub fn set_player_vanitypet_slot_18(&mut self, v: Guid) {
        self.set_guid(920, v);
    }

    pub fn player_vanitypet_slot_18(&self) -> Option<Guid> {
        self.get_guid(920)
    }

    pub fn set_player_farsight(&mut self, v: Guid) {
        self.set_guid(922, v);
    }

    pub fn player_farsight(&self) -> Option<Guid> {
        self.get_guid(922)
    }

    pub fn set_player_known_titles(&mut self, v: Guid) {
        self.set_guid(924, v);
    }

    pub fn player_known_titles(&self) -> Option<Guid> {
        self.get_guid(924)
    }

    pub fn set_player_xp(&mut self, v: i32) {
        self.set_int(926, v);
    }

    pub fn player_xp(&self) -> Option<i32> {
        self.get_int(926)
    }

    pub fn set_player_next_level_xp(&mut self, v: i32) {
        self.set_int(927, v);
    }

    pub fn player_next_level_xp(&self) -> Option<i32> {
        self.get_int(927)
    }

    pub fn set_player_skill_info(&mut self, skill_info: crate::tbc::SkillInfo, index: SkillInfoIndex) {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
    }

    pub fn player_skill_info(&self, index: SkillInfoIndex) -> Option<crate::tbc::SkillInfo> {
        crate::tbc::SkillInfo::from_range(self.values.range(index.first()..=index.last()))
    }

    pub fn set_player_character_points1(&mut self, v: i32) {
        self.set_int(1312, v);
    }

    pub fn player_character_points1(&self) -> Option<i32> {
        self.get_int(1312)
    }

    pub fn set_player_character_points2(&mut self, v: i32) {
        self.set_int(1313, v);
    }

    pub fn player_character_points2(&self) -> Option<i32> {
        self.get_int(1313)
    }

    pub fn set_player_track_creatures(&mut self, v: i32) {
        self.set_int(1314, v);
    }

    pub fn player_track_creatures(&self) -> Option<i32> {
        self.get_int(1314)
    }

    pub fn set_player_track_resources(&mut self, v: i32) {
        self.set_int(1315, v);
    }

    pub fn player_track_resources(&self) -> Option<i32> {
        self.get_int(1315)
    }

    pub fn set_player_block_percentage(&mut self, v: f32) {
        self.set_float(1316, v);
    }

    pub fn player_block_percentage(&self) -> Option<f32> {
        self.get_float(1316)
    }

    pub fn set_player_dodge_percentage(&mut self, v: f32) {
        self.set_float(1317, v);
    }

    pub fn player_dodge_percentage(&self) -> Option<f32> {
        self.get_float(1317)
    }

    pub fn set_player_parry_percentage(&mut self, v: f32) {
        self.set_float(1318, v);
    }

    pub fn player_parry_percentage(&self) -> Option<f32> {
        self.get_float(1318)
    }

    pub fn set_player_expertise(&mut self, v: i32) {
        self.set_int(1319, v);
    }

    pub fn player_expertise(&self) -> Option<i32> {
        self.get_int(1319)
    }

    pub fn set_player_offhand_expertise(&mut self, v: i32) {
        self.set_int(1320, v);
    }

    pub fn player_offhand_expertise(&self) -> Option<i32> {
        self.get_int(1320)
    }

    pub fn set_player_crit_percentage(&mut self, v: f32) {
        self.set_float(1321, v);
    }

    pub fn player_crit_percentage(&self) -> Option<f32> {
        self.get_float(1321)
    }

    pub fn set_player_ranged_crit_percentage(&mut self, v: f32) {
        self.set_float(1322, v);
    }

    pub fn player_ranged_crit_percentage(&self) -> Option<f32> {
        self.get_float(1322)
    }

    pub fn set_player_offhand_crit_percentage(&mut self, v: f32) {
        self.set_float(1323, v);
    }

    pub fn player_offhand_crit_percentage(&self) -> Option<f32> {
        self.get_float(1323)
    }

    pub fn set_player_spell_crit_percentage1(&mut self, v: f32) {
        self.set_float(1324, v);
    }

    pub fn player_spell_crit_percentage1(&self) -> Option<f32> {
        self.get_float(1324)
    }

    pub fn set_player_shield_block(&mut self, v: i32) {
        self.set_int(1331, v);
    }

    pub fn player_shield_block(&self) -> Option<i32> {
        self.get_int(1331)
    }

    pub fn set_player_explored_zones_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(1332, a, b, c, d);
    }

    pub fn player_explored_zones_1(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(1332)
    }

    pub fn set_player_rest_state_experience(&mut self, v: i32) {
        self.set_int(1460, v);
    }

    pub fn player_rest_state_experience(&self) -> Option<i32> {
        self.get_int(1460)
    }

    pub fn set_player_coinage(&mut self, v: i32) {
        self.set_int(1461, v);
    }

    pub fn player_coinage(&self) -> Option<i32> {
        self.get_int(1461)
    }

    pub fn set_player_mod_damage_done_pos(&mut self, v: i32) {
        self.set_int(1462, v);
    }

    pub fn player_mod_damage_done_pos(&self) -> Option<i32> {
        self.get_int(1462)
    }

    pub fn set_player_mod_damage_done_neg(&mut self, v: i32) {
        self.set_int(1469, v);
    }

    pub fn player_mod_damage_done_neg(&self) -> Option<i32> {
        self.get_int(1469)
    }

    pub fn set_player_mod_damage_done_pct(&mut self, v: i32) {
        self.set_int(1476, v);
    }

    pub fn player_mod_damage_done_pct(&self) -> Option<i32> {
        self.get_int(1476)
    }

    pub fn set_player_mod_healing_done_pos(&mut self, v: i32) {
        self.set_int(1483, v);
    }

    pub fn player_mod_healing_done_pos(&self) -> Option<i32> {
        self.get_int(1483)
    }

    pub fn set_player_mod_target_resistance(&mut self, v: i32) {
        self.set_int(1484, v);
    }

    pub fn player_mod_target_resistance(&self) -> Option<i32> {
        self.get_int(1484)
    }

    pub fn set_player_mod_target_physical_resistance(&mut self, v: i32) {
        self.set_int(1485, v);
    }

    pub fn player_mod_target_physical_resistance(&self) -> Option<i32> {
        self.get_int(1485)
    }

    pub fn set_player_features(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(1486, a, b, c, d);
    }

    pub fn player_features(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(1486)
    }

    pub fn set_player_ammo_id(&mut self, v: i32) {
        self.set_int(1487, v);
    }

    pub fn player_ammo_id(&self) -> Option<i32> {
        self.get_int(1487)
    }

    pub fn set_player_self_res_spell(&mut self, v: i32) {
        self.set_int(1488, v);
    }

    pub fn player_self_res_spell(&self) -> Option<i32> {
        self.get_int(1488)
    }

    pub fn set_player_pvp_medals(&mut self, v: i32) {
        self.set_int(1489, v);
    }

    pub fn player_pvp_medals(&self) -> Option<i32> {
        self.get_int(1489)
    }

    pub fn set_player_buyback_price_1(&mut self, v: i32) {
        self.set_int(1490, v);
    }

    pub fn player_buyback_price_1(&self) -> Option<i32> {
        self.get_int(1490)
    }

    pub fn set_player_buyback_timestamp_1(&mut self, v: i32) {
        self.set_int(1502, v);
    }

    pub fn player_buyback_timestamp_1(&self) -> Option<i32> {
        self.get_int(1502)
    }

    pub fn set_player_kills(&mut self, a: u16, b: u16) {
        self.set_shorts(1514, a, b);
    }

    pub fn player_kills(&self) -> Option<(u16, u16)> {
        self.get_shorts(1514)
    }

    pub fn set_player_today_contribution(&mut self, v: i32) {
        self.set_int(1515, v);
    }

    pub fn player_today_contribution(&self) -> Option<i32> {
        self.get_int(1515)
    }

    pub fn set_player_yesterday_contribution(&mut self, v: i32) {
        self.set_int(1516, v);
    }

    pub fn player_yesterday_contribution(&self) -> Option<i32> {
        self.get_int(1516)
    }

    pub fn set_player_lifetime_honorable_kills(&mut self, v: i32) {
        self.set_int(1517, v);
    }

    pub fn player_lifetime_honorable_kills(&self) -> Option<i32> {
        self.get_int(1517)
    }

    pub fn set_player_bytes2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(1518, a, b, c, d);
    }

    pub fn player_bytes2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(1518)
    }

    pub fn set_player_watched_faction_index(&mut self, v: i32) {
        self.set_int(1519, v);
    }

    pub fn player_watched_faction_index(&self) -> Option<i32> {
        self.get_int(1519)
    }

    pub fn set_player_combat_rating_1(&mut self, v: i32) {
        self.set_int(1520, v);
    }

    pub fn player_combat_rating_1(&self) -> Option<i32> {
        self.get_int(1520)
    }

    pub fn set_player_arena_team_info_1_1(&mut self, v: i32) {
        self.set_int(1544, v);
    }

    pub fn player_arena_team_info_1_1(&self) -> Option<i32> {
        self.get_int(1544)
    }

    pub fn set_player_honor_currency(&mut self, v: i32) {
        self.set_int(1562, v);
    }

    pub fn player_honor_currency(&self) -> Option<i32> {
        self.get_int(1562)
    }

    pub fn set_player_arena_currency(&mut self, v: i32) {
        self.set_int(1563, v);
    }

    pub fn player_arena_currency(&self) -> Option<i32> {
        self.get_int(1563)
    }

    pub fn set_player_mod_mana_regen(&mut self, v: f32) {
        self.set_float(1564, v);
    }

    pub fn player_mod_mana_regen(&self) -> Option<f32> {
        self.get_float(1564)
    }

    pub fn set_player_mod_mana_regen_interrupt(&mut self, v: f32) {
        self.set_float(1565, v);
    }

    pub fn player_mod_mana_regen_interrupt(&self) -> Option<f32> {
        self.get_float(1565)
    }

    pub fn set_player_max_level(&mut self, v: i32) {
        self.set_int(1566, v);
    }

    pub fn player_max_level(&self) -> Option<i32> {
        self.get_int(1566)
    }

    pub fn set_player_daily_quests_1(&mut self, v: i32) {
        self.set_int(1567, v);
    }

    pub fn player_daily_quests_1(&self) -> Option<i32> {
        self.get_int(1567)
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

    pub fn set_gameobject_rotation(&mut self, v: f32) {
        self.set_float(10, v);
    }

    pub fn gameobject_rotation(&self) -> Option<f32> {
        self.get_float(10)
    }

    pub fn set_gameobject_state(&mut self, v: i32) {
        self.set_int(14, v);
    }

    pub fn gameobject_state(&self) -> Option<i32> {
        self.get_int(14)
    }

    pub fn set_gameobject_pos_x(&mut self, v: f32) {
        self.set_float(15, v);
    }

    pub fn gameobject_pos_x(&self) -> Option<f32> {
        self.get_float(15)
    }

    pub fn set_gameobject_pos_y(&mut self, v: f32) {
        self.set_float(16, v);
    }

    pub fn gameobject_pos_y(&self) -> Option<f32> {
        self.get_float(16)
    }

    pub fn set_gameobject_pos_z(&mut self, v: f32) {
        self.set_float(17, v);
    }

    pub fn gameobject_pos_z(&self) -> Option<f32> {
        self.get_float(17)
    }

    pub fn set_gameobject_facing(&mut self, v: f32) {
        self.set_float(18, v);
    }

    pub fn gameobject_facing(&self) -> Option<f32> {
        self.get_float(18)
    }

    pub fn set_gameobject_dyn_flags(&mut self, v: i32) {
        self.set_int(19, v);
    }

    pub fn gameobject_dyn_flags(&self) -> Option<i32> {
        self.get_int(19)
    }

    pub fn set_gameobject_faction(&mut self, v: i32) {
        self.set_int(20, v);
    }

    pub fn gameobject_faction(&self) -> Option<i32> {
        self.get_int(20)
    }

    pub fn set_gameobject_type_id(&mut self, v: i32) {
        self.set_int(21, v);
    }

    pub fn gameobject_type_id(&self) -> Option<i32> {
        self.get_int(21)
    }

    pub fn set_gameobject_level(&mut self, v: i32) {
        self.set_int(22, v);
    }

    pub fn gameobject_level(&self) -> Option<i32> {
        self.get_int(22)
    }

    pub fn set_gameobject_artkit(&mut self, v: i32) {
        self.set_int(23, v);
    }

    pub fn gameobject_artkit(&self) -> Option<i32> {
        self.get_int(23)
    }

    pub fn set_gameobject_animprogress(&mut self, v: i32) {
        self.set_int(24, v);
    }

    pub fn gameobject_animprogress(&self) -> Option<i32> {
        self.get_int(24)
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

    pub fn set_dynamicobject_pos_x(&mut self, v: f32) {
        self.set_float(11, v);
    }

    pub fn dynamicobject_pos_x(&self) -> Option<f32> {
        self.get_float(11)
    }

    pub fn set_dynamicobject_pos_y(&mut self, v: f32) {
        self.set_float(12, v);
    }

    pub fn dynamicobject_pos_y(&self) -> Option<f32> {
        self.get_float(12)
    }

    pub fn set_dynamicobject_pos_z(&mut self, v: f32) {
        self.set_float(13, v);
    }

    pub fn dynamicobject_pos_z(&self) -> Option<f32> {
        self.get_float(13)
    }

    pub fn set_dynamicobject_facing(&mut self, v: f32) {
        self.set_float(14, v);
    }

    pub fn dynamicobject_facing(&self) -> Option<f32> {
        self.get_float(14)
    }

    pub fn set_dynamicobject_casttime(&mut self, v: i32) {
        self.set_int(15, v);
    }

    pub fn dynamicobject_casttime(&self) -> Option<i32> {
        self.get_int(15)
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

    pub fn set_corpse_facing(&mut self, v: f32) {
        self.set_float(10, v);
    }

    pub fn corpse_facing(&self) -> Option<f32> {
        self.get_float(10)
    }

    pub fn set_corpse_pos_x(&mut self, v: f32) {
        self.set_float(11, v);
    }

    pub fn corpse_pos_x(&self) -> Option<f32> {
        self.get_float(11)
    }

    pub fn set_corpse_pos_y(&mut self, v: f32) {
        self.set_float(12, v);
    }

    pub fn corpse_pos_y(&self) -> Option<f32> {
        self.get_float(12)
    }

    pub fn set_corpse_pos_z(&mut self, v: f32) {
        self.set_float(13, v);
    }

    pub fn corpse_pos_z(&self) -> Option<f32> {
        self.get_float(13)
    }

    pub fn set_corpse_display_id(&mut self, v: i32) {
        self.set_int(14, v);
    }

    pub fn corpse_display_id(&self) -> Option<i32> {
        self.get_int(14)
    }

    pub fn set_corpse_item(&mut self, v: i32) {
        self.set_int(15, v);
    }

    pub fn corpse_item(&self) -> Option<i32> {
        self.get_int(15)
    }

    pub fn set_corpse_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(34, a, b, c, d);
    }

    pub fn corpse_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(34)
    }

    pub fn set_corpse_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(35, a, b, c, d);
    }

    pub fn corpse_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(35)
    }

    pub fn set_corpse_guild(&mut self, v: i32) {
        self.set_int(36, v);
    }

    pub fn corpse_guild(&self) -> Option<i32> {
        self.get_int(36)
    }

    pub fn set_corpse_flags(&mut self, v: i32) {
        self.set_int(37, v);
    }

    pub fn corpse_flags(&self) -> Option<i32> {
        self.get_int(37)
    }

    pub fn set_corpse_dynamic_flags(&mut self, v: i32) {
        self.set_int(38, v);
    }

    pub fn corpse_dynamic_flags(&self) -> Option<i32> {
        self.get_int(38)
    }

}

