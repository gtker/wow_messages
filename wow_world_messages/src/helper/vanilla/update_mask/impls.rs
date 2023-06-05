use crate::Guid;
use std::convert::TryInto;
use super::indices::*;
use crate::vanilla::{Race};
use crate::vanilla::{Class};
use crate::vanilla::{Gender};
use crate::vanilla::{Power};
use crate::vanilla::{UpdateContainer, UpdateContainerBuilder, UpdateCorpse, UpdateCorpseBuilder, UpdateDynamicObject, UpdateDynamicObjectBuilder, UpdateGameObject, UpdateGameObjectBuilder, UpdateItem, UpdateItemBuilder, UpdatePlayer, UpdatePlayerBuilder, UpdateUnit, UpdateUnitBuilder};

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

    pub fn set_item_enchantment(mut self, v: i32) -> Self {
        self.set_int(22, v);
        self
    }

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.set_int(43, v);
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.set_int(44, v);
        self
    }

    pub fn set_item_item_text_id(mut self, v: i32) -> Self {
        self.set_int(45, v);
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.set_int(46, v);
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.set_int(47, v);
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

    pub fn set_item_enchantment(mut self, v: i32) -> Self {
        self.set_int(22, v);
        self
    }

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.set_int(43, v);
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.set_int(44, v);
        self
    }

    pub fn set_item_item_text_id(mut self, v: i32) -> Self {
        self.set_int(45, v);
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.set_int(46, v);
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.set_int(47, v);
        self
    }

    pub fn set_container_num_slots(mut self, v: i32) -> Self {
        self.set_int(48, v);
        self
    }

    pub fn set_container_slot_1(mut self, v: Guid) -> Self {
        self.set_guid(50, v);
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

    pub fn set_unit_aura(mut self, v: i32) -> Self {
        self.set_int(47, v);
        self
    }

    pub fn set_unit_auraflags(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(95, a, b, c, d);
        self
    }

    pub fn set_unit_auralevels(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(101, a, b, c, d);
        self
    }

    pub fn set_unit_auraapplications(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(113, a, b, c, d);
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.set_int(125, v);
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.set_int(126, v);
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.set_int(128, v);
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.set_float(129, v);
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.set_float(130, v);
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.set_int(131, v);
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.set_int(132, v);
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.set_int(133, v);
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.set_float(134, v);
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.set_float(135, v);
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(136, v);
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(137, v);
        self
    }

    pub fn set_unit_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(138, a, b, c, d);
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.set_int(139, v);
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.set_int(140, v);
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.set_int(141, v);
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.set_int(142, v);
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.set_int(143, v);
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.set_int(144, v);
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.set_float(145, v);
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.set_int(146, v);
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.set_int(147, v);
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.set_int(148, v);
        self
    }

    pub fn set_unit_training_points(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(149, a, b);
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.set_int(150, v);
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.set_int(151, v);
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.set_int(152, v);
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.set_int(153, v);
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.set_int(154, v);
        self
    }

    pub fn set_unit_normal_resistance(mut self, v: i32) -> Self {
        self.set_int(155, v);
        self
    }

    pub fn set_unit_holy_resistance(mut self, v: i32) -> Self {
        self.set_int(156, v);
        self
    }

    pub fn set_unit_fire_resistance(mut self, v: i32) -> Self {
        self.set_int(157, v);
        self
    }

    pub fn set_unit_nature_resistance(mut self, v: i32) -> Self {
        self.set_int(158, v);
        self
    }

    pub fn set_unit_frost_resistance(mut self, v: i32) -> Self {
        self.set_int(159, v);
        self
    }

    pub fn set_unit_shadow_resistance(mut self, v: i32) -> Self {
        self.set_int(160, v);
        self
    }

    pub fn set_unit_arcane_resistance(mut self, v: i32) -> Self {
        self.set_int(161, v);
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.set_int(162, v);
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.set_int(163, v);
        self
    }

    pub fn set_unit_bytes_2(mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) -> Self {
        self.set_bytes(164, facial_hair, unknown, bank_bag_slots, rested_state);
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.set_int(165, v);
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(166, a, b);
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(167, v);
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.set_int(168, v);
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(169, a, b);
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(170, v);
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.set_float(171, v);
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.set_float(172, v);
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.set_int(173, v);
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.set_float(180, v);
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

    pub fn set_unit_aura(mut self, v: i32) -> Self {
        self.set_int(47, v);
        self
    }

    pub fn set_unit_auraflags(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(95, a, b, c, d);
        self
    }

    pub fn set_unit_auralevels(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(101, a, b, c, d);
        self
    }

    pub fn set_unit_auraapplications(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(113, a, b, c, d);
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.set_int(125, v);
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.set_int(126, v);
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.set_int(128, v);
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.set_float(129, v);
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.set_float(130, v);
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.set_int(131, v);
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.set_int(132, v);
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.set_int(133, v);
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.set_float(134, v);
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.set_float(135, v);
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(136, v);
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.set_float(137, v);
        self
    }

    pub fn set_unit_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(138, a, b, c, d);
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.set_int(139, v);
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.set_int(140, v);
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.set_int(141, v);
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.set_int(142, v);
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.set_int(143, v);
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.set_int(144, v);
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.set_float(145, v);
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.set_int(146, v);
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.set_int(147, v);
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.set_int(148, v);
        self
    }

    pub fn set_unit_training_points(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(149, a, b);
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.set_int(150, v);
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.set_int(151, v);
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.set_int(152, v);
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.set_int(153, v);
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.set_int(154, v);
        self
    }

    pub fn set_unit_normal_resistance(mut self, v: i32) -> Self {
        self.set_int(155, v);
        self
    }

    pub fn set_unit_holy_resistance(mut self, v: i32) -> Self {
        self.set_int(156, v);
        self
    }

    pub fn set_unit_fire_resistance(mut self, v: i32) -> Self {
        self.set_int(157, v);
        self
    }

    pub fn set_unit_nature_resistance(mut self, v: i32) -> Self {
        self.set_int(158, v);
        self
    }

    pub fn set_unit_frost_resistance(mut self, v: i32) -> Self {
        self.set_int(159, v);
        self
    }

    pub fn set_unit_shadow_resistance(mut self, v: i32) -> Self {
        self.set_int(160, v);
        self
    }

    pub fn set_unit_arcane_resistance(mut self, v: i32) -> Self {
        self.set_int(161, v);
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.set_int(162, v);
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.set_int(163, v);
        self
    }

    pub fn set_unit_bytes_2(mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) -> Self {
        self.set_bytes(164, facial_hair, unknown, bank_bag_slots, rested_state);
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.set_int(165, v);
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(166, a, b);
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(167, v);
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.set_int(168, v);
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(169, a, b);
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.set_float(170, v);
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.set_float(171, v);
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.set_float(172, v);
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.set_int(173, v);
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.set_float(180, v);
        self
    }

    pub fn set_player_duel_arbiter(mut self, v: Guid) -> Self {
        self.set_guid(188, v);
        self
    }

    pub fn set_player_flags(mut self, v: i32) -> Self {
        self.set_int(190, v);
        self
    }

    pub fn set_player_guildid(mut self, v: i32) -> Self {
        self.set_int(191, v);
        self
    }

    pub fn set_player_guildrank(mut self, v: i32) -> Self {
        self.set_int(192, v);
        self
    }

    pub fn set_player_features(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(193, a, b, c, d);
        self
    }

    pub fn set_player_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(194, a, b, c, d);
        self
    }

    pub fn set_player_bytes_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(195, a, b, c, d);
        self
    }

    pub fn set_player_duel_team(mut self, v: i32) -> Self {
        self.set_int(196, v);
        self
    }

    pub fn set_player_guild_timestamp(mut self, v: i32) -> Self {
        self.set_int(197, v);
        self
    }

    pub fn set_player_quest_log_1_1(mut self, v: i32) -> Self {
        self.set_int(198, v);
        self
    }

    pub fn set_player_quest_log_1_2(mut self, v: i32) -> Self {
        self.set_int(199, v);
        self
    }

    pub fn set_player_quest_log_2_1(mut self, v: i32) -> Self {
        self.set_int(201, v);
        self
    }

    pub fn set_player_quest_log_2_2(mut self, v: i32) -> Self {
        self.set_int(202, v);
        self
    }

    pub fn set_player_quest_log_3_1(mut self, v: i32) -> Self {
        self.set_int(204, v);
        self
    }

    pub fn set_player_quest_log_3_2(mut self, v: i32) -> Self {
        self.set_int(205, v);
        self
    }

    pub fn set_player_quest_log_4_1(mut self, v: i32) -> Self {
        self.set_int(207, v);
        self
    }

    pub fn set_player_quest_log_4_2(mut self, v: i32) -> Self {
        self.set_int(208, v);
        self
    }

    pub fn set_player_quest_log_5_1(mut self, v: i32) -> Self {
        self.set_int(210, v);
        self
    }

    pub fn set_player_quest_log_5_2(mut self, v: i32) -> Self {
        self.set_int(211, v);
        self
    }

    pub fn set_player_quest_log_6_1(mut self, v: i32) -> Self {
        self.set_int(213, v);
        self
    }

    pub fn set_player_quest_log_6_2(mut self, v: i32) -> Self {
        self.set_int(214, v);
        self
    }

    pub fn set_player_quest_log_7_1(mut self, v: i32) -> Self {
        self.set_int(216, v);
        self
    }

    pub fn set_player_quest_log_7_2(mut self, v: i32) -> Self {
        self.set_int(217, v);
        self
    }

    pub fn set_player_quest_log_8_1(mut self, v: i32) -> Self {
        self.set_int(219, v);
        self
    }

    pub fn set_player_quest_log_8_2(mut self, v: i32) -> Self {
        self.set_int(220, v);
        self
    }

    pub fn set_player_quest_log_9_1(mut self, v: i32) -> Self {
        self.set_int(222, v);
        self
    }

    pub fn set_player_quest_log_9_2(mut self, v: i32) -> Self {
        self.set_int(223, v);
        self
    }

    pub fn set_player_quest_log_10_1(mut self, v: i32) -> Self {
        self.set_int(225, v);
        self
    }

    pub fn set_player_quest_log_10_2(mut self, v: i32) -> Self {
        self.set_int(226, v);
        self
    }

    pub fn set_player_quest_log_11_1(mut self, v: i32) -> Self {
        self.set_int(228, v);
        self
    }

    pub fn set_player_quest_log_11_2(mut self, v: i32) -> Self {
        self.set_int(229, v);
        self
    }

    pub fn set_player_quest_log_12_1(mut self, v: i32) -> Self {
        self.set_int(231, v);
        self
    }

    pub fn set_player_quest_log_12_2(mut self, v: i32) -> Self {
        self.set_int(232, v);
        self
    }

    pub fn set_player_quest_log_13_1(mut self, v: i32) -> Self {
        self.set_int(234, v);
        self
    }

    pub fn set_player_quest_log_13_2(mut self, v: i32) -> Self {
        self.set_int(235, v);
        self
    }

    pub fn set_player_quest_log_14_1(mut self, v: i32) -> Self {
        self.set_int(237, v);
        self
    }

    pub fn set_player_quest_log_14_2(mut self, v: i32) -> Self {
        self.set_int(238, v);
        self
    }

    pub fn set_player_quest_log_15_1(mut self, v: i32) -> Self {
        self.set_int(240, v);
        self
    }

    pub fn set_player_quest_log_15_2(mut self, v: i32) -> Self {
        self.set_int(241, v);
        self
    }

    pub fn set_player_quest_log_16_1(mut self, v: i32) -> Self {
        self.set_int(243, v);
        self
    }

    pub fn set_player_quest_log_16_2(mut self, v: i32) -> Self {
        self.set_int(244, v);
        self
    }

    pub fn set_player_quest_log_17_1(mut self, v: i32) -> Self {
        self.set_int(246, v);
        self
    }

    pub fn set_player_quest_log_17_2(mut self, v: i32) -> Self {
        self.set_int(247, v);
        self
    }

    pub fn set_player_quest_log_18_1(mut self, v: i32) -> Self {
        self.set_int(249, v);
        self
    }

    pub fn set_player_quest_log_18_2(mut self, v: i32) -> Self {
        self.set_int(250, v);
        self
    }

    pub fn set_player_quest_log_19_1(mut self, v: i32) -> Self {
        self.set_int(252, v);
        self
    }

    pub fn set_player_quest_log_19_2(mut self, v: i32) -> Self {
        self.set_int(253, v);
        self
    }

    pub fn set_player_quest_log_20_1(mut self, v: i32) -> Self {
        self.set_int(255, v);
        self
    }

    pub fn set_player_quest_log_20_2(mut self, v: i32) -> Self {
        self.set_int(256, v);
        self
    }

    pub fn set_player_visible_item(mut self, visible_item: crate::vanilla::VisibleItem, index: VisibleItemIndex) -> Self {
        for (index, value) in visible_item.mask_values(index) {
            self.header_set(index, value);
        }
        self
    }

    pub fn set_player_field_inv(mut self, item_slot: crate::vanilla::ItemSlot, item: Guid) -> Self {
        let offset = 486 + item_slot.as_int() as u16 * 2;
        self.set_guid(offset, item);
        self
    }

    pub fn set_player_farsight(mut self, v: Guid) -> Self {
        self.set_guid(712, v);
        self
    }

    pub fn set_player_field_combo_target(mut self, v: Guid) -> Self {
        self.set_guid(714, v);
        self
    }

    pub fn set_player_xp(mut self, v: i32) -> Self {
        self.set_int(716, v);
        self
    }

    pub fn set_player_next_level_xp(mut self, v: i32) -> Self {
        self.set_int(717, v);
        self
    }

    pub fn set_player_skill_info(mut self, skill_info: crate::vanilla::SkillInfo, index: SkillInfoIndex) -> Self {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
        self
    }

    pub fn set_player_character_points1(mut self, v: i32) -> Self {
        self.set_int(1102, v);
        self
    }

    pub fn set_player_character_points2(mut self, v: i32) -> Self {
        self.set_int(1103, v);
        self
    }

    pub fn set_player_track_creatures(mut self, v: i32) -> Self {
        self.set_int(1104, v);
        self
    }

    pub fn set_player_track_resources(mut self, v: i32) -> Self {
        self.set_int(1105, v);
        self
    }

    pub fn set_player_block_percentage(mut self, v: f32) -> Self {
        self.set_float(1106, v);
        self
    }

    pub fn set_player_dodge_percentage(mut self, v: f32) -> Self {
        self.set_float(1107, v);
        self
    }

    pub fn set_player_parry_percentage(mut self, v: f32) -> Self {
        self.set_float(1108, v);
        self
    }

    pub fn set_player_crit_percentage(mut self, v: f32) -> Self {
        self.set_float(1109, v);
        self
    }

    pub fn set_player_ranged_crit_percentage(mut self, v: f32) -> Self {
        self.set_float(1110, v);
        self
    }

    pub fn set_player_explored_zones_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(1111, a, b, c, d);
        self
    }

    pub fn set_player_rest_state_experience(mut self, v: i32) -> Self {
        self.set_int(1175, v);
        self
    }

    pub fn set_player_field_coinage(mut self, v: i32) -> Self {
        self.set_int(1176, v);
        self
    }

    pub fn set_player_field_posstat0(mut self, v: i32) -> Self {
        self.set_int(1177, v);
        self
    }

    pub fn set_player_field_posstat1(mut self, v: i32) -> Self {
        self.set_int(1178, v);
        self
    }

    pub fn set_player_field_posstat2(mut self, v: i32) -> Self {
        self.set_int(1179, v);
        self
    }

    pub fn set_player_field_posstat3(mut self, v: i32) -> Self {
        self.set_int(1180, v);
        self
    }

    pub fn set_player_field_posstat4(mut self, v: i32) -> Self {
        self.set_int(1181, v);
        self
    }

    pub fn set_player_field_negstat0(mut self, v: i32) -> Self {
        self.set_int(1182, v);
        self
    }

    pub fn set_player_field_negstat1(mut self, v: i32) -> Self {
        self.set_int(1183, v);
        self
    }

    pub fn set_player_field_negstat2(mut self, v: i32) -> Self {
        self.set_int(1184, v);
        self
    }

    pub fn set_player_field_negstat3(mut self, v: i32) -> Self {
        self.set_int(1185, v);
        self
    }

    pub fn set_player_field_negstat4(mut self, v: i32) -> Self {
        self.set_int(1186, v);
        self
    }

    pub fn set_player_field_resistancebuffmodspositive(mut self, v: i32) -> Self {
        self.set_int(1187, v);
        self
    }

    pub fn set_player_field_resistancebuffmodsnegative(mut self, v: i32) -> Self {
        self.set_int(1194, v);
        self
    }

    pub fn set_player_field_mod_damage_done_pos(mut self, v: i32) -> Self {
        self.set_int(1201, v);
        self
    }

    pub fn set_player_field_mod_damage_done_neg(mut self, v: i32) -> Self {
        self.set_int(1208, v);
        self
    }

    pub fn set_player_field_mod_damage_done_pct(mut self, v: i32) -> Self {
        self.set_int(1215, v);
        self
    }

    pub fn set_player_field_bytes(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(1222, a, b, c, d);
        self
    }

    pub fn set_player_ammo_id(mut self, v: i32) -> Self {
        self.set_int(1223, v);
        self
    }

    pub fn set_player_self_res_spell(mut self, v: i32) -> Self {
        self.set_int(1224, v);
        self
    }

    pub fn set_player_field_pvp_medals(mut self, v: i32) -> Self {
        self.set_int(1225, v);
        self
    }

    pub fn set_player_field_buyback_price_1(mut self, v: i32) -> Self {
        self.set_int(1226, v);
        self
    }

    pub fn set_player_field_buyback_timestamp_1(mut self, v: i32) -> Self {
        self.set_int(1238, v);
        self
    }

    pub fn set_player_field_session_kills(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(1250, a, b);
        self
    }

    pub fn set_player_field_yesterday_kills(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(1251, a, b);
        self
    }

    pub fn set_player_field_last_week_kills(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(1252, a, b);
        self
    }

    pub fn set_player_field_this_week_kills(mut self, a: u16, b: u16) -> Self {
        self.set_shorts(1253, a, b);
        self
    }

    pub fn set_player_field_this_week_contribution(mut self, v: i32) -> Self {
        self.set_int(1254, v);
        self
    }

    pub fn set_player_field_lifetime_honorbale_kills(mut self, v: i32) -> Self {
        self.set_int(1255, v);
        self
    }

    pub fn set_player_field_lifetime_dishonorbale_kills(mut self, v: i32) -> Self {
        self.set_int(1256, v);
        self
    }

    pub fn set_player_field_yesterday_contribution(mut self, v: i32) -> Self {
        self.set_int(1257, v);
        self
    }

    pub fn set_player_field_last_week_contribution(mut self, v: i32) -> Self {
        self.set_int(1258, v);
        self
    }

    pub fn set_player_field_last_week_rank(mut self, v: i32) -> Self {
        self.set_int(1259, v);
        self
    }

    pub fn set_player_field_bytes2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(1260, a, b, c, d);
        self
    }

    pub fn set_player_field_watched_faction_index(mut self, v: i32) -> Self {
        self.set_int(1261, v);
        self
    }

    pub fn set_player_field_combat_rating_1(mut self, v: i32) -> Self {
        self.set_int(1262, v);
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

    pub fn set_gameobject_created_by(mut self, v: Guid) -> Self {
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

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.set_int(3, v);
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.set_float(4, v);
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

    pub fn set_corpse_owner(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
        self
    }

    pub fn set_corpse_facing(mut self, v: f32) -> Self {
        self.set_float(8, v);
        self
    }

    pub fn set_corpse_pos_x(mut self, v: f32) -> Self {
        self.set_float(9, v);
        self
    }

    pub fn set_corpse_pos_y(mut self, v: f32) -> Self {
        self.set_float(10, v);
        self
    }

    pub fn set_corpse_pos_z(mut self, v: f32) -> Self {
        self.set_float(11, v);
        self
    }

    pub fn set_corpse_display_id(mut self, v: i32) -> Self {
        self.set_int(12, v);
        self
    }

    pub fn set_corpse_item(mut self, v: i32) -> Self {
        self.set_int(13, v);
        self
    }

    pub fn set_corpse_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(32, a, b, c, d);
        self
    }

    pub fn set_corpse_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.set_bytes(33, a, b, c, d);
        self
    }

    pub fn set_corpse_guild(mut self, v: i32) -> Self {
        self.set_int(34, v);
        self
    }

    pub fn set_corpse_flags(mut self, v: i32) -> Self {
        self.set_int(35, v);
        self
    }

    pub fn set_corpse_dynamic_flags(mut self, v: i32) -> Self {
        self.set_int(36, v);
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

    pub fn set_item_enchantment(&mut self, v: i32) {
        self.set_int(22, v);
    }

    pub fn item_enchantment(&self) -> Option<i32> {
        self.get_int(22)
    }

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.set_int(43, v);
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.get_int(43)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.set_int(44, v);
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.get_int(44)
    }

    pub fn set_item_item_text_id(&mut self, v: i32) {
        self.set_int(45, v);
    }

    pub fn item_item_text_id(&self) -> Option<i32> {
        self.get_int(45)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.set_int(46, v);
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.get_int(46)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.set_int(47, v);
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.get_int(47)
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

    pub fn set_item_enchantment(&mut self, v: i32) {
        self.set_int(22, v);
    }

    pub fn item_enchantment(&self) -> Option<i32> {
        self.get_int(22)
    }

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.set_int(43, v);
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.get_int(43)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.set_int(44, v);
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.get_int(44)
    }

    pub fn set_item_item_text_id(&mut self, v: i32) {
        self.set_int(45, v);
    }

    pub fn item_item_text_id(&self) -> Option<i32> {
        self.get_int(45)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.set_int(46, v);
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.get_int(46)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.set_int(47, v);
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.get_int(47)
    }

    pub fn set_container_num_slots(&mut self, v: i32) {
        self.set_int(48, v);
    }

    pub fn container_num_slots(&self) -> Option<i32> {
        self.get_int(48)
    }

    pub fn set_container_slot_1(&mut self, v: Guid) {
        self.set_guid(50, v);
    }

    pub fn container_slot_1(&self) -> Option<Guid> {
        self.get_guid(50)
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

    pub fn set_unit_aura(&mut self, v: i32) {
        self.set_int(47, v);
    }

    pub fn unit_aura(&self) -> Option<i32> {
        self.get_int(47)
    }

    pub fn set_unit_auraflags(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(95, a, b, c, d);
    }

    pub fn unit_auraflags(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(95)
    }

    pub fn set_unit_auralevels(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(101, a, b, c, d);
    }

    pub fn unit_auralevels(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(101)
    }

    pub fn set_unit_auraapplications(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(113, a, b, c, d);
    }

    pub fn unit_auraapplications(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(113)
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.set_int(125, v);
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.get_int(125)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.set_int(126, v);
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.get_int(126)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.set_int(128, v);
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.get_int(128)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.set_float(129, v);
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.get_float(129)
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.set_float(130, v);
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.get_float(130)
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.set_int(131, v);
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.get_int(131)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.set_int(132, v);
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.get_int(132)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.set_int(133, v);
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.get_int(133)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.set_float(134, v);
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.get_float(134)
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.set_float(135, v);
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.get_float(135)
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.set_float(136, v);
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.get_float(136)
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.set_float(137, v);
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.get_float(137)
    }

    pub fn set_unit_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(138, a, b, c, d);
    }

    pub fn unit_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(138)
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.set_int(139, v);
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.get_int(139)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.set_int(140, v);
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.get_int(140)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.set_int(141, v);
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.get_int(141)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.set_int(142, v);
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.get_int(142)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.set_int(143, v);
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.get_int(143)
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.set_int(144, v);
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.get_int(144)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.set_float(145, v);
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.get_float(145)
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.set_int(146, v);
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.get_int(146)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.set_int(147, v);
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.get_int(147)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.set_int(148, v);
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.get_int(148)
    }

    pub fn set_unit_training_points(&mut self, a: u16, b: u16) {
        self.set_shorts(149, a, b);
    }

    pub fn unit_training_points(&self) -> Option<(u16, u16)> {
        self.get_shorts(149)
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.set_int(150, v);
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.get_int(150)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.set_int(151, v);
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.get_int(151)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.set_int(152, v);
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.get_int(152)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.set_int(153, v);
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.get_int(153)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.set_int(154, v);
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.get_int(154)
    }

    pub fn set_unit_normal_resistance(&mut self, v: i32) {
        self.set_int(155, v);
    }

    pub fn unit_normal_resistance(&self) -> Option<i32> {
        self.get_int(155)
    }

    pub fn set_unit_holy_resistance(&mut self, v: i32) {
        self.set_int(156, v);
    }

    pub fn unit_holy_resistance(&self) -> Option<i32> {
        self.get_int(156)
    }

    pub fn set_unit_fire_resistance(&mut self, v: i32) {
        self.set_int(157, v);
    }

    pub fn unit_fire_resistance(&self) -> Option<i32> {
        self.get_int(157)
    }

    pub fn set_unit_nature_resistance(&mut self, v: i32) {
        self.set_int(158, v);
    }

    pub fn unit_nature_resistance(&self) -> Option<i32> {
        self.get_int(158)
    }

    pub fn set_unit_frost_resistance(&mut self, v: i32) {
        self.set_int(159, v);
    }

    pub fn unit_frost_resistance(&self) -> Option<i32> {
        self.get_int(159)
    }

    pub fn set_unit_shadow_resistance(&mut self, v: i32) {
        self.set_int(160, v);
    }

    pub fn unit_shadow_resistance(&self) -> Option<i32> {
        self.get_int(160)
    }

    pub fn set_unit_arcane_resistance(&mut self, v: i32) {
        self.set_int(161, v);
    }

    pub fn unit_arcane_resistance(&self) -> Option<i32> {
        self.get_int(161)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.set_int(162, v);
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.get_int(162)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.set_int(163, v);
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.get_int(163)
    }

    pub fn set_unit_bytes_2(&mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) {
        self.set_bytes(164, facial_hair, unknown, bank_bag_slots, rested_state);
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(164)
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.set_int(165, v);
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.get_int(165)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(166, a, b);
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(166)
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(167, v);
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(167)
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.set_int(168, v);
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.get_int(168)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(169, a, b);
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(169)
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(170, v);
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(170)
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.set_float(171, v);
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.get_float(171)
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.set_float(172, v);
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.get_float(172)
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.set_int(173, v);
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.get_int(173)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.set_float(180, v);
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.get_float(180)
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

    pub fn set_unit_aura(&mut self, v: i32) {
        self.set_int(47, v);
    }

    pub fn unit_aura(&self) -> Option<i32> {
        self.get_int(47)
    }

    pub fn set_unit_auraflags(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(95, a, b, c, d);
    }

    pub fn unit_auraflags(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(95)
    }

    pub fn set_unit_auralevels(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(101, a, b, c, d);
    }

    pub fn unit_auralevels(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(101)
    }

    pub fn set_unit_auraapplications(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(113, a, b, c, d);
    }

    pub fn unit_auraapplications(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(113)
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.set_int(125, v);
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.get_int(125)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.set_int(126, v);
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.get_int(126)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.set_int(128, v);
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.get_int(128)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.set_float(129, v);
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.get_float(129)
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.set_float(130, v);
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.get_float(130)
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.set_int(131, v);
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.get_int(131)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.set_int(132, v);
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.get_int(132)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.set_int(133, v);
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.get_int(133)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.set_float(134, v);
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.get_float(134)
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.set_float(135, v);
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.get_float(135)
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.set_float(136, v);
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.get_float(136)
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.set_float(137, v);
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.get_float(137)
    }

    pub fn set_unit_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(138, a, b, c, d);
    }

    pub fn unit_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(138)
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.set_int(139, v);
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.get_int(139)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.set_int(140, v);
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.get_int(140)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.set_int(141, v);
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.get_int(141)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.set_int(142, v);
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.get_int(142)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.set_int(143, v);
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.get_int(143)
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.set_int(144, v);
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.get_int(144)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.set_float(145, v);
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.get_float(145)
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.set_int(146, v);
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.get_int(146)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.set_int(147, v);
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.get_int(147)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.set_int(148, v);
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.get_int(148)
    }

    pub fn set_unit_training_points(&mut self, a: u16, b: u16) {
        self.set_shorts(149, a, b);
    }

    pub fn unit_training_points(&self) -> Option<(u16, u16)> {
        self.get_shorts(149)
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.set_int(150, v);
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.get_int(150)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.set_int(151, v);
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.get_int(151)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.set_int(152, v);
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.get_int(152)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.set_int(153, v);
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.get_int(153)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.set_int(154, v);
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.get_int(154)
    }

    pub fn set_unit_normal_resistance(&mut self, v: i32) {
        self.set_int(155, v);
    }

    pub fn unit_normal_resistance(&self) -> Option<i32> {
        self.get_int(155)
    }

    pub fn set_unit_holy_resistance(&mut self, v: i32) {
        self.set_int(156, v);
    }

    pub fn unit_holy_resistance(&self) -> Option<i32> {
        self.get_int(156)
    }

    pub fn set_unit_fire_resistance(&mut self, v: i32) {
        self.set_int(157, v);
    }

    pub fn unit_fire_resistance(&self) -> Option<i32> {
        self.get_int(157)
    }

    pub fn set_unit_nature_resistance(&mut self, v: i32) {
        self.set_int(158, v);
    }

    pub fn unit_nature_resistance(&self) -> Option<i32> {
        self.get_int(158)
    }

    pub fn set_unit_frost_resistance(&mut self, v: i32) {
        self.set_int(159, v);
    }

    pub fn unit_frost_resistance(&self) -> Option<i32> {
        self.get_int(159)
    }

    pub fn set_unit_shadow_resistance(&mut self, v: i32) {
        self.set_int(160, v);
    }

    pub fn unit_shadow_resistance(&self) -> Option<i32> {
        self.get_int(160)
    }

    pub fn set_unit_arcane_resistance(&mut self, v: i32) {
        self.set_int(161, v);
    }

    pub fn unit_arcane_resistance(&self) -> Option<i32> {
        self.get_int(161)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.set_int(162, v);
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.get_int(162)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.set_int(163, v);
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.get_int(163)
    }

    pub fn set_unit_bytes_2(&mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) {
        self.set_bytes(164, facial_hair, unknown, bank_bag_slots, rested_state);
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(164)
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.set_int(165, v);
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.get_int(165)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(166, a, b);
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(166)
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(167, v);
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(167)
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.set_int(168, v);
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.get_int(168)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.set_shorts(169, a, b);
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        self.get_shorts(169)
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.set_float(170, v);
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.get_float(170)
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.set_float(171, v);
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.get_float(171)
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.set_float(172, v);
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.get_float(172)
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.set_int(173, v);
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.get_int(173)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.set_float(180, v);
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.get_float(180)
    }

    pub fn set_player_duel_arbiter(&mut self, v: Guid) {
        self.set_guid(188, v);
    }

    pub fn player_duel_arbiter(&self) -> Option<Guid> {
        self.get_guid(188)
    }

    pub fn set_player_flags(&mut self, v: i32) {
        self.set_int(190, v);
    }

    pub fn player_flags(&self) -> Option<i32> {
        self.get_int(190)
    }

    pub fn set_player_guildid(&mut self, v: i32) {
        self.set_int(191, v);
    }

    pub fn player_guildid(&self) -> Option<i32> {
        self.get_int(191)
    }

    pub fn set_player_guildrank(&mut self, v: i32) {
        self.set_int(192, v);
    }

    pub fn player_guildrank(&self) -> Option<i32> {
        self.get_int(192)
    }

    pub fn set_player_features(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(193, a, b, c, d);
    }

    pub fn player_features(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(193)
    }

    pub fn set_player_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(194, a, b, c, d);
    }

    pub fn player_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(194)
    }

    pub fn set_player_bytes_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(195, a, b, c, d);
    }

    pub fn player_bytes_3(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(195)
    }

    pub fn set_player_duel_team(&mut self, v: i32) {
        self.set_int(196, v);
    }

    pub fn player_duel_team(&self) -> Option<i32> {
        self.get_int(196)
    }

    pub fn set_player_guild_timestamp(&mut self, v: i32) {
        self.set_int(197, v);
    }

    pub fn player_guild_timestamp(&self) -> Option<i32> {
        self.get_int(197)
    }

    pub fn set_player_quest_log_1_1(&mut self, v: i32) {
        self.set_int(198, v);
    }

    pub fn player_quest_log_1_1(&self) -> Option<i32> {
        self.get_int(198)
    }

    pub fn set_player_quest_log_1_2(&mut self, v: i32) {
        self.set_int(199, v);
    }

    pub fn player_quest_log_1_2(&self) -> Option<i32> {
        self.get_int(199)
    }

    pub fn set_player_quest_log_2_1(&mut self, v: i32) {
        self.set_int(201, v);
    }

    pub fn player_quest_log_2_1(&self) -> Option<i32> {
        self.get_int(201)
    }

    pub fn set_player_quest_log_2_2(&mut self, v: i32) {
        self.set_int(202, v);
    }

    pub fn player_quest_log_2_2(&self) -> Option<i32> {
        self.get_int(202)
    }

    pub fn set_player_quest_log_3_1(&mut self, v: i32) {
        self.set_int(204, v);
    }

    pub fn player_quest_log_3_1(&self) -> Option<i32> {
        self.get_int(204)
    }

    pub fn set_player_quest_log_3_2(&mut self, v: i32) {
        self.set_int(205, v);
    }

    pub fn player_quest_log_3_2(&self) -> Option<i32> {
        self.get_int(205)
    }

    pub fn set_player_quest_log_4_1(&mut self, v: i32) {
        self.set_int(207, v);
    }

    pub fn player_quest_log_4_1(&self) -> Option<i32> {
        self.get_int(207)
    }

    pub fn set_player_quest_log_4_2(&mut self, v: i32) {
        self.set_int(208, v);
    }

    pub fn player_quest_log_4_2(&self) -> Option<i32> {
        self.get_int(208)
    }

    pub fn set_player_quest_log_5_1(&mut self, v: i32) {
        self.set_int(210, v);
    }

    pub fn player_quest_log_5_1(&self) -> Option<i32> {
        self.get_int(210)
    }

    pub fn set_player_quest_log_5_2(&mut self, v: i32) {
        self.set_int(211, v);
    }

    pub fn player_quest_log_5_2(&self) -> Option<i32> {
        self.get_int(211)
    }

    pub fn set_player_quest_log_6_1(&mut self, v: i32) {
        self.set_int(213, v);
    }

    pub fn player_quest_log_6_1(&self) -> Option<i32> {
        self.get_int(213)
    }

    pub fn set_player_quest_log_6_2(&mut self, v: i32) {
        self.set_int(214, v);
    }

    pub fn player_quest_log_6_2(&self) -> Option<i32> {
        self.get_int(214)
    }

    pub fn set_player_quest_log_7_1(&mut self, v: i32) {
        self.set_int(216, v);
    }

    pub fn player_quest_log_7_1(&self) -> Option<i32> {
        self.get_int(216)
    }

    pub fn set_player_quest_log_7_2(&mut self, v: i32) {
        self.set_int(217, v);
    }

    pub fn player_quest_log_7_2(&self) -> Option<i32> {
        self.get_int(217)
    }

    pub fn set_player_quest_log_8_1(&mut self, v: i32) {
        self.set_int(219, v);
    }

    pub fn player_quest_log_8_1(&self) -> Option<i32> {
        self.get_int(219)
    }

    pub fn set_player_quest_log_8_2(&mut self, v: i32) {
        self.set_int(220, v);
    }

    pub fn player_quest_log_8_2(&self) -> Option<i32> {
        self.get_int(220)
    }

    pub fn set_player_quest_log_9_1(&mut self, v: i32) {
        self.set_int(222, v);
    }

    pub fn player_quest_log_9_1(&self) -> Option<i32> {
        self.get_int(222)
    }

    pub fn set_player_quest_log_9_2(&mut self, v: i32) {
        self.set_int(223, v);
    }

    pub fn player_quest_log_9_2(&self) -> Option<i32> {
        self.get_int(223)
    }

    pub fn set_player_quest_log_10_1(&mut self, v: i32) {
        self.set_int(225, v);
    }

    pub fn player_quest_log_10_1(&self) -> Option<i32> {
        self.get_int(225)
    }

    pub fn set_player_quest_log_10_2(&mut self, v: i32) {
        self.set_int(226, v);
    }

    pub fn player_quest_log_10_2(&self) -> Option<i32> {
        self.get_int(226)
    }

    pub fn set_player_quest_log_11_1(&mut self, v: i32) {
        self.set_int(228, v);
    }

    pub fn player_quest_log_11_1(&self) -> Option<i32> {
        self.get_int(228)
    }

    pub fn set_player_quest_log_11_2(&mut self, v: i32) {
        self.set_int(229, v);
    }

    pub fn player_quest_log_11_2(&self) -> Option<i32> {
        self.get_int(229)
    }

    pub fn set_player_quest_log_12_1(&mut self, v: i32) {
        self.set_int(231, v);
    }

    pub fn player_quest_log_12_1(&self) -> Option<i32> {
        self.get_int(231)
    }

    pub fn set_player_quest_log_12_2(&mut self, v: i32) {
        self.set_int(232, v);
    }

    pub fn player_quest_log_12_2(&self) -> Option<i32> {
        self.get_int(232)
    }

    pub fn set_player_quest_log_13_1(&mut self, v: i32) {
        self.set_int(234, v);
    }

    pub fn player_quest_log_13_1(&self) -> Option<i32> {
        self.get_int(234)
    }

    pub fn set_player_quest_log_13_2(&mut self, v: i32) {
        self.set_int(235, v);
    }

    pub fn player_quest_log_13_2(&self) -> Option<i32> {
        self.get_int(235)
    }

    pub fn set_player_quest_log_14_1(&mut self, v: i32) {
        self.set_int(237, v);
    }

    pub fn player_quest_log_14_1(&self) -> Option<i32> {
        self.get_int(237)
    }

    pub fn set_player_quest_log_14_2(&mut self, v: i32) {
        self.set_int(238, v);
    }

    pub fn player_quest_log_14_2(&self) -> Option<i32> {
        self.get_int(238)
    }

    pub fn set_player_quest_log_15_1(&mut self, v: i32) {
        self.set_int(240, v);
    }

    pub fn player_quest_log_15_1(&self) -> Option<i32> {
        self.get_int(240)
    }

    pub fn set_player_quest_log_15_2(&mut self, v: i32) {
        self.set_int(241, v);
    }

    pub fn player_quest_log_15_2(&self) -> Option<i32> {
        self.get_int(241)
    }

    pub fn set_player_quest_log_16_1(&mut self, v: i32) {
        self.set_int(243, v);
    }

    pub fn player_quest_log_16_1(&self) -> Option<i32> {
        self.get_int(243)
    }

    pub fn set_player_quest_log_16_2(&mut self, v: i32) {
        self.set_int(244, v);
    }

    pub fn player_quest_log_16_2(&self) -> Option<i32> {
        self.get_int(244)
    }

    pub fn set_player_quest_log_17_1(&mut self, v: i32) {
        self.set_int(246, v);
    }

    pub fn player_quest_log_17_1(&self) -> Option<i32> {
        self.get_int(246)
    }

    pub fn set_player_quest_log_17_2(&mut self, v: i32) {
        self.set_int(247, v);
    }

    pub fn player_quest_log_17_2(&self) -> Option<i32> {
        self.get_int(247)
    }

    pub fn set_player_quest_log_18_1(&mut self, v: i32) {
        self.set_int(249, v);
    }

    pub fn player_quest_log_18_1(&self) -> Option<i32> {
        self.get_int(249)
    }

    pub fn set_player_quest_log_18_2(&mut self, v: i32) {
        self.set_int(250, v);
    }

    pub fn player_quest_log_18_2(&self) -> Option<i32> {
        self.get_int(250)
    }

    pub fn set_player_quest_log_19_1(&mut self, v: i32) {
        self.set_int(252, v);
    }

    pub fn player_quest_log_19_1(&self) -> Option<i32> {
        self.get_int(252)
    }

    pub fn set_player_quest_log_19_2(&mut self, v: i32) {
        self.set_int(253, v);
    }

    pub fn player_quest_log_19_2(&self) -> Option<i32> {
        self.get_int(253)
    }

    pub fn set_player_quest_log_20_1(&mut self, v: i32) {
        self.set_int(255, v);
    }

    pub fn player_quest_log_20_1(&self) -> Option<i32> {
        self.get_int(255)
    }

    pub fn set_player_quest_log_20_2(&mut self, v: i32) {
        self.set_int(256, v);
    }

    pub fn player_quest_log_20_2(&self) -> Option<i32> {
        self.get_int(256)
    }

    pub fn set_player_visible_item(&mut self, visible_item: crate::vanilla::VisibleItem, index: VisibleItemIndex) {
        for (index, value) in visible_item.mask_values(index) {
            self.header_set(index, value);
        }
    }

    pub fn player_visible_item(&self, index: VisibleItemIndex) -> Option<crate::vanilla::VisibleItem> {
        crate::vanilla::VisibleItem::from_range(self.values.range(index.first()..=index.last()))
    }

    pub fn set_player_field_inv(&mut self, item_slot: crate::vanilla::ItemSlot, item: Guid) {
        let offset = 486 + item_slot.as_int() as u16 * 2;
        self.set_guid(offset, item);
    }

    pub fn player_field_inv(&self, item_slot: crate::vanilla::ItemSlot) -> Option<Guid> {
        let offset = 486 + item_slot.as_int() as u16 * 2;
        self.get_guid(offset)
    }

    pub fn set_player_farsight(&mut self, v: Guid) {
        self.set_guid(712, v);
    }

    pub fn player_farsight(&self) -> Option<Guid> {
        self.get_guid(712)
    }

    pub fn set_player_field_combo_target(&mut self, v: Guid) {
        self.set_guid(714, v);
    }

    pub fn player_field_combo_target(&self) -> Option<Guid> {
        self.get_guid(714)
    }

    pub fn set_player_xp(&mut self, v: i32) {
        self.set_int(716, v);
    }

    pub fn player_xp(&self) -> Option<i32> {
        self.get_int(716)
    }

    pub fn set_player_next_level_xp(&mut self, v: i32) {
        self.set_int(717, v);
    }

    pub fn player_next_level_xp(&self) -> Option<i32> {
        self.get_int(717)
    }

    pub fn set_player_skill_info(&mut self, skill_info: crate::vanilla::SkillInfo, index: SkillInfoIndex) {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
    }

    pub fn player_skill_info(&self, index: SkillInfoIndex) -> Option<crate::vanilla::SkillInfo> {
        crate::vanilla::SkillInfo::from_range(self.values.range(index.first()..=index.last()))
    }

    pub fn set_player_character_points1(&mut self, v: i32) {
        self.set_int(1102, v);
    }

    pub fn player_character_points1(&self) -> Option<i32> {
        self.get_int(1102)
    }

    pub fn set_player_character_points2(&mut self, v: i32) {
        self.set_int(1103, v);
    }

    pub fn player_character_points2(&self) -> Option<i32> {
        self.get_int(1103)
    }

    pub fn set_player_track_creatures(&mut self, v: i32) {
        self.set_int(1104, v);
    }

    pub fn player_track_creatures(&self) -> Option<i32> {
        self.get_int(1104)
    }

    pub fn set_player_track_resources(&mut self, v: i32) {
        self.set_int(1105, v);
    }

    pub fn player_track_resources(&self) -> Option<i32> {
        self.get_int(1105)
    }

    pub fn set_player_block_percentage(&mut self, v: f32) {
        self.set_float(1106, v);
    }

    pub fn player_block_percentage(&self) -> Option<f32> {
        self.get_float(1106)
    }

    pub fn set_player_dodge_percentage(&mut self, v: f32) {
        self.set_float(1107, v);
    }

    pub fn player_dodge_percentage(&self) -> Option<f32> {
        self.get_float(1107)
    }

    pub fn set_player_parry_percentage(&mut self, v: f32) {
        self.set_float(1108, v);
    }

    pub fn player_parry_percentage(&self) -> Option<f32> {
        self.get_float(1108)
    }

    pub fn set_player_crit_percentage(&mut self, v: f32) {
        self.set_float(1109, v);
    }

    pub fn player_crit_percentage(&self) -> Option<f32> {
        self.get_float(1109)
    }

    pub fn set_player_ranged_crit_percentage(&mut self, v: f32) {
        self.set_float(1110, v);
    }

    pub fn player_ranged_crit_percentage(&self) -> Option<f32> {
        self.get_float(1110)
    }

    pub fn set_player_explored_zones_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(1111, a, b, c, d);
    }

    pub fn player_explored_zones_1(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(1111)
    }

    pub fn set_player_rest_state_experience(&mut self, v: i32) {
        self.set_int(1175, v);
    }

    pub fn player_rest_state_experience(&self) -> Option<i32> {
        self.get_int(1175)
    }

    pub fn set_player_field_coinage(&mut self, v: i32) {
        self.set_int(1176, v);
    }

    pub fn player_field_coinage(&self) -> Option<i32> {
        self.get_int(1176)
    }

    pub fn set_player_field_posstat0(&mut self, v: i32) {
        self.set_int(1177, v);
    }

    pub fn player_field_posstat0(&self) -> Option<i32> {
        self.get_int(1177)
    }

    pub fn set_player_field_posstat1(&mut self, v: i32) {
        self.set_int(1178, v);
    }

    pub fn player_field_posstat1(&self) -> Option<i32> {
        self.get_int(1178)
    }

    pub fn set_player_field_posstat2(&mut self, v: i32) {
        self.set_int(1179, v);
    }

    pub fn player_field_posstat2(&self) -> Option<i32> {
        self.get_int(1179)
    }

    pub fn set_player_field_posstat3(&mut self, v: i32) {
        self.set_int(1180, v);
    }

    pub fn player_field_posstat3(&self) -> Option<i32> {
        self.get_int(1180)
    }

    pub fn set_player_field_posstat4(&mut self, v: i32) {
        self.set_int(1181, v);
    }

    pub fn player_field_posstat4(&self) -> Option<i32> {
        self.get_int(1181)
    }

    pub fn set_player_field_negstat0(&mut self, v: i32) {
        self.set_int(1182, v);
    }

    pub fn player_field_negstat0(&self) -> Option<i32> {
        self.get_int(1182)
    }

    pub fn set_player_field_negstat1(&mut self, v: i32) {
        self.set_int(1183, v);
    }

    pub fn player_field_negstat1(&self) -> Option<i32> {
        self.get_int(1183)
    }

    pub fn set_player_field_negstat2(&mut self, v: i32) {
        self.set_int(1184, v);
    }

    pub fn player_field_negstat2(&self) -> Option<i32> {
        self.get_int(1184)
    }

    pub fn set_player_field_negstat3(&mut self, v: i32) {
        self.set_int(1185, v);
    }

    pub fn player_field_negstat3(&self) -> Option<i32> {
        self.get_int(1185)
    }

    pub fn set_player_field_negstat4(&mut self, v: i32) {
        self.set_int(1186, v);
    }

    pub fn player_field_negstat4(&self) -> Option<i32> {
        self.get_int(1186)
    }

    pub fn set_player_field_resistancebuffmodspositive(&mut self, v: i32) {
        self.set_int(1187, v);
    }

    pub fn player_field_resistancebuffmodspositive(&self) -> Option<i32> {
        self.get_int(1187)
    }

    pub fn set_player_field_resistancebuffmodsnegative(&mut self, v: i32) {
        self.set_int(1194, v);
    }

    pub fn player_field_resistancebuffmodsnegative(&self) -> Option<i32> {
        self.get_int(1194)
    }

    pub fn set_player_field_mod_damage_done_pos(&mut self, v: i32) {
        self.set_int(1201, v);
    }

    pub fn player_field_mod_damage_done_pos(&self) -> Option<i32> {
        self.get_int(1201)
    }

    pub fn set_player_field_mod_damage_done_neg(&mut self, v: i32) {
        self.set_int(1208, v);
    }

    pub fn player_field_mod_damage_done_neg(&self) -> Option<i32> {
        self.get_int(1208)
    }

    pub fn set_player_field_mod_damage_done_pct(&mut self, v: i32) {
        self.set_int(1215, v);
    }

    pub fn player_field_mod_damage_done_pct(&self) -> Option<i32> {
        self.get_int(1215)
    }

    pub fn set_player_field_bytes(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(1222, a, b, c, d);
    }

    pub fn player_field_bytes(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(1222)
    }

    pub fn set_player_ammo_id(&mut self, v: i32) {
        self.set_int(1223, v);
    }

    pub fn player_ammo_id(&self) -> Option<i32> {
        self.get_int(1223)
    }

    pub fn set_player_self_res_spell(&mut self, v: i32) {
        self.set_int(1224, v);
    }

    pub fn player_self_res_spell(&self) -> Option<i32> {
        self.get_int(1224)
    }

    pub fn set_player_field_pvp_medals(&mut self, v: i32) {
        self.set_int(1225, v);
    }

    pub fn player_field_pvp_medals(&self) -> Option<i32> {
        self.get_int(1225)
    }

    pub fn set_player_field_buyback_price_1(&mut self, v: i32) {
        self.set_int(1226, v);
    }

    pub fn player_field_buyback_price_1(&self) -> Option<i32> {
        self.get_int(1226)
    }

    pub fn set_player_field_buyback_timestamp_1(&mut self, v: i32) {
        self.set_int(1238, v);
    }

    pub fn player_field_buyback_timestamp_1(&self) -> Option<i32> {
        self.get_int(1238)
    }

    pub fn set_player_field_session_kills(&mut self, a: u16, b: u16) {
        self.set_shorts(1250, a, b);
    }

    pub fn player_field_session_kills(&self) -> Option<(u16, u16)> {
        self.get_shorts(1250)
    }

    pub fn set_player_field_yesterday_kills(&mut self, a: u16, b: u16) {
        self.set_shorts(1251, a, b);
    }

    pub fn player_field_yesterday_kills(&self) -> Option<(u16, u16)> {
        self.get_shorts(1251)
    }

    pub fn set_player_field_last_week_kills(&mut self, a: u16, b: u16) {
        self.set_shorts(1252, a, b);
    }

    pub fn player_field_last_week_kills(&self) -> Option<(u16, u16)> {
        self.get_shorts(1252)
    }

    pub fn set_player_field_this_week_kills(&mut self, a: u16, b: u16) {
        self.set_shorts(1253, a, b);
    }

    pub fn player_field_this_week_kills(&self) -> Option<(u16, u16)> {
        self.get_shorts(1253)
    }

    pub fn set_player_field_this_week_contribution(&mut self, v: i32) {
        self.set_int(1254, v);
    }

    pub fn player_field_this_week_contribution(&self) -> Option<i32> {
        self.get_int(1254)
    }

    pub fn set_player_field_lifetime_honorbale_kills(&mut self, v: i32) {
        self.set_int(1255, v);
    }

    pub fn player_field_lifetime_honorbale_kills(&self) -> Option<i32> {
        self.get_int(1255)
    }

    pub fn set_player_field_lifetime_dishonorbale_kills(&mut self, v: i32) {
        self.set_int(1256, v);
    }

    pub fn player_field_lifetime_dishonorbale_kills(&self) -> Option<i32> {
        self.get_int(1256)
    }

    pub fn set_player_field_yesterday_contribution(&mut self, v: i32) {
        self.set_int(1257, v);
    }

    pub fn player_field_yesterday_contribution(&self) -> Option<i32> {
        self.get_int(1257)
    }

    pub fn set_player_field_last_week_contribution(&mut self, v: i32) {
        self.set_int(1258, v);
    }

    pub fn player_field_last_week_contribution(&self) -> Option<i32> {
        self.get_int(1258)
    }

    pub fn set_player_field_last_week_rank(&mut self, v: i32) {
        self.set_int(1259, v);
    }

    pub fn player_field_last_week_rank(&self) -> Option<i32> {
        self.get_int(1259)
    }

    pub fn set_player_field_bytes2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(1260, a, b, c, d);
    }

    pub fn player_field_bytes2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(1260)
    }

    pub fn set_player_field_watched_faction_index(&mut self, v: i32) {
        self.set_int(1261, v);
    }

    pub fn player_field_watched_faction_index(&self) -> Option<i32> {
        self.get_int(1261)
    }

    pub fn set_player_field_combat_rating_1(&mut self, v: i32) {
        self.set_int(1262, v);
    }

    pub fn player_field_combat_rating_1(&self) -> Option<i32> {
        self.get_int(1262)
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

    pub fn set_gameobject_created_by(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn gameobject_created_by(&self) -> Option<Guid> {
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

    pub fn set_corpse_owner(&mut self, v: Guid) {
        self.set_guid(6, v);
    }

    pub fn corpse_owner(&self) -> Option<Guid> {
        self.get_guid(6)
    }

    pub fn set_corpse_facing(&mut self, v: f32) {
        self.set_float(8, v);
    }

    pub fn corpse_facing(&self) -> Option<f32> {
        self.get_float(8)
    }

    pub fn set_corpse_pos_x(&mut self, v: f32) {
        self.set_float(9, v);
    }

    pub fn corpse_pos_x(&self) -> Option<f32> {
        self.get_float(9)
    }

    pub fn set_corpse_pos_y(&mut self, v: f32) {
        self.set_float(10, v);
    }

    pub fn corpse_pos_y(&self) -> Option<f32> {
        self.get_float(10)
    }

    pub fn set_corpse_pos_z(&mut self, v: f32) {
        self.set_float(11, v);
    }

    pub fn corpse_pos_z(&self) -> Option<f32> {
        self.get_float(11)
    }

    pub fn set_corpse_display_id(&mut self, v: i32) {
        self.set_int(12, v);
    }

    pub fn corpse_display_id(&self) -> Option<i32> {
        self.get_int(12)
    }

    pub fn set_corpse_item(&mut self, v: i32) {
        self.set_int(13, v);
    }

    pub fn corpse_item(&self) -> Option<i32> {
        self.get_int(13)
    }

    pub fn set_corpse_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(32, a, b, c, d);
    }

    pub fn corpse_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(32)
    }

    pub fn set_corpse_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.set_bytes(33, a, b, c, d);
    }

    pub fn corpse_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        self.get_bytes(33)
    }

    pub fn set_corpse_guild(&mut self, v: i32) {
        self.set_int(34, v);
    }

    pub fn corpse_guild(&self) -> Option<i32> {
        self.get_int(34)
    }

    pub fn set_corpse_flags(&mut self, v: i32) {
        self.set_int(35, v);
    }

    pub fn corpse_flags(&self) -> Option<i32> {
        self.get_int(35)
    }

    pub fn set_corpse_dynamic_flags(&mut self, v: i32) {
        self.set_int(36, v);
    }

    pub fn corpse_dynamic_flags(&self) -> Option<i32> {
        self.get_int(36)
    }

}

