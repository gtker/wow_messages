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

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_item_text_id(mut self, v: i32) -> Self {
        self.header_set(57, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
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

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_item_text_id(mut self, v: i32) -> Self {
        self.header_set(57, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_container_num_slots(mut self, v: i32) -> Self {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
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
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power1(mut self, v: i32) -> Self {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power2(mut self, v: i32) -> Self {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power3(mut self, v: i32) -> Self {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power4(mut self, v: i32) -> Self {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power5(mut self, v: i32) -> Self {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxhealth(mut self, v: i32) -> Self {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower1(mut self, v: i32) -> Self {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower2(mut self, v: i32) -> Self {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower3(mut self, v: i32) -> Self {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower4(mut self, v: i32) -> Self {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower5(mut self, v: i32) -> Self {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_level(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_factiontemplate(mut self, v: i32) -> Self {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_virtual_item_slot_display(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_virtual_item_info(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_flags(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_flags_2(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_aura(mut self, v: i32) -> Self {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_auraflags(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(104, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_auralevels(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(118, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_auraapplications(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(132, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.header_set(149, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(159, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.header_set(164, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.header_set(166, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.header_set(169, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_training_points(mut self, a: u16, b: u16) -> Self {
        self.header_set(170, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.header_set(174, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.header_set(175, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat1(mut self, v: i32) -> Self {
        self.header_set(177, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat2(mut self, v: i32) -> Self {
        self.header_set(178, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat3(mut self, v: i32) -> Self {
        self.header_set(179, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat1(mut self, v: i32) -> Self {
        self.header_set(182, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat2(mut self, v: i32) -> Self {
        self.header_set(183, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat3(mut self, v: i32) -> Self {
        self.header_set(184, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_resistances(mut self, v: i32) -> Self {
        self.header_set(186, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(209, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(211, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(212, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(214, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(215, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.header_set(218, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxhealthmodifier(mut self, v: f32) -> Self {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
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
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power1(mut self, v: i32) -> Self {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power2(mut self, v: i32) -> Self {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power3(mut self, v: i32) -> Self {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power4(mut self, v: i32) -> Self {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power5(mut self, v: i32) -> Self {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxhealth(mut self, v: i32) -> Self {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower1(mut self, v: i32) -> Self {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower2(mut self, v: i32) -> Self {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower3(mut self, v: i32) -> Self {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower4(mut self, v: i32) -> Self {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxpower5(mut self, v: i32) -> Self {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_level(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_factiontemplate(mut self, v: i32) -> Self {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_virtual_item_slot_display(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_virtual_item_info(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_flags(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_flags_2(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_aura(mut self, v: i32) -> Self {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_auraflags(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(104, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_auralevels(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(118, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_auraapplications(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(132, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.header_set(149, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(159, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.header_set(164, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.header_set(166, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.header_set(169, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_training_points(mut self, a: u16, b: u16) -> Self {
        self.header_set(170, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.header_set(174, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.header_set(175, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_posstat0(mut self, v: i32) -> Self {
        self.header_set(176, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat1(mut self, v: i32) -> Self {
        self.header_set(177, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat2(mut self, v: i32) -> Self {
        self.header_set(178, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_posstat3(mut self, v: i32) -> Self {
        self.header_set(179, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_posstat4(mut self, v: i32) -> Self {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_negstat0(mut self, v: i32) -> Self {
        self.header_set(181, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat1(mut self, v: i32) -> Self {
        self.header_set(182, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat2(mut self, v: i32) -> Self {
        self.header_set(183, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_negstat3(mut self, v: i32) -> Self {
        self.header_set(184, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_negstat4(mut self, v: i32) -> Self {
        self.header_set(185, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_resistances(mut self, v: i32) -> Self {
        self.header_set(186, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_resistancebuffmodspositive(mut self, v: i32) -> Self {
        self.header_set(193, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_resistancebuffmodsnegative(mut self, v: i32) -> Self {
        self.header_set(200, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(209, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(211, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(212, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(214, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(215, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.header_set(218, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxhealthmodifier(mut self, v: f32) -> Self {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_duel_arbiter(mut self, v: Guid) -> Self {
        self.set_guid(234, v);
        self
    }

    pub fn set_player_flags(mut self, v: i32) -> Self {
        self.header_set(236, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_guildid(mut self, v: i32) -> Self {
        self.header_set(237, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_guildrank(mut self, v: i32) -> Self {
        self.header_set(238, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_bytes(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(239, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(240, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_bytes_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(241, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_duel_team(mut self, v: i32) -> Self {
        self.header_set(242, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_guild_timestamp(mut self, v: i32) -> Self {
        self.header_set(243, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_1_1(mut self, v: i32) -> Self {
        self.header_set(244, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_1_2(mut self, v: i32) -> Self {
        self.header_set(245, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_1_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(246, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_1_4(mut self, v: i32) -> Self {
        self.header_set(247, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_2_1(mut self, v: i32) -> Self {
        self.header_set(248, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_2_2(mut self, v: i32) -> Self {
        self.header_set(249, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_2_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(250, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_2_4(mut self, v: i32) -> Self {
        self.header_set(251, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_3_1(mut self, v: i32) -> Self {
        self.header_set(252, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_3_2(mut self, v: i32) -> Self {
        self.header_set(253, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_3_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(254, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_3_4(mut self, v: i32) -> Self {
        self.header_set(255, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_4_1(mut self, v: i32) -> Self {
        self.header_set(256, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_4_2(mut self, v: i32) -> Self {
        self.header_set(257, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_4_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(258, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_4_4(mut self, v: i32) -> Self {
        self.header_set(259, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_5_1(mut self, v: i32) -> Self {
        self.header_set(260, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_5_2(mut self, v: i32) -> Self {
        self.header_set(261, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_5_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(262, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_5_4(mut self, v: i32) -> Self {
        self.header_set(263, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_6_1(mut self, v: i32) -> Self {
        self.header_set(264, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_6_2(mut self, v: i32) -> Self {
        self.header_set(265, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_6_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(266, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_6_4(mut self, v: i32) -> Self {
        self.header_set(267, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_7_1(mut self, v: i32) -> Self {
        self.header_set(268, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_7_2(mut self, v: i32) -> Self {
        self.header_set(269, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_7_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(270, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_7_4(mut self, v: i32) -> Self {
        self.header_set(271, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_8_1(mut self, v: i32) -> Self {
        self.header_set(272, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_8_2(mut self, v: i32) -> Self {
        self.header_set(273, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_8_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(274, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_8_4(mut self, v: i32) -> Self {
        self.header_set(275, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_9_1(mut self, v: i32) -> Self {
        self.header_set(276, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_9_2(mut self, v: i32) -> Self {
        self.header_set(277, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_9_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(278, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_9_4(mut self, v: i32) -> Self {
        self.header_set(279, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_10_1(mut self, v: i32) -> Self {
        self.header_set(280, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_10_2(mut self, v: i32) -> Self {
        self.header_set(281, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_10_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(282, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_10_4(mut self, v: i32) -> Self {
        self.header_set(283, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_11_1(mut self, v: i32) -> Self {
        self.header_set(284, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_11_2(mut self, v: i32) -> Self {
        self.header_set(285, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_11_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(286, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_11_4(mut self, v: i32) -> Self {
        self.header_set(287, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_12_1(mut self, v: i32) -> Self {
        self.header_set(288, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_12_2(mut self, v: i32) -> Self {
        self.header_set(289, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_12_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(290, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_12_4(mut self, v: i32) -> Self {
        self.header_set(291, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_13_1(mut self, v: i32) -> Self {
        self.header_set(292, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_13_2(mut self, v: i32) -> Self {
        self.header_set(293, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_13_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(294, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_13_4(mut self, v: i32) -> Self {
        self.header_set(295, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_14_1(mut self, v: i32) -> Self {
        self.header_set(296, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_14_2(mut self, v: i32) -> Self {
        self.header_set(297, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_14_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(298, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_14_4(mut self, v: i32) -> Self {
        self.header_set(299, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_15_1(mut self, v: i32) -> Self {
        self.header_set(300, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_15_2(mut self, v: i32) -> Self {
        self.header_set(301, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_15_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(302, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_15_4(mut self, v: i32) -> Self {
        self.header_set(303, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_16_1(mut self, v: i32) -> Self {
        self.header_set(304, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_16_2(mut self, v: i32) -> Self {
        self.header_set(305, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_16_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(306, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_16_4(mut self, v: i32) -> Self {
        self.header_set(307, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_17_1(mut self, v: i32) -> Self {
        self.header_set(308, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_17_2(mut self, v: i32) -> Self {
        self.header_set(309, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_17_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(310, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_17_4(mut self, v: i32) -> Self {
        self.header_set(311, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_18_1(mut self, v: i32) -> Self {
        self.header_set(312, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_18_2(mut self, v: i32) -> Self {
        self.header_set(313, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_18_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(314, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_18_4(mut self, v: i32) -> Self {
        self.header_set(315, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_19_1(mut self, v: i32) -> Self {
        self.header_set(316, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_19_2(mut self, v: i32) -> Self {
        self.header_set(317, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_19_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(318, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_19_4(mut self, v: i32) -> Self {
        self.header_set(319, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_20_1(mut self, v: i32) -> Self {
        self.header_set(320, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_20_2(mut self, v: i32) -> Self {
        self.header_set(321, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_20_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(322, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_20_4(mut self, v: i32) -> Self {
        self.header_set(323, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_21_1(mut self, v: i32) -> Self {
        self.header_set(324, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_21_2(mut self, v: i32) -> Self {
        self.header_set(325, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_21_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(326, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_21_4(mut self, v: i32) -> Self {
        self.header_set(327, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_22_1(mut self, v: i32) -> Self {
        self.header_set(328, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_22_2(mut self, v: i32) -> Self {
        self.header_set(329, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_22_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(330, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_22_4(mut self, v: i32) -> Self {
        self.header_set(331, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_23_1(mut self, v: i32) -> Self {
        self.header_set(332, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_23_2(mut self, v: i32) -> Self {
        self.header_set(333, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_23_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(334, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_23_4(mut self, v: i32) -> Self {
        self.header_set(335, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_24_1(mut self, v: i32) -> Self {
        self.header_set(336, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_24_2(mut self, v: i32) -> Self {
        self.header_set(337, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_24_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(338, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_24_4(mut self, v: i32) -> Self {
        self.header_set(339, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_25_1(mut self, v: i32) -> Self {
        self.header_set(340, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_25_2(mut self, v: i32) -> Self {
        self.header_set(341, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_25_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(342, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_quest_log_25_4(mut self, v: i32) -> Self {
        self.header_set(343, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_1_creator(mut self, v: Guid) -> Self {
        self.set_guid(344, v);
        self
    }

    pub fn set_player_visible_item_1_0(mut self, v: i32) -> Self {
        self.header_set(346, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_1_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(358, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_2_creator(mut self, v: Guid) -> Self {
        self.set_guid(360, v);
        self
    }

    pub fn set_player_visible_item_2_0(mut self, v: i32) -> Self {
        self.header_set(362, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_2_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(374, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_3_creator(mut self, v: Guid) -> Self {
        self.set_guid(376, v);
        self
    }

    pub fn set_player_visible_item_3_0(mut self, v: i32) -> Self {
        self.header_set(378, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_3_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(390, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_4_creator(mut self, v: Guid) -> Self {
        self.set_guid(392, v);
        self
    }

    pub fn set_player_visible_item_4_0(mut self, v: i32) -> Self {
        self.header_set(394, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_4_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(406, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_5_creator(mut self, v: Guid) -> Self {
        self.set_guid(408, v);
        self
    }

    pub fn set_player_visible_item_5_0(mut self, v: i32) -> Self {
        self.header_set(410, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_5_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(422, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_6_creator(mut self, v: Guid) -> Self {
        self.set_guid(424, v);
        self
    }

    pub fn set_player_visible_item_6_0(mut self, v: i32) -> Self {
        self.header_set(426, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_6_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(438, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_7_creator(mut self, v: Guid) -> Self {
        self.set_guid(440, v);
        self
    }

    pub fn set_player_visible_item_7_0(mut self, v: i32) -> Self {
        self.header_set(442, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_7_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(454, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_8_creator(mut self, v: Guid) -> Self {
        self.set_guid(456, v);
        self
    }

    pub fn set_player_visible_item_8_0(mut self, v: i32) -> Self {
        self.header_set(458, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_8_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(470, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_9_creator(mut self, v: Guid) -> Self {
        self.set_guid(472, v);
        self
    }

    pub fn set_player_visible_item_9_0(mut self, v: i32) -> Self {
        self.header_set(474, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_9_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(486, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_10_creator(mut self, v: Guid) -> Self {
        self.set_guid(488, v);
        self
    }

    pub fn set_player_visible_item_10_0(mut self, v: i32) -> Self {
        self.header_set(490, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_10_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(502, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_11_creator(mut self, v: Guid) -> Self {
        self.set_guid(504, v);
        self
    }

    pub fn set_player_visible_item_11_0(mut self, v: i32) -> Self {
        self.header_set(506, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_11_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(518, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_12_creator(mut self, v: Guid) -> Self {
        self.set_guid(520, v);
        self
    }

    pub fn set_player_visible_item_12_0(mut self, v: i32) -> Self {
        self.header_set(522, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_12_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(534, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_13_creator(mut self, v: Guid) -> Self {
        self.set_guid(536, v);
        self
    }

    pub fn set_player_visible_item_13_0(mut self, v: i32) -> Self {
        self.header_set(538, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_13_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(550, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_14_creator(mut self, v: Guid) -> Self {
        self.set_guid(552, v);
        self
    }

    pub fn set_player_visible_item_14_0(mut self, v: i32) -> Self {
        self.header_set(554, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_14_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(566, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_15_creator(mut self, v: Guid) -> Self {
        self.set_guid(568, v);
        self
    }

    pub fn set_player_visible_item_15_0(mut self, v: i32) -> Self {
        self.header_set(570, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_15_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(582, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_16_creator(mut self, v: Guid) -> Self {
        self.set_guid(584, v);
        self
    }

    pub fn set_player_visible_item_16_0(mut self, v: i32) -> Self {
        self.header_set(586, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_16_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(598, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_17_creator(mut self, v: Guid) -> Self {
        self.set_guid(600, v);
        self
    }

    pub fn set_player_visible_item_17_0(mut self, v: i32) -> Self {
        self.header_set(602, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_17_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(614, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_18_creator(mut self, v: Guid) -> Self {
        self.set_guid(616, v);
        self
    }

    pub fn set_player_visible_item_18_0(mut self, v: i32) -> Self {
        self.header_set(618, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_18_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(630, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_19_creator(mut self, v: Guid) -> Self {
        self.set_guid(632, v);
        self
    }

    pub fn set_player_visible_item_19_0(mut self, v: i32) -> Self {
        self.header_set(634, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_19_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(646, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_chosen_title(mut self, v: i32) -> Self {
        self.header_set(648, u32::from_le_bytes(v.to_le_bytes()));
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
        self.header_set(926, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_next_level_xp(mut self, v: i32) -> Self {
        self.header_set(927, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_skill_info(mut self, skill_info: crate::tbc::SkillInfo, index: SkillInfoIndex) -> Self {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
        self
    }

    pub fn set_player_character_points1(mut self, v: i32) -> Self {
        self.header_set(1312, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_character_points2(mut self, v: i32) -> Self {
        self.header_set(1313, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_track_creatures(mut self, v: i32) -> Self {
        self.header_set(1314, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_track_resources(mut self, v: i32) -> Self {
        self.header_set(1315, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_block_percentage(mut self, v: f32) -> Self {
        self.header_set(1316, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_dodge_percentage(mut self, v: f32) -> Self {
        self.header_set(1317, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_parry_percentage(mut self, v: f32) -> Self {
        self.header_set(1318, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_expertise(mut self, v: i32) -> Self {
        self.header_set(1319, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_offhand_expertise(mut self, v: i32) -> Self {
        self.header_set(1320, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_crit_percentage(mut self, v: f32) -> Self {
        self.header_set(1321, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_ranged_crit_percentage(mut self, v: f32) -> Self {
        self.header_set(1322, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_offhand_crit_percentage(mut self, v: f32) -> Self {
        self.header_set(1323, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_spell_crit_percentage1(mut self, v: f32) -> Self {
        self.header_set(1324, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_shield_block(mut self, v: i32) -> Self {
        self.header_set(1331, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_explored_zones_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1332, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_rest_state_experience(mut self, v: i32) -> Self {
        self.header_set(1460, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_coinage(mut self, v: i32) -> Self {
        self.header_set(1461, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_damage_done_pos(mut self, v: i32) -> Self {
        self.header_set(1462, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_damage_done_neg(mut self, v: i32) -> Self {
        self.header_set(1469, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_damage_done_pct(mut self, v: i32) -> Self {
        self.header_set(1476, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_healing_done_pos(mut self, v: i32) -> Self {
        self.header_set(1483, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_target_resistance(mut self, v: i32) -> Self {
        self.header_set(1484, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_target_physical_resistance(mut self, v: i32) -> Self {
        self.header_set(1485, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_features(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1486, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_ammo_id(mut self, v: i32) -> Self {
        self.header_set(1487, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_self_res_spell(mut self, v: i32) -> Self {
        self.header_set(1488, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_pvp_medals(mut self, v: i32) -> Self {
        self.header_set(1489, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_buyback_price_1(mut self, v: i32) -> Self {
        self.header_set(1490, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_buyback_timestamp_1(mut self, v: i32) -> Self {
        self.header_set(1502, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_kills(mut self, a: u16, b: u16) -> Self {
        self.header_set(1514, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_today_contribution(mut self, v: i32) -> Self {
        self.header_set(1515, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_yesterday_contribution(mut self, v: i32) -> Self {
        self.header_set(1516, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_lifetime_honorable_kills(mut self, v: i32) -> Self {
        self.header_set(1517, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_bytes2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1518, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_watched_faction_index(mut self, v: i32) -> Self {
        self.header_set(1519, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_combat_rating_1(mut self, v: i32) -> Self {
        self.header_set(1520, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_arena_team_info_1_1(mut self, v: i32) -> Self {
        self.header_set(1544, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_honor_currency(mut self, v: i32) -> Self {
        self.header_set(1562, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_arena_currency(mut self, v: i32) -> Self {
        self.header_set(1563, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_mana_regen(mut self, v: f32) -> Self {
        self.header_set(1564, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_mod_mana_regen_interrupt(mut self, v: f32) -> Self {
        self.header_set(1565, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_max_level(mut self, v: i32) -> Self {
        self.header_set(1566, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_daily_quests_1(mut self, v: i32) -> Self {
        self.header_set(1567, u32::from_le_bytes(v.to_le_bytes()));
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
        self.set_guid(6, v);
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

    pub fn set_gameobject_rotation(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_state(mut self, v: i32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_pos_x(mut self, v: f32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_pos_y(mut self, v: f32) -> Self {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_pos_z(mut self, v: f32) -> Self {
        self.header_set(17, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_facing(mut self, v: f32) -> Self {
        self.header_set(18, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_dyn_flags(mut self, v: i32) -> Self {
        self.header_set(19, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_faction(mut self, v: i32) -> Self {
        self.header_set(20, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_type_id(mut self, v: i32) -> Self {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_level(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_artkit(mut self, v: i32) -> Self {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_animprogress(mut self, v: i32) -> Self {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateDynamicObjectBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.set_guid(0, v);
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
        self.set_guid(6, v);
        self
    }

    pub fn set_dynamicobject_caster(mut self, v: Guid) -> Self {
        self.set_guid(6, v);
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

    pub fn set_dynamicobject_pos_x(mut self, v: f32) -> Self {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_pos_y(mut self, v: f32) -> Self {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_pos_z(mut self, v: f32) -> Self {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_facing(mut self, v: f32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_casttime(mut self, v: i32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateCorpseBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.set_guid(0, v);
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
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_pos_x(mut self, v: f32) -> Self {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_pos_y(mut self, v: f32) -> Self {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_pos_z(mut self, v: f32) -> Self {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_display_id(mut self, v: i32) -> Self {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_item(mut self, v: i32) -> Self {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(34, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(35, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_guild(mut self, v: i32) -> Self {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_flags(mut self, v: i32) -> Self {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_dynamic_flags(mut self, v: i32) -> Self {
        self.header_set(38, u32::from_le_bytes(v.to_le_bytes()));
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

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.values.get(&56).map(|v| *v as i32)
    }

    pub fn set_item_item_text_id(&mut self, v: i32) {
        self.header_set(57, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_item_text_id(&self) -> Option<i32> {
        self.values.get(&57).map(|v| *v as i32)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.values.get(&58).map(|v| *v as i32)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
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

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.header_set(55, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.values.get(&55).map(|v| *v as i32)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.header_set(56, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.values.get(&56).map(|v| *v as i32)
    }

    pub fn set_item_item_text_id(&mut self, v: i32) {
        self.header_set(57, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_item_text_id(&self) -> Option<i32> {
        self.values.get(&57).map(|v| *v as i32)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.header_set(58, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.values.get(&58).map(|v| *v as i32)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.header_set(59, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.values.get(&59).map(|v| *v as i32)
    }

    pub fn set_container_num_slots(&mut self, v: i32) {
        self.header_set(60, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn container_num_slots(&self) -> Option<i32> {
        self.values.get(&60).map(|v| *v as i32)
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
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_health(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_power1(&mut self, v: i32) {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power1(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_unit_power2(&mut self, v: i32) {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power2(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_power3(&mut self, v: i32) {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power3(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_power4(&mut self, v: i32) {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power4(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_power5(&mut self, v: i32) {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power5(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_maxhealth(&mut self, v: i32) {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxhealth(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower1(&mut self, v: i32) {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower1(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower2(&mut self, v: i32) {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower2(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower3(&mut self, v: i32) {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower3(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower4(&mut self, v: i32) {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower4(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower5(&mut self, v: i32) {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower5(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_level(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_level(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_factiontemplate(&mut self, v: i32) {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_factiontemplate(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_unit_bytes_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
    }

    pub fn unit_bytes_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_virtual_item_slot_display(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_virtual_item_slot_display(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_unit_virtual_item_info(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_virtual_item_info(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&40) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_flags(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_flags(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_unit_flags_2(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_flags_2(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
    }

    pub fn set_unit_aura(&mut self, v: i32) {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_aura(&self) -> Option<i32> {
        self.values.get(&48).map(|v| *v as i32)
    }

    pub fn set_unit_auraflags(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(104, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auraflags(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&104) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_auralevels(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(118, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auralevels(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&118) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_auraapplications(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(132, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auraapplications(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&132) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.values.get(&146).map(|v| *v as i32)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.values.get(&147).map(|v| *v as i32)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.header_set(149, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.values.get(&149).map(|v| *v as i32)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.values.get(&150).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.values.get(&151).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.values.get(&153).map(|v| *v as i32)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.values.get(&154).map(|v| *v as i32)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.values.get(&155).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.values.get(&156).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.values.get(&157).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.values.get(&158).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(159, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&159) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.values.get(&160).map(|v| *v as i32)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.values.get(&161).map(|v| *v as i32)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.values.get(&163).map(|v| *v as i32)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.header_set(164, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.values.get(&164).map(|v| *v as i32)
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.values.get(&165).map(|v| *v as i32)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.header_set(166, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.values.get(&166).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.values.get(&167).map(|v| *v as i32)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.header_set(169, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.values.get(&169).map(|v| *v as i32)
    }

    pub fn set_unit_training_points(&mut self, a: u16, b: u16) {
        self.header_set(170, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_training_points(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&170) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.values.get(&171).map(|v| *v as i32)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.values.get(&172).map(|v| *v as i32)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.values.get(&173).map(|v| *v as i32)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.header_set(174, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.values.get(&174).map(|v| *v as i32)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.header_set(175, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.values.get(&175).map(|v| *v as i32)
    }

    pub fn set_unit_posstat1(&mut self, v: i32) {
        self.header_set(177, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat1(&self) -> Option<i32> {
        self.values.get(&177).map(|v| *v as i32)
    }

    pub fn set_unit_posstat2(&mut self, v: i32) {
        self.header_set(178, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat2(&self) -> Option<i32> {
        self.values.get(&178).map(|v| *v as i32)
    }

    pub fn set_unit_posstat3(&mut self, v: i32) {
        self.header_set(179, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat3(&self) -> Option<i32> {
        self.values.get(&179).map(|v| *v as i32)
    }

    pub fn set_unit_negstat1(&mut self, v: i32) {
        self.header_set(182, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat1(&self) -> Option<i32> {
        self.values.get(&182).map(|v| *v as i32)
    }

    pub fn set_unit_negstat2(&mut self, v: i32) {
        self.header_set(183, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat2(&self) -> Option<i32> {
        self.values.get(&183).map(|v| *v as i32)
    }

    pub fn set_unit_negstat3(&mut self, v: i32) {
        self.header_set(184, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat3(&self) -> Option<i32> {
        self.values.get(&184).map(|v| *v as i32)
    }

    pub fn set_unit_resistances(&mut self, v: i32) {
        self.header_set(186, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_resistances(&self) -> Option<i32> {
        self.values.get(&186).map(|v| *v as i32)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.values.get(&207).map(|v| *v as i32)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.values.get(&208).map(|v| *v as i32)
    }

    pub fn set_unit_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(209, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&209) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.values.get(&210).map(|v| *v as i32)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(211, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&211) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(212, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&212).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.values.get(&213).map(|v| *v as i32)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(214, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&214) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(215, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&215).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.values.get(&216).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.values.get(&217).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.header_set(218, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.values.get(&218).map(|v| *v as i32)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.values.get(&225).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxhealthmodifier(&mut self, v: f32) {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxhealthmodifier(&self) -> Option<f32> {
        self.values.get(&232).map(|v| f32::from_le_bytes(v.to_le_bytes()))
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
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_health(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_unit_power1(&mut self, v: i32) {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power1(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_unit_power2(&mut self, v: i32) {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power2(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
    }

    pub fn set_unit_power3(&mut self, v: i32) {
        self.header_set(25, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power3(&self) -> Option<i32> {
        self.values.get(&25).map(|v| *v as i32)
    }

    pub fn set_unit_power4(&mut self, v: i32) {
        self.header_set(26, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power4(&self) -> Option<i32> {
        self.values.get(&26).map(|v| *v as i32)
    }

    pub fn set_unit_power5(&mut self, v: i32) {
        self.header_set(27, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power5(&self) -> Option<i32> {
        self.values.get(&27).map(|v| *v as i32)
    }

    pub fn set_unit_maxhealth(&mut self, v: i32) {
        self.header_set(28, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxhealth(&self) -> Option<i32> {
        self.values.get(&28).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower1(&mut self, v: i32) {
        self.header_set(29, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower1(&self) -> Option<i32> {
        self.values.get(&29).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower2(&mut self, v: i32) {
        self.header_set(30, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower2(&self) -> Option<i32> {
        self.values.get(&30).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower3(&mut self, v: i32) {
        self.header_set(31, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower3(&self) -> Option<i32> {
        self.values.get(&31).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower4(&mut self, v: i32) {
        self.header_set(32, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower4(&self) -> Option<i32> {
        self.values.get(&32).map(|v| *v as i32)
    }

    pub fn set_unit_maxpower5(&mut self, v: i32) {
        self.header_set(33, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxpower5(&self) -> Option<i32> {
        self.values.get(&33).map(|v| *v as i32)
    }

    pub fn set_unit_level(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_level(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_unit_factiontemplate(&mut self, v: i32) {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_factiontemplate(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_unit_bytes_0(&mut self, race: Race, class: Class, gender: Gender, power: Power) {
        self.header_set(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
    }

    pub fn unit_bytes_0(&self) -> Option<(Race, Class, Gender, Power)> {
        if let Some(v) = self.values.get(&36) {
            let v = v.to_le_bytes();
            let (race, class, gender, power) = (v[0], v[1], v[2], v[3]);
            Some((race.try_into().unwrap(), class.try_into().unwrap(), gender.try_into().unwrap(), power.try_into().unwrap()))
        } else {
            None
        }
    }

    pub fn set_unit_virtual_item_slot_display(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_virtual_item_slot_display(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_unit_virtual_item_info(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(40, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_virtual_item_info(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&40) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_flags(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_flags(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_unit_flags_2(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_flags_2(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
    }

    pub fn set_unit_aura(&mut self, v: i32) {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_aura(&self) -> Option<i32> {
        self.values.get(&48).map(|v| *v as i32)
    }

    pub fn set_unit_auraflags(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(104, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auraflags(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&104) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_auralevels(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(118, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auralevels(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&118) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_auraapplications(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(132, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auraapplications(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&132) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.values.get(&146).map(|v| *v as i32)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.values.get(&147).map(|v| *v as i32)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.header_set(149, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.values.get(&149).map(|v| *v as i32)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.values.get(&150).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.values.get(&151).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.values.get(&153).map(|v| *v as i32)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.values.get(&154).map(|v| *v as i32)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.values.get(&155).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.values.get(&156).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.values.get(&157).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.values.get(&158).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(159, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&159) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.values.get(&160).map(|v| *v as i32)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.values.get(&161).map(|v| *v as i32)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.values.get(&163).map(|v| *v as i32)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.header_set(164, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.values.get(&164).map(|v| *v as i32)
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.values.get(&165).map(|v| *v as i32)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.header_set(166, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.values.get(&166).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.values.get(&167).map(|v| *v as i32)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.header_set(169, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.values.get(&169).map(|v| *v as i32)
    }

    pub fn set_unit_training_points(&mut self, a: u16, b: u16) {
        self.header_set(170, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_training_points(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&170) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.values.get(&171).map(|v| *v as i32)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.values.get(&172).map(|v| *v as i32)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.values.get(&173).map(|v| *v as i32)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.header_set(174, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.values.get(&174).map(|v| *v as i32)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.header_set(175, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.values.get(&175).map(|v| *v as i32)
    }

    pub fn set_player_posstat0(&mut self, v: i32) {
        self.header_set(176, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_posstat0(&self) -> Option<i32> {
        self.values.get(&176).map(|v| *v as i32)
    }

    pub fn set_unit_posstat1(&mut self, v: i32) {
        self.header_set(177, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat1(&self) -> Option<i32> {
        self.values.get(&177).map(|v| *v as i32)
    }

    pub fn set_unit_posstat2(&mut self, v: i32) {
        self.header_set(178, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat2(&self) -> Option<i32> {
        self.values.get(&178).map(|v| *v as i32)
    }

    pub fn set_unit_posstat3(&mut self, v: i32) {
        self.header_set(179, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_posstat3(&self) -> Option<i32> {
        self.values.get(&179).map(|v| *v as i32)
    }

    pub fn set_player_posstat4(&mut self, v: i32) {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_posstat4(&self) -> Option<i32> {
        self.values.get(&180).map(|v| *v as i32)
    }

    pub fn set_player_negstat0(&mut self, v: i32) {
        self.header_set(181, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_negstat0(&self) -> Option<i32> {
        self.values.get(&181).map(|v| *v as i32)
    }

    pub fn set_unit_negstat1(&mut self, v: i32) {
        self.header_set(182, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat1(&self) -> Option<i32> {
        self.values.get(&182).map(|v| *v as i32)
    }

    pub fn set_unit_negstat2(&mut self, v: i32) {
        self.header_set(183, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat2(&self) -> Option<i32> {
        self.values.get(&183).map(|v| *v as i32)
    }

    pub fn set_unit_negstat3(&mut self, v: i32) {
        self.header_set(184, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_negstat3(&self) -> Option<i32> {
        self.values.get(&184).map(|v| *v as i32)
    }

    pub fn set_player_negstat4(&mut self, v: i32) {
        self.header_set(185, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_negstat4(&self) -> Option<i32> {
        self.values.get(&185).map(|v| *v as i32)
    }

    pub fn set_unit_resistances(&mut self, v: i32) {
        self.header_set(186, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_resistances(&self) -> Option<i32> {
        self.values.get(&186).map(|v| *v as i32)
    }

    pub fn set_player_resistancebuffmodspositive(&mut self, v: i32) {
        self.header_set(193, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_resistancebuffmodspositive(&self) -> Option<i32> {
        self.values.get(&193).map(|v| *v as i32)
    }

    pub fn set_player_resistancebuffmodsnegative(&mut self, v: i32) {
        self.header_set(200, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_resistancebuffmodsnegative(&self) -> Option<i32> {
        self.values.get(&200).map(|v| *v as i32)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.values.get(&207).map(|v| *v as i32)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.values.get(&208).map(|v| *v as i32)
    }

    pub fn set_unit_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(209, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&209) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.values.get(&210).map(|v| *v as i32)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(211, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&211) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(212, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&212).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.values.get(&213).map(|v| *v as i32)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(214, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&214) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(215, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&215).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.values.get(&216).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.values.get(&217).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.header_set(218, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.values.get(&218).map(|v| *v as i32)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.values.get(&225).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxhealthmodifier(&mut self, v: f32) {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxhealthmodifier(&self) -> Option<f32> {
        self.values.get(&232).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_duel_arbiter(&mut self, v: Guid) {
        self.set_guid(234, v);
    }

    pub fn player_duel_arbiter(&self) -> Option<Guid> {
        self.get_guid(234)
    }

    pub fn set_player_flags(&mut self, v: i32) {
        self.header_set(236, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_flags(&self) -> Option<i32> {
        self.values.get(&236).map(|v| *v as i32)
    }

    pub fn set_player_guildid(&mut self, v: i32) {
        self.header_set(237, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_guildid(&self) -> Option<i32> {
        self.values.get(&237).map(|v| *v as i32)
    }

    pub fn set_player_guildrank(&mut self, v: i32) {
        self.header_set(238, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_guildrank(&self) -> Option<i32> {
        self.values.get(&238).map(|v| *v as i32)
    }

    pub fn set_player_field_bytes(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(239, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_field_bytes(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&239) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(240, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&240) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_bytes_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(241, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_bytes_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&241) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_duel_team(&mut self, v: i32) {
        self.header_set(242, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_duel_team(&self) -> Option<i32> {
        self.values.get(&242).map(|v| *v as i32)
    }

    pub fn set_player_guild_timestamp(&mut self, v: i32) {
        self.header_set(243, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_guild_timestamp(&self) -> Option<i32> {
        self.values.get(&243).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_1_1(&mut self, v: i32) {
        self.header_set(244, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_1_1(&self) -> Option<i32> {
        self.values.get(&244).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_1_2(&mut self, v: i32) {
        self.header_set(245, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_1_2(&self) -> Option<i32> {
        self.values.get(&245).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_1_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(246, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_1_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&246) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_1_4(&mut self, v: i32) {
        self.header_set(247, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_1_4(&self) -> Option<i32> {
        self.values.get(&247).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_2_1(&mut self, v: i32) {
        self.header_set(248, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_2_1(&self) -> Option<i32> {
        self.values.get(&248).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_2_2(&mut self, v: i32) {
        self.header_set(249, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_2_2(&self) -> Option<i32> {
        self.values.get(&249).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_2_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(250, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_2_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&250) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_2_4(&mut self, v: i32) {
        self.header_set(251, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_2_4(&self) -> Option<i32> {
        self.values.get(&251).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_3_1(&mut self, v: i32) {
        self.header_set(252, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_3_1(&self) -> Option<i32> {
        self.values.get(&252).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_3_2(&mut self, v: i32) {
        self.header_set(253, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_3_2(&self) -> Option<i32> {
        self.values.get(&253).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_3_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(254, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_3_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&254) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_3_4(&mut self, v: i32) {
        self.header_set(255, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_3_4(&self) -> Option<i32> {
        self.values.get(&255).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_4_1(&mut self, v: i32) {
        self.header_set(256, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_4_1(&self) -> Option<i32> {
        self.values.get(&256).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_4_2(&mut self, v: i32) {
        self.header_set(257, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_4_2(&self) -> Option<i32> {
        self.values.get(&257).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_4_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(258, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_4_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&258) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_4_4(&mut self, v: i32) {
        self.header_set(259, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_4_4(&self) -> Option<i32> {
        self.values.get(&259).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_5_1(&mut self, v: i32) {
        self.header_set(260, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_5_1(&self) -> Option<i32> {
        self.values.get(&260).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_5_2(&mut self, v: i32) {
        self.header_set(261, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_5_2(&self) -> Option<i32> {
        self.values.get(&261).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_5_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(262, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_5_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&262) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_5_4(&mut self, v: i32) {
        self.header_set(263, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_5_4(&self) -> Option<i32> {
        self.values.get(&263).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_6_1(&mut self, v: i32) {
        self.header_set(264, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_6_1(&self) -> Option<i32> {
        self.values.get(&264).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_6_2(&mut self, v: i32) {
        self.header_set(265, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_6_2(&self) -> Option<i32> {
        self.values.get(&265).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_6_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(266, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_6_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&266) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_6_4(&mut self, v: i32) {
        self.header_set(267, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_6_4(&self) -> Option<i32> {
        self.values.get(&267).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_7_1(&mut self, v: i32) {
        self.header_set(268, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_7_1(&self) -> Option<i32> {
        self.values.get(&268).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_7_2(&mut self, v: i32) {
        self.header_set(269, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_7_2(&self) -> Option<i32> {
        self.values.get(&269).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_7_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(270, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_7_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&270) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_7_4(&mut self, v: i32) {
        self.header_set(271, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_7_4(&self) -> Option<i32> {
        self.values.get(&271).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_8_1(&mut self, v: i32) {
        self.header_set(272, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_8_1(&self) -> Option<i32> {
        self.values.get(&272).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_8_2(&mut self, v: i32) {
        self.header_set(273, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_8_2(&self) -> Option<i32> {
        self.values.get(&273).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_8_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(274, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_8_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&274) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_8_4(&mut self, v: i32) {
        self.header_set(275, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_8_4(&self) -> Option<i32> {
        self.values.get(&275).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_9_1(&mut self, v: i32) {
        self.header_set(276, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_9_1(&self) -> Option<i32> {
        self.values.get(&276).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_9_2(&mut self, v: i32) {
        self.header_set(277, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_9_2(&self) -> Option<i32> {
        self.values.get(&277).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_9_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(278, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_9_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&278) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_9_4(&mut self, v: i32) {
        self.header_set(279, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_9_4(&self) -> Option<i32> {
        self.values.get(&279).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_10_1(&mut self, v: i32) {
        self.header_set(280, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_10_1(&self) -> Option<i32> {
        self.values.get(&280).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_10_2(&mut self, v: i32) {
        self.header_set(281, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_10_2(&self) -> Option<i32> {
        self.values.get(&281).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_10_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(282, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_10_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&282) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_10_4(&mut self, v: i32) {
        self.header_set(283, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_10_4(&self) -> Option<i32> {
        self.values.get(&283).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_11_1(&mut self, v: i32) {
        self.header_set(284, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_11_1(&self) -> Option<i32> {
        self.values.get(&284).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_11_2(&mut self, v: i32) {
        self.header_set(285, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_11_2(&self) -> Option<i32> {
        self.values.get(&285).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_11_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(286, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_11_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&286) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_11_4(&mut self, v: i32) {
        self.header_set(287, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_11_4(&self) -> Option<i32> {
        self.values.get(&287).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_12_1(&mut self, v: i32) {
        self.header_set(288, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_12_1(&self) -> Option<i32> {
        self.values.get(&288).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_12_2(&mut self, v: i32) {
        self.header_set(289, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_12_2(&self) -> Option<i32> {
        self.values.get(&289).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_12_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(290, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_12_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&290) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_12_4(&mut self, v: i32) {
        self.header_set(291, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_12_4(&self) -> Option<i32> {
        self.values.get(&291).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_13_1(&mut self, v: i32) {
        self.header_set(292, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_13_1(&self) -> Option<i32> {
        self.values.get(&292).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_13_2(&mut self, v: i32) {
        self.header_set(293, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_13_2(&self) -> Option<i32> {
        self.values.get(&293).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_13_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(294, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_13_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&294) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_13_4(&mut self, v: i32) {
        self.header_set(295, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_13_4(&self) -> Option<i32> {
        self.values.get(&295).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_14_1(&mut self, v: i32) {
        self.header_set(296, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_14_1(&self) -> Option<i32> {
        self.values.get(&296).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_14_2(&mut self, v: i32) {
        self.header_set(297, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_14_2(&self) -> Option<i32> {
        self.values.get(&297).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_14_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(298, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_14_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&298) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_14_4(&mut self, v: i32) {
        self.header_set(299, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_14_4(&self) -> Option<i32> {
        self.values.get(&299).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_15_1(&mut self, v: i32) {
        self.header_set(300, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_15_1(&self) -> Option<i32> {
        self.values.get(&300).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_15_2(&mut self, v: i32) {
        self.header_set(301, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_15_2(&self) -> Option<i32> {
        self.values.get(&301).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_15_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(302, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_15_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&302) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_15_4(&mut self, v: i32) {
        self.header_set(303, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_15_4(&self) -> Option<i32> {
        self.values.get(&303).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_16_1(&mut self, v: i32) {
        self.header_set(304, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_16_1(&self) -> Option<i32> {
        self.values.get(&304).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_16_2(&mut self, v: i32) {
        self.header_set(305, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_16_2(&self) -> Option<i32> {
        self.values.get(&305).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_16_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(306, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_16_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&306) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_16_4(&mut self, v: i32) {
        self.header_set(307, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_16_4(&self) -> Option<i32> {
        self.values.get(&307).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_17_1(&mut self, v: i32) {
        self.header_set(308, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_17_1(&self) -> Option<i32> {
        self.values.get(&308).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_17_2(&mut self, v: i32) {
        self.header_set(309, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_17_2(&self) -> Option<i32> {
        self.values.get(&309).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_17_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(310, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_17_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&310) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_17_4(&mut self, v: i32) {
        self.header_set(311, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_17_4(&self) -> Option<i32> {
        self.values.get(&311).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_18_1(&mut self, v: i32) {
        self.header_set(312, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_18_1(&self) -> Option<i32> {
        self.values.get(&312).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_18_2(&mut self, v: i32) {
        self.header_set(313, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_18_2(&self) -> Option<i32> {
        self.values.get(&313).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_18_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(314, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_18_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&314) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_18_4(&mut self, v: i32) {
        self.header_set(315, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_18_4(&self) -> Option<i32> {
        self.values.get(&315).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_19_1(&mut self, v: i32) {
        self.header_set(316, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_19_1(&self) -> Option<i32> {
        self.values.get(&316).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_19_2(&mut self, v: i32) {
        self.header_set(317, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_19_2(&self) -> Option<i32> {
        self.values.get(&317).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_19_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(318, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_19_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&318) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_19_4(&mut self, v: i32) {
        self.header_set(319, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_19_4(&self) -> Option<i32> {
        self.values.get(&319).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_20_1(&mut self, v: i32) {
        self.header_set(320, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_20_1(&self) -> Option<i32> {
        self.values.get(&320).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_20_2(&mut self, v: i32) {
        self.header_set(321, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_20_2(&self) -> Option<i32> {
        self.values.get(&321).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_20_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(322, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_20_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&322) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_20_4(&mut self, v: i32) {
        self.header_set(323, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_20_4(&self) -> Option<i32> {
        self.values.get(&323).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_21_1(&mut self, v: i32) {
        self.header_set(324, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_21_1(&self) -> Option<i32> {
        self.values.get(&324).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_21_2(&mut self, v: i32) {
        self.header_set(325, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_21_2(&self) -> Option<i32> {
        self.values.get(&325).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_21_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(326, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_21_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&326) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_21_4(&mut self, v: i32) {
        self.header_set(327, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_21_4(&self) -> Option<i32> {
        self.values.get(&327).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_22_1(&mut self, v: i32) {
        self.header_set(328, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_22_1(&self) -> Option<i32> {
        self.values.get(&328).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_22_2(&mut self, v: i32) {
        self.header_set(329, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_22_2(&self) -> Option<i32> {
        self.values.get(&329).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_22_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(330, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_22_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&330) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_22_4(&mut self, v: i32) {
        self.header_set(331, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_22_4(&self) -> Option<i32> {
        self.values.get(&331).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_23_1(&mut self, v: i32) {
        self.header_set(332, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_23_1(&self) -> Option<i32> {
        self.values.get(&332).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_23_2(&mut self, v: i32) {
        self.header_set(333, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_23_2(&self) -> Option<i32> {
        self.values.get(&333).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_23_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(334, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_23_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&334) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_23_4(&mut self, v: i32) {
        self.header_set(335, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_23_4(&self) -> Option<i32> {
        self.values.get(&335).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_24_1(&mut self, v: i32) {
        self.header_set(336, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_24_1(&self) -> Option<i32> {
        self.values.get(&336).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_24_2(&mut self, v: i32) {
        self.header_set(337, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_24_2(&self) -> Option<i32> {
        self.values.get(&337).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_24_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(338, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_24_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&338) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_24_4(&mut self, v: i32) {
        self.header_set(339, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_24_4(&self) -> Option<i32> {
        self.values.get(&339).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_25_1(&mut self, v: i32) {
        self.header_set(340, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_25_1(&self) -> Option<i32> {
        self.values.get(&340).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_25_2(&mut self, v: i32) {
        self.header_set(341, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_25_2(&self) -> Option<i32> {
        self.values.get(&341).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_25_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(342, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_quest_log_25_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&342) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_quest_log_25_4(&mut self, v: i32) {
        self.header_set(343, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_25_4(&self) -> Option<i32> {
        self.values.get(&343).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_1_creator(&mut self, v: Guid) {
        self.set_guid(344, v);
    }

    pub fn player_visible_item_1_creator(&self) -> Option<Guid> {
        self.get_guid(344)
    }

    pub fn set_player_visible_item_1_0(&mut self, v: i32) {
        self.header_set(346, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_1_0(&self) -> Option<i32> {
        self.values.get(&346).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_1_properties(&mut self, a: u16, b: u16) {
        self.header_set(358, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_1_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&358) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_2_creator(&mut self, v: Guid) {
        self.set_guid(360, v);
    }

    pub fn player_visible_item_2_creator(&self) -> Option<Guid> {
        self.get_guid(360)
    }

    pub fn set_player_visible_item_2_0(&mut self, v: i32) {
        self.header_set(362, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_2_0(&self) -> Option<i32> {
        self.values.get(&362).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_2_properties(&mut self, a: u16, b: u16) {
        self.header_set(374, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_2_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&374) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_3_creator(&mut self, v: Guid) {
        self.set_guid(376, v);
    }

    pub fn player_visible_item_3_creator(&self) -> Option<Guid> {
        self.get_guid(376)
    }

    pub fn set_player_visible_item_3_0(&mut self, v: i32) {
        self.header_set(378, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_3_0(&self) -> Option<i32> {
        self.values.get(&378).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_3_properties(&mut self, a: u16, b: u16) {
        self.header_set(390, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_3_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&390) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_4_creator(&mut self, v: Guid) {
        self.set_guid(392, v);
    }

    pub fn player_visible_item_4_creator(&self) -> Option<Guid> {
        self.get_guid(392)
    }

    pub fn set_player_visible_item_4_0(&mut self, v: i32) {
        self.header_set(394, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_4_0(&self) -> Option<i32> {
        self.values.get(&394).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_4_properties(&mut self, a: u16, b: u16) {
        self.header_set(406, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_4_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&406) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_5_creator(&mut self, v: Guid) {
        self.set_guid(408, v);
    }

    pub fn player_visible_item_5_creator(&self) -> Option<Guid> {
        self.get_guid(408)
    }

    pub fn set_player_visible_item_5_0(&mut self, v: i32) {
        self.header_set(410, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_5_0(&self) -> Option<i32> {
        self.values.get(&410).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_5_properties(&mut self, a: u16, b: u16) {
        self.header_set(422, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_5_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&422) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_6_creator(&mut self, v: Guid) {
        self.set_guid(424, v);
    }

    pub fn player_visible_item_6_creator(&self) -> Option<Guid> {
        self.get_guid(424)
    }

    pub fn set_player_visible_item_6_0(&mut self, v: i32) {
        self.header_set(426, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_6_0(&self) -> Option<i32> {
        self.values.get(&426).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_6_properties(&mut self, a: u16, b: u16) {
        self.header_set(438, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_6_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&438) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_7_creator(&mut self, v: Guid) {
        self.set_guid(440, v);
    }

    pub fn player_visible_item_7_creator(&self) -> Option<Guid> {
        self.get_guid(440)
    }

    pub fn set_player_visible_item_7_0(&mut self, v: i32) {
        self.header_set(442, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_7_0(&self) -> Option<i32> {
        self.values.get(&442).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_7_properties(&mut self, a: u16, b: u16) {
        self.header_set(454, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_7_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&454) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_8_creator(&mut self, v: Guid) {
        self.set_guid(456, v);
    }

    pub fn player_visible_item_8_creator(&self) -> Option<Guid> {
        self.get_guid(456)
    }

    pub fn set_player_visible_item_8_0(&mut self, v: i32) {
        self.header_set(458, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_8_0(&self) -> Option<i32> {
        self.values.get(&458).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_8_properties(&mut self, a: u16, b: u16) {
        self.header_set(470, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_8_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&470) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_9_creator(&mut self, v: Guid) {
        self.set_guid(472, v);
    }

    pub fn player_visible_item_9_creator(&self) -> Option<Guid> {
        self.get_guid(472)
    }

    pub fn set_player_visible_item_9_0(&mut self, v: i32) {
        self.header_set(474, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_9_0(&self) -> Option<i32> {
        self.values.get(&474).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_9_properties(&mut self, a: u16, b: u16) {
        self.header_set(486, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_9_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&486) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_10_creator(&mut self, v: Guid) {
        self.set_guid(488, v);
    }

    pub fn player_visible_item_10_creator(&self) -> Option<Guid> {
        self.get_guid(488)
    }

    pub fn set_player_visible_item_10_0(&mut self, v: i32) {
        self.header_set(490, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_10_0(&self) -> Option<i32> {
        self.values.get(&490).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_10_properties(&mut self, a: u16, b: u16) {
        self.header_set(502, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_10_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&502) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_11_creator(&mut self, v: Guid) {
        self.set_guid(504, v);
    }

    pub fn player_visible_item_11_creator(&self) -> Option<Guid> {
        self.get_guid(504)
    }

    pub fn set_player_visible_item_11_0(&mut self, v: i32) {
        self.header_set(506, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_11_0(&self) -> Option<i32> {
        self.values.get(&506).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_11_properties(&mut self, a: u16, b: u16) {
        self.header_set(518, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_11_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&518) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_12_creator(&mut self, v: Guid) {
        self.set_guid(520, v);
    }

    pub fn player_visible_item_12_creator(&self) -> Option<Guid> {
        self.get_guid(520)
    }

    pub fn set_player_visible_item_12_0(&mut self, v: i32) {
        self.header_set(522, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_12_0(&self) -> Option<i32> {
        self.values.get(&522).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_12_properties(&mut self, a: u16, b: u16) {
        self.header_set(534, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_12_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&534) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_13_creator(&mut self, v: Guid) {
        self.set_guid(536, v);
    }

    pub fn player_visible_item_13_creator(&self) -> Option<Guid> {
        self.get_guid(536)
    }

    pub fn set_player_visible_item_13_0(&mut self, v: i32) {
        self.header_set(538, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_13_0(&self) -> Option<i32> {
        self.values.get(&538).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_13_properties(&mut self, a: u16, b: u16) {
        self.header_set(550, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_13_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&550) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_14_creator(&mut self, v: Guid) {
        self.set_guid(552, v);
    }

    pub fn player_visible_item_14_creator(&self) -> Option<Guid> {
        self.get_guid(552)
    }

    pub fn set_player_visible_item_14_0(&mut self, v: i32) {
        self.header_set(554, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_14_0(&self) -> Option<i32> {
        self.values.get(&554).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_14_properties(&mut self, a: u16, b: u16) {
        self.header_set(566, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_14_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&566) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_15_creator(&mut self, v: Guid) {
        self.set_guid(568, v);
    }

    pub fn player_visible_item_15_creator(&self) -> Option<Guid> {
        self.get_guid(568)
    }

    pub fn set_player_visible_item_15_0(&mut self, v: i32) {
        self.header_set(570, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_15_0(&self) -> Option<i32> {
        self.values.get(&570).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_15_properties(&mut self, a: u16, b: u16) {
        self.header_set(582, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_15_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&582) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_16_creator(&mut self, v: Guid) {
        self.set_guid(584, v);
    }

    pub fn player_visible_item_16_creator(&self) -> Option<Guid> {
        self.get_guid(584)
    }

    pub fn set_player_visible_item_16_0(&mut self, v: i32) {
        self.header_set(586, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_16_0(&self) -> Option<i32> {
        self.values.get(&586).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_16_properties(&mut self, a: u16, b: u16) {
        self.header_set(598, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_16_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&598) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_17_creator(&mut self, v: Guid) {
        self.set_guid(600, v);
    }

    pub fn player_visible_item_17_creator(&self) -> Option<Guid> {
        self.get_guid(600)
    }

    pub fn set_player_visible_item_17_0(&mut self, v: i32) {
        self.header_set(602, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_17_0(&self) -> Option<i32> {
        self.values.get(&602).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_17_properties(&mut self, a: u16, b: u16) {
        self.header_set(614, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_17_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&614) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_18_creator(&mut self, v: Guid) {
        self.set_guid(616, v);
    }

    pub fn player_visible_item_18_creator(&self) -> Option<Guid> {
        self.get_guid(616)
    }

    pub fn set_player_visible_item_18_0(&mut self, v: i32) {
        self.header_set(618, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_18_0(&self) -> Option<i32> {
        self.values.get(&618).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_18_properties(&mut self, a: u16, b: u16) {
        self.header_set(630, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_18_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&630) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_19_creator(&mut self, v: Guid) {
        self.set_guid(632, v);
    }

    pub fn player_visible_item_19_creator(&self) -> Option<Guid> {
        self.get_guid(632)
    }

    pub fn set_player_visible_item_19_0(&mut self, v: i32) {
        self.header_set(634, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_19_0(&self) -> Option<i32> {
        self.values.get(&634).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_19_properties(&mut self, a: u16, b: u16) {
        self.header_set(646, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_19_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&646) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_chosen_title(&mut self, v: i32) {
        self.header_set(648, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_chosen_title(&self) -> Option<i32> {
        self.values.get(&648).map(|v| *v as i32)
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
        self.header_set(926, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_xp(&self) -> Option<i32> {
        self.values.get(&926).map(|v| *v as i32)
    }

    pub fn set_player_next_level_xp(&mut self, v: i32) {
        self.header_set(927, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_next_level_xp(&self) -> Option<i32> {
        self.values.get(&927).map(|v| *v as i32)
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
        self.header_set(1312, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_character_points1(&self) -> Option<i32> {
        self.values.get(&1312).map(|v| *v as i32)
    }

    pub fn set_player_character_points2(&mut self, v: i32) {
        self.header_set(1313, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_character_points2(&self) -> Option<i32> {
        self.values.get(&1313).map(|v| *v as i32)
    }

    pub fn set_player_track_creatures(&mut self, v: i32) {
        self.header_set(1314, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_track_creatures(&self) -> Option<i32> {
        self.values.get(&1314).map(|v| *v as i32)
    }

    pub fn set_player_track_resources(&mut self, v: i32) {
        self.header_set(1315, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_track_resources(&self) -> Option<i32> {
        self.values.get(&1315).map(|v| *v as i32)
    }

    pub fn set_player_block_percentage(&mut self, v: f32) {
        self.header_set(1316, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_block_percentage(&self) -> Option<f32> {
        self.values.get(&1316).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_dodge_percentage(&mut self, v: f32) {
        self.header_set(1317, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_dodge_percentage(&self) -> Option<f32> {
        self.values.get(&1317).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_parry_percentage(&mut self, v: f32) {
        self.header_set(1318, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_parry_percentage(&self) -> Option<f32> {
        self.values.get(&1318).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_expertise(&mut self, v: i32) {
        self.header_set(1319, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_expertise(&self) -> Option<i32> {
        self.values.get(&1319).map(|v| *v as i32)
    }

    pub fn set_player_offhand_expertise(&mut self, v: i32) {
        self.header_set(1320, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_offhand_expertise(&self) -> Option<i32> {
        self.values.get(&1320).map(|v| *v as i32)
    }

    pub fn set_player_crit_percentage(&mut self, v: f32) {
        self.header_set(1321, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_crit_percentage(&self) -> Option<f32> {
        self.values.get(&1321).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_ranged_crit_percentage(&mut self, v: f32) {
        self.header_set(1322, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_ranged_crit_percentage(&self) -> Option<f32> {
        self.values.get(&1322).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_offhand_crit_percentage(&mut self, v: f32) {
        self.header_set(1323, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_offhand_crit_percentage(&self) -> Option<f32> {
        self.values.get(&1323).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_spell_crit_percentage1(&mut self, v: f32) {
        self.header_set(1324, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_spell_crit_percentage1(&self) -> Option<f32> {
        self.values.get(&1324).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_shield_block(&mut self, v: i32) {
        self.header_set(1331, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_shield_block(&self) -> Option<i32> {
        self.values.get(&1331).map(|v| *v as i32)
    }

    pub fn set_player_explored_zones_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1332, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_explored_zones_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1332) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_rest_state_experience(&mut self, v: i32) {
        self.header_set(1460, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_rest_state_experience(&self) -> Option<i32> {
        self.values.get(&1460).map(|v| *v as i32)
    }

    pub fn set_player_coinage(&mut self, v: i32) {
        self.header_set(1461, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_coinage(&self) -> Option<i32> {
        self.values.get(&1461).map(|v| *v as i32)
    }

    pub fn set_player_mod_damage_done_pos(&mut self, v: i32) {
        self.header_set(1462, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_damage_done_pos(&self) -> Option<i32> {
        self.values.get(&1462).map(|v| *v as i32)
    }

    pub fn set_player_mod_damage_done_neg(&mut self, v: i32) {
        self.header_set(1469, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_damage_done_neg(&self) -> Option<i32> {
        self.values.get(&1469).map(|v| *v as i32)
    }

    pub fn set_player_mod_damage_done_pct(&mut self, v: i32) {
        self.header_set(1476, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_damage_done_pct(&self) -> Option<i32> {
        self.values.get(&1476).map(|v| *v as i32)
    }

    pub fn set_player_mod_healing_done_pos(&mut self, v: i32) {
        self.header_set(1483, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_healing_done_pos(&self) -> Option<i32> {
        self.values.get(&1483).map(|v| *v as i32)
    }

    pub fn set_player_mod_target_resistance(&mut self, v: i32) {
        self.header_set(1484, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_target_resistance(&self) -> Option<i32> {
        self.values.get(&1484).map(|v| *v as i32)
    }

    pub fn set_player_mod_target_physical_resistance(&mut self, v: i32) {
        self.header_set(1485, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_target_physical_resistance(&self) -> Option<i32> {
        self.values.get(&1485).map(|v| *v as i32)
    }

    pub fn set_player_features(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1486, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_features(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1486) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_ammo_id(&mut self, v: i32) {
        self.header_set(1487, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_ammo_id(&self) -> Option<i32> {
        self.values.get(&1487).map(|v| *v as i32)
    }

    pub fn set_player_self_res_spell(&mut self, v: i32) {
        self.header_set(1488, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_self_res_spell(&self) -> Option<i32> {
        self.values.get(&1488).map(|v| *v as i32)
    }

    pub fn set_player_pvp_medals(&mut self, v: i32) {
        self.header_set(1489, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_pvp_medals(&self) -> Option<i32> {
        self.values.get(&1489).map(|v| *v as i32)
    }

    pub fn set_player_buyback_price_1(&mut self, v: i32) {
        self.header_set(1490, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_buyback_price_1(&self) -> Option<i32> {
        self.values.get(&1490).map(|v| *v as i32)
    }

    pub fn set_player_buyback_timestamp_1(&mut self, v: i32) {
        self.header_set(1502, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_buyback_timestamp_1(&self) -> Option<i32> {
        self.values.get(&1502).map(|v| *v as i32)
    }

    pub fn set_player_kills(&mut self, a: u16, b: u16) {
        self.header_set(1514, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_kills(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1514) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_today_contribution(&mut self, v: i32) {
        self.header_set(1515, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_today_contribution(&self) -> Option<i32> {
        self.values.get(&1515).map(|v| *v as i32)
    }

    pub fn set_player_yesterday_contribution(&mut self, v: i32) {
        self.header_set(1516, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_yesterday_contribution(&self) -> Option<i32> {
        self.values.get(&1516).map(|v| *v as i32)
    }

    pub fn set_player_lifetime_honorable_kills(&mut self, v: i32) {
        self.header_set(1517, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_lifetime_honorable_kills(&self) -> Option<i32> {
        self.values.get(&1517).map(|v| *v as i32)
    }

    pub fn set_player_bytes2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1518, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_bytes2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1518) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_watched_faction_index(&mut self, v: i32) {
        self.header_set(1519, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_watched_faction_index(&self) -> Option<i32> {
        self.values.get(&1519).map(|v| *v as i32)
    }

    pub fn set_player_combat_rating_1(&mut self, v: i32) {
        self.header_set(1520, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_combat_rating_1(&self) -> Option<i32> {
        self.values.get(&1520).map(|v| *v as i32)
    }

    pub fn set_player_arena_team_info_1_1(&mut self, v: i32) {
        self.header_set(1544, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_arena_team_info_1_1(&self) -> Option<i32> {
        self.values.get(&1544).map(|v| *v as i32)
    }

    pub fn set_player_honor_currency(&mut self, v: i32) {
        self.header_set(1562, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_honor_currency(&self) -> Option<i32> {
        self.values.get(&1562).map(|v| *v as i32)
    }

    pub fn set_player_arena_currency(&mut self, v: i32) {
        self.header_set(1563, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_arena_currency(&self) -> Option<i32> {
        self.values.get(&1563).map(|v| *v as i32)
    }

    pub fn set_player_mod_mana_regen(&mut self, v: f32) {
        self.header_set(1564, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_mana_regen(&self) -> Option<f32> {
        self.values.get(&1564).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_mod_mana_regen_interrupt(&mut self, v: f32) {
        self.header_set(1565, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_mod_mana_regen_interrupt(&self) -> Option<f32> {
        self.values.get(&1565).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_max_level(&mut self, v: i32) {
        self.header_set(1566, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_max_level(&self) -> Option<i32> {
        self.values.get(&1566).map(|v| *v as i32)
    }

    pub fn set_player_daily_quests_1(&mut self, v: i32) {
        self.header_set(1567, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_daily_quests_1(&self) -> Option<i32> {
        self.values.get(&1567).map(|v| *v as i32)
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
        self.set_guid(6, v);
    }

    pub fn object_created_by(&self) -> Option<Guid> {
        self.get_guid(6)
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

    pub fn set_gameobject_rotation(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_rotation(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_state(&mut self, v: i32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_state(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_gameobject_pos_x(&mut self, v: f32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_pos_x(&self) -> Option<f32> {
        self.values.get(&15).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_pos_y(&mut self, v: f32) {
        self.header_set(16, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_pos_y(&self) -> Option<f32> {
        self.values.get(&16).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_pos_z(&mut self, v: f32) {
        self.header_set(17, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_pos_z(&self) -> Option<f32> {
        self.values.get(&17).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_facing(&mut self, v: f32) {
        self.header_set(18, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_facing(&self) -> Option<f32> {
        self.values.get(&18).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_gameobject_dyn_flags(&mut self, v: i32) {
        self.header_set(19, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_dyn_flags(&self) -> Option<i32> {
        self.values.get(&19).map(|v| *v as i32)
    }

    pub fn set_gameobject_faction(&mut self, v: i32) {
        self.header_set(20, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_faction(&self) -> Option<i32> {
        self.values.get(&20).map(|v| *v as i32)
    }

    pub fn set_gameobject_type_id(&mut self, v: i32) {
        self.header_set(21, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_type_id(&self) -> Option<i32> {
        self.values.get(&21).map(|v| *v as i32)
    }

    pub fn set_gameobject_level(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_level(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_gameobject_artkit(&mut self, v: i32) {
        self.header_set(23, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_artkit(&self) -> Option<i32> {
        self.values.get(&23).map(|v| *v as i32)
    }

    pub fn set_gameobject_animprogress(&mut self, v: i32) {
        self.header_set(24, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn gameobject_animprogress(&self) -> Option<i32> {
        self.values.get(&24).map(|v| *v as i32)
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

    pub fn set_dynamicobject_pos_x(&mut self, v: f32) {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_pos_x(&self) -> Option<f32> {
        self.values.get(&11).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_pos_y(&mut self, v: f32) {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_pos_y(&self) -> Option<f32> {
        self.values.get(&12).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_pos_z(&mut self, v: f32) {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_pos_z(&self) -> Option<f32> {
        self.values.get(&13).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_facing(&mut self, v: f32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_facing(&self) -> Option<f32> {
        self.values.get(&14).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_dynamicobject_casttime(&mut self, v: i32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn dynamicobject_casttime(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
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
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_facing(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_pos_x(&mut self, v: f32) {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_pos_x(&self) -> Option<f32> {
        self.values.get(&11).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_pos_y(&mut self, v: f32) {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_pos_y(&self) -> Option<f32> {
        self.values.get(&12).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_pos_z(&mut self, v: f32) {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_pos_z(&self) -> Option<f32> {
        self.values.get(&13).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_display_id(&mut self, v: i32) {
        self.header_set(14, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_display_id(&self) -> Option<i32> {
        self.values.get(&14).map(|v| *v as i32)
    }

    pub fn set_corpse_item(&mut self, v: i32) {
        self.header_set(15, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_item(&self) -> Option<i32> {
        self.values.get(&15).map(|v| *v as i32)
    }

    pub fn set_corpse_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(34, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&34) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(35, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&35) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_guild(&mut self, v: i32) {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_guild(&self) -> Option<i32> {
        self.values.get(&36).map(|v| *v as i32)
    }

    pub fn set_corpse_flags(&mut self, v: i32) {
        self.header_set(37, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_flags(&self) -> Option<i32> {
        self.values.get(&37).map(|v| *v as i32)
    }

    pub fn set_corpse_dynamic_flags(&mut self, v: i32) {
        self.header_set(38, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_dynamic_flags(&self) -> Option<i32> {
        self.values.get(&38).map(|v| *v as i32)
    }

}

