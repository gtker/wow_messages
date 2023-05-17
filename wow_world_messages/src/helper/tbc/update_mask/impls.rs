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
        self.header_set(62, v.guid() as u32);
        self.header_set(63, (v.guid() >> 32) as u32);
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

    pub fn set_unit_charmedby(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_summonedby(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_createdby(mut self, v: Guid) -> Self {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_target(mut self, v: Guid) -> Self {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_persuaded(mut self, v: Guid) -> Self {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_channel_object(mut self, v: Guid) -> Self {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
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

    pub fn set_unit_charmedby(mut self, v: Guid) -> Self {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_summonedby(mut self, v: Guid) -> Self {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_createdby(mut self, v: Guid) -> Self {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_target(mut self, v: Guid) -> Self {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_persuaded(mut self, v: Guid) -> Self {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_channel_object(mut self, v: Guid) -> Self {
        self.header_set(20, v.guid() as u32);
        self.header_set(21, (v.guid() >> 32) as u32);
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
        self.header_set(234, v.guid() as u32);
        self.header_set(235, (v.guid() >> 32) as u32);
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
        self.header_set(344, v.guid() as u32);
        self.header_set(345, (v.guid() >> 32) as u32);
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
        self.header_set(360, v.guid() as u32);
        self.header_set(361, (v.guid() >> 32) as u32);
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
        self.header_set(376, v.guid() as u32);
        self.header_set(377, (v.guid() >> 32) as u32);
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
        self.header_set(392, v.guid() as u32);
        self.header_set(393, (v.guid() >> 32) as u32);
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
        self.header_set(408, v.guid() as u32);
        self.header_set(409, (v.guid() >> 32) as u32);
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
        self.header_set(424, v.guid() as u32);
        self.header_set(425, (v.guid() >> 32) as u32);
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
        self.header_set(440, v.guid() as u32);
        self.header_set(441, (v.guid() >> 32) as u32);
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
        self.header_set(456, v.guid() as u32);
        self.header_set(457, (v.guid() >> 32) as u32);
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
        self.header_set(472, v.guid() as u32);
        self.header_set(473, (v.guid() >> 32) as u32);
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
        self.header_set(488, v.guid() as u32);
        self.header_set(489, (v.guid() >> 32) as u32);
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
        self.header_set(504, v.guid() as u32);
        self.header_set(505, (v.guid() >> 32) as u32);
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
        self.header_set(520, v.guid() as u32);
        self.header_set(521, (v.guid() >> 32) as u32);
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
        self.header_set(536, v.guid() as u32);
        self.header_set(537, (v.guid() >> 32) as u32);
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
        self.header_set(552, v.guid() as u32);
        self.header_set(553, (v.guid() >> 32) as u32);
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
        self.header_set(568, v.guid() as u32);
        self.header_set(569, (v.guid() >> 32) as u32);
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
        self.header_set(584, v.guid() as u32);
        self.header_set(585, (v.guid() >> 32) as u32);
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
        self.header_set(600, v.guid() as u32);
        self.header_set(601, (v.guid() >> 32) as u32);
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
        self.header_set(616, v.guid() as u32);
        self.header_set(617, (v.guid() >> 32) as u32);
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
        self.header_set(632, v.guid() as u32);
        self.header_set(633, (v.guid() >> 32) as u32);
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
        self.header_set(650, v.guid() as u32);
        self.header_set(651, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_neck(mut self, v: Guid) -> Self {
        self.header_set(652, v.guid() as u32);
        self.header_set(653, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_shoulders(mut self, v: Guid) -> Self {
        self.header_set(654, v.guid() as u32);
        self.header_set(655, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_body(mut self, v: Guid) -> Self {
        self.header_set(656, v.guid() as u32);
        self.header_set(657, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_chest(mut self, v: Guid) -> Self {
        self.header_set(658, v.guid() as u32);
        self.header_set(659, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_waist(mut self, v: Guid) -> Self {
        self.header_set(660, v.guid() as u32);
        self.header_set(661, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_legs(mut self, v: Guid) -> Self {
        self.header_set(662, v.guid() as u32);
        self.header_set(663, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_feet(mut self, v: Guid) -> Self {
        self.header_set(664, v.guid() as u32);
        self.header_set(665, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_wrists(mut self, v: Guid) -> Self {
        self.header_set(666, v.guid() as u32);
        self.header_set(667, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_hands(mut self, v: Guid) -> Self {
        self.header_set(668, v.guid() as u32);
        self.header_set(669, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_finger1(mut self, v: Guid) -> Self {
        self.header_set(670, v.guid() as u32);
        self.header_set(671, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_finger2(mut self, v: Guid) -> Self {
        self.header_set(672, v.guid() as u32);
        self.header_set(673, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_trinket1(mut self, v: Guid) -> Self {
        self.header_set(674, v.guid() as u32);
        self.header_set(675, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_trinket2(mut self, v: Guid) -> Self {
        self.header_set(676, v.guid() as u32);
        self.header_set(677, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_back(mut self, v: Guid) -> Self {
        self.header_set(678, v.guid() as u32);
        self.header_set(679, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_main_hand(mut self, v: Guid) -> Self {
        self.header_set(680, v.guid() as u32);
        self.header_set(681, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_off_hand(mut self, v: Guid) -> Self {
        self.header_set(682, v.guid() as u32);
        self.header_set(683, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_ranged(mut self, v: Guid) -> Self {
        self.header_set(684, v.guid() as u32);
        self.header_set(685, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_tabard(mut self, v: Guid) -> Self {
        self.header_set(686, v.guid() as u32);
        self.header_set(687, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_bag1(mut self, v: Guid) -> Self {
        self.header_set(688, v.guid() as u32);
        self.header_set(689, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_bag2(mut self, v: Guid) -> Self {
        self.header_set(690, v.guid() as u32);
        self.header_set(691, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_bag3(mut self, v: Guid) -> Self {
        self.header_set(692, v.guid() as u32);
        self.header_set(693, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_inv_slot_bag4(mut self, v: Guid) -> Self {
        self.header_set(694, v.guid() as u32);
        self.header_set(695, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_1(mut self, v: Guid) -> Self {
        self.header_set(696, v.guid() as u32);
        self.header_set(697, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_2(mut self, v: Guid) -> Self {
        self.header_set(698, v.guid() as u32);
        self.header_set(699, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_3(mut self, v: Guid) -> Self {
        self.header_set(700, v.guid() as u32);
        self.header_set(701, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_4(mut self, v: Guid) -> Self {
        self.header_set(702, v.guid() as u32);
        self.header_set(703, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_5(mut self, v: Guid) -> Self {
        self.header_set(704, v.guid() as u32);
        self.header_set(705, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_6(mut self, v: Guid) -> Self {
        self.header_set(706, v.guid() as u32);
        self.header_set(707, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_7(mut self, v: Guid) -> Self {
        self.header_set(708, v.guid() as u32);
        self.header_set(709, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_8(mut self, v: Guid) -> Self {
        self.header_set(710, v.guid() as u32);
        self.header_set(711, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_9(mut self, v: Guid) -> Self {
        self.header_set(712, v.guid() as u32);
        self.header_set(713, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_10(mut self, v: Guid) -> Self {
        self.header_set(714, v.guid() as u32);
        self.header_set(715, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_11(mut self, v: Guid) -> Self {
        self.header_set(716, v.guid() as u32);
        self.header_set(717, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_12(mut self, v: Guid) -> Self {
        self.header_set(718, v.guid() as u32);
        self.header_set(719, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_13(mut self, v: Guid) -> Self {
        self.header_set(720, v.guid() as u32);
        self.header_set(721, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_14(mut self, v: Guid) -> Self {
        self.header_set(722, v.guid() as u32);
        self.header_set(723, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_15(mut self, v: Guid) -> Self {
        self.header_set(724, v.guid() as u32);
        self.header_set(725, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_pack_slot_16(mut self, v: Guid) -> Self {
        self.header_set(726, v.guid() as u32);
        self.header_set(727, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_1(mut self, v: Guid) -> Self {
        self.header_set(728, v.guid() as u32);
        self.header_set(729, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_2(mut self, v: Guid) -> Self {
        self.header_set(730, v.guid() as u32);
        self.header_set(731, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_3(mut self, v: Guid) -> Self {
        self.header_set(732, v.guid() as u32);
        self.header_set(733, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_4(mut self, v: Guid) -> Self {
        self.header_set(734, v.guid() as u32);
        self.header_set(735, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_5(mut self, v: Guid) -> Self {
        self.header_set(736, v.guid() as u32);
        self.header_set(737, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_6(mut self, v: Guid) -> Self {
        self.header_set(738, v.guid() as u32);
        self.header_set(739, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_7(mut self, v: Guid) -> Self {
        self.header_set(740, v.guid() as u32);
        self.header_set(741, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_8(mut self, v: Guid) -> Self {
        self.header_set(742, v.guid() as u32);
        self.header_set(743, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_9(mut self, v: Guid) -> Self {
        self.header_set(744, v.guid() as u32);
        self.header_set(745, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_10(mut self, v: Guid) -> Self {
        self.header_set(746, v.guid() as u32);
        self.header_set(747, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_11(mut self, v: Guid) -> Self {
        self.header_set(748, v.guid() as u32);
        self.header_set(749, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_12(mut self, v: Guid) -> Self {
        self.header_set(750, v.guid() as u32);
        self.header_set(751, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_13(mut self, v: Guid) -> Self {
        self.header_set(752, v.guid() as u32);
        self.header_set(753, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_14(mut self, v: Guid) -> Self {
        self.header_set(754, v.guid() as u32);
        self.header_set(755, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_15(mut self, v: Guid) -> Self {
        self.header_set(756, v.guid() as u32);
        self.header_set(757, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_16(mut self, v: Guid) -> Self {
        self.header_set(758, v.guid() as u32);
        self.header_set(759, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_17(mut self, v: Guid) -> Self {
        self.header_set(760, v.guid() as u32);
        self.header_set(761, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_18(mut self, v: Guid) -> Self {
        self.header_set(762, v.guid() as u32);
        self.header_set(763, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_19(mut self, v: Guid) -> Self {
        self.header_set(764, v.guid() as u32);
        self.header_set(765, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_20(mut self, v: Guid) -> Self {
        self.header_set(766, v.guid() as u32);
        self.header_set(767, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_21(mut self, v: Guid) -> Self {
        self.header_set(768, v.guid() as u32);
        self.header_set(769, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_22(mut self, v: Guid) -> Self {
        self.header_set(770, v.guid() as u32);
        self.header_set(771, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_23(mut self, v: Guid) -> Self {
        self.header_set(772, v.guid() as u32);
        self.header_set(773, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_24(mut self, v: Guid) -> Self {
        self.header_set(774, v.guid() as u32);
        self.header_set(775, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_25(mut self, v: Guid) -> Self {
        self.header_set(776, v.guid() as u32);
        self.header_set(777, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_26(mut self, v: Guid) -> Self {
        self.header_set(778, v.guid() as u32);
        self.header_set(779, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_27(mut self, v: Guid) -> Self {
        self.header_set(780, v.guid() as u32);
        self.header_set(781, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bank_slot_28(mut self, v: Guid) -> Self {
        self.header_set(782, v.guid() as u32);
        self.header_set(783, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_1(mut self, v: Guid) -> Self {
        self.header_set(784, v.guid() as u32);
        self.header_set(785, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_2(mut self, v: Guid) -> Self {
        self.header_set(786, v.guid() as u32);
        self.header_set(787, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_3(mut self, v: Guid) -> Self {
        self.header_set(788, v.guid() as u32);
        self.header_set(789, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_4(mut self, v: Guid) -> Self {
        self.header_set(790, v.guid() as u32);
        self.header_set(791, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_5(mut self, v: Guid) -> Self {
        self.header_set(792, v.guid() as u32);
        self.header_set(793, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_6(mut self, v: Guid) -> Self {
        self.header_set(794, v.guid() as u32);
        self.header_set(795, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_bankbag_slot_7(mut self, v: Guid) -> Self {
        self.header_set(796, v.guid() as u32);
        self.header_set(797, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_1(mut self, v: Guid) -> Self {
        self.header_set(798, v.guid() as u32);
        self.header_set(799, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_2(mut self, v: Guid) -> Self {
        self.header_set(800, v.guid() as u32);
        self.header_set(801, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_3(mut self, v: Guid) -> Self {
        self.header_set(802, v.guid() as u32);
        self.header_set(803, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_4(mut self, v: Guid) -> Self {
        self.header_set(804, v.guid() as u32);
        self.header_set(805, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_5(mut self, v: Guid) -> Self {
        self.header_set(806, v.guid() as u32);
        self.header_set(807, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_6(mut self, v: Guid) -> Self {
        self.header_set(808, v.guid() as u32);
        self.header_set(809, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_7(mut self, v: Guid) -> Self {
        self.header_set(810, v.guid() as u32);
        self.header_set(811, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_8(mut self, v: Guid) -> Self {
        self.header_set(812, v.guid() as u32);
        self.header_set(813, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_9(mut self, v: Guid) -> Self {
        self.header_set(814, v.guid() as u32);
        self.header_set(815, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_10(mut self, v: Guid) -> Self {
        self.header_set(816, v.guid() as u32);
        self.header_set(817, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_11(mut self, v: Guid) -> Self {
        self.header_set(818, v.guid() as u32);
        self.header_set(819, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vendorbuyback_slot_12(mut self, v: Guid) -> Self {
        self.header_set(820, v.guid() as u32);
        self.header_set(821, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_1(mut self, v: Guid) -> Self {
        self.header_set(822, v.guid() as u32);
        self.header_set(823, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_2(mut self, v: Guid) -> Self {
        self.header_set(824, v.guid() as u32);
        self.header_set(825, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_3(mut self, v: Guid) -> Self {
        self.header_set(826, v.guid() as u32);
        self.header_set(827, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_4(mut self, v: Guid) -> Self {
        self.header_set(828, v.guid() as u32);
        self.header_set(829, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_5(mut self, v: Guid) -> Self {
        self.header_set(830, v.guid() as u32);
        self.header_set(831, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_6(mut self, v: Guid) -> Self {
        self.header_set(832, v.guid() as u32);
        self.header_set(833, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_7(mut self, v: Guid) -> Self {
        self.header_set(834, v.guid() as u32);
        self.header_set(835, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_8(mut self, v: Guid) -> Self {
        self.header_set(836, v.guid() as u32);
        self.header_set(837, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_9(mut self, v: Guid) -> Self {
        self.header_set(838, v.guid() as u32);
        self.header_set(839, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_10(mut self, v: Guid) -> Self {
        self.header_set(840, v.guid() as u32);
        self.header_set(841, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_11(mut self, v: Guid) -> Self {
        self.header_set(842, v.guid() as u32);
        self.header_set(843, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_12(mut self, v: Guid) -> Self {
        self.header_set(844, v.guid() as u32);
        self.header_set(845, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_13(mut self, v: Guid) -> Self {
        self.header_set(846, v.guid() as u32);
        self.header_set(847, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_14(mut self, v: Guid) -> Self {
        self.header_set(848, v.guid() as u32);
        self.header_set(849, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_15(mut self, v: Guid) -> Self {
        self.header_set(850, v.guid() as u32);
        self.header_set(851, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_16(mut self, v: Guid) -> Self {
        self.header_set(852, v.guid() as u32);
        self.header_set(853, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_17(mut self, v: Guid) -> Self {
        self.header_set(854, v.guid() as u32);
        self.header_set(855, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_18(mut self, v: Guid) -> Self {
        self.header_set(856, v.guid() as u32);
        self.header_set(857, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_19(mut self, v: Guid) -> Self {
        self.header_set(858, v.guid() as u32);
        self.header_set(859, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_20(mut self, v: Guid) -> Self {
        self.header_set(860, v.guid() as u32);
        self.header_set(861, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_21(mut self, v: Guid) -> Self {
        self.header_set(862, v.guid() as u32);
        self.header_set(863, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_22(mut self, v: Guid) -> Self {
        self.header_set(864, v.guid() as u32);
        self.header_set(865, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_23(mut self, v: Guid) -> Self {
        self.header_set(866, v.guid() as u32);
        self.header_set(867, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_24(mut self, v: Guid) -> Self {
        self.header_set(868, v.guid() as u32);
        self.header_set(869, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_25(mut self, v: Guid) -> Self {
        self.header_set(870, v.guid() as u32);
        self.header_set(871, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_26(mut self, v: Guid) -> Self {
        self.header_set(872, v.guid() as u32);
        self.header_set(873, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_27(mut self, v: Guid) -> Self {
        self.header_set(874, v.guid() as u32);
        self.header_set(875, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_28(mut self, v: Guid) -> Self {
        self.header_set(876, v.guid() as u32);
        self.header_set(877, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_29(mut self, v: Guid) -> Self {
        self.header_set(878, v.guid() as u32);
        self.header_set(879, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_30(mut self, v: Guid) -> Self {
        self.header_set(880, v.guid() as u32);
        self.header_set(881, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_31(mut self, v: Guid) -> Self {
        self.header_set(882, v.guid() as u32);
        self.header_set(883, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_keyring_slot_32(mut self, v: Guid) -> Self {
        self.header_set(884, v.guid() as u32);
        self.header_set(885, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_1(mut self, v: Guid) -> Self {
        self.header_set(886, v.guid() as u32);
        self.header_set(887, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_2(mut self, v: Guid) -> Self {
        self.header_set(888, v.guid() as u32);
        self.header_set(889, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_3(mut self, v: Guid) -> Self {
        self.header_set(890, v.guid() as u32);
        self.header_set(891, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_4(mut self, v: Guid) -> Self {
        self.header_set(892, v.guid() as u32);
        self.header_set(893, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_5(mut self, v: Guid) -> Self {
        self.header_set(894, v.guid() as u32);
        self.header_set(895, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_6(mut self, v: Guid) -> Self {
        self.header_set(896, v.guid() as u32);
        self.header_set(897, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_7(mut self, v: Guid) -> Self {
        self.header_set(898, v.guid() as u32);
        self.header_set(899, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_8(mut self, v: Guid) -> Self {
        self.header_set(900, v.guid() as u32);
        self.header_set(901, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_9(mut self, v: Guid) -> Self {
        self.header_set(902, v.guid() as u32);
        self.header_set(903, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_10(mut self, v: Guid) -> Self {
        self.header_set(904, v.guid() as u32);
        self.header_set(905, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_11(mut self, v: Guid) -> Self {
        self.header_set(906, v.guid() as u32);
        self.header_set(907, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_12(mut self, v: Guid) -> Self {
        self.header_set(908, v.guid() as u32);
        self.header_set(909, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_13(mut self, v: Guid) -> Self {
        self.header_set(910, v.guid() as u32);
        self.header_set(911, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_14(mut self, v: Guid) -> Self {
        self.header_set(912, v.guid() as u32);
        self.header_set(913, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_15(mut self, v: Guid) -> Self {
        self.header_set(914, v.guid() as u32);
        self.header_set(915, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_16(mut self, v: Guid) -> Self {
        self.header_set(916, v.guid() as u32);
        self.header_set(917, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_17(mut self, v: Guid) -> Self {
        self.header_set(918, v.guid() as u32);
        self.header_set(919, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_vanitypet_slot_18(mut self, v: Guid) -> Self {
        self.header_set(920, v.guid() as u32);
        self.header_set(921, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_farsight(mut self, v: Guid) -> Self {
        self.header_set(922, v.guid() as u32);
        self.header_set(923, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_known_titles(mut self, v: Guid) -> Self {
        self.header_set(924, v.guid() as u32);
        self.header_set(925, (v.guid() >> 32) as u32);
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
        self.header_set(62, v.guid() as u32);
        self.header_set(63, (v.guid() >> 32) as u32);
    }

    pub fn container_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&62);
        let upper = self.values.get(&63);

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

    pub fn set_unit_charmedby(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn unit_charmedby(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_summonedby(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn unit_summonedby(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_createdby(&mut self, v: Guid) {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
    }

    pub fn unit_createdby(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_target(&mut self, v: Guid) {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
    }

    pub fn unit_target(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_persuaded(&mut self, v: Guid) {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
    }

    pub fn unit_persuaded(&self) -> Option<Guid> {
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

    pub fn set_unit_charmedby(&mut self, v: Guid) {
        self.header_set(10, v.guid() as u32);
        self.header_set(11, (v.guid() >> 32) as u32);
    }

    pub fn unit_charmedby(&self) -> Option<Guid> {
        let lower = self.values.get(&10);
        let upper = self.values.get(&11);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_summonedby(&mut self, v: Guid) {
        self.header_set(12, v.guid() as u32);
        self.header_set(13, (v.guid() >> 32) as u32);
    }

    pub fn unit_summonedby(&self) -> Option<Guid> {
        let lower = self.values.get(&12);
        let upper = self.values.get(&13);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_createdby(&mut self, v: Guid) {
        self.header_set(14, v.guid() as u32);
        self.header_set(15, (v.guid() >> 32) as u32);
    }

    pub fn unit_createdby(&self) -> Option<Guid> {
        let lower = self.values.get(&14);
        let upper = self.values.get(&15);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_target(&mut self, v: Guid) {
        self.header_set(16, v.guid() as u32);
        self.header_set(17, (v.guid() >> 32) as u32);
    }

    pub fn unit_target(&self) -> Option<Guid> {
        let lower = self.values.get(&16);
        let upper = self.values.get(&17);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_unit_persuaded(&mut self, v: Guid) {
        self.header_set(18, v.guid() as u32);
        self.header_set(19, (v.guid() >> 32) as u32);
    }

    pub fn unit_persuaded(&self) -> Option<Guid> {
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
        self.header_set(234, v.guid() as u32);
        self.header_set(235, (v.guid() >> 32) as u32);
    }

    pub fn player_duel_arbiter(&self) -> Option<Guid> {
        let lower = self.values.get(&234);
        let upper = self.values.get(&235);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(344, v.guid() as u32);
        self.header_set(345, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_1_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&344);
        let upper = self.values.get(&345);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(360, v.guid() as u32);
        self.header_set(361, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_2_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&360);
        let upper = self.values.get(&361);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(376, v.guid() as u32);
        self.header_set(377, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_3_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&376);
        let upper = self.values.get(&377);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(392, v.guid() as u32);
        self.header_set(393, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_4_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&392);
        let upper = self.values.get(&393);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(408, v.guid() as u32);
        self.header_set(409, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_5_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&408);
        let upper = self.values.get(&409);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(424, v.guid() as u32);
        self.header_set(425, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_6_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&424);
        let upper = self.values.get(&425);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(440, v.guid() as u32);
        self.header_set(441, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_7_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&440);
        let upper = self.values.get(&441);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(456, v.guid() as u32);
        self.header_set(457, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_8_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&456);
        let upper = self.values.get(&457);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(472, v.guid() as u32);
        self.header_set(473, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_9_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&472);
        let upper = self.values.get(&473);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(488, v.guid() as u32);
        self.header_set(489, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_10_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&488);
        let upper = self.values.get(&489);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(504, v.guid() as u32);
        self.header_set(505, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_11_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&504);
        let upper = self.values.get(&505);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(520, v.guid() as u32);
        self.header_set(521, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_12_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&520);
        let upper = self.values.get(&521);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(536, v.guid() as u32);
        self.header_set(537, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_13_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&536);
        let upper = self.values.get(&537);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(552, v.guid() as u32);
        self.header_set(553, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_14_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&552);
        let upper = self.values.get(&553);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(568, v.guid() as u32);
        self.header_set(569, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_15_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&568);
        let upper = self.values.get(&569);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(584, v.guid() as u32);
        self.header_set(585, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_16_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&584);
        let upper = self.values.get(&585);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(600, v.guid() as u32);
        self.header_set(601, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_17_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&600);
        let upper = self.values.get(&601);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(616, v.guid() as u32);
        self.header_set(617, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_18_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&616);
        let upper = self.values.get(&617);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(632, v.guid() as u32);
        self.header_set(633, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_19_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&632);
        let upper = self.values.get(&633);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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
        self.header_set(650, v.guid() as u32);
        self.header_set(651, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_head(&self) -> Option<Guid> {
        let lower = self.values.get(&650);
        let upper = self.values.get(&651);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_neck(&mut self, v: Guid) {
        self.header_set(652, v.guid() as u32);
        self.header_set(653, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_neck(&self) -> Option<Guid> {
        let lower = self.values.get(&652);
        let upper = self.values.get(&653);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_shoulders(&mut self, v: Guid) {
        self.header_set(654, v.guid() as u32);
        self.header_set(655, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_shoulders(&self) -> Option<Guid> {
        let lower = self.values.get(&654);
        let upper = self.values.get(&655);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_body(&mut self, v: Guid) {
        self.header_set(656, v.guid() as u32);
        self.header_set(657, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_body(&self) -> Option<Guid> {
        let lower = self.values.get(&656);
        let upper = self.values.get(&657);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_chest(&mut self, v: Guid) {
        self.header_set(658, v.guid() as u32);
        self.header_set(659, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_chest(&self) -> Option<Guid> {
        let lower = self.values.get(&658);
        let upper = self.values.get(&659);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_waist(&mut self, v: Guid) {
        self.header_set(660, v.guid() as u32);
        self.header_set(661, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_waist(&self) -> Option<Guid> {
        let lower = self.values.get(&660);
        let upper = self.values.get(&661);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_legs(&mut self, v: Guid) {
        self.header_set(662, v.guid() as u32);
        self.header_set(663, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_legs(&self) -> Option<Guid> {
        let lower = self.values.get(&662);
        let upper = self.values.get(&663);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_feet(&mut self, v: Guid) {
        self.header_set(664, v.guid() as u32);
        self.header_set(665, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_feet(&self) -> Option<Guid> {
        let lower = self.values.get(&664);
        let upper = self.values.get(&665);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_wrists(&mut self, v: Guid) {
        self.header_set(666, v.guid() as u32);
        self.header_set(667, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_wrists(&self) -> Option<Guid> {
        let lower = self.values.get(&666);
        let upper = self.values.get(&667);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_hands(&mut self, v: Guid) {
        self.header_set(668, v.guid() as u32);
        self.header_set(669, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_hands(&self) -> Option<Guid> {
        let lower = self.values.get(&668);
        let upper = self.values.get(&669);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_finger1(&mut self, v: Guid) {
        self.header_set(670, v.guid() as u32);
        self.header_set(671, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_finger1(&self) -> Option<Guid> {
        let lower = self.values.get(&670);
        let upper = self.values.get(&671);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_finger2(&mut self, v: Guid) {
        self.header_set(672, v.guid() as u32);
        self.header_set(673, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_finger2(&self) -> Option<Guid> {
        let lower = self.values.get(&672);
        let upper = self.values.get(&673);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_trinket1(&mut self, v: Guid) {
        self.header_set(674, v.guid() as u32);
        self.header_set(675, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_trinket1(&self) -> Option<Guid> {
        let lower = self.values.get(&674);
        let upper = self.values.get(&675);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_trinket2(&mut self, v: Guid) {
        self.header_set(676, v.guid() as u32);
        self.header_set(677, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_trinket2(&self) -> Option<Guid> {
        let lower = self.values.get(&676);
        let upper = self.values.get(&677);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_back(&mut self, v: Guid) {
        self.header_set(678, v.guid() as u32);
        self.header_set(679, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_back(&self) -> Option<Guid> {
        let lower = self.values.get(&678);
        let upper = self.values.get(&679);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_main_hand(&mut self, v: Guid) {
        self.header_set(680, v.guid() as u32);
        self.header_set(681, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_main_hand(&self) -> Option<Guid> {
        let lower = self.values.get(&680);
        let upper = self.values.get(&681);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_off_hand(&mut self, v: Guid) {
        self.header_set(682, v.guid() as u32);
        self.header_set(683, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_off_hand(&self) -> Option<Guid> {
        let lower = self.values.get(&682);
        let upper = self.values.get(&683);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_ranged(&mut self, v: Guid) {
        self.header_set(684, v.guid() as u32);
        self.header_set(685, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_ranged(&self) -> Option<Guid> {
        let lower = self.values.get(&684);
        let upper = self.values.get(&685);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_tabard(&mut self, v: Guid) {
        self.header_set(686, v.guid() as u32);
        self.header_set(687, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_tabard(&self) -> Option<Guid> {
        let lower = self.values.get(&686);
        let upper = self.values.get(&687);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_bag1(&mut self, v: Guid) {
        self.header_set(688, v.guid() as u32);
        self.header_set(689, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_bag1(&self) -> Option<Guid> {
        let lower = self.values.get(&688);
        let upper = self.values.get(&689);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_bag2(&mut self, v: Guid) {
        self.header_set(690, v.guid() as u32);
        self.header_set(691, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_bag2(&self) -> Option<Guid> {
        let lower = self.values.get(&690);
        let upper = self.values.get(&691);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_bag3(&mut self, v: Guid) {
        self.header_set(692, v.guid() as u32);
        self.header_set(693, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_bag3(&self) -> Option<Guid> {
        let lower = self.values.get(&692);
        let upper = self.values.get(&693);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_inv_slot_bag4(&mut self, v: Guid) {
        self.header_set(694, v.guid() as u32);
        self.header_set(695, (v.guid() >> 32) as u32);
    }

    pub fn player_inv_slot_bag4(&self) -> Option<Guid> {
        let lower = self.values.get(&694);
        let upper = self.values.get(&695);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_1(&mut self, v: Guid) {
        self.header_set(696, v.guid() as u32);
        self.header_set(697, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&696);
        let upper = self.values.get(&697);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_2(&mut self, v: Guid) {
        self.header_set(698, v.guid() as u32);
        self.header_set(699, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&698);
        let upper = self.values.get(&699);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_3(&mut self, v: Guid) {
        self.header_set(700, v.guid() as u32);
        self.header_set(701, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&700);
        let upper = self.values.get(&701);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_4(&mut self, v: Guid) {
        self.header_set(702, v.guid() as u32);
        self.header_set(703, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&702);
        let upper = self.values.get(&703);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_5(&mut self, v: Guid) {
        self.header_set(704, v.guid() as u32);
        self.header_set(705, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&704);
        let upper = self.values.get(&705);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_6(&mut self, v: Guid) {
        self.header_set(706, v.guid() as u32);
        self.header_set(707, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&706);
        let upper = self.values.get(&707);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_7(&mut self, v: Guid) {
        self.header_set(708, v.guid() as u32);
        self.header_set(709, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&708);
        let upper = self.values.get(&709);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_8(&mut self, v: Guid) {
        self.header_set(710, v.guid() as u32);
        self.header_set(711, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_8(&self) -> Option<Guid> {
        let lower = self.values.get(&710);
        let upper = self.values.get(&711);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_9(&mut self, v: Guid) {
        self.header_set(712, v.guid() as u32);
        self.header_set(713, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_9(&self) -> Option<Guid> {
        let lower = self.values.get(&712);
        let upper = self.values.get(&713);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_10(&mut self, v: Guid) {
        self.header_set(714, v.guid() as u32);
        self.header_set(715, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_10(&self) -> Option<Guid> {
        let lower = self.values.get(&714);
        let upper = self.values.get(&715);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_11(&mut self, v: Guid) {
        self.header_set(716, v.guid() as u32);
        self.header_set(717, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_11(&self) -> Option<Guid> {
        let lower = self.values.get(&716);
        let upper = self.values.get(&717);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_12(&mut self, v: Guid) {
        self.header_set(718, v.guid() as u32);
        self.header_set(719, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_12(&self) -> Option<Guid> {
        let lower = self.values.get(&718);
        let upper = self.values.get(&719);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_13(&mut self, v: Guid) {
        self.header_set(720, v.guid() as u32);
        self.header_set(721, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_13(&self) -> Option<Guid> {
        let lower = self.values.get(&720);
        let upper = self.values.get(&721);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_14(&mut self, v: Guid) {
        self.header_set(722, v.guid() as u32);
        self.header_set(723, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_14(&self) -> Option<Guid> {
        let lower = self.values.get(&722);
        let upper = self.values.get(&723);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_15(&mut self, v: Guid) {
        self.header_set(724, v.guid() as u32);
        self.header_set(725, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_15(&self) -> Option<Guid> {
        let lower = self.values.get(&724);
        let upper = self.values.get(&725);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_pack_slot_16(&mut self, v: Guid) {
        self.header_set(726, v.guid() as u32);
        self.header_set(727, (v.guid() >> 32) as u32);
    }

    pub fn player_pack_slot_16(&self) -> Option<Guid> {
        let lower = self.values.get(&726);
        let upper = self.values.get(&727);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_1(&mut self, v: Guid) {
        self.header_set(728, v.guid() as u32);
        self.header_set(729, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&728);
        let upper = self.values.get(&729);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_2(&mut self, v: Guid) {
        self.header_set(730, v.guid() as u32);
        self.header_set(731, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&730);
        let upper = self.values.get(&731);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_3(&mut self, v: Guid) {
        self.header_set(732, v.guid() as u32);
        self.header_set(733, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&732);
        let upper = self.values.get(&733);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_4(&mut self, v: Guid) {
        self.header_set(734, v.guid() as u32);
        self.header_set(735, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&734);
        let upper = self.values.get(&735);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_5(&mut self, v: Guid) {
        self.header_set(736, v.guid() as u32);
        self.header_set(737, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&736);
        let upper = self.values.get(&737);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_6(&mut self, v: Guid) {
        self.header_set(738, v.guid() as u32);
        self.header_set(739, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&738);
        let upper = self.values.get(&739);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_7(&mut self, v: Guid) {
        self.header_set(740, v.guid() as u32);
        self.header_set(741, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&740);
        let upper = self.values.get(&741);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_8(&mut self, v: Guid) {
        self.header_set(742, v.guid() as u32);
        self.header_set(743, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_8(&self) -> Option<Guid> {
        let lower = self.values.get(&742);
        let upper = self.values.get(&743);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_9(&mut self, v: Guid) {
        self.header_set(744, v.guid() as u32);
        self.header_set(745, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_9(&self) -> Option<Guid> {
        let lower = self.values.get(&744);
        let upper = self.values.get(&745);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_10(&mut self, v: Guid) {
        self.header_set(746, v.guid() as u32);
        self.header_set(747, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_10(&self) -> Option<Guid> {
        let lower = self.values.get(&746);
        let upper = self.values.get(&747);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_11(&mut self, v: Guid) {
        self.header_set(748, v.guid() as u32);
        self.header_set(749, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_11(&self) -> Option<Guid> {
        let lower = self.values.get(&748);
        let upper = self.values.get(&749);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_12(&mut self, v: Guid) {
        self.header_set(750, v.guid() as u32);
        self.header_set(751, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_12(&self) -> Option<Guid> {
        let lower = self.values.get(&750);
        let upper = self.values.get(&751);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_13(&mut self, v: Guid) {
        self.header_set(752, v.guid() as u32);
        self.header_set(753, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_13(&self) -> Option<Guid> {
        let lower = self.values.get(&752);
        let upper = self.values.get(&753);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_14(&mut self, v: Guid) {
        self.header_set(754, v.guid() as u32);
        self.header_set(755, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_14(&self) -> Option<Guid> {
        let lower = self.values.get(&754);
        let upper = self.values.get(&755);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_15(&mut self, v: Guid) {
        self.header_set(756, v.guid() as u32);
        self.header_set(757, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_15(&self) -> Option<Guid> {
        let lower = self.values.get(&756);
        let upper = self.values.get(&757);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_16(&mut self, v: Guid) {
        self.header_set(758, v.guid() as u32);
        self.header_set(759, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_16(&self) -> Option<Guid> {
        let lower = self.values.get(&758);
        let upper = self.values.get(&759);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_17(&mut self, v: Guid) {
        self.header_set(760, v.guid() as u32);
        self.header_set(761, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_17(&self) -> Option<Guid> {
        let lower = self.values.get(&760);
        let upper = self.values.get(&761);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_18(&mut self, v: Guid) {
        self.header_set(762, v.guid() as u32);
        self.header_set(763, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_18(&self) -> Option<Guid> {
        let lower = self.values.get(&762);
        let upper = self.values.get(&763);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_19(&mut self, v: Guid) {
        self.header_set(764, v.guid() as u32);
        self.header_set(765, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_19(&self) -> Option<Guid> {
        let lower = self.values.get(&764);
        let upper = self.values.get(&765);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_20(&mut self, v: Guid) {
        self.header_set(766, v.guid() as u32);
        self.header_set(767, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_20(&self) -> Option<Guid> {
        let lower = self.values.get(&766);
        let upper = self.values.get(&767);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_21(&mut self, v: Guid) {
        self.header_set(768, v.guid() as u32);
        self.header_set(769, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_21(&self) -> Option<Guid> {
        let lower = self.values.get(&768);
        let upper = self.values.get(&769);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_22(&mut self, v: Guid) {
        self.header_set(770, v.guid() as u32);
        self.header_set(771, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_22(&self) -> Option<Guid> {
        let lower = self.values.get(&770);
        let upper = self.values.get(&771);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_23(&mut self, v: Guid) {
        self.header_set(772, v.guid() as u32);
        self.header_set(773, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_23(&self) -> Option<Guid> {
        let lower = self.values.get(&772);
        let upper = self.values.get(&773);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_24(&mut self, v: Guid) {
        self.header_set(774, v.guid() as u32);
        self.header_set(775, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_24(&self) -> Option<Guid> {
        let lower = self.values.get(&774);
        let upper = self.values.get(&775);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_25(&mut self, v: Guid) {
        self.header_set(776, v.guid() as u32);
        self.header_set(777, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_25(&self) -> Option<Guid> {
        let lower = self.values.get(&776);
        let upper = self.values.get(&777);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_26(&mut self, v: Guid) {
        self.header_set(778, v.guid() as u32);
        self.header_set(779, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_26(&self) -> Option<Guid> {
        let lower = self.values.get(&778);
        let upper = self.values.get(&779);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_27(&mut self, v: Guid) {
        self.header_set(780, v.guid() as u32);
        self.header_set(781, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_27(&self) -> Option<Guid> {
        let lower = self.values.get(&780);
        let upper = self.values.get(&781);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bank_slot_28(&mut self, v: Guid) {
        self.header_set(782, v.guid() as u32);
        self.header_set(783, (v.guid() >> 32) as u32);
    }

    pub fn player_bank_slot_28(&self) -> Option<Guid> {
        let lower = self.values.get(&782);
        let upper = self.values.get(&783);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_1(&mut self, v: Guid) {
        self.header_set(784, v.guid() as u32);
        self.header_set(785, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&784);
        let upper = self.values.get(&785);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_2(&mut self, v: Guid) {
        self.header_set(786, v.guid() as u32);
        self.header_set(787, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&786);
        let upper = self.values.get(&787);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_3(&mut self, v: Guid) {
        self.header_set(788, v.guid() as u32);
        self.header_set(789, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&788);
        let upper = self.values.get(&789);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_4(&mut self, v: Guid) {
        self.header_set(790, v.guid() as u32);
        self.header_set(791, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&790);
        let upper = self.values.get(&791);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_5(&mut self, v: Guid) {
        self.header_set(792, v.guid() as u32);
        self.header_set(793, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&792);
        let upper = self.values.get(&793);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_6(&mut self, v: Guid) {
        self.header_set(794, v.guid() as u32);
        self.header_set(795, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&794);
        let upper = self.values.get(&795);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_bankbag_slot_7(&mut self, v: Guid) {
        self.header_set(796, v.guid() as u32);
        self.header_set(797, (v.guid() >> 32) as u32);
    }

    pub fn player_bankbag_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&796);
        let upper = self.values.get(&797);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_1(&mut self, v: Guid) {
        self.header_set(798, v.guid() as u32);
        self.header_set(799, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&798);
        let upper = self.values.get(&799);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_2(&mut self, v: Guid) {
        self.header_set(800, v.guid() as u32);
        self.header_set(801, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&800);
        let upper = self.values.get(&801);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_3(&mut self, v: Guid) {
        self.header_set(802, v.guid() as u32);
        self.header_set(803, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&802);
        let upper = self.values.get(&803);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_4(&mut self, v: Guid) {
        self.header_set(804, v.guid() as u32);
        self.header_set(805, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&804);
        let upper = self.values.get(&805);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_5(&mut self, v: Guid) {
        self.header_set(806, v.guid() as u32);
        self.header_set(807, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&806);
        let upper = self.values.get(&807);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_6(&mut self, v: Guid) {
        self.header_set(808, v.guid() as u32);
        self.header_set(809, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&808);
        let upper = self.values.get(&809);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_7(&mut self, v: Guid) {
        self.header_set(810, v.guid() as u32);
        self.header_set(811, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&810);
        let upper = self.values.get(&811);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_8(&mut self, v: Guid) {
        self.header_set(812, v.guid() as u32);
        self.header_set(813, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_8(&self) -> Option<Guid> {
        let lower = self.values.get(&812);
        let upper = self.values.get(&813);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_9(&mut self, v: Guid) {
        self.header_set(814, v.guid() as u32);
        self.header_set(815, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_9(&self) -> Option<Guid> {
        let lower = self.values.get(&814);
        let upper = self.values.get(&815);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_10(&mut self, v: Guid) {
        self.header_set(816, v.guid() as u32);
        self.header_set(817, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_10(&self) -> Option<Guid> {
        let lower = self.values.get(&816);
        let upper = self.values.get(&817);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_11(&mut self, v: Guid) {
        self.header_set(818, v.guid() as u32);
        self.header_set(819, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_11(&self) -> Option<Guid> {
        let lower = self.values.get(&818);
        let upper = self.values.get(&819);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vendorbuyback_slot_12(&mut self, v: Guid) {
        self.header_set(820, v.guid() as u32);
        self.header_set(821, (v.guid() >> 32) as u32);
    }

    pub fn player_vendorbuyback_slot_12(&self) -> Option<Guid> {
        let lower = self.values.get(&820);
        let upper = self.values.get(&821);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_1(&mut self, v: Guid) {
        self.header_set(822, v.guid() as u32);
        self.header_set(823, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&822);
        let upper = self.values.get(&823);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_2(&mut self, v: Guid) {
        self.header_set(824, v.guid() as u32);
        self.header_set(825, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&824);
        let upper = self.values.get(&825);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_3(&mut self, v: Guid) {
        self.header_set(826, v.guid() as u32);
        self.header_set(827, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&826);
        let upper = self.values.get(&827);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_4(&mut self, v: Guid) {
        self.header_set(828, v.guid() as u32);
        self.header_set(829, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&828);
        let upper = self.values.get(&829);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_5(&mut self, v: Guid) {
        self.header_set(830, v.guid() as u32);
        self.header_set(831, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&830);
        let upper = self.values.get(&831);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_6(&mut self, v: Guid) {
        self.header_set(832, v.guid() as u32);
        self.header_set(833, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&832);
        let upper = self.values.get(&833);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_7(&mut self, v: Guid) {
        self.header_set(834, v.guid() as u32);
        self.header_set(835, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&834);
        let upper = self.values.get(&835);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_8(&mut self, v: Guid) {
        self.header_set(836, v.guid() as u32);
        self.header_set(837, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_8(&self) -> Option<Guid> {
        let lower = self.values.get(&836);
        let upper = self.values.get(&837);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_9(&mut self, v: Guid) {
        self.header_set(838, v.guid() as u32);
        self.header_set(839, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_9(&self) -> Option<Guid> {
        let lower = self.values.get(&838);
        let upper = self.values.get(&839);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_10(&mut self, v: Guid) {
        self.header_set(840, v.guid() as u32);
        self.header_set(841, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_10(&self) -> Option<Guid> {
        let lower = self.values.get(&840);
        let upper = self.values.get(&841);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_11(&mut self, v: Guid) {
        self.header_set(842, v.guid() as u32);
        self.header_set(843, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_11(&self) -> Option<Guid> {
        let lower = self.values.get(&842);
        let upper = self.values.get(&843);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_12(&mut self, v: Guid) {
        self.header_set(844, v.guid() as u32);
        self.header_set(845, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_12(&self) -> Option<Guid> {
        let lower = self.values.get(&844);
        let upper = self.values.get(&845);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_13(&mut self, v: Guid) {
        self.header_set(846, v.guid() as u32);
        self.header_set(847, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_13(&self) -> Option<Guid> {
        let lower = self.values.get(&846);
        let upper = self.values.get(&847);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_14(&mut self, v: Guid) {
        self.header_set(848, v.guid() as u32);
        self.header_set(849, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_14(&self) -> Option<Guid> {
        let lower = self.values.get(&848);
        let upper = self.values.get(&849);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_15(&mut self, v: Guid) {
        self.header_set(850, v.guid() as u32);
        self.header_set(851, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_15(&self) -> Option<Guid> {
        let lower = self.values.get(&850);
        let upper = self.values.get(&851);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_16(&mut self, v: Guid) {
        self.header_set(852, v.guid() as u32);
        self.header_set(853, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_16(&self) -> Option<Guid> {
        let lower = self.values.get(&852);
        let upper = self.values.get(&853);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_17(&mut self, v: Guid) {
        self.header_set(854, v.guid() as u32);
        self.header_set(855, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_17(&self) -> Option<Guid> {
        let lower = self.values.get(&854);
        let upper = self.values.get(&855);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_18(&mut self, v: Guid) {
        self.header_set(856, v.guid() as u32);
        self.header_set(857, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_18(&self) -> Option<Guid> {
        let lower = self.values.get(&856);
        let upper = self.values.get(&857);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_19(&mut self, v: Guid) {
        self.header_set(858, v.guid() as u32);
        self.header_set(859, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_19(&self) -> Option<Guid> {
        let lower = self.values.get(&858);
        let upper = self.values.get(&859);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_20(&mut self, v: Guid) {
        self.header_set(860, v.guid() as u32);
        self.header_set(861, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_20(&self) -> Option<Guid> {
        let lower = self.values.get(&860);
        let upper = self.values.get(&861);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_21(&mut self, v: Guid) {
        self.header_set(862, v.guid() as u32);
        self.header_set(863, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_21(&self) -> Option<Guid> {
        let lower = self.values.get(&862);
        let upper = self.values.get(&863);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_22(&mut self, v: Guid) {
        self.header_set(864, v.guid() as u32);
        self.header_set(865, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_22(&self) -> Option<Guid> {
        let lower = self.values.get(&864);
        let upper = self.values.get(&865);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_23(&mut self, v: Guid) {
        self.header_set(866, v.guid() as u32);
        self.header_set(867, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_23(&self) -> Option<Guid> {
        let lower = self.values.get(&866);
        let upper = self.values.get(&867);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_24(&mut self, v: Guid) {
        self.header_set(868, v.guid() as u32);
        self.header_set(869, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_24(&self) -> Option<Guid> {
        let lower = self.values.get(&868);
        let upper = self.values.get(&869);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_25(&mut self, v: Guid) {
        self.header_set(870, v.guid() as u32);
        self.header_set(871, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_25(&self) -> Option<Guid> {
        let lower = self.values.get(&870);
        let upper = self.values.get(&871);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_26(&mut self, v: Guid) {
        self.header_set(872, v.guid() as u32);
        self.header_set(873, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_26(&self) -> Option<Guid> {
        let lower = self.values.get(&872);
        let upper = self.values.get(&873);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_27(&mut self, v: Guid) {
        self.header_set(874, v.guid() as u32);
        self.header_set(875, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_27(&self) -> Option<Guid> {
        let lower = self.values.get(&874);
        let upper = self.values.get(&875);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_28(&mut self, v: Guid) {
        self.header_set(876, v.guid() as u32);
        self.header_set(877, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_28(&self) -> Option<Guid> {
        let lower = self.values.get(&876);
        let upper = self.values.get(&877);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_29(&mut self, v: Guid) {
        self.header_set(878, v.guid() as u32);
        self.header_set(879, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_29(&self) -> Option<Guid> {
        let lower = self.values.get(&878);
        let upper = self.values.get(&879);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_30(&mut self, v: Guid) {
        self.header_set(880, v.guid() as u32);
        self.header_set(881, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_30(&self) -> Option<Guid> {
        let lower = self.values.get(&880);
        let upper = self.values.get(&881);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_31(&mut self, v: Guid) {
        self.header_set(882, v.guid() as u32);
        self.header_set(883, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_31(&self) -> Option<Guid> {
        let lower = self.values.get(&882);
        let upper = self.values.get(&883);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_keyring_slot_32(&mut self, v: Guid) {
        self.header_set(884, v.guid() as u32);
        self.header_set(885, (v.guid() >> 32) as u32);
    }

    pub fn player_keyring_slot_32(&self) -> Option<Guid> {
        let lower = self.values.get(&884);
        let upper = self.values.get(&885);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_1(&mut self, v: Guid) {
        self.header_set(886, v.guid() as u32);
        self.header_set(887, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&886);
        let upper = self.values.get(&887);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_2(&mut self, v: Guid) {
        self.header_set(888, v.guid() as u32);
        self.header_set(889, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_2(&self) -> Option<Guid> {
        let lower = self.values.get(&888);
        let upper = self.values.get(&889);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_3(&mut self, v: Guid) {
        self.header_set(890, v.guid() as u32);
        self.header_set(891, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_3(&self) -> Option<Guid> {
        let lower = self.values.get(&890);
        let upper = self.values.get(&891);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_4(&mut self, v: Guid) {
        self.header_set(892, v.guid() as u32);
        self.header_set(893, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_4(&self) -> Option<Guid> {
        let lower = self.values.get(&892);
        let upper = self.values.get(&893);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_5(&mut self, v: Guid) {
        self.header_set(894, v.guid() as u32);
        self.header_set(895, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_5(&self) -> Option<Guid> {
        let lower = self.values.get(&894);
        let upper = self.values.get(&895);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_6(&mut self, v: Guid) {
        self.header_set(896, v.guid() as u32);
        self.header_set(897, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_6(&self) -> Option<Guid> {
        let lower = self.values.get(&896);
        let upper = self.values.get(&897);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_7(&mut self, v: Guid) {
        self.header_set(898, v.guid() as u32);
        self.header_set(899, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_7(&self) -> Option<Guid> {
        let lower = self.values.get(&898);
        let upper = self.values.get(&899);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_8(&mut self, v: Guid) {
        self.header_set(900, v.guid() as u32);
        self.header_set(901, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_8(&self) -> Option<Guid> {
        let lower = self.values.get(&900);
        let upper = self.values.get(&901);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_9(&mut self, v: Guid) {
        self.header_set(902, v.guid() as u32);
        self.header_set(903, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_9(&self) -> Option<Guid> {
        let lower = self.values.get(&902);
        let upper = self.values.get(&903);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_10(&mut self, v: Guid) {
        self.header_set(904, v.guid() as u32);
        self.header_set(905, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_10(&self) -> Option<Guid> {
        let lower = self.values.get(&904);
        let upper = self.values.get(&905);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_11(&mut self, v: Guid) {
        self.header_set(906, v.guid() as u32);
        self.header_set(907, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_11(&self) -> Option<Guid> {
        let lower = self.values.get(&906);
        let upper = self.values.get(&907);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_12(&mut self, v: Guid) {
        self.header_set(908, v.guid() as u32);
        self.header_set(909, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_12(&self) -> Option<Guid> {
        let lower = self.values.get(&908);
        let upper = self.values.get(&909);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_13(&mut self, v: Guid) {
        self.header_set(910, v.guid() as u32);
        self.header_set(911, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_13(&self) -> Option<Guid> {
        let lower = self.values.get(&910);
        let upper = self.values.get(&911);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_14(&mut self, v: Guid) {
        self.header_set(912, v.guid() as u32);
        self.header_set(913, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_14(&self) -> Option<Guid> {
        let lower = self.values.get(&912);
        let upper = self.values.get(&913);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_15(&mut self, v: Guid) {
        self.header_set(914, v.guid() as u32);
        self.header_set(915, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_15(&self) -> Option<Guid> {
        let lower = self.values.get(&914);
        let upper = self.values.get(&915);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_16(&mut self, v: Guid) {
        self.header_set(916, v.guid() as u32);
        self.header_set(917, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_16(&self) -> Option<Guid> {
        let lower = self.values.get(&916);
        let upper = self.values.get(&917);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_17(&mut self, v: Guid) {
        self.header_set(918, v.guid() as u32);
        self.header_set(919, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_17(&self) -> Option<Guid> {
        let lower = self.values.get(&918);
        let upper = self.values.get(&919);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_vanitypet_slot_18(&mut self, v: Guid) {
        self.header_set(920, v.guid() as u32);
        self.header_set(921, (v.guid() >> 32) as u32);
    }

    pub fn player_vanitypet_slot_18(&self) -> Option<Guid> {
        let lower = self.values.get(&920);
        let upper = self.values.get(&921);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_farsight(&mut self, v: Guid) {
        self.header_set(922, v.guid() as u32);
        self.header_set(923, (v.guid() >> 32) as u32);
    }

    pub fn player_farsight(&self) -> Option<Guid> {
        let lower = self.values.get(&922);
        let upper = self.values.get(&923);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_known_titles(&mut self, v: Guid) {
        self.header_set(924, v.guid() as u32);
        self.header_set(925, (v.guid() >> 32) as u32);
    }

    pub fn player_known_titles(&self) -> Option<Guid> {
        let lower = self.values.get(&924);
        let upper = self.values.get(&925);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
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

