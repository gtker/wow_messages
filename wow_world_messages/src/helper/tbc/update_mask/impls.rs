use crate::Guid;
use crate::tbc::{Race};
use crate::tbc::{Class};
use crate::tbc::{Gender};
use crate::tbc::{Power};
use crate::tbc::{UpdateContainer, UpdateCorpse, UpdateDynamicObject, UpdateGameObject, UpdateItem, UpdatePlayer, UpdateUnit};

impl UpdateItem {
    pub fn set_object_FIELD_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_FIELD_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_FIELD_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_FIELD_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_FIELD_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_FIELD_CONTAINED(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_FIELD_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_FIELD_GIFTCREATOR(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_FIELD_STACK_COUNT(mut self, v: i32) -> Self {
        self.header_set(14);
        self.values.insert(14, v as u32);
        self
    }

    pub fn set_item_FIELD_DURATION(mut self, v: i32) -> Self {
        self.header_set(15);
        self.values.insert(15, v as u32);
        self
    }

    pub fn set_item_FIELD_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16);
        self.values.insert(16, v as u32);
        self
    }

    pub fn set_item_FIELD_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21);
        self.values.insert(21, v as u32);
        self
    }

    pub fn set_item_FIELD_ENCHANTMENT_1_1(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn set_item_FIELD_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(55);
        self.values.insert(55, v as u32);
        self
    }

    pub fn set_item_FIELD_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(56);
        self.values.insert(56, v as u32);
        self
    }

    pub fn set_item_FIELD_ITEM_TEXT_ID(mut self, v: i32) -> Self {
        self.header_set(57);
        self.values.insert(57, v as u32);
        self
    }

    pub fn set_item_FIELD_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(58);
        self.values.insert(58, v as u32);
        self
    }

    pub fn set_item_FIELD_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(59);
        self.values.insert(59, v as u32);
        self
    }

    pub fn set_object_FIELD_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateContainer {
    pub fn set_object_FIELD_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_FIELD_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_FIELD_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_FIELD_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_item_FIELD_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_FIELD_CONTAINED(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_FIELD_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_FIELD_GIFTCREATOR(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_item_FIELD_STACK_COUNT(mut self, v: i32) -> Self {
        self.header_set(14);
        self.values.insert(14, v as u32);
        self
    }

    pub fn set_item_FIELD_DURATION(mut self, v: i32) -> Self {
        self.header_set(15);
        self.values.insert(15, v as u32);
        self
    }

    pub fn set_item_FIELD_SPELL_CHARGES(mut self, v: i32) -> Self {
        self.header_set(16);
        self.values.insert(16, v as u32);
        self
    }

    pub fn set_item_FIELD_FLAGS(mut self, v: i32) -> Self {
        self.header_set(21);
        self.values.insert(21, v as u32);
        self
    }

    pub fn set_item_FIELD_ENCHANTMENT_1_1(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn set_item_FIELD_PROPERTY_SEED(mut self, v: i32) -> Self {
        self.header_set(55);
        self.values.insert(55, v as u32);
        self
    }

    pub fn set_item_FIELD_RANDOM_PROPERTIES_ID(mut self, v: i32) -> Self {
        self.header_set(56);
        self.values.insert(56, v as u32);
        self
    }

    pub fn set_item_FIELD_ITEM_TEXT_ID(mut self, v: i32) -> Self {
        self.header_set(57);
        self.values.insert(57, v as u32);
        self
    }

    pub fn set_item_FIELD_DURABILITY(mut self, v: i32) -> Self {
        self.header_set(58);
        self.values.insert(58, v as u32);
        self
    }

    pub fn set_item_FIELD_MAXDURABILITY(mut self, v: i32) -> Self {
        self.header_set(59);
        self.values.insert(59, v as u32);
        self
    }

    pub fn set_container_FIELD_NUM_SLOTS(mut self, v: i32) -> Self {
        self.header_set(60);
        self.values.insert(60, v as u32);
        self
    }

    pub fn set_container_FIELD_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(62);
        self.header_set(63);
        self.values.insert(62, v.guid() as u32);
        self.values.insert(63, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_FIELD_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateUnit {
    pub fn set_object_FIELD_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_FIELD_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_FIELD_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_FIELD_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_CHARM(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_SUMMON(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(14);
        self.header_set(15);
        self.values.insert(14, v.guid() as u32);
        self.values.insert(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_TARGET(mut self, v: Guid) -> Self {
        self.header_set(16);
        self.header_set(17);
        self.values.insert(16, v.guid() as u32);
        self.values.insert(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_PERSUADED(mut self, v: Guid) -> Self {
        self.header_set(18);
        self.header_set(19);
        self.values.insert(18, v.guid() as u32);
        self.values.insert(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20);
        self.header_set(21);
        self.values.insert(20, v.guid() as u32);
        self.values.insert(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_HEALTH(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER1(mut self, v: i32) -> Self {
        self.header_set(23);
        self.values.insert(23, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER2(mut self, v: i32) -> Self {
        self.header_set(24);
        self.values.insert(24, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER3(mut self, v: i32) -> Self {
        self.header_set(25);
        self.values.insert(25, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER4(mut self, v: i32) -> Self {
        self.header_set(26);
        self.values.insert(26, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER5(mut self, v: i32) -> Self {
        self.header_set(27);
        self.values.insert(27, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(28);
        self.values.insert(28, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(29);
        self.values.insert(29, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(30);
        self.values.insert(30, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(31);
        self.values.insert(31, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(32);
        self.values.insert(32, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(33);
        self.values.insert(33, v as u32);
        self
    }

    pub fn set_unit_FIELD_LEVEL(mut self, v: i32) -> Self {
        self.header_set(34);
        self.values.insert(34, v as u32);
        self
    }

    pub fn set_unit_FIELD_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(35);
        self.values.insert(35, v as u32);
        self
    }

    pub fn set_unit_FIELD_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36);
        self.values.insert(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(mut self, v: i32) -> Self {
        self.header_set(37);
        self.values.insert(37, v as u32);
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40);
        self.values.insert(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_FLAGS(mut self, v: i32) -> Self {
        self.header_set(46);
        self.values.insert(46, v as u32);
        self
    }

    pub fn set_unit_FIELD_FLAGS_2(mut self, v: i32) -> Self {
        self.header_set(47);
        self.values.insert(47, v as u32);
        self
    }

    pub fn set_unit_FIELD_AURA(mut self, v: i32) -> Self {
        self.header_set(48);
        self.values.insert(48, v as u32);
        self
    }

    pub fn set_unit_FIELD_AURAFLAGS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(104);
        self.values.insert(104, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_AURALEVELS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(118);
        self.values.insert(118, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_AURAAPPLICATIONS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(132);
        self.values.insert(132, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(146);
        self.values.insert(146, v as u32);
        self
    }

    pub fn set_unit_FIELD_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(147);
        self.values.insert(147, v as u32);
        self
    }

    pub fn set_unit_FIELD_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(149);
        self.values.insert(149, v as u32);
        self
    }

    pub fn set_unit_FIELD_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(150);
        self.values.insert(150, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(151);
        self.values.insert(151, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(152);
        self.values.insert(152, v as u32);
        self
    }

    pub fn set_unit_FIELD_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(153);
        self.values.insert(153, v as u32);
        self
    }

    pub fn set_unit_FIELD_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(154);
        self.values.insert(154, v as u32);
        self
    }

    pub fn set_unit_FIELD_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(155);
        self.values.insert(155, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(156);
        self.values.insert(156, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(157);
        self.values.insert(157, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(158);
        self.values.insert(158, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(159);
        self.values.insert(159, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(160);
        self.values.insert(160, v as u32);
        self
    }

    pub fn set_unit_FIELD_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(161);
        self.values.insert(161, v as u32);
        self
    }

    pub fn set_unit_FIELD_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(162);
        self.values.insert(162, v as u32);
        self
    }

    pub fn set_unit_FIELD_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(163);
        self.values.insert(163, v as u32);
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(164);
        self.values.insert(164, v as u32);
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(165);
        self.values.insert(165, v as u32);
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(166);
        self.values.insert(166, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(167);
        self.values.insert(167, v as u32);
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(168);
        self.values.insert(168, v as u32);
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(169);
        self.values.insert(169, v as u32);
        self
    }

    pub fn set_unit_TRAINING_POINTS(mut self, v: u32) -> Self {
        self.header_set(170);
        self.values.insert(170, v);
        self
    }

    pub fn set_unit_FIELD_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(171);
        self.values.insert(171, v as u32);
        self
    }

    pub fn set_unit_FIELD_AGILITY(mut self, v: i32) -> Self {
        self.header_set(172);
        self.values.insert(172, v as u32);
        self
    }

    pub fn set_unit_FIELD_STAMINA(mut self, v: i32) -> Self {
        self.header_set(173);
        self.values.insert(173, v as u32);
        self
    }

    pub fn set_unit_FIELD_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(174);
        self.values.insert(174, v as u32);
        self
    }

    pub fn set_unit_FIELD_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(175);
        self.values.insert(175, v as u32);
        self
    }

    pub fn set_unit_FIELD_POSSTAT1(mut self, v: i32) -> Self {
        self.header_set(177);
        self.values.insert(177, v as u32);
        self
    }

    pub fn set_unit_FIELD_POSSTAT2(mut self, v: i32) -> Self {
        self.header_set(178);
        self.values.insert(178, v as u32);
        self
    }

    pub fn set_unit_FIELD_POSSTAT3(mut self, v: i32) -> Self {
        self.header_set(179);
        self.values.insert(179, v as u32);
        self
    }

    pub fn set_unit_FIELD_NEGSTAT1(mut self, v: i32) -> Self {
        self.header_set(182);
        self.values.insert(182, v as u32);
        self
    }

    pub fn set_unit_FIELD_NEGSTAT2(mut self, v: i32) -> Self {
        self.header_set(183);
        self.values.insert(183, v as u32);
        self
    }

    pub fn set_unit_FIELD_NEGSTAT3(mut self, v: i32) -> Self {
        self.header_set(184);
        self.values.insert(184, v as u32);
        self
    }

    pub fn set_unit_FIELD_RESISTANCES(mut self, v: i32) -> Self {
        self.header_set(186);
        self.values.insert(186, v as u32);
        self
    }

    pub fn set_unit_FIELD_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(207);
        self.values.insert(207, v as u32);
        self
    }

    pub fn set_unit_FIELD_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(208);
        self.values.insert(208, v as u32);
        self
    }

    pub fn set_unit_FIELD_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(209);
        self.values.insert(209, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(210);
        self.values.insert(210, v as u32);
        self
    }

    pub fn set_unit_FIELD_ATTACK_POWER_MODS(mut self, v: u32) -> Self {
        self.header_set(211);
        self.values.insert(211, v);
        self
    }

    pub fn set_unit_FIELD_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(212);
        self.values.insert(212, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(213);
        self.values.insert(213, v as u32);
        self
    }

    pub fn set_unit_FIELD_RANGED_ATTACK_POWER_MODS(mut self, v: u32) -> Self {
        self.header_set(214);
        self.values.insert(214, v);
        self
    }

    pub fn set_unit_FIELD_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(215);
        self.values.insert(215, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(216);
        self.values.insert(216, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(217);
        self.values.insert(217, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(218);
        self.values.insert(218, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(225);
        self.values.insert(225, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MAXHEALTHMODIFIER(mut self, v: f32) -> Self {
        self.header_set(232);
        self.values.insert(232, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_FIELD_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdatePlayer {
    pub fn set_object_FIELD_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_FIELD_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_FIELD_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_FIELD_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_CHARM(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_SUMMON(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_CHARMEDBY(mut self, v: Guid) -> Self {
        self.header_set(10);
        self.header_set(11);
        self.values.insert(10, v.guid() as u32);
        self.values.insert(11, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_SUMMONEDBY(mut self, v: Guid) -> Self {
        self.header_set(12);
        self.header_set(13);
        self.values.insert(12, v.guid() as u32);
        self.values.insert(13, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_CREATEDBY(mut self, v: Guid) -> Self {
        self.header_set(14);
        self.header_set(15);
        self.values.insert(14, v.guid() as u32);
        self.values.insert(15, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_TARGET(mut self, v: Guid) -> Self {
        self.header_set(16);
        self.header_set(17);
        self.values.insert(16, v.guid() as u32);
        self.values.insert(17, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_PERSUADED(mut self, v: Guid) -> Self {
        self.header_set(18);
        self.header_set(19);
        self.values.insert(18, v.guid() as u32);
        self.values.insert(19, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_CHANNEL_OBJECT(mut self, v: Guid) -> Self {
        self.header_set(20);
        self.header_set(21);
        self.values.insert(20, v.guid() as u32);
        self.values.insert(21, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_unit_FIELD_HEALTH(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER1(mut self, v: i32) -> Self {
        self.header_set(23);
        self.values.insert(23, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER2(mut self, v: i32) -> Self {
        self.header_set(24);
        self.values.insert(24, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER3(mut self, v: i32) -> Self {
        self.header_set(25);
        self.values.insert(25, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER4(mut self, v: i32) -> Self {
        self.header_set(26);
        self.values.insert(26, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER5(mut self, v: i32) -> Self {
        self.header_set(27);
        self.values.insert(27, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXHEALTH(mut self, v: i32) -> Self {
        self.header_set(28);
        self.values.insert(28, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXPOWER1(mut self, v: i32) -> Self {
        self.header_set(29);
        self.values.insert(29, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXPOWER2(mut self, v: i32) -> Self {
        self.header_set(30);
        self.values.insert(30, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXPOWER3(mut self, v: i32) -> Self {
        self.header_set(31);
        self.values.insert(31, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXPOWER4(mut self, v: i32) -> Self {
        self.header_set(32);
        self.values.insert(32, v as u32);
        self
    }

    pub fn set_unit_FIELD_MAXPOWER5(mut self, v: i32) -> Self {
        self.header_set(33);
        self.values.insert(33, v as u32);
        self
    }

    pub fn set_unit_FIELD_LEVEL(mut self, v: i32) -> Self {
        self.header_set(34);
        self.values.insert(34, v as u32);
        self
    }

    pub fn set_unit_FIELD_FACTIONTEMPLATE(mut self, v: i32) -> Self {
        self.header_set(35);
        self.values.insert(35, v as u32);
        self
    }

    pub fn set_unit_FIELD_BYTES_0(mut self, race: Race, class: Class, gender: Gender, power: Power) -> Self {
        self.header_set(36);
        self.values.insert(36, u32::from_le_bytes([race.as_int(), class.as_int(), gender.as_int(), power.as_int()]));
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(mut self, v: i32) -> Self {
        self.header_set(37);
        self.values.insert(37, v as u32);
        self
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(40);
        self.values.insert(40, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_FLAGS(mut self, v: i32) -> Self {
        self.header_set(46);
        self.values.insert(46, v as u32);
        self
    }

    pub fn set_unit_FIELD_FLAGS_2(mut self, v: i32) -> Self {
        self.header_set(47);
        self.values.insert(47, v as u32);
        self
    }

    pub fn set_unit_FIELD_AURA(mut self, v: i32) -> Self {
        self.header_set(48);
        self.values.insert(48, v as u32);
        self
    }

    pub fn set_unit_FIELD_AURAFLAGS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(104);
        self.values.insert(104, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_AURALEVELS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(118);
        self.values.insert(118, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_AURAAPPLICATIONS(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(132);
        self.values.insert(132, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_AURASTATE(mut self, v: i32) -> Self {
        self.header_set(146);
        self.values.insert(146, v as u32);
        self
    }

    pub fn set_unit_FIELD_BASEATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(147);
        self.values.insert(147, v as u32);
        self
    }

    pub fn set_unit_FIELD_RANGEDATTACKTIME(mut self, v: i32) -> Self {
        self.header_set(149);
        self.values.insert(149, v as u32);
        self
    }

    pub fn set_unit_FIELD_BOUNDINGRADIUS(mut self, v: f32) -> Self {
        self.header_set(150);
        self.values.insert(150, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_COMBATREACH(mut self, v: f32) -> Self {
        self.header_set(151);
        self.values.insert(151, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(152);
        self.values.insert(152, v as u32);
        self
    }

    pub fn set_unit_FIELD_NATIVEDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(153);
        self.values.insert(153, v as u32);
        self
    }

    pub fn set_unit_FIELD_MOUNTDISPLAYID(mut self, v: i32) -> Self {
        self.header_set(154);
        self.values.insert(154, v as u32);
        self
    }

    pub fn set_unit_FIELD_MINDAMAGE(mut self, v: f32) -> Self {
        self.header_set(155);
        self.values.insert(155, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MAXDAMAGE(mut self, v: f32) -> Self {
        self.header_set(156);
        self.values.insert(156, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MINOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(157);
        self.values.insert(157, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MAXOFFHANDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(158);
        self.values.insert(158, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(159);
        self.values.insert(159, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_PETNUMBER(mut self, v: i32) -> Self {
        self.header_set(160);
        self.values.insert(160, v as u32);
        self
    }

    pub fn set_unit_FIELD_PET_NAME_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(161);
        self.values.insert(161, v as u32);
        self
    }

    pub fn set_unit_FIELD_PETEXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(162);
        self.values.insert(162, v as u32);
        self
    }

    pub fn set_unit_FIELD_PETNEXTLEVELEXP(mut self, v: i32) -> Self {
        self.header_set(163);
        self.values.insert(163, v as u32);
        self
    }

    pub fn set_unit_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(164);
        self.values.insert(164, v as u32);
        self
    }

    pub fn set_unit_CHANNEL_SPELL(mut self, v: i32) -> Self {
        self.header_set(165);
        self.values.insert(165, v as u32);
        self
    }

    pub fn set_unit_MOD_CAST_SPEED(mut self, v: f32) -> Self {
        self.header_set(166);
        self.values.insert(166, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_CREATED_BY_SPELL(mut self, v: i32) -> Self {
        self.header_set(167);
        self.values.insert(167, v as u32);
        self
    }

    pub fn set_unit_NPC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(168);
        self.values.insert(168, v as u32);
        self
    }

    pub fn set_unit_NPC_EMOTESTATE(mut self, v: i32) -> Self {
        self.header_set(169);
        self.values.insert(169, v as u32);
        self
    }

    pub fn set_unit_TRAINING_POINTS(mut self, v: u32) -> Self {
        self.header_set(170);
        self.values.insert(170, v);
        self
    }

    pub fn set_unit_FIELD_STRENGTH(mut self, v: i32) -> Self {
        self.header_set(171);
        self.values.insert(171, v as u32);
        self
    }

    pub fn set_unit_FIELD_AGILITY(mut self, v: i32) -> Self {
        self.header_set(172);
        self.values.insert(172, v as u32);
        self
    }

    pub fn set_unit_FIELD_STAMINA(mut self, v: i32) -> Self {
        self.header_set(173);
        self.values.insert(173, v as u32);
        self
    }

    pub fn set_unit_FIELD_INTELLECT(mut self, v: i32) -> Self {
        self.header_set(174);
        self.values.insert(174, v as u32);
        self
    }

    pub fn set_unit_FIELD_SPIRIT(mut self, v: i32) -> Self {
        self.header_set(175);
        self.values.insert(175, v as u32);
        self
    }

    pub fn set_player_FIELD_POSSTAT0(mut self, v: i32) -> Self {
        self.header_set(176);
        self.values.insert(176, v as u32);
        self
    }

    pub fn set_unit_FIELD_POSSTAT1(mut self, v: i32) -> Self {
        self.header_set(177);
        self.values.insert(177, v as u32);
        self
    }

    pub fn set_unit_FIELD_POSSTAT2(mut self, v: i32) -> Self {
        self.header_set(178);
        self.values.insert(178, v as u32);
        self
    }

    pub fn set_unit_FIELD_POSSTAT3(mut self, v: i32) -> Self {
        self.header_set(179);
        self.values.insert(179, v as u32);
        self
    }

    pub fn set_player_FIELD_POSSTAT4(mut self, v: i32) -> Self {
        self.header_set(180);
        self.values.insert(180, v as u32);
        self
    }

    pub fn set_player_FIELD_NEGSTAT0(mut self, v: i32) -> Self {
        self.header_set(181);
        self.values.insert(181, v as u32);
        self
    }

    pub fn set_unit_FIELD_NEGSTAT1(mut self, v: i32) -> Self {
        self.header_set(182);
        self.values.insert(182, v as u32);
        self
    }

    pub fn set_unit_FIELD_NEGSTAT2(mut self, v: i32) -> Self {
        self.header_set(183);
        self.values.insert(183, v as u32);
        self
    }

    pub fn set_unit_FIELD_NEGSTAT3(mut self, v: i32) -> Self {
        self.header_set(184);
        self.values.insert(184, v as u32);
        self
    }

    pub fn set_player_FIELD_NEGSTAT4(mut self, v: i32) -> Self {
        self.header_set(185);
        self.values.insert(185, v as u32);
        self
    }

    pub fn set_unit_FIELD_RESISTANCES(mut self, v: i32) -> Self {
        self.header_set(186);
        self.values.insert(186, v as u32);
        self
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSPOSITIVE(mut self, v: i32) -> Self {
        self.header_set(193);
        self.values.insert(193, v as u32);
        self
    }

    pub fn set_player_FIELD_RESISTANCEBUFFMODSNEGATIVE(mut self, v: i32) -> Self {
        self.header_set(200);
        self.values.insert(200, v as u32);
        self
    }

    pub fn set_unit_FIELD_BASE_MANA(mut self, v: i32) -> Self {
        self.header_set(207);
        self.values.insert(207, v as u32);
        self
    }

    pub fn set_unit_FIELD_BASE_HEALTH(mut self, v: i32) -> Self {
        self.header_set(208);
        self.values.insert(208, v as u32);
        self
    }

    pub fn set_unit_FIELD_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(209);
        self.values.insert(209, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_unit_FIELD_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(210);
        self.values.insert(210, v as u32);
        self
    }

    pub fn set_unit_FIELD_ATTACK_POWER_MODS(mut self, v: u32) -> Self {
        self.header_set(211);
        self.values.insert(211, v);
        self
    }

    pub fn set_unit_FIELD_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(212);
        self.values.insert(212, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_RANGED_ATTACK_POWER(mut self, v: i32) -> Self {
        self.header_set(213);
        self.values.insert(213, v as u32);
        self
    }

    pub fn set_unit_FIELD_RANGED_ATTACK_POWER_MODS(mut self, v: u32) -> Self {
        self.header_set(214);
        self.values.insert(214, v);
        self
    }

    pub fn set_unit_FIELD_RANGED_ATTACK_POWER_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(215);
        self.values.insert(215, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MINRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(216);
        self.values.insert(216, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MAXRANGEDDAMAGE(mut self, v: f32) -> Self {
        self.header_set(217);
        self.values.insert(217, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_POWER_COST_MODIFIER(mut self, v: i32) -> Self {
        self.header_set(218);
        self.values.insert(218, v as u32);
        self
    }

    pub fn set_unit_FIELD_POWER_COST_MULTIPLIER(mut self, v: f32) -> Self {
        self.header_set(225);
        self.values.insert(225, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_unit_FIELD_MAXHEALTHMODIFIER(mut self, v: f32) -> Self {
        self.header_set(232);
        self.values.insert(232, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_DUEL_ARBITER(mut self, v: Guid) -> Self {
        self.header_set(234);
        self.header_set(235);
        self.values.insert(234, v.guid() as u32);
        self.values.insert(235, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FLAGS(mut self, v: i32) -> Self {
        self.header_set(236);
        self.values.insert(236, v as u32);
        self
    }

    pub fn set_player_GUILDID(mut self, v: i32) -> Self {
        self.header_set(237);
        self.values.insert(237, v as u32);
        self
    }

    pub fn set_player_GUILDRANK(mut self, v: i32) -> Self {
        self.header_set(238);
        self.values.insert(238, v as u32);
        self
    }

    pub fn set_player_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(239);
        self.values.insert(239, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(240);
        self.values.insert(240, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_BYTES_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(241);
        self.values.insert(241, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_DUEL_TEAM(mut self, v: i32) -> Self {
        self.header_set(242);
        self.values.insert(242, v as u32);
        self
    }

    pub fn set_player_GUILD_TIMESTAMP(mut self, v: i32) -> Self {
        self.header_set(243);
        self.values.insert(243, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_1(mut self, v: i32) -> Self {
        self.header_set(244);
        self.values.insert(244, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_2(mut self, v: i32) -> Self {
        self.header_set(245);
        self.values.insert(245, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_1_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(246);
        self.values.insert(246, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_1_4(mut self, v: i32) -> Self {
        self.header_set(247);
        self.values.insert(247, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_1(mut self, v: i32) -> Self {
        self.header_set(248);
        self.values.insert(248, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_2(mut self, v: i32) -> Self {
        self.header_set(249);
        self.values.insert(249, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_2_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(250);
        self.values.insert(250, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_2_4(mut self, v: i32) -> Self {
        self.header_set(251);
        self.values.insert(251, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_1(mut self, v: i32) -> Self {
        self.header_set(252);
        self.values.insert(252, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_2(mut self, v: i32) -> Self {
        self.header_set(253);
        self.values.insert(253, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_3_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(254);
        self.values.insert(254, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_3_4(mut self, v: i32) -> Self {
        self.header_set(255);
        self.values.insert(255, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_1(mut self, v: i32) -> Self {
        self.header_set(256);
        self.values.insert(256, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_2(mut self, v: i32) -> Self {
        self.header_set(257);
        self.values.insert(257, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_4_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(258);
        self.values.insert(258, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_4_4(mut self, v: i32) -> Self {
        self.header_set(259);
        self.values.insert(259, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_1(mut self, v: i32) -> Self {
        self.header_set(260);
        self.values.insert(260, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_2(mut self, v: i32) -> Self {
        self.header_set(261);
        self.values.insert(261, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_5_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(262);
        self.values.insert(262, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_5_4(mut self, v: i32) -> Self {
        self.header_set(263);
        self.values.insert(263, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_1(mut self, v: i32) -> Self {
        self.header_set(264);
        self.values.insert(264, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_2(mut self, v: i32) -> Self {
        self.header_set(265);
        self.values.insert(265, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_6_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(266);
        self.values.insert(266, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_6_4(mut self, v: i32) -> Self {
        self.header_set(267);
        self.values.insert(267, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_1(mut self, v: i32) -> Self {
        self.header_set(268);
        self.values.insert(268, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_2(mut self, v: i32) -> Self {
        self.header_set(269);
        self.values.insert(269, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_7_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(270);
        self.values.insert(270, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_7_4(mut self, v: i32) -> Self {
        self.header_set(271);
        self.values.insert(271, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_1(mut self, v: i32) -> Self {
        self.header_set(272);
        self.values.insert(272, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_2(mut self, v: i32) -> Self {
        self.header_set(273);
        self.values.insert(273, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_8_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(274);
        self.values.insert(274, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_8_4(mut self, v: i32) -> Self {
        self.header_set(275);
        self.values.insert(275, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_1(mut self, v: i32) -> Self {
        self.header_set(276);
        self.values.insert(276, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_2(mut self, v: i32) -> Self {
        self.header_set(277);
        self.values.insert(277, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_9_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(278);
        self.values.insert(278, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_9_4(mut self, v: i32) -> Self {
        self.header_set(279);
        self.values.insert(279, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_1(mut self, v: i32) -> Self {
        self.header_set(280);
        self.values.insert(280, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_2(mut self, v: i32) -> Self {
        self.header_set(281);
        self.values.insert(281, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_10_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(282);
        self.values.insert(282, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_10_4(mut self, v: i32) -> Self {
        self.header_set(283);
        self.values.insert(283, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_1(mut self, v: i32) -> Self {
        self.header_set(284);
        self.values.insert(284, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_2(mut self, v: i32) -> Self {
        self.header_set(285);
        self.values.insert(285, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_11_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(286);
        self.values.insert(286, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_11_4(mut self, v: i32) -> Self {
        self.header_set(287);
        self.values.insert(287, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_1(mut self, v: i32) -> Self {
        self.header_set(288);
        self.values.insert(288, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_2(mut self, v: i32) -> Self {
        self.header_set(289);
        self.values.insert(289, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_12_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(290);
        self.values.insert(290, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_12_4(mut self, v: i32) -> Self {
        self.header_set(291);
        self.values.insert(291, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_1(mut self, v: i32) -> Self {
        self.header_set(292);
        self.values.insert(292, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_2(mut self, v: i32) -> Self {
        self.header_set(293);
        self.values.insert(293, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_13_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(294);
        self.values.insert(294, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_13_4(mut self, v: i32) -> Self {
        self.header_set(295);
        self.values.insert(295, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_1(mut self, v: i32) -> Self {
        self.header_set(296);
        self.values.insert(296, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_2(mut self, v: i32) -> Self {
        self.header_set(297);
        self.values.insert(297, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_14_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(298);
        self.values.insert(298, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_14_4(mut self, v: i32) -> Self {
        self.header_set(299);
        self.values.insert(299, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_1(mut self, v: i32) -> Self {
        self.header_set(300);
        self.values.insert(300, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_2(mut self, v: i32) -> Self {
        self.header_set(301);
        self.values.insert(301, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_15_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(302);
        self.values.insert(302, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_15_4(mut self, v: i32) -> Self {
        self.header_set(303);
        self.values.insert(303, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_1(mut self, v: i32) -> Self {
        self.header_set(304);
        self.values.insert(304, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_2(mut self, v: i32) -> Self {
        self.header_set(305);
        self.values.insert(305, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_16_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(306);
        self.values.insert(306, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_16_4(mut self, v: i32) -> Self {
        self.header_set(307);
        self.values.insert(307, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_1(mut self, v: i32) -> Self {
        self.header_set(308);
        self.values.insert(308, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_2(mut self, v: i32) -> Self {
        self.header_set(309);
        self.values.insert(309, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_17_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(310);
        self.values.insert(310, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_17_4(mut self, v: i32) -> Self {
        self.header_set(311);
        self.values.insert(311, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_1(mut self, v: i32) -> Self {
        self.header_set(312);
        self.values.insert(312, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_2(mut self, v: i32) -> Self {
        self.header_set(313);
        self.values.insert(313, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_18_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(314);
        self.values.insert(314, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_18_4(mut self, v: i32) -> Self {
        self.header_set(315);
        self.values.insert(315, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_1(mut self, v: i32) -> Self {
        self.header_set(316);
        self.values.insert(316, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_2(mut self, v: i32) -> Self {
        self.header_set(317);
        self.values.insert(317, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_19_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(318);
        self.values.insert(318, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_19_4(mut self, v: i32) -> Self {
        self.header_set(319);
        self.values.insert(319, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_1(mut self, v: i32) -> Self {
        self.header_set(320);
        self.values.insert(320, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_2(mut self, v: i32) -> Self {
        self.header_set(321);
        self.values.insert(321, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_20_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(322);
        self.values.insert(322, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_20_4(mut self, v: i32) -> Self {
        self.header_set(323);
        self.values.insert(323, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_21_1(mut self, v: i32) -> Self {
        self.header_set(324);
        self.values.insert(324, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_21_2(mut self, v: i32) -> Self {
        self.header_set(325);
        self.values.insert(325, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_21_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(326);
        self.values.insert(326, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_21_4(mut self, v: i32) -> Self {
        self.header_set(327);
        self.values.insert(327, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_22_1(mut self, v: i32) -> Self {
        self.header_set(328);
        self.values.insert(328, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_22_2(mut self, v: i32) -> Self {
        self.header_set(329);
        self.values.insert(329, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_22_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(330);
        self.values.insert(330, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_22_4(mut self, v: i32) -> Self {
        self.header_set(331);
        self.values.insert(331, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_23_1(mut self, v: i32) -> Self {
        self.header_set(332);
        self.values.insert(332, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_23_2(mut self, v: i32) -> Self {
        self.header_set(333);
        self.values.insert(333, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_23_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(334);
        self.values.insert(334, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_23_4(mut self, v: i32) -> Self {
        self.header_set(335);
        self.values.insert(335, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_24_1(mut self, v: i32) -> Self {
        self.header_set(336);
        self.values.insert(336, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_24_2(mut self, v: i32) -> Self {
        self.header_set(337);
        self.values.insert(337, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_24_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(338);
        self.values.insert(338, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_24_4(mut self, v: i32) -> Self {
        self.header_set(339);
        self.values.insert(339, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_25_1(mut self, v: i32) -> Self {
        self.header_set(340);
        self.values.insert(340, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_25_2(mut self, v: i32) -> Self {
        self.header_set(341);
        self.values.insert(341, v as u32);
        self
    }

    pub fn set_player_QUEST_LOG_25_3(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(342);
        self.values.insert(342, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_QUEST_LOG_25_4(mut self, v: i32) -> Self {
        self.header_set(343);
        self.values.insert(343, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(344);
        self.header_set(345);
        self.values.insert(344, v.guid() as u32);
        self.values.insert(345, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_0(mut self, v: i32) -> Self {
        self.header_set(346);
        self.values.insert(346, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_1_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(358);
        self.values.insert(358, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(360);
        self.header_set(361);
        self.values.insert(360, v.guid() as u32);
        self.values.insert(361, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_0(mut self, v: i32) -> Self {
        self.header_set(362);
        self.values.insert(362, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_2_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(374);
        self.values.insert(374, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(376);
        self.header_set(377);
        self.values.insert(376, v.guid() as u32);
        self.values.insert(377, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_0(mut self, v: i32) -> Self {
        self.header_set(378);
        self.values.insert(378, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_3_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(390);
        self.values.insert(390, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(392);
        self.header_set(393);
        self.values.insert(392, v.guid() as u32);
        self.values.insert(393, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_0(mut self, v: i32) -> Self {
        self.header_set(394);
        self.values.insert(394, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_4_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(406);
        self.values.insert(406, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(408);
        self.header_set(409);
        self.values.insert(408, v.guid() as u32);
        self.values.insert(409, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_0(mut self, v: i32) -> Self {
        self.header_set(410);
        self.values.insert(410, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_5_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(422);
        self.values.insert(422, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(424);
        self.header_set(425);
        self.values.insert(424, v.guid() as u32);
        self.values.insert(425, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_0(mut self, v: i32) -> Self {
        self.header_set(426);
        self.values.insert(426, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_6_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(438);
        self.values.insert(438, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(440);
        self.header_set(441);
        self.values.insert(440, v.guid() as u32);
        self.values.insert(441, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_0(mut self, v: i32) -> Self {
        self.header_set(442);
        self.values.insert(442, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_7_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(454);
        self.values.insert(454, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(456);
        self.header_set(457);
        self.values.insert(456, v.guid() as u32);
        self.values.insert(457, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_0(mut self, v: i32) -> Self {
        self.header_set(458);
        self.values.insert(458, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_8_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(470);
        self.values.insert(470, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(472);
        self.header_set(473);
        self.values.insert(472, v.guid() as u32);
        self.values.insert(473, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_0(mut self, v: i32) -> Self {
        self.header_set(474);
        self.values.insert(474, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_9_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(486);
        self.values.insert(486, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(488);
        self.header_set(489);
        self.values.insert(488, v.guid() as u32);
        self.values.insert(489, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_0(mut self, v: i32) -> Self {
        self.header_set(490);
        self.values.insert(490, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_10_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(502);
        self.values.insert(502, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(504);
        self.header_set(505);
        self.values.insert(504, v.guid() as u32);
        self.values.insert(505, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_0(mut self, v: i32) -> Self {
        self.header_set(506);
        self.values.insert(506, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_11_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(518);
        self.values.insert(518, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(520);
        self.header_set(521);
        self.values.insert(520, v.guid() as u32);
        self.values.insert(521, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_0(mut self, v: i32) -> Self {
        self.header_set(522);
        self.values.insert(522, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_12_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(534);
        self.values.insert(534, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(536);
        self.header_set(537);
        self.values.insert(536, v.guid() as u32);
        self.values.insert(537, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_0(mut self, v: i32) -> Self {
        self.header_set(538);
        self.values.insert(538, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_13_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(550);
        self.values.insert(550, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(552);
        self.header_set(553);
        self.values.insert(552, v.guid() as u32);
        self.values.insert(553, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_0(mut self, v: i32) -> Self {
        self.header_set(554);
        self.values.insert(554, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_14_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(566);
        self.values.insert(566, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(568);
        self.header_set(569);
        self.values.insert(568, v.guid() as u32);
        self.values.insert(569, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_0(mut self, v: i32) -> Self {
        self.header_set(570);
        self.values.insert(570, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_15_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(582);
        self.values.insert(582, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(584);
        self.header_set(585);
        self.values.insert(584, v.guid() as u32);
        self.values.insert(585, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_0(mut self, v: i32) -> Self {
        self.header_set(586);
        self.values.insert(586, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_16_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(598);
        self.values.insert(598, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(600);
        self.header_set(601);
        self.values.insert(600, v.guid() as u32);
        self.values.insert(601, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_0(mut self, v: i32) -> Self {
        self.header_set(602);
        self.values.insert(602, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_17_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(614);
        self.values.insert(614, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(616);
        self.header_set(617);
        self.values.insert(616, v.guid() as u32);
        self.values.insert(617, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_0(mut self, v: i32) -> Self {
        self.header_set(618);
        self.values.insert(618, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_18_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(630);
        self.values.insert(630, v);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_CREATOR(mut self, v: Guid) -> Self {
        self.header_set(632);
        self.header_set(633);
        self.values.insert(632, v.guid() as u32);
        self.values.insert(633, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_0(mut self, v: i32) -> Self {
        self.header_set(634);
        self.values.insert(634, v as u32);
        self
    }

    pub fn set_player_VISIBLE_ITEM_19_PROPERTIES(mut self, v: u32) -> Self {
        self.header_set(646);
        self.values.insert(646, v);
        self
    }

    pub fn set_player_CHOSEN_TITLE(mut self, v: i32) -> Self {
        self.header_set(648);
        self.values.insert(648, v as u32);
        self
    }

    pub fn set_player_FIELD_INV_SLOT_HEAD(mut self, v: Guid) -> Self {
        self.header_set(650);
        self.header_set(651);
        self.values.insert(650, v.guid() as u32);
        self.values.insert(651, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_PACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(696);
        self.header_set(697);
        self.values.insert(696, v.guid() as u32);
        self.values.insert(697, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(728);
        self.header_set(729);
        self.values.insert(728, v.guid() as u32);
        self.values.insert(729, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_BANKBAG_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(784);
        self.header_set(785);
        self.values.insert(784, v.guid() as u32);
        self.values.insert(785, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VENDORBUYBACK_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(798);
        self.header_set(799);
        self.values.insert(798, v.guid() as u32);
        self.values.insert(799, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_KEYRING_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(822);
        self.header_set(823);
        self.values.insert(822, v.guid() as u32);
        self.values.insert(823, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FIELD_VANITYPET_SLOT_1(mut self, v: Guid) -> Self {
        self.header_set(886);
        self.header_set(887);
        self.values.insert(886, v.guid() as u32);
        self.values.insert(887, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_FARSIGHT(mut self, v: Guid) -> Self {
        self.header_set(922);
        self.header_set(923);
        self.values.insert(922, v.guid() as u32);
        self.values.insert(923, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player__FIELD_KNOWN_TITLES(mut self, v: Guid) -> Self {
        self.header_set(924);
        self.header_set(925);
        self.values.insert(924, v.guid() as u32);
        self.values.insert(925, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_player_XP(mut self, v: i32) -> Self {
        self.header_set(926);
        self.values.insert(926, v as u32);
        self
    }

    pub fn set_player_NEXT_LEVEL_XP(mut self, v: i32) -> Self {
        self.header_set(927);
        self.values.insert(927, v as u32);
        self
    }

    pub fn set_player_SKILL_INFO_1_1(mut self, v: u32) -> Self {
        self.header_set(928);
        self.values.insert(928, v);
        self
    }

    pub fn set_player_CHARACTER_POINTS1(mut self, v: i32) -> Self {
        self.header_set(1312);
        self.values.insert(1312, v as u32);
        self
    }

    pub fn set_player_CHARACTER_POINTS2(mut self, v: i32) -> Self {
        self.header_set(1313);
        self.values.insert(1313, v as u32);
        self
    }

    pub fn set_player_TRACK_CREATURES(mut self, v: i32) -> Self {
        self.header_set(1314);
        self.values.insert(1314, v as u32);
        self
    }

    pub fn set_player_TRACK_RESOURCES(mut self, v: i32) -> Self {
        self.header_set(1315);
        self.values.insert(1315, v as u32);
        self
    }

    pub fn set_player_BLOCK_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1316);
        self.values.insert(1316, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_DODGE_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1317);
        self.values.insert(1317, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_PARRY_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1318);
        self.values.insert(1318, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_EXPERTISE(mut self, v: i32) -> Self {
        self.header_set(1319);
        self.values.insert(1319, v as u32);
        self
    }

    pub fn set_player_OFFHAND_EXPERTISE(mut self, v: i32) -> Self {
        self.header_set(1320);
        self.values.insert(1320, v as u32);
        self
    }

    pub fn set_player_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1321);
        self.values.insert(1321, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_RANGED_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1322);
        self.values.insert(1322, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_OFFHAND_CRIT_PERCENTAGE(mut self, v: f32) -> Self {
        self.header_set(1323);
        self.values.insert(1323, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SPELL_CRIT_PERCENTAGE1(mut self, v: f32) -> Self {
        self.header_set(1324);
        self.values.insert(1324, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_SHIELD_BLOCK(mut self, v: i32) -> Self {
        self.header_set(1331);
        self.values.insert(1331, v as u32);
        self
    }

    pub fn set_player_EXPLORED_ZONES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1332);
        self.values.insert(1332, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_REST_STATE_EXPERIENCE(mut self, v: i32) -> Self {
        self.header_set(1460);
        self.values.insert(1460, v as u32);
        self
    }

    pub fn set_player_FIELD_COINAGE(mut self, v: i32) -> Self {
        self.header_set(1461);
        self.values.insert(1461, v as u32);
        self
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_POS(mut self, v: i32) -> Self {
        self.header_set(1462);
        self.values.insert(1462, v as u32);
        self
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_NEG(mut self, v: i32) -> Self {
        self.header_set(1469);
        self.values.insert(1469, v as u32);
        self
    }

    pub fn set_player_FIELD_MOD_DAMAGE_DONE_PCT(mut self, v: i32) -> Self {
        self.header_set(1476);
        self.values.insert(1476, v as u32);
        self
    }

    pub fn set_player_FIELD_MOD_HEALING_DONE_POS(mut self, v: i32) -> Self {
        self.header_set(1483);
        self.values.insert(1483, v as u32);
        self
    }

    pub fn set_player_FIELD_MOD_TARGET_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(1484);
        self.values.insert(1484, v as u32);
        self
    }

    pub fn set_player_FIELD_MOD_TARGET_PHYSICAL_RESISTANCE(mut self, v: i32) -> Self {
        self.header_set(1485);
        self.values.insert(1485, v as u32);
        self
    }

    pub fn set_player_FIELD_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1486);
        self.values.insert(1486, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_AMMO_ID(mut self, v: i32) -> Self {
        self.header_set(1487);
        self.values.insert(1487, v as u32);
        self
    }

    pub fn set_player_SELF_RES_SPELL(mut self, v: i32) -> Self {
        self.header_set(1488);
        self.values.insert(1488, v as u32);
        self
    }

    pub fn set_player_FIELD_PVP_MEDALS(mut self, v: i32) -> Self {
        self.header_set(1489);
        self.values.insert(1489, v as u32);
        self
    }

    pub fn set_player_FIELD_BUYBACK_PRICE_1(mut self, v: i32) -> Self {
        self.header_set(1490);
        self.values.insert(1490, v as u32);
        self
    }

    pub fn set_player_FIELD_BUYBACK_TIMESTAMP_1(mut self, v: i32) -> Self {
        self.header_set(1502);
        self.values.insert(1502, v as u32);
        self
    }

    pub fn set_player_FIELD_KILLS(mut self, v: u32) -> Self {
        self.header_set(1514);
        self.values.insert(1514, v);
        self
    }

    pub fn set_player_FIELD_TODAY_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1515);
        self.values.insert(1515, v as u32);
        self
    }

    pub fn set_player_FIELD_YESTERDAY_CONTRIBUTION(mut self, v: i32) -> Self {
        self.header_set(1516);
        self.values.insert(1516, v as u32);
        self
    }

    pub fn set_player_FIELD_LIFETIME_HONORABLE_KILLS(mut self, v: i32) -> Self {
        self.header_set(1517);
        self.values.insert(1517, v as u32);
        self
    }

    pub fn set_player_FIELD_BYTES2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(1518);
        self.values.insert(1518, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_player_FIELD_WATCHED_FACTION_INDEX(mut self, v: i32) -> Self {
        self.header_set(1519);
        self.values.insert(1519, v as u32);
        self
    }

    pub fn set_player_FIELD_COMBAT_RATING_1(mut self, v: i32) -> Self {
        self.header_set(1520);
        self.values.insert(1520, v as u32);
        self
    }

    pub fn set_player_FIELD_ARENA_TEAM_INFO_1_1(mut self, v: i32) -> Self {
        self.header_set(1544);
        self.values.insert(1544, v as u32);
        self
    }

    pub fn set_player_FIELD_HONOR_CURRENCY(mut self, v: i32) -> Self {
        self.header_set(1562);
        self.values.insert(1562, v as u32);
        self
    }

    pub fn set_player_FIELD_ARENA_CURRENCY(mut self, v: i32) -> Self {
        self.header_set(1563);
        self.values.insert(1563, v as u32);
        self
    }

    pub fn set_player_FIELD_MOD_MANA_REGEN(mut self, v: f32) -> Self {
        self.header_set(1564);
        self.values.insert(1564, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_MOD_MANA_REGEN_INTERRUPT(mut self, v: f32) -> Self {
        self.header_set(1565);
        self.values.insert(1565, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_player_FIELD_MAX_LEVEL(mut self, v: i32) -> Self {
        self.header_set(1566);
        self.values.insert(1566, v as u32);
        self
    }

    pub fn set_player_FIELD_DAILY_QUESTS_1(mut self, v: i32) -> Self {
        self.header_set(1567);
        self.values.insert(1567, v as u32);
        self
    }

    pub fn set_object_FIELD_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

}

impl UpdateGameObject {
    pub fn set_object_FIELD_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_FIELD_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_FIELD_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_FIELD_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_FIELD_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_gameobject_DISPLAYID(mut self, v: i32) -> Self {
        self.header_set(8);
        self.values.insert(8, v as u32);
        self
    }

    pub fn set_gameobject_FLAGS(mut self, v: i32) -> Self {
        self.header_set(9);
        self.values.insert(9, v as u32);
        self
    }

    pub fn set_gameobject_ROTATION(mut self, v: f32) -> Self {
        self.header_set(10);
        self.values.insert(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_STATE(mut self, v: i32) -> Self {
        self.header_set(14);
        self.values.insert(14, v as u32);
        self
    }

    pub fn set_gameobject_POS_X(mut self, v: f32) -> Self {
        self.header_set(15);
        self.values.insert(15, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_POS_Y(mut self, v: f32) -> Self {
        self.header_set(16);
        self.values.insert(16, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_POS_Z(mut self, v: f32) -> Self {
        self.header_set(17);
        self.values.insert(17, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_FACING(mut self, v: f32) -> Self {
        self.header_set(18);
        self.values.insert(18, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_gameobject_DYN_FLAGS(mut self, v: i32) -> Self {
        self.header_set(19);
        self.values.insert(19, v as u32);
        self
    }

    pub fn set_gameobject_FACTION(mut self, v: i32) -> Self {
        self.header_set(20);
        self.values.insert(20, v as u32);
        self
    }

    pub fn set_gameobject_TYPE_ID(mut self, v: i32) -> Self {
        self.header_set(21);
        self.values.insert(21, v as u32);
        self
    }

    pub fn set_gameobject_LEVEL(mut self, v: i32) -> Self {
        self.header_set(22);
        self.values.insert(22, v as u32);
        self
    }

    pub fn set_gameobject_ARTKIT(mut self, v: i32) -> Self {
        self.header_set(23);
        self.values.insert(23, v as u32);
        self
    }

    pub fn set_gameobject_ANIMPROGRESS(mut self, v: i32) -> Self {
        self.header_set(24);
        self.values.insert(24, v as u32);
        self
    }

}

impl UpdateDynamicObject {
    pub fn set_object_FIELD_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_FIELD_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_FIELD_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_FIELD_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_FIELD_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_dynamicobject_CASTER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_dynamicobject_BYTES(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(8);
        self.values.insert(8, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_dynamicobject_SPELLID(mut self, v: i32) -> Self {
        self.header_set(9);
        self.values.insert(9, v as u32);
        self
    }

    pub fn set_dynamicobject_RADIUS(mut self, v: f32) -> Self {
        self.header_set(10);
        self.values.insert(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_POS_X(mut self, v: f32) -> Self {
        self.header_set(11);
        self.values.insert(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_POS_Y(mut self, v: f32) -> Self {
        self.header_set(12);
        self.values.insert(12, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_POS_Z(mut self, v: f32) -> Self {
        self.header_set(13);
        self.values.insert(13, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_FACING(mut self, v: f32) -> Self {
        self.header_set(14);
        self.values.insert(14, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_dynamicobject_CASTTIME(mut self, v: i32) -> Self {
        self.header_set(15);
        self.values.insert(15, v as u32);
        self
    }

}

impl UpdateCorpse {
    pub fn set_object_FIELD_GUID(mut self, v: Guid) -> Self {
        self.header_set(0);
        self.header_set(1);
        self.values.insert(0, v.guid() as u32);
        self.values.insert(1, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_object_FIELD_TYPE(mut self, v: i32) -> Self {
        self.header_set(2);
        self.values.insert(2, v as u32);
        self
    }

    pub fn set_object_FIELD_ENTRY(mut self, v: i32) -> Self {
        self.header_set(3);
        self.values.insert(3, v as u32);
        self
    }

    pub fn set_object_FIELD_SCALE_X(mut self, v: f32) -> Self {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_object_FIELD_CREATED_BY(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_FIELD_OWNER(mut self, v: Guid) -> Self {
        self.header_set(6);
        self.header_set(7);
        self.values.insert(6, v.guid() as u32);
        self.values.insert(7, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_FIELD_PARTY(mut self, v: Guid) -> Self {
        self.header_set(8);
        self.header_set(9);
        self.values.insert(8, v.guid() as u32);
        self.values.insert(9, (v.guid() >> 32) as u32);
        self
    }

    pub fn set_corpse_FIELD_FACING(mut self, v: f32) -> Self {
        self.header_set(10);
        self.values.insert(10, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_FIELD_POS_X(mut self, v: f32) -> Self {
        self.header_set(11);
        self.values.insert(11, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_FIELD_POS_Y(mut self, v: f32) -> Self {
        self.header_set(12);
        self.values.insert(12, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_FIELD_POS_Z(mut self, v: f32) -> Self {
        self.header_set(13);
        self.values.insert(13, u32::from_le_bytes(v.to_le_bytes()));
        self
    }

    pub fn set_corpse_FIELD_DISPLAY_ID(mut self, v: i32) -> Self {
        self.header_set(14);
        self.values.insert(14, v as u32);
        self
    }

    pub fn set_corpse_FIELD_ITEM(mut self, v: i32) -> Self {
        self.header_set(15);
        self.values.insert(15, v as u32);
        self
    }

    pub fn set_corpse_FIELD_BYTES_1(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(34);
        self.values.insert(34, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_FIELD_BYTES_2(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.header_set(35);
        self.values.insert(35, u32::from_le_bytes([a, b, c, d]));
        self
    }

    pub fn set_corpse_FIELD_GUILD(mut self, v: i32) -> Self {
        self.header_set(36);
        self.values.insert(36, v as u32);
        self
    }

    pub fn set_corpse_FIELD_FLAGS(mut self, v: i32) -> Self {
        self.header_set(37);
        self.values.insert(37, v as u32);
        self
    }

    pub fn set_corpse_FIELD_DYNAMIC_FLAGS(mut self, v: i32) -> Self {
        self.header_set(38);
        self.values.insert(38, v as u32);
        self
    }

}

