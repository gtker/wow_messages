use crate::{Guid, UpdateMask};

impl UpdateMask {
    pub fn set_object_GUID(&mut self, v: Guid) {
        self.header_set(0);
        self.values.insert(0, v.guid());
    }

    pub fn set_object_TYPE(&mut self, v: i32) {
        self.header_set(2);
        self.values.insert(2, v as u64);
    }

    pub fn set_object_ENTRY(&mut self, v: i32) {
        self.header_set(3);
        self.values.insert(3, v as u64);
    }

    pub fn set_object_SCALE_X(&mut self, v: f32) {
        self.header_set(4);
        self.values.insert(4, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_item_OWNER(&mut self, v: Guid) {
        self.header_set(6);
        self.values.insert(6, v.guid());
    }

    pub fn set_item_CONTAINED(&mut self, v: Guid) {
        self.header_set(8);
        self.values.insert(8, v.guid());
    }

    pub fn set_item_CREATOR(&mut self, v: Guid) {
        self.header_set(10);
        self.values.insert(10, v.guid());
    }

    pub fn set_item_GIFTCREATOR(&mut self, v: Guid) {
        self.header_set(12);
        self.values.insert(12, v.guid());
    }

    pub fn set_item_STACK_COUNT(&mut self, v: i32) {
        self.header_set(14);
        self.values.insert(14, v as u64);
    }

    pub fn set_item_DURATION(&mut self, v: i32) {
        self.header_set(15);
        self.values.insert(15, v as u64);
    }

    pub fn set_item_SPELL_CHARGES(&mut self, v: i32) {
        self.header_set(16);
        self.values.insert(16, v as u64);
    }

    pub fn set_item_FLAGS(&mut self, v: i32) {
        self.header_set(21);
        self.values.insert(21, v as u64);
    }

    pub fn set_item_ENCHANTMENT(&mut self, v: i32) {
        self.header_set(22);
        self.values.insert(22, v as u64);
    }

    pub fn set_item_PROPERTY_SEED(&mut self, v: i32) {
        self.header_set(43);
        self.values.insert(43, v as u64);
    }

    pub fn set_item_RANDOM_PROPERTIES_ID(&mut self, v: i32) {
        self.header_set(44);
        self.values.insert(44, v as u64);
    }

    pub fn set_item_ITEM_TEXT_ID(&mut self, v: i32) {
        self.header_set(45);
        self.values.insert(45, v as u64);
    }

    pub fn set_item_DURABILITY(&mut self, v: i32) {
        self.header_set(46);
        self.values.insert(46, v as u64);
    }

    pub fn set_item_MAXDURABILITY(&mut self, v: i32) {
        self.header_set(47);
        self.values.insert(47, v as u64);
    }

    pub fn set_unit_CHARM(&mut self, v: Guid) {
        self.header_set(6);
        self.values.insert(6, v.guid());
    }

    pub fn set_unit_SUMMON(&mut self, v: Guid) {
        self.header_set(8);
        self.values.insert(8, v.guid());
    }

    pub fn set_unit_CHARMEDBY(&mut self, v: Guid) {
        self.header_set(10);
        self.values.insert(10, v.guid());
    }

    pub fn set_unit_SUMMONEDBY(&mut self, v: Guid) {
        self.header_set(12);
        self.values.insert(12, v.guid());
    }

    pub fn set_unit_CREATEDBY(&mut self, v: Guid) {
        self.header_set(14);
        self.values.insert(14, v.guid());
    }

    pub fn set_unit_TARGET(&mut self, v: Guid) {
        self.header_set(16);
        self.values.insert(16, v.guid());
    }

    pub fn set_unit_PERSUADED(&mut self, v: Guid) {
        self.header_set(18);
        self.values.insert(18, v.guid());
    }

    pub fn set_unit_CHANNEL_OBJECT(&mut self, v: Guid) {
        self.header_set(20);
        self.values.insert(20, v.guid());
    }

    pub fn set_unit_HEALTH(&mut self, v: i32) {
        self.header_set(22);
        self.values.insert(22, v as u64);
    }

    pub fn set_unit_POWER1(&mut self, v: i32) {
        self.header_set(23);
        self.values.insert(23, v as u64);
    }

    pub fn set_unit_POWER2(&mut self, v: i32) {
        self.header_set(24);
        self.values.insert(24, v as u64);
    }

    pub fn set_unit_POWER3(&mut self, v: i32) {
        self.header_set(25);
        self.values.insert(25, v as u64);
    }

    pub fn set_unit_POWER4(&mut self, v: i32) {
        self.header_set(26);
        self.values.insert(26, v as u64);
    }

    pub fn set_unit_POWER5(&mut self, v: i32) {
        self.header_set(27);
        self.values.insert(27, v as u64);
    }

    pub fn set_unit_MAXHEALTH(&mut self, v: i32) {
        self.header_set(28);
        self.values.insert(28, v as u64);
    }

    pub fn set_unit_MAXPOWER1(&mut self, v: i32) {
        self.header_set(29);
        self.values.insert(29, v as u64);
    }

    pub fn set_unit_MAXPOWER2(&mut self, v: i32) {
        self.header_set(30);
        self.values.insert(30, v as u64);
    }

    pub fn set_unit_MAXPOWER3(&mut self, v: i32) {
        self.header_set(31);
        self.values.insert(31, v as u64);
    }

    pub fn set_unit_MAXPOWER4(&mut self, v: i32) {
        self.header_set(32);
        self.values.insert(32, v as u64);
    }

    pub fn set_unit_MAXPOWER5(&mut self, v: i32) {
        self.header_set(33);
        self.values.insert(33, v as u64);
    }

    pub fn set_unit_LEVEL(&mut self, v: i32) {
        self.header_set(34);
        self.values.insert(34, v as u64);
    }

    pub fn set_unit_FACTIONTEMPLATE(&mut self, v: i32) {
        self.header_set(35);
        self.values.insert(35, v as u64);
    }

    pub fn set_unit_BYTES_0(&mut self, v: u32) {
        self.header_set(36);
        self.values.insert(36, v as u64);
    }

    pub fn set_unit_VIRTUAL_ITEM_SLOT_DISPLAY(&mut self, v: i32) {
        self.header_set(37);
        self.values.insert(37, v as u64);
    }

    pub fn set_unit_VIRTUAL_ITEM_INFO(&mut self, v: u32) {
        self.header_set(40);
        self.values.insert(40, v as u64);
    }

    pub fn set_unit_FLAGS(&mut self, v: i32) {
        self.header_set(46);
        self.values.insert(46, v as u64);
    }

    pub fn set_unit_AURA(&mut self, v: i32) {
        self.header_set(47);
        self.values.insert(47, v as u64);
    }

    pub fn set_unit_AURAFLAGS(&mut self, v: u32) {
        self.header_set(95);
        self.values.insert(95, v as u64);
    }

    pub fn set_unit_AURALEVELS(&mut self, v: u32) {
        self.header_set(101);
        self.values.insert(101, v as u64);
    }

    pub fn set_unit_AURAAPPLICATIONS(&mut self, v: u32) {
        self.header_set(113);
        self.values.insert(113, v as u64);
    }

    pub fn set_unit_AURASTATE(&mut self, v: i32) {
        self.header_set(125);
        self.values.insert(125, v as u64);
    }

    pub fn set_unit_BASEATTACKTIME(&mut self, v: i32) {
        self.header_set(126);
        self.values.insert(126, v as u64);
    }

    pub fn set_unit_RANGEDATTACKTIME(&mut self, v: i32) {
        self.header_set(128);
        self.values.insert(128, v as u64);
    }

    pub fn set_unit_BOUNDINGRADIUS(&mut self, v: f32) {
        self.header_set(129);
        self.values.insert(129, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_unit_COMBATREACH(&mut self, v: f32) {
        self.header_set(130);
        self.values.insert(130, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_unit_DISPLAYID(&mut self, v: i32) {
        self.header_set(131);
        self.values.insert(131, v as u64);
    }

    pub fn set_unit_NATIVEDISPLAYID(&mut self, v: i32) {
        self.header_set(132);
        self.values.insert(132, v as u64);
    }

    pub fn set_unit_MOUNTDISPLAYID(&mut self, v: i32) {
        self.header_set(133);
        self.values.insert(133, v as u64);
    }

    pub fn set_unit_MINDAMAGE(&mut self, v: f32) {
        self.header_set(134);
        self.values.insert(134, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_unit_MAXDAMAGE(&mut self, v: f32) {
        self.header_set(135);
        self.values.insert(135, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_unit_MINOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(136);
        self.values.insert(136, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_unit_MAXOFFHANDDAMAGE(&mut self, v: f32) {
        self.header_set(137);
        self.values.insert(137, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_unit_BYTES_1(&mut self, v: u32) {
        self.header_set(138);
        self.values.insert(138, v as u64);
    }

    pub fn set_unit_PETNUMBER(&mut self, v: i32) {
        self.header_set(139);
        self.values.insert(139, v as u64);
    }

    pub fn set_unit_PET_NAME_TIMESTAMP(&mut self, v: i32) {
        self.header_set(140);
        self.values.insert(140, v as u64);
    }

    pub fn set_unit_PETEXPERIENCE(&mut self, v: i32) {
        self.header_set(141);
        self.values.insert(141, v as u64);
    }

    pub fn set_unit_PETNEXTLEVELEXP(&mut self, v: i32) {
        self.header_set(142);
        self.values.insert(142, v as u64);
    }

    pub fn set_unit_DYNAMIC_FLAGS(&mut self, v: i32) {
        self.header_set(143);
        self.values.insert(143, v as u64);
    }

    pub fn set_unit_CHANNEL_SPELL(&mut self, v: i32) {
        self.header_set(144);
        self.values.insert(144, v as u64);
    }

    pub fn set_unit_MOD_CAST_SPEED(&mut self, v: f32) {
        self.header_set(145);
        self.values.insert(145, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_unit_CREATED_BY_SPELL(&mut self, v: i32) {
        self.header_set(146);
        self.values.insert(146, v as u64);
    }

    pub fn set_unit_NPC_FLAGS(&mut self, v: i32) {
        self.header_set(147);
        self.values.insert(147, v as u64);
    }

    pub fn set_unit_NPC_EMOTESTATE(&mut self, v: i32) {
        self.header_set(148);
        self.values.insert(148, v as u64);
    }

    pub fn set_unit_TRAINING_POINTS(&mut self, v: u32) {
        self.header_set(149);
        self.values.insert(149, v as u64);
    }

    pub fn set_unit_STAT0(&mut self, v: i32) {
        self.header_set(150);
        self.values.insert(150, v as u64);
    }

    pub fn set_unit_STAT1(&mut self, v: i32) {
        self.header_set(151);
        self.values.insert(151, v as u64);
    }

    pub fn set_unit_STAT2(&mut self, v: i32) {
        self.header_set(152);
        self.values.insert(152, v as u64);
    }

    pub fn set_unit_STAT3(&mut self, v: i32) {
        self.header_set(153);
        self.values.insert(153, v as u64);
    }

    pub fn set_unit_STAT4(&mut self, v: i32) {
        self.header_set(154);
        self.values.insert(154, v as u64);
    }

    pub fn set_unit_RESISTANCES(&mut self, v: i32) {
        self.header_set(155);
        self.values.insert(155, v as u64);
    }

    pub fn set_unit_BASE_MANA(&mut self, v: i32) {
        self.header_set(162);
        self.values.insert(162, v as u64);
    }

    pub fn set_unit_BASE_HEALTH(&mut self, v: i32) {
        self.header_set(163);
        self.values.insert(163, v as u64);
    }

    pub fn set_unit_BYTES_2(&mut self, v: u32) {
        self.header_set(164);
        self.values.insert(164, v as u64);
    }

    pub fn set_unit_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(165);
        self.values.insert(165, v as u64);
    }

    pub fn set_unit_ATTACK_POWER_MODS(&mut self, v: u32) {
        self.header_set(166);
        self.values.insert(166, v as u64);
    }

    pub fn set_unit_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(167);
        self.values.insert(167, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_unit_RANGED_ATTACK_POWER(&mut self, v: i32) {
        self.header_set(168);
        self.values.insert(168, v as u64);
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MODS(&mut self, v: u32) {
        self.header_set(169);
        self.values.insert(169, v as u64);
    }

    pub fn set_unit_RANGED_ATTACK_POWER_MULTIPLIER(&mut self, v: f32) {
        self.header_set(170);
        self.values.insert(170, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_unit_MINRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(171);
        self.values.insert(171, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_unit_MAXRANGEDDAMAGE(&mut self, v: f32) {
        self.header_set(172);
        self.values.insert(172, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

    pub fn set_unit_POWER_COST_MODIFIER(&mut self, v: i32) {
        self.header_set(173);
        self.values.insert(173, v as u64);
    }

    pub fn set_unit_POWER_COST_MULTIPLIER(&mut self, v: f32) {
        self.header_set(180);
        self.values.insert(180, u32::from_le_bytes(v.to_le_bytes()) as u64);
    }

}

