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
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
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

    pub fn set_item_enchantment(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.header_set(44, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_item_text_id(mut self, v: i32) -> Self {
        self.header_set(45, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateContainerBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
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

    pub fn set_item_enchantment(mut self, v: i32) -> Self {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_property_seed(mut self, v: i32) -> Self {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_random_properties_id(mut self, v: i32) -> Self {
        self.header_set(44, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_item_text_id(mut self, v: i32) -> Self {
        self.header_set(45, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_durability(mut self, v: i32) -> Self {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_maxdurability(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_container_num_slots(mut self, v: i32) -> Self {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_container_slot_1(mut self, v: Guid) -> Self {
        self.header_set(50, v.guid() as u32);
        self.header_set(51, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateUnitBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
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

    pub fn set_unit_aura(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_auraflags(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(95, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_auralevels(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(101, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_auraapplications(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(113, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.header_set(132, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.header_set(133, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.header_set(134, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.header_set(135, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(136, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(137, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(138, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.header_set(139, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.header_set(140, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.header_set(141, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.header_set(142, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.header_set(143, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.header_set(144, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.header_set(148, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_training_points(mut self, a: u16, b: u16) -> Self {
        self.header_set(149, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_normal_resistance(mut self, v: i32) -> Self {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_holy_resistance(mut self, v: i32) -> Self {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_fire_resistance(mut self, v: i32) -> Self {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_nature_resistance(mut self, v: i32) -> Self {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_frost_resistance(mut self, v: i32) -> Self {
        self.header_set(159, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_shadow_resistance(mut self, v: i32) -> Self {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_arcane_resistance(mut self, v: i32) -> Self {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_2(mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) -> Self {
        self.header_set(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(166, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(169, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(170, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdatePlayerBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
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

    pub fn set_unit_aura(mut self, v: i32) -> Self {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_auraflags(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(95, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_auralevels(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(101, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_auraapplications(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(113, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_aurastate(mut self, v: i32) -> Self {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_baseattacktime(mut self, v: i32) -> Self {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_rangedattacktime(mut self, v: i32) -> Self {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_boundingradius(mut self, v: f32) -> Self {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_combatreach(mut self, v: f32) -> Self {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_displayid(mut self, v: i32) -> Self {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_nativedisplayid(mut self, v: i32) -> Self {
        self.header_set(132, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mountdisplayid(mut self, v: i32) -> Self {
        self.header_set(133, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mindamage(mut self, v: f32) -> Self {
        self.header_set(134, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxdamage(mut self, v: f32) -> Self {
        self.header_set(135, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(136, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxoffhanddamage(mut self, v: f32) -> Self {
        self.header_set(137, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(138, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_petnumber(mut self, v: i32) -> Self {
        self.header_set(139, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_pet_name_timestamp(mut self, v: i32) -> Self {
        self.header_set(140, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petexperience(mut self, v: i32) -> Self {
        self.header_set(141, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_petnextlevelexp(mut self, v: i32) -> Self {
        self.header_set(142, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_dynamic_flags(mut self, v: i32) -> Self {
        self.header_set(143, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_channel_spell(mut self, v: i32) -> Self {
        self.header_set(144, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_mod_cast_speed(mut self, v: f32) -> Self {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_created_by_spell(mut self, v: i32) -> Self {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_flags(mut self, v: i32) -> Self {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_npc_emotestate(mut self, v: i32) -> Self {
        self.header_set(148, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_training_points(mut self, a: u16, b: u16) -> Self {
        self.header_set(149, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_strength(mut self, v: i32) -> Self {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_agility(mut self, v: i32) -> Self {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_stamina(mut self, v: i32) -> Self {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_intellect(mut self, v: i32) -> Self {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_spirit(mut self, v: i32) -> Self {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_normal_resistance(mut self, v: i32) -> Self {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_holy_resistance(mut self, v: i32) -> Self {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_fire_resistance(mut self, v: i32) -> Self {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_nature_resistance(mut self, v: i32) -> Self {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_frost_resistance(mut self, v: i32) -> Self {
        self.header_set(159, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_shadow_resistance(mut self, v: i32) -> Self {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_arcane_resistance(mut self, v: i32) -> Self {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_mana(mut self, v: i32) -> Self {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_base_health(mut self, v: i32) -> Self {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_bytes_2(mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) -> Self {
        self.header_set(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
        self
    }

    pub fn set_unit_attack_power(mut self, v: i32) -> Self {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(166, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power(mut self, v: i32) -> Self {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_ranged_attack_power_mods(mut self, a: u16, b: u16) -> Self {
        self.header_set(169, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_unit_ranged_attack_power_multiplier(mut self, v: f32) -> Self {
        self.header_set(170, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_minrangeddamage(mut self, v: f32) -> Self {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_maxrangeddamage(mut self, v: f32) -> Self {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_modifier(mut self, v: i32) -> Self {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_power_cost_multiplier(mut self, v: f32) -> Self {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_duel_arbiter(mut self, v: Guid) -> Self {
        self.header_set(188, v.guid() as u32);
        self.header_set(189, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_flags(mut self, v: i32) -> Self {
        self.header_set(190, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_guildid(mut self, v: i32) -> Self {
        self.header_set(191, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_guildrank(mut self, v: i32) -> Self {
        self.header_set(192, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_features(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(193, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(194, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_bytes_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(195, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_duel_team(mut self, v: i32) -> Self {
        self.header_set(196, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_guild_timestamp(mut self, v: i32) -> Self {
        self.header_set(197, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_1_1(mut self, v: i32) -> Self {
        self.header_set(198, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_1_2(mut self, v: i32) -> Self {
        self.header_set(199, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_2_1(mut self, v: i32) -> Self {
        self.header_set(201, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_2_2(mut self, v: i32) -> Self {
        self.header_set(202, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_3_1(mut self, v: i32) -> Self {
        self.header_set(204, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_3_2(mut self, v: i32) -> Self {
        self.header_set(205, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_4_1(mut self, v: i32) -> Self {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_4_2(mut self, v: i32) -> Self {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_5_1(mut self, v: i32) -> Self {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_5_2(mut self, v: i32) -> Self {
        self.header_set(211, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_6_1(mut self, v: i32) -> Self {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_6_2(mut self, v: i32) -> Self {
        self.header_set(214, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_7_1(mut self, v: i32) -> Self {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_7_2(mut self, v: i32) -> Self {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_8_1(mut self, v: i32) -> Self {
        self.header_set(219, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_8_2(mut self, v: i32) -> Self {
        self.header_set(220, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_9_1(mut self, v: i32) -> Self {
        self.header_set(222, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_9_2(mut self, v: i32) -> Self {
        self.header_set(223, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_10_1(mut self, v: i32) -> Self {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_10_2(mut self, v: i32) -> Self {
        self.header_set(226, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_11_1(mut self, v: i32) -> Self {
        self.header_set(228, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_11_2(mut self, v: i32) -> Self {
        self.header_set(229, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_12_1(mut self, v: i32) -> Self {
        self.header_set(231, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_12_2(mut self, v: i32) -> Self {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_13_1(mut self, v: i32) -> Self {
        self.header_set(234, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_13_2(mut self, v: i32) -> Self {
        self.header_set(235, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_14_1(mut self, v: i32) -> Self {
        self.header_set(237, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_14_2(mut self, v: i32) -> Self {
        self.header_set(238, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_15_1(mut self, v: i32) -> Self {
        self.header_set(240, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_15_2(mut self, v: i32) -> Self {
        self.header_set(241, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_16_1(mut self, v: i32) -> Self {
        self.header_set(243, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_16_2(mut self, v: i32) -> Self {
        self.header_set(244, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_17_1(mut self, v: i32) -> Self {
        self.header_set(246, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_17_2(mut self, v: i32) -> Self {
        self.header_set(247, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_18_1(mut self, v: i32) -> Self {
        self.header_set(249, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_18_2(mut self, v: i32) -> Self {
        self.header_set(250, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_19_1(mut self, v: i32) -> Self {
        self.header_set(252, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_19_2(mut self, v: i32) -> Self {
        self.header_set(253, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_20_1(mut self, v: i32) -> Self {
        self.header_set(255, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_quest_log_20_2(mut self, v: i32) -> Self {
        self.header_set(256, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_1_creator(mut self, v: Guid) -> Self {
        self.header_set(258, v.guid() as u32);
        self.header_set(259, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_1_0(mut self, v: i32) -> Self {
        self.header_set(260, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_1_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(268, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_2_creator(mut self, v: Guid) -> Self {
        self.header_set(270, v.guid() as u32);
        self.header_set(271, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_2_0(mut self, v: i32) -> Self {
        self.header_set(272, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_2_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(280, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_3_creator(mut self, v: Guid) -> Self {
        self.header_set(282, v.guid() as u32);
        self.header_set(283, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_3_0(mut self, v: i32) -> Self {
        self.header_set(284, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_3_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(292, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_4_creator(mut self, v: Guid) -> Self {
        self.header_set(294, v.guid() as u32);
        self.header_set(295, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_4_0(mut self, v: i32) -> Self {
        self.header_set(296, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_4_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(304, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_5_creator(mut self, v: Guid) -> Self {
        self.header_set(306, v.guid() as u32);
        self.header_set(307, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_5_0(mut self, v: i32) -> Self {
        self.header_set(308, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_5_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(316, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_6_creator(mut self, v: Guid) -> Self {
        self.header_set(318, v.guid() as u32);
        self.header_set(319, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_6_0(mut self, v: i32) -> Self {
        self.header_set(320, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_6_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(328, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_7_creator(mut self, v: Guid) -> Self {
        self.header_set(330, v.guid() as u32);
        self.header_set(331, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_7_0(mut self, v: i32) -> Self {
        self.header_set(332, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_7_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(340, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_8_creator(mut self, v: Guid) -> Self {
        self.header_set(342, v.guid() as u32);
        self.header_set(343, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_8_0(mut self, v: i32) -> Self {
        self.header_set(344, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_8_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(352, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_9_creator(mut self, v: Guid) -> Self {
        self.header_set(354, v.guid() as u32);
        self.header_set(355, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_9_0(mut self, v: i32) -> Self {
        self.header_set(356, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_9_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(364, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_10_creator(mut self, v: Guid) -> Self {
        self.header_set(366, v.guid() as u32);
        self.header_set(367, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_10_0(mut self, v: i32) -> Self {
        self.header_set(368, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_10_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(376, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_11_creator(mut self, v: Guid) -> Self {
        self.header_set(378, v.guid() as u32);
        self.header_set(379, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_11_0(mut self, v: i32) -> Self {
        self.header_set(380, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_11_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(388, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_12_creator(mut self, v: Guid) -> Self {
        self.header_set(390, v.guid() as u32);
        self.header_set(391, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_12_0(mut self, v: i32) -> Self {
        self.header_set(392, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_12_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(400, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_13_creator(mut self, v: Guid) -> Self {
        self.header_set(402, v.guid() as u32);
        self.header_set(403, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_13_0(mut self, v: i32) -> Self {
        self.header_set(404, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_13_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(412, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_14_creator(mut self, v: Guid) -> Self {
        self.header_set(414, v.guid() as u32);
        self.header_set(415, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_14_0(mut self, v: i32) -> Self {
        self.header_set(416, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_14_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(424, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_15_creator(mut self, v: Guid) -> Self {
        self.header_set(426, v.guid() as u32);
        self.header_set(427, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_15_0(mut self, v: i32) -> Self {
        self.header_set(428, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_15_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(436, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_16_creator(mut self, v: Guid) -> Self {
        self.header_set(438, v.guid() as u32);
        self.header_set(439, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_16_0(mut self, v: i32) -> Self {
        self.header_set(440, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_16_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(448, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_17_creator(mut self, v: Guid) -> Self {
        self.header_set(450, v.guid() as u32);
        self.header_set(451, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_17_0(mut self, v: i32) -> Self {
        self.header_set(452, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_17_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(460, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_18_creator(mut self, v: Guid) -> Self {
        self.header_set(462, v.guid() as u32);
        self.header_set(463, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_18_0(mut self, v: i32) -> Self {
        self.header_set(464, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_18_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(472, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_visible_item_19_creator(mut self, v: Guid) -> Self {
        self.header_set(474, v.guid() as u32);
        self.header_set(475, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_visible_item_19_0(mut self, v: i32) -> Self {
        self.header_set(476, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_visible_item_19_properties(mut self, a: u16, b: u16) -> Self {
        self.header_set(484, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_field_inv(mut self, item_slot: crate::vanilla::ItemSlot, item: Guid) -> Self {
        let offset = 486 + item_slot as u16 * 2;
        self.header_set(offset, item.guid() as u32);
        self.header_set(offset + 1, (item.guid() >> 32) as u32);
        self
    }

    pub fn set_player_farsight(mut self, v: Guid) -> Self {
        self.header_set(712, v.guid() as u32);
        self.header_set(713, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_field_combo_target(mut self, v: Guid) -> Self {
        self.header_set(714, v.guid() as u32);
        self.header_set(715, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_xp(mut self, v: i32) -> Self {
        self.header_set(716, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_next_level_xp(mut self, v: i32) -> Self {
        self.header_set(717, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_skill_info(mut self, skill_info: crate::vanilla::SkillInfo, index: SkillInfoIndex) -> Self {
        for (index, value) in skill_info.mask_values(index) {
            self.header_set(index, value);
        }
        self
    }

    pub fn set_player_character_points1(mut self, v: i32) -> Self {
        self.header_set(1102, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_character_points2(mut self, v: i32) -> Self {
        self.header_set(1103, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_track_creatures(mut self, v: i32) -> Self {
        self.header_set(1104, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_track_resources(mut self, v: i32) -> Self {
        self.header_set(1105, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_block_percentage(mut self, v: f32) -> Self {
        self.header_set(1106, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_dodge_percentage(mut self, v: f32) -> Self {
        self.header_set(1107, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_parry_percentage(mut self, v: f32) -> Self {
        self.header_set(1108, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_crit_percentage(mut self, v: f32) -> Self {
        self.header_set(1109, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_ranged_crit_percentage(mut self, v: f32) -> Self {
        self.header_set(1110, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_explored_zones_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1111, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_rest_state_experience(mut self, v: i32) -> Self {
        self.header_set(1175, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_coinage(mut self, v: i32) -> Self {
        self.header_set(1176, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_posstat0(mut self, v: i32) -> Self {
        self.header_set(1177, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_posstat1(mut self, v: i32) -> Self {
        self.header_set(1178, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_posstat2(mut self, v: i32) -> Self {
        self.header_set(1179, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_posstat3(mut self, v: i32) -> Self {
        self.header_set(1180, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_posstat4(mut self, v: i32) -> Self {
        self.header_set(1181, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_negstat0(mut self, v: i32) -> Self {
        self.header_set(1182, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_negstat1(mut self, v: i32) -> Self {
        self.header_set(1183, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_negstat2(mut self, v: i32) -> Self {
        self.header_set(1184, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_negstat3(mut self, v: i32) -> Self {
        self.header_set(1185, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_negstat4(mut self, v: i32) -> Self {
        self.header_set(1186, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_resistancebuffmodspositive(mut self, v: i32) -> Self {
        self.header_set(1187, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_resistancebuffmodsnegative(mut self, v: i32) -> Self {
        self.header_set(1194, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_mod_damage_done_pos(mut self, v: i32) -> Self {
        self.header_set(1201, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_mod_damage_done_neg(mut self, v: i32) -> Self {
        self.header_set(1208, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_mod_damage_done_pct(mut self, v: i32) -> Self {
        self.header_set(1215, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_bytes(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1222, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_ammo_id(mut self, v: i32) -> Self {
        self.header_set(1223, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_self_res_spell(mut self, v: i32) -> Self {
        self.header_set(1224, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_pvp_medals(mut self, v: i32) -> Self {
        self.header_set(1225, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_buyback_price_1(mut self, v: i32) -> Self {
        self.header_set(1226, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_buyback_timestamp_1(mut self, v: i32) -> Self {
        self.header_set(1238, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_session_kills(mut self, a: u16, b: u16) -> Self {
        self.header_set(1250, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_field_yesterday_kills(mut self, a: u16, b: u16) -> Self {
        self.header_set(1251, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_field_last_week_kills(mut self, a: u16, b: u16) -> Self {
        self.header_set(1252, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_field_this_week_kills(mut self, a: u16, b: u16) -> Self {
        self.header_set(1253, crate::util::u16s_to_u32(a, b));
        self
    }

    pub fn set_player_field_this_week_contribution(mut self, v: i32) -> Self {
        self.header_set(1254, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_lifetime_honorbale_kills(mut self, v: i32) -> Self {
        self.header_set(1255, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_lifetime_dishonorbale_kills(mut self, v: i32) -> Self {
        self.header_set(1256, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_yesterday_contribution(mut self, v: i32) -> Self {
        self.header_set(1257, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_last_week_contribution(mut self, v: i32) -> Self {
        self.header_set(1258, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_last_week_rank(mut self, v: i32) -> Self {
        self.header_set(1259, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_bytes2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1260, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_field_watched_faction_index(mut self, v: i32) -> Self {
        self.header_set(1261, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_field_combat_rating_1(mut self, v: i32) -> Self {
        self.header_set(1262, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

}

impl UpdateGameObjectBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
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

    pub fn set_gameobject_created_by(mut self, v: Guid) -> Self {
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

    pub fn set_object_entry(mut self, v: i32) -> Self {
        self.header_set(3, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_scale_x(mut self, v: f32) -> Self {
        self.header_set(4, u32::from_le_bytes(v.to_le_bytes()));
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

}

impl UpdateCorpseBuilder {
    pub fn set_object_guid(mut self, v: Guid) -> Self {
        self.header_set(0, v.guid() as u32);
        self.header_set(1, (v.guid() >> 32) as u32);
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

    pub fn set_corpse_owner(mut self, v: Guid) -> Self {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_facing(mut self, v: f32) -> Self {
        self.header_set(8, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_pos_x(mut self, v: f32) -> Self {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_pos_y(mut self, v: f32) -> Self {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_pos_z(mut self, v: f32) -> Self {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_display_id(mut self, v: i32) -> Self {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_item(mut self, v: i32) -> Self {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_bytes_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(32, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_bytes_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(33, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_guild(mut self, v: i32) -> Self {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_flags(mut self, v: i32) -> Self {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_dynamic_flags(mut self, v: i32) -> Self {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
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

    pub fn set_item_enchantment(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.values.get(&43).map(|v| *v as i32)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.header_set(44, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.values.get(&44).map(|v| *v as i32)
    }

    pub fn set_item_item_text_id(&mut self, v: i32) {
        self.header_set(45, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_item_text_id(&self) -> Option<i32> {
        self.values.get(&45).map(|v| *v as i32)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
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

    pub fn set_item_enchantment(&mut self, v: i32) {
        self.header_set(22, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_enchantment(&self) -> Option<i32> {
        self.values.get(&22).map(|v| *v as i32)
    }

    pub fn set_item_property_seed(&mut self, v: i32) {
        self.header_set(43, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_property_seed(&self) -> Option<i32> {
        self.values.get(&43).map(|v| *v as i32)
    }

    pub fn set_item_random_properties_id(&mut self, v: i32) {
        self.header_set(44, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_random_properties_id(&self) -> Option<i32> {
        self.values.get(&44).map(|v| *v as i32)
    }

    pub fn set_item_item_text_id(&mut self, v: i32) {
        self.header_set(45, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_item_text_id(&self) -> Option<i32> {
        self.values.get(&45).map(|v| *v as i32)
    }

    pub fn set_item_durability(&mut self, v: i32) {
        self.header_set(46, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_durability(&self) -> Option<i32> {
        self.values.get(&46).map(|v| *v as i32)
    }

    pub fn set_item_maxdurability(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn item_maxdurability(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
    }

    pub fn set_container_num_slots(&mut self, v: i32) {
        self.header_set(48, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn container_num_slots(&self) -> Option<i32> {
        self.values.get(&48).map(|v| *v as i32)
    }

    pub fn set_container_slot_1(&mut self, v: Guid) {
        self.header_set(50, v.guid() as u32);
        self.header_set(51, (v.guid() >> 32) as u32);
    }

    pub fn container_slot_1(&self) -> Option<Guid> {
        let lower = self.values.get(&50);
        let upper = self.values.get(&51);

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

    pub fn set_unit_aura(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_aura(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
    }

    pub fn set_unit_auraflags(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(95, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auraflags(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&95) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_auralevels(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(101, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auralevels(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&101) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_auraapplications(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(113, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auraapplications(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&113) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.values.get(&125).map(|v| *v as i32)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.values.get(&126).map(|v| *v as i32)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.values.get(&128).map(|v| *v as i32)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.values.get(&129).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.values.get(&130).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.values.get(&131).map(|v| *v as i32)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.header_set(132, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.values.get(&132).map(|v| *v as i32)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.header_set(133, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.values.get(&133).map(|v| *v as i32)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.header_set(134, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.values.get(&134).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.header_set(135, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.values.get(&135).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.header_set(136, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.values.get(&136).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.header_set(137, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.values.get(&137).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(138, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&138) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.header_set(139, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.values.get(&139).map(|v| *v as i32)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.header_set(140, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.values.get(&140).map(|v| *v as i32)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.header_set(141, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.values.get(&141).map(|v| *v as i32)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.header_set(142, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.values.get(&142).map(|v| *v as i32)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.header_set(143, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.values.get(&143).map(|v| *v as i32)
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.header_set(144, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.values.get(&144).map(|v| *v as i32)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.values.get(&145).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.values.get(&146).map(|v| *v as i32)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.values.get(&147).map(|v| *v as i32)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.header_set(148, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.values.get(&148).map(|v| *v as i32)
    }

    pub fn set_unit_training_points(&mut self, a: u16, b: u16) {
        self.header_set(149, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_training_points(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&149) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.values.get(&150).map(|v| *v as i32)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.values.get(&151).map(|v| *v as i32)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.values.get(&153).map(|v| *v as i32)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.values.get(&154).map(|v| *v as i32)
    }

    pub fn set_unit_normal_resistance(&mut self, v: i32) {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_normal_resistance(&self) -> Option<i32> {
        self.values.get(&155).map(|v| *v as i32)
    }

    pub fn set_unit_holy_resistance(&mut self, v: i32) {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_holy_resistance(&self) -> Option<i32> {
        self.values.get(&156).map(|v| *v as i32)
    }

    pub fn set_unit_fire_resistance(&mut self, v: i32) {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_fire_resistance(&self) -> Option<i32> {
        self.values.get(&157).map(|v| *v as i32)
    }

    pub fn set_unit_nature_resistance(&mut self, v: i32) {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_nature_resistance(&self) -> Option<i32> {
        self.values.get(&158).map(|v| *v as i32)
    }

    pub fn set_unit_frost_resistance(&mut self, v: i32) {
        self.header_set(159, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_frost_resistance(&self) -> Option<i32> {
        self.values.get(&159).map(|v| *v as i32)
    }

    pub fn set_unit_shadow_resistance(&mut self, v: i32) {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_shadow_resistance(&self) -> Option<i32> {
        self.values.get(&160).map(|v| *v as i32)
    }

    pub fn set_unit_arcane_resistance(&mut self, v: i32) {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_arcane_resistance(&self) -> Option<i32> {
        self.values.get(&161).map(|v| *v as i32)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.values.get(&163).map(|v| *v as i32)
    }

    pub fn set_unit_bytes_2(&mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) {
        self.header_set(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&164) {
            let v = v.to_le_bytes();
            let (facial_hair, unknown, bank_bag_slots, rested_state) = (v[0], v[1], v[2], v[3]);
            Some((facial_hair, unknown, bank_bag_slots, rested_state))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.values.get(&165).map(|v| *v as i32)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(166, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&166) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&167).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(169, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&169) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(170, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&170).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.values.get(&171).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.values.get(&172).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.values.get(&173).map(|v| *v as i32)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.values.get(&180).map(|v| f32::from_le_bytes(v.to_le_bytes()))
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

    pub fn set_unit_aura(&mut self, v: i32) {
        self.header_set(47, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_aura(&self) -> Option<i32> {
        self.values.get(&47).map(|v| *v as i32)
    }

    pub fn set_unit_auraflags(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(95, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auraflags(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&95) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_auralevels(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(101, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auralevels(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&101) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_auraapplications(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(113, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_auraapplications(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&113) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_aurastate(&mut self, v: i32) {
        self.header_set(125, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_aurastate(&self) -> Option<i32> {
        self.values.get(&125).map(|v| *v as i32)
    }

    pub fn set_unit_baseattacktime(&mut self, v: i32) {
        self.header_set(126, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_baseattacktime(&self) -> Option<i32> {
        self.values.get(&126).map(|v| *v as i32)
    }

    pub fn set_unit_rangedattacktime(&mut self, v: i32) {
        self.header_set(128, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_rangedattacktime(&self) -> Option<i32> {
        self.values.get(&128).map(|v| *v as i32)
    }

    pub fn set_unit_boundingradius(&mut self, v: f32) {
        self.header_set(129, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_boundingradius(&self) -> Option<f32> {
        self.values.get(&129).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_combatreach(&mut self, v: f32) {
        self.header_set(130, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_combatreach(&self) -> Option<f32> {
        self.values.get(&130).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_displayid(&mut self, v: i32) {
        self.header_set(131, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_displayid(&self) -> Option<i32> {
        self.values.get(&131).map(|v| *v as i32)
    }

    pub fn set_unit_nativedisplayid(&mut self, v: i32) {
        self.header_set(132, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_nativedisplayid(&self) -> Option<i32> {
        self.values.get(&132).map(|v| *v as i32)
    }

    pub fn set_unit_mountdisplayid(&mut self, v: i32) {
        self.header_set(133, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mountdisplayid(&self) -> Option<i32> {
        self.values.get(&133).map(|v| *v as i32)
    }

    pub fn set_unit_mindamage(&mut self, v: f32) {
        self.header_set(134, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mindamage(&self) -> Option<f32> {
        self.values.get(&134).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxdamage(&mut self, v: f32) {
        self.header_set(135, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxdamage(&self) -> Option<f32> {
        self.values.get(&135).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minoffhanddamage(&mut self, v: f32) {
        self.header_set(136, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minoffhanddamage(&self) -> Option<f32> {
        self.values.get(&136).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxoffhanddamage(&mut self, v: f32) {
        self.header_set(137, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxoffhanddamage(&self) -> Option<f32> {
        self.values.get(&137).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(138, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn unit_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&138) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_unit_petnumber(&mut self, v: i32) {
        self.header_set(139, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnumber(&self) -> Option<i32> {
        self.values.get(&139).map(|v| *v as i32)
    }

    pub fn set_unit_pet_name_timestamp(&mut self, v: i32) {
        self.header_set(140, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_pet_name_timestamp(&self) -> Option<i32> {
        self.values.get(&140).map(|v| *v as i32)
    }

    pub fn set_unit_petexperience(&mut self, v: i32) {
        self.header_set(141, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petexperience(&self) -> Option<i32> {
        self.values.get(&141).map(|v| *v as i32)
    }

    pub fn set_unit_petnextlevelexp(&mut self, v: i32) {
        self.header_set(142, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_petnextlevelexp(&self) -> Option<i32> {
        self.values.get(&142).map(|v| *v as i32)
    }

    pub fn set_unit_dynamic_flags(&mut self, v: i32) {
        self.header_set(143, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_dynamic_flags(&self) -> Option<i32> {
        self.values.get(&143).map(|v| *v as i32)
    }

    pub fn set_unit_channel_spell(&mut self, v: i32) {
        self.header_set(144, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_channel_spell(&self) -> Option<i32> {
        self.values.get(&144).map(|v| *v as i32)
    }

    pub fn set_unit_mod_cast_speed(&mut self, v: f32) {
        self.header_set(145, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_mod_cast_speed(&self) -> Option<f32> {
        self.values.get(&145).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_created_by_spell(&mut self, v: i32) {
        self.header_set(146, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_created_by_spell(&self) -> Option<i32> {
        self.values.get(&146).map(|v| *v as i32)
    }

    pub fn set_unit_npc_flags(&mut self, v: i32) {
        self.header_set(147, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_flags(&self) -> Option<i32> {
        self.values.get(&147).map(|v| *v as i32)
    }

    pub fn set_unit_npc_emotestate(&mut self, v: i32) {
        self.header_set(148, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_npc_emotestate(&self) -> Option<i32> {
        self.values.get(&148).map(|v| *v as i32)
    }

    pub fn set_unit_training_points(&mut self, a: u16, b: u16) {
        self.header_set(149, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_training_points(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&149) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_strength(&mut self, v: i32) {
        self.header_set(150, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_strength(&self) -> Option<i32> {
        self.values.get(&150).map(|v| *v as i32)
    }

    pub fn set_unit_agility(&mut self, v: i32) {
        self.header_set(151, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_agility(&self) -> Option<i32> {
        self.values.get(&151).map(|v| *v as i32)
    }

    pub fn set_unit_stamina(&mut self, v: i32) {
        self.header_set(152, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_stamina(&self) -> Option<i32> {
        self.values.get(&152).map(|v| *v as i32)
    }

    pub fn set_unit_intellect(&mut self, v: i32) {
        self.header_set(153, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_intellect(&self) -> Option<i32> {
        self.values.get(&153).map(|v| *v as i32)
    }

    pub fn set_unit_spirit(&mut self, v: i32) {
        self.header_set(154, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_spirit(&self) -> Option<i32> {
        self.values.get(&154).map(|v| *v as i32)
    }

    pub fn set_unit_normal_resistance(&mut self, v: i32) {
        self.header_set(155, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_normal_resistance(&self) -> Option<i32> {
        self.values.get(&155).map(|v| *v as i32)
    }

    pub fn set_unit_holy_resistance(&mut self, v: i32) {
        self.header_set(156, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_holy_resistance(&self) -> Option<i32> {
        self.values.get(&156).map(|v| *v as i32)
    }

    pub fn set_unit_fire_resistance(&mut self, v: i32) {
        self.header_set(157, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_fire_resistance(&self) -> Option<i32> {
        self.values.get(&157).map(|v| *v as i32)
    }

    pub fn set_unit_nature_resistance(&mut self, v: i32) {
        self.header_set(158, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_nature_resistance(&self) -> Option<i32> {
        self.values.get(&158).map(|v| *v as i32)
    }

    pub fn set_unit_frost_resistance(&mut self, v: i32) {
        self.header_set(159, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_frost_resistance(&self) -> Option<i32> {
        self.values.get(&159).map(|v| *v as i32)
    }

    pub fn set_unit_shadow_resistance(&mut self, v: i32) {
        self.header_set(160, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_shadow_resistance(&self) -> Option<i32> {
        self.values.get(&160).map(|v| *v as i32)
    }

    pub fn set_unit_arcane_resistance(&mut self, v: i32) {
        self.header_set(161, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_arcane_resistance(&self) -> Option<i32> {
        self.values.get(&161).map(|v| *v as i32)
    }

    pub fn set_unit_base_mana(&mut self, v: i32) {
        self.header_set(162, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_mana(&self) -> Option<i32> {
        self.values.get(&162).map(|v| *v as i32)
    }

    pub fn set_unit_base_health(&mut self, v: i32) {
        self.header_set(163, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_base_health(&self) -> Option<i32> {
        self.values.get(&163).map(|v| *v as i32)
    }

    pub fn set_unit_bytes_2(&mut self, facial_hair: u8, unknown: u8, bank_bag_slots: u8, rested_state: u8) {
        self.header_set(164, u32::from_le_bytes([facial_hair, unknown, bank_bag_slots, rested_state]));
    }

    pub fn unit_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&164) {
            let v = v.to_le_bytes();
            let (facial_hair, unknown, bank_bag_slots, rested_state) = (v[0], v[1], v[2], v[3]);
            Some((facial_hair, unknown, bank_bag_slots, rested_state))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power(&mut self, v: i32) {
        self.header_set(165, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power(&self) -> Option<i32> {
        self.values.get(&165).map(|v| *v as i32)
    }

    pub fn set_unit_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(166, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&166) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(167, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&167).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_ranged_attack_power(&mut self, v: i32) {
        self.header_set(168, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power(&self) -> Option<i32> {
        self.values.get(&168).map(|v| *v as i32)
    }

    pub fn set_unit_ranged_attack_power_mods(&mut self, a: u16, b: u16) {
        self.header_set(169, crate::util::u16s_to_u32(a, b));
    }

    pub fn unit_ranged_attack_power_mods(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&169) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_unit_ranged_attack_power_multiplier(&mut self, v: f32) {
        self.header_set(170, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_ranged_attack_power_multiplier(&self) -> Option<f32> {
        self.values.get(&170).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_minrangeddamage(&mut self, v: f32) {
        self.header_set(171, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_minrangeddamage(&self) -> Option<f32> {
        self.values.get(&171).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_maxrangeddamage(&mut self, v: f32) {
        self.header_set(172, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_maxrangeddamage(&self) -> Option<f32> {
        self.values.get(&172).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_unit_power_cost_modifier(&mut self, v: i32) {
        self.header_set(173, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_modifier(&self) -> Option<i32> {
        self.values.get(&173).map(|v| *v as i32)
    }

    pub fn set_unit_power_cost_multiplier(&mut self, v: f32) {
        self.header_set(180, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn unit_power_cost_multiplier(&self) -> Option<f32> {
        self.values.get(&180).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_duel_arbiter(&mut self, v: Guid) {
        self.header_set(188, v.guid() as u32);
        self.header_set(189, (v.guid() >> 32) as u32);
    }

    pub fn player_duel_arbiter(&self) -> Option<Guid> {
        let lower = self.values.get(&188);
        let upper = self.values.get(&189);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_flags(&mut self, v: i32) {
        self.header_set(190, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_flags(&self) -> Option<i32> {
        self.values.get(&190).map(|v| *v as i32)
    }

    pub fn set_player_guildid(&mut self, v: i32) {
        self.header_set(191, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_guildid(&self) -> Option<i32> {
        self.values.get(&191).map(|v| *v as i32)
    }

    pub fn set_player_guildrank(&mut self, v: i32) {
        self.header_set(192, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_guildrank(&self) -> Option<i32> {
        self.values.get(&192).map(|v| *v as i32)
    }

    pub fn set_player_features(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(193, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_features(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&193) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(194, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&194) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_bytes_3(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(195, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_bytes_3(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&195) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_duel_team(&mut self, v: i32) {
        self.header_set(196, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_duel_team(&self) -> Option<i32> {
        self.values.get(&196).map(|v| *v as i32)
    }

    pub fn set_player_guild_timestamp(&mut self, v: i32) {
        self.header_set(197, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_guild_timestamp(&self) -> Option<i32> {
        self.values.get(&197).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_1_1(&mut self, v: i32) {
        self.header_set(198, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_1_1(&self) -> Option<i32> {
        self.values.get(&198).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_1_2(&mut self, v: i32) {
        self.header_set(199, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_1_2(&self) -> Option<i32> {
        self.values.get(&199).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_2_1(&mut self, v: i32) {
        self.header_set(201, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_2_1(&self) -> Option<i32> {
        self.values.get(&201).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_2_2(&mut self, v: i32) {
        self.header_set(202, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_2_2(&self) -> Option<i32> {
        self.values.get(&202).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_3_1(&mut self, v: i32) {
        self.header_set(204, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_3_1(&self) -> Option<i32> {
        self.values.get(&204).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_3_2(&mut self, v: i32) {
        self.header_set(205, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_3_2(&self) -> Option<i32> {
        self.values.get(&205).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_4_1(&mut self, v: i32) {
        self.header_set(207, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_4_1(&self) -> Option<i32> {
        self.values.get(&207).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_4_2(&mut self, v: i32) {
        self.header_set(208, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_4_2(&self) -> Option<i32> {
        self.values.get(&208).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_5_1(&mut self, v: i32) {
        self.header_set(210, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_5_1(&self) -> Option<i32> {
        self.values.get(&210).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_5_2(&mut self, v: i32) {
        self.header_set(211, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_5_2(&self) -> Option<i32> {
        self.values.get(&211).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_6_1(&mut self, v: i32) {
        self.header_set(213, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_6_1(&self) -> Option<i32> {
        self.values.get(&213).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_6_2(&mut self, v: i32) {
        self.header_set(214, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_6_2(&self) -> Option<i32> {
        self.values.get(&214).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_7_1(&mut self, v: i32) {
        self.header_set(216, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_7_1(&self) -> Option<i32> {
        self.values.get(&216).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_7_2(&mut self, v: i32) {
        self.header_set(217, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_7_2(&self) -> Option<i32> {
        self.values.get(&217).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_8_1(&mut self, v: i32) {
        self.header_set(219, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_8_1(&self) -> Option<i32> {
        self.values.get(&219).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_8_2(&mut self, v: i32) {
        self.header_set(220, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_8_2(&self) -> Option<i32> {
        self.values.get(&220).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_9_1(&mut self, v: i32) {
        self.header_set(222, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_9_1(&self) -> Option<i32> {
        self.values.get(&222).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_9_2(&mut self, v: i32) {
        self.header_set(223, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_9_2(&self) -> Option<i32> {
        self.values.get(&223).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_10_1(&mut self, v: i32) {
        self.header_set(225, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_10_1(&self) -> Option<i32> {
        self.values.get(&225).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_10_2(&mut self, v: i32) {
        self.header_set(226, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_10_2(&self) -> Option<i32> {
        self.values.get(&226).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_11_1(&mut self, v: i32) {
        self.header_set(228, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_11_1(&self) -> Option<i32> {
        self.values.get(&228).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_11_2(&mut self, v: i32) {
        self.header_set(229, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_11_2(&self) -> Option<i32> {
        self.values.get(&229).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_12_1(&mut self, v: i32) {
        self.header_set(231, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_12_1(&self) -> Option<i32> {
        self.values.get(&231).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_12_2(&mut self, v: i32) {
        self.header_set(232, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_12_2(&self) -> Option<i32> {
        self.values.get(&232).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_13_1(&mut self, v: i32) {
        self.header_set(234, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_13_1(&self) -> Option<i32> {
        self.values.get(&234).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_13_2(&mut self, v: i32) {
        self.header_set(235, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_13_2(&self) -> Option<i32> {
        self.values.get(&235).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_14_1(&mut self, v: i32) {
        self.header_set(237, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_14_1(&self) -> Option<i32> {
        self.values.get(&237).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_14_2(&mut self, v: i32) {
        self.header_set(238, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_14_2(&self) -> Option<i32> {
        self.values.get(&238).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_15_1(&mut self, v: i32) {
        self.header_set(240, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_15_1(&self) -> Option<i32> {
        self.values.get(&240).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_15_2(&mut self, v: i32) {
        self.header_set(241, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_15_2(&self) -> Option<i32> {
        self.values.get(&241).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_16_1(&mut self, v: i32) {
        self.header_set(243, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_16_1(&self) -> Option<i32> {
        self.values.get(&243).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_16_2(&mut self, v: i32) {
        self.header_set(244, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_16_2(&self) -> Option<i32> {
        self.values.get(&244).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_17_1(&mut self, v: i32) {
        self.header_set(246, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_17_1(&self) -> Option<i32> {
        self.values.get(&246).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_17_2(&mut self, v: i32) {
        self.header_set(247, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_17_2(&self) -> Option<i32> {
        self.values.get(&247).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_18_1(&mut self, v: i32) {
        self.header_set(249, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_18_1(&self) -> Option<i32> {
        self.values.get(&249).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_18_2(&mut self, v: i32) {
        self.header_set(250, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_18_2(&self) -> Option<i32> {
        self.values.get(&250).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_19_1(&mut self, v: i32) {
        self.header_set(252, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_19_1(&self) -> Option<i32> {
        self.values.get(&252).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_19_2(&mut self, v: i32) {
        self.header_set(253, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_19_2(&self) -> Option<i32> {
        self.values.get(&253).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_20_1(&mut self, v: i32) {
        self.header_set(255, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_20_1(&self) -> Option<i32> {
        self.values.get(&255).map(|v| *v as i32)
    }

    pub fn set_player_quest_log_20_2(&mut self, v: i32) {
        self.header_set(256, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_quest_log_20_2(&self) -> Option<i32> {
        self.values.get(&256).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_1_creator(&mut self, v: Guid) {
        self.header_set(258, v.guid() as u32);
        self.header_set(259, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_1_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&258);
        let upper = self.values.get(&259);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_1_0(&mut self, v: i32) {
        self.header_set(260, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_1_0(&self) -> Option<i32> {
        self.values.get(&260).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_1_properties(&mut self, a: u16, b: u16) {
        self.header_set(268, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_1_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&268) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_2_creator(&mut self, v: Guid) {
        self.header_set(270, v.guid() as u32);
        self.header_set(271, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_2_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&270);
        let upper = self.values.get(&271);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_2_0(&mut self, v: i32) {
        self.header_set(272, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_2_0(&self) -> Option<i32> {
        self.values.get(&272).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_2_properties(&mut self, a: u16, b: u16) {
        self.header_set(280, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_2_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&280) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_3_creator(&mut self, v: Guid) {
        self.header_set(282, v.guid() as u32);
        self.header_set(283, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_3_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&282);
        let upper = self.values.get(&283);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_3_0(&mut self, v: i32) {
        self.header_set(284, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_3_0(&self) -> Option<i32> {
        self.values.get(&284).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_3_properties(&mut self, a: u16, b: u16) {
        self.header_set(292, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_3_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&292) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_4_creator(&mut self, v: Guid) {
        self.header_set(294, v.guid() as u32);
        self.header_set(295, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_4_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&294);
        let upper = self.values.get(&295);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_4_0(&mut self, v: i32) {
        self.header_set(296, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_4_0(&self) -> Option<i32> {
        self.values.get(&296).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_4_properties(&mut self, a: u16, b: u16) {
        self.header_set(304, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_4_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&304) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_5_creator(&mut self, v: Guid) {
        self.header_set(306, v.guid() as u32);
        self.header_set(307, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_5_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&306);
        let upper = self.values.get(&307);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_5_0(&mut self, v: i32) {
        self.header_set(308, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_5_0(&self) -> Option<i32> {
        self.values.get(&308).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_5_properties(&mut self, a: u16, b: u16) {
        self.header_set(316, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_5_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&316) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_6_creator(&mut self, v: Guid) {
        self.header_set(318, v.guid() as u32);
        self.header_set(319, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_6_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&318);
        let upper = self.values.get(&319);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_6_0(&mut self, v: i32) {
        self.header_set(320, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_6_0(&self) -> Option<i32> {
        self.values.get(&320).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_6_properties(&mut self, a: u16, b: u16) {
        self.header_set(328, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_6_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&328) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_7_creator(&mut self, v: Guid) {
        self.header_set(330, v.guid() as u32);
        self.header_set(331, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_7_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&330);
        let upper = self.values.get(&331);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_7_0(&mut self, v: i32) {
        self.header_set(332, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_7_0(&self) -> Option<i32> {
        self.values.get(&332).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_7_properties(&mut self, a: u16, b: u16) {
        self.header_set(340, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_7_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&340) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_8_creator(&mut self, v: Guid) {
        self.header_set(342, v.guid() as u32);
        self.header_set(343, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_8_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&342);
        let upper = self.values.get(&343);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_8_0(&mut self, v: i32) {
        self.header_set(344, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_8_0(&self) -> Option<i32> {
        self.values.get(&344).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_8_properties(&mut self, a: u16, b: u16) {
        self.header_set(352, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_8_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&352) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_9_creator(&mut self, v: Guid) {
        self.header_set(354, v.guid() as u32);
        self.header_set(355, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_9_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&354);
        let upper = self.values.get(&355);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_9_0(&mut self, v: i32) {
        self.header_set(356, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_9_0(&self) -> Option<i32> {
        self.values.get(&356).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_9_properties(&mut self, a: u16, b: u16) {
        self.header_set(364, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_9_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&364) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_10_creator(&mut self, v: Guid) {
        self.header_set(366, v.guid() as u32);
        self.header_set(367, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_10_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&366);
        let upper = self.values.get(&367);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_10_0(&mut self, v: i32) {
        self.header_set(368, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_10_0(&self) -> Option<i32> {
        self.values.get(&368).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_10_properties(&mut self, a: u16, b: u16) {
        self.header_set(376, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_10_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&376) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_11_creator(&mut self, v: Guid) {
        self.header_set(378, v.guid() as u32);
        self.header_set(379, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_11_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&378);
        let upper = self.values.get(&379);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_11_0(&mut self, v: i32) {
        self.header_set(380, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_11_0(&self) -> Option<i32> {
        self.values.get(&380).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_11_properties(&mut self, a: u16, b: u16) {
        self.header_set(388, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_11_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&388) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_12_creator(&mut self, v: Guid) {
        self.header_set(390, v.guid() as u32);
        self.header_set(391, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_12_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&390);
        let upper = self.values.get(&391);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_12_0(&mut self, v: i32) {
        self.header_set(392, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_12_0(&self) -> Option<i32> {
        self.values.get(&392).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_12_properties(&mut self, a: u16, b: u16) {
        self.header_set(400, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_12_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&400) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_13_creator(&mut self, v: Guid) {
        self.header_set(402, v.guid() as u32);
        self.header_set(403, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_13_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&402);
        let upper = self.values.get(&403);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_13_0(&mut self, v: i32) {
        self.header_set(404, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_13_0(&self) -> Option<i32> {
        self.values.get(&404).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_13_properties(&mut self, a: u16, b: u16) {
        self.header_set(412, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_13_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&412) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_14_creator(&mut self, v: Guid) {
        self.header_set(414, v.guid() as u32);
        self.header_set(415, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_14_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&414);
        let upper = self.values.get(&415);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_14_0(&mut self, v: i32) {
        self.header_set(416, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_14_0(&self) -> Option<i32> {
        self.values.get(&416).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_14_properties(&mut self, a: u16, b: u16) {
        self.header_set(424, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_14_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&424) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_15_creator(&mut self, v: Guid) {
        self.header_set(426, v.guid() as u32);
        self.header_set(427, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_15_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&426);
        let upper = self.values.get(&427);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_15_0(&mut self, v: i32) {
        self.header_set(428, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_15_0(&self) -> Option<i32> {
        self.values.get(&428).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_15_properties(&mut self, a: u16, b: u16) {
        self.header_set(436, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_15_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&436) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_16_creator(&mut self, v: Guid) {
        self.header_set(438, v.guid() as u32);
        self.header_set(439, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_16_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&438);
        let upper = self.values.get(&439);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_16_0(&mut self, v: i32) {
        self.header_set(440, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_16_0(&self) -> Option<i32> {
        self.values.get(&440).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_16_properties(&mut self, a: u16, b: u16) {
        self.header_set(448, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_16_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&448) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_17_creator(&mut self, v: Guid) {
        self.header_set(450, v.guid() as u32);
        self.header_set(451, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_17_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&450);
        let upper = self.values.get(&451);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_17_0(&mut self, v: i32) {
        self.header_set(452, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_17_0(&self) -> Option<i32> {
        self.values.get(&452).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_17_properties(&mut self, a: u16, b: u16) {
        self.header_set(460, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_17_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&460) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_18_creator(&mut self, v: Guid) {
        self.header_set(462, v.guid() as u32);
        self.header_set(463, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_18_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&462);
        let upper = self.values.get(&463);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_18_0(&mut self, v: i32) {
        self.header_set(464, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_18_0(&self) -> Option<i32> {
        self.values.get(&464).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_18_properties(&mut self, a: u16, b: u16) {
        self.header_set(472, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_18_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&472) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_visible_item_19_creator(&mut self, v: Guid) {
        self.header_set(474, v.guid() as u32);
        self.header_set(475, (v.guid() >> 32) as u32);
    }

    pub fn player_visible_item_19_creator(&self) -> Option<Guid> {
        let lower = self.values.get(&474);
        let upper = self.values.get(&475);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_visible_item_19_0(&mut self, v: i32) {
        self.header_set(476, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_visible_item_19_0(&self) -> Option<i32> {
        self.values.get(&476).map(|v| *v as i32)
    }

    pub fn set_player_visible_item_19_properties(&mut self, a: u16, b: u16) {
        self.header_set(484, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_visible_item_19_properties(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&484) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_field_inv(&mut self, item_slot: crate::vanilla::ItemSlot, item: Guid) {
        let offset = 486 + item_slot as u16 * 2;
        self.header_set(offset, item.guid() as u32);
        self.header_set(offset + 1, (item.guid() >> 32) as u32);
    }

    pub fn player_field_inv(&self, item_slot: crate::vanilla::ItemSlot) -> Option<Guid> {
        let offset = 486 + item_slot.as_int() as u16 * 2;
        let lower = self.values.get(&offset);
        let upper = self.values.get(&(offset + 1));

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_farsight(&mut self, v: Guid) {
        self.header_set(712, v.guid() as u32);
        self.header_set(713, (v.guid() >> 32) as u32);
    }

    pub fn player_farsight(&self) -> Option<Guid> {
        let lower = self.values.get(&712);
        let upper = self.values.get(&713);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_field_combo_target(&mut self, v: Guid) {
        self.header_set(714, v.guid() as u32);
        self.header_set(715, (v.guid() >> 32) as u32);
    }

    pub fn player_field_combo_target(&self) -> Option<Guid> {
        let lower = self.values.get(&714);
        let upper = self.values.get(&715);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_player_xp(&mut self, v: i32) {
        self.header_set(716, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_xp(&self) -> Option<i32> {
        self.values.get(&716).map(|v| *v as i32)
    }

    pub fn set_player_next_level_xp(&mut self, v: i32) {
        self.header_set(717, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_next_level_xp(&self) -> Option<i32> {
        self.values.get(&717).map(|v| *v as i32)
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
        self.header_set(1102, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_character_points1(&self) -> Option<i32> {
        self.values.get(&1102).map(|v| *v as i32)
    }

    pub fn set_player_character_points2(&mut self, v: i32) {
        self.header_set(1103, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_character_points2(&self) -> Option<i32> {
        self.values.get(&1103).map(|v| *v as i32)
    }

    pub fn set_player_track_creatures(&mut self, v: i32) {
        self.header_set(1104, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_track_creatures(&self) -> Option<i32> {
        self.values.get(&1104).map(|v| *v as i32)
    }

    pub fn set_player_track_resources(&mut self, v: i32) {
        self.header_set(1105, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_track_resources(&self) -> Option<i32> {
        self.values.get(&1105).map(|v| *v as i32)
    }

    pub fn set_player_block_percentage(&mut self, v: f32) {
        self.header_set(1106, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_block_percentage(&self) -> Option<f32> {
        self.values.get(&1106).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_dodge_percentage(&mut self, v: f32) {
        self.header_set(1107, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_dodge_percentage(&self) -> Option<f32> {
        self.values.get(&1107).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_parry_percentage(&mut self, v: f32) {
        self.header_set(1108, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_parry_percentage(&self) -> Option<f32> {
        self.values.get(&1108).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_crit_percentage(&mut self, v: f32) {
        self.header_set(1109, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_crit_percentage(&self) -> Option<f32> {
        self.values.get(&1109).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_ranged_crit_percentage(&mut self, v: f32) {
        self.header_set(1110, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_ranged_crit_percentage(&self) -> Option<f32> {
        self.values.get(&1110).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_player_explored_zones_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1111, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_explored_zones_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1111) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_rest_state_experience(&mut self, v: i32) {
        self.header_set(1175, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_rest_state_experience(&self) -> Option<i32> {
        self.values.get(&1175).map(|v| *v as i32)
    }

    pub fn set_player_field_coinage(&mut self, v: i32) {
        self.header_set(1176, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_coinage(&self) -> Option<i32> {
        self.values.get(&1176).map(|v| *v as i32)
    }

    pub fn set_player_field_posstat0(&mut self, v: i32) {
        self.header_set(1177, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_posstat0(&self) -> Option<i32> {
        self.values.get(&1177).map(|v| *v as i32)
    }

    pub fn set_player_field_posstat1(&mut self, v: i32) {
        self.header_set(1178, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_posstat1(&self) -> Option<i32> {
        self.values.get(&1178).map(|v| *v as i32)
    }

    pub fn set_player_field_posstat2(&mut self, v: i32) {
        self.header_set(1179, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_posstat2(&self) -> Option<i32> {
        self.values.get(&1179).map(|v| *v as i32)
    }

    pub fn set_player_field_posstat3(&mut self, v: i32) {
        self.header_set(1180, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_posstat3(&self) -> Option<i32> {
        self.values.get(&1180).map(|v| *v as i32)
    }

    pub fn set_player_field_posstat4(&mut self, v: i32) {
        self.header_set(1181, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_posstat4(&self) -> Option<i32> {
        self.values.get(&1181).map(|v| *v as i32)
    }

    pub fn set_player_field_negstat0(&mut self, v: i32) {
        self.header_set(1182, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_negstat0(&self) -> Option<i32> {
        self.values.get(&1182).map(|v| *v as i32)
    }

    pub fn set_player_field_negstat1(&mut self, v: i32) {
        self.header_set(1183, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_negstat1(&self) -> Option<i32> {
        self.values.get(&1183).map(|v| *v as i32)
    }

    pub fn set_player_field_negstat2(&mut self, v: i32) {
        self.header_set(1184, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_negstat2(&self) -> Option<i32> {
        self.values.get(&1184).map(|v| *v as i32)
    }

    pub fn set_player_field_negstat3(&mut self, v: i32) {
        self.header_set(1185, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_negstat3(&self) -> Option<i32> {
        self.values.get(&1185).map(|v| *v as i32)
    }

    pub fn set_player_field_negstat4(&mut self, v: i32) {
        self.header_set(1186, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_negstat4(&self) -> Option<i32> {
        self.values.get(&1186).map(|v| *v as i32)
    }

    pub fn set_player_field_resistancebuffmodspositive(&mut self, v: i32) {
        self.header_set(1187, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_resistancebuffmodspositive(&self) -> Option<i32> {
        self.values.get(&1187).map(|v| *v as i32)
    }

    pub fn set_player_field_resistancebuffmodsnegative(&mut self, v: i32) {
        self.header_set(1194, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_resistancebuffmodsnegative(&self) -> Option<i32> {
        self.values.get(&1194).map(|v| *v as i32)
    }

    pub fn set_player_field_mod_damage_done_pos(&mut self, v: i32) {
        self.header_set(1201, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_mod_damage_done_pos(&self) -> Option<i32> {
        self.values.get(&1201).map(|v| *v as i32)
    }

    pub fn set_player_field_mod_damage_done_neg(&mut self, v: i32) {
        self.header_set(1208, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_mod_damage_done_neg(&self) -> Option<i32> {
        self.values.get(&1208).map(|v| *v as i32)
    }

    pub fn set_player_field_mod_damage_done_pct(&mut self, v: i32) {
        self.header_set(1215, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_mod_damage_done_pct(&self) -> Option<i32> {
        self.values.get(&1215).map(|v| *v as i32)
    }

    pub fn set_player_field_bytes(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1222, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_field_bytes(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1222) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_ammo_id(&mut self, v: i32) {
        self.header_set(1223, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_ammo_id(&self) -> Option<i32> {
        self.values.get(&1223).map(|v| *v as i32)
    }

    pub fn set_player_self_res_spell(&mut self, v: i32) {
        self.header_set(1224, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_self_res_spell(&self) -> Option<i32> {
        self.values.get(&1224).map(|v| *v as i32)
    }

    pub fn set_player_field_pvp_medals(&mut self, v: i32) {
        self.header_set(1225, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_pvp_medals(&self) -> Option<i32> {
        self.values.get(&1225).map(|v| *v as i32)
    }

    pub fn set_player_field_buyback_price_1(&mut self, v: i32) {
        self.header_set(1226, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_buyback_price_1(&self) -> Option<i32> {
        self.values.get(&1226).map(|v| *v as i32)
    }

    pub fn set_player_field_buyback_timestamp_1(&mut self, v: i32) {
        self.header_set(1238, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_buyback_timestamp_1(&self) -> Option<i32> {
        self.values.get(&1238).map(|v| *v as i32)
    }

    pub fn set_player_field_session_kills(&mut self, a: u16, b: u16) {
        self.header_set(1250, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_field_session_kills(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1250) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_field_yesterday_kills(&mut self, a: u16, b: u16) {
        self.header_set(1251, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_field_yesterday_kills(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1251) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_field_last_week_kills(&mut self, a: u16, b: u16) {
        self.header_set(1252, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_field_last_week_kills(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1252) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_field_this_week_kills(&mut self, a: u16, b: u16) {
        self.header_set(1253, crate::util::u16s_to_u32(a, b));
    }

    pub fn player_field_this_week_kills(&self) -> Option<(u16, u16)> {
        if let Some(v) = self.values.get(&1253) {
            let v = v.to_le_bytes();
            let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));
            Some((a, b))
        } else {
            None
        }
    }

    pub fn set_player_field_this_week_contribution(&mut self, v: i32) {
        self.header_set(1254, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_this_week_contribution(&self) -> Option<i32> {
        self.values.get(&1254).map(|v| *v as i32)
    }

    pub fn set_player_field_lifetime_honorbale_kills(&mut self, v: i32) {
        self.header_set(1255, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_lifetime_honorbale_kills(&self) -> Option<i32> {
        self.values.get(&1255).map(|v| *v as i32)
    }

    pub fn set_player_field_lifetime_dishonorbale_kills(&mut self, v: i32) {
        self.header_set(1256, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_lifetime_dishonorbale_kills(&self) -> Option<i32> {
        self.values.get(&1256).map(|v| *v as i32)
    }

    pub fn set_player_field_yesterday_contribution(&mut self, v: i32) {
        self.header_set(1257, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_yesterday_contribution(&self) -> Option<i32> {
        self.values.get(&1257).map(|v| *v as i32)
    }

    pub fn set_player_field_last_week_contribution(&mut self, v: i32) {
        self.header_set(1258, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_last_week_contribution(&self) -> Option<i32> {
        self.values.get(&1258).map(|v| *v as i32)
    }

    pub fn set_player_field_last_week_rank(&mut self, v: i32) {
        self.header_set(1259, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_last_week_rank(&self) -> Option<i32> {
        self.values.get(&1259).map(|v| *v as i32)
    }

    pub fn set_player_field_bytes2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(1260, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn player_field_bytes2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&1260) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_player_field_watched_faction_index(&mut self, v: i32) {
        self.header_set(1261, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_watched_faction_index(&self) -> Option<i32> {
        self.values.get(&1261).map(|v| *v as i32)
    }

    pub fn set_player_field_combat_rating_1(&mut self, v: i32) {
        self.header_set(1262, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn player_field_combat_rating_1(&self) -> Option<i32> {
        self.values.get(&1262).map(|v| *v as i32)
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

    pub fn set_gameobject_created_by(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn gameobject_created_by(&self) -> Option<Guid> {
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

    pub fn set_corpse_owner(&mut self, v: Guid) {
        self.header_set(6, v.guid() as u32);
        self.header_set(7, (v.guid() >> 32) as u32);
    }

    pub fn corpse_owner(&self) -> Option<Guid> {
        let lower = self.values.get(&6);
        let upper = self.values.get(&7);

        lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))
    }

    pub fn set_corpse_facing(&mut self, v: f32) {
        self.header_set(8, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_facing(&self) -> Option<f32> {
        self.values.get(&8).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_pos_x(&mut self, v: f32) {
        self.header_set(9, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_pos_x(&self) -> Option<f32> {
        self.values.get(&9).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_pos_y(&mut self, v: f32) {
        self.header_set(10, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_pos_y(&self) -> Option<f32> {
        self.values.get(&10).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_pos_z(&mut self, v: f32) {
        self.header_set(11, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_pos_z(&self) -> Option<f32> {
        self.values.get(&11).map(|v| f32::from_le_bytes(v.to_le_bytes()))
    }

    pub fn set_corpse_display_id(&mut self, v: i32) {
        self.header_set(12, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_display_id(&self) -> Option<i32> {
        self.values.get(&12).map(|v| *v as i32)
    }

    pub fn set_corpse_item(&mut self, v: i32) {
        self.header_set(13, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_item(&self) -> Option<i32> {
        self.values.get(&13).map(|v| *v as i32)
    }

    pub fn set_corpse_bytes_1(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(32, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_bytes_1(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&32) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_bytes_2(&mut self, a: u8, b: u8, c: u8, d: u8) {
        self.header_set(33, u32::from_le_bytes([a, b, c, d]));
    }

    pub fn corpse_bytes_2(&self) -> Option<(u8, u8, u8, u8)> {
        if let Some(v) = self.values.get(&33) {
            let v = v.to_le_bytes();
            let (a, b, c, d) = (v[0], v[1], v[2], v[3]);
            Some((a, b, c, d))
        } else {
            None
        }
    }

    pub fn set_corpse_guild(&mut self, v: i32) {
        self.header_set(34, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_guild(&self) -> Option<i32> {
        self.values.get(&34).map(|v| *v as i32)
    }

    pub fn set_corpse_flags(&mut self, v: i32) {
        self.header_set(35, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_flags(&self) -> Option<i32> {
        self.values.get(&35).map(|v| *v as i32)
    }

    pub fn set_corpse_dynamic_flags(&mut self, v: i32) {
        self.header_set(36, u32::from_le_bytes(v.to_le_bytes()));
    }

    pub fn corpse_dynamic_flags(&self) -> Option<i32> {
        self.values.get(&36).map(|v| *v as i32)
    }

}

