use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    DamageInfo, HitInfo, VictimState,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm:64`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm#L64):
/// ```text
/// smsg SMSG_ATTACKERSTATEUPDATE = 0x014A {
///     HitInfo hit_info;
///     PackedGuid attacker;
///     PackedGuid target;
///     u32 total_damage;
///     u32 overkill;
///     u8 amount_of_damages;
///     DamageInfo[amount_of_damages] damage_infos;
///     if (hit_info & ALL_ABSORB) {
///         u32 absorb;
///     }
///     if (hit_info & ALL_RESIST) {
///         u32 resist;
///     }
///     VictimState victim_state;
///     u32 unknown1;
///     u32 unknown2;
///     if (hit_info & BLOCK) {
///         u32 blocked_amount;
///     }
///     if (hit_info & UNK19) {
///         u32 unknown3;
///     }
///     if (hit_info & UNK1) {
///         u32 unknown4;
///         f32 unknown5;
///         f32 unknown6;
///         f32 unknown7;
///         f32 unknown8;
///         f32 unknown9;
///         f32 unknown10;
///         f32 unknown11;
///         f32 unknown12;
///         f32 unknown13;
///         f32 unknown14;
///         u32 unknown15;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE {
    pub hit_info: SMSG_ATTACKERSTATEUPDATE_HitInfo,
    pub attacker: Guid,
    pub target: Guid,
    pub total_damage: u32,
    pub overkill: u32,
    pub damage_infos: Vec<DamageInfo>,
    pub victim_state: VictimState,
    /// arcemu: can be 0,1000 or -1
    pub unknown1: u32,
    pub unknown2: u32,
}

impl crate::private::Sealed for SMSG_ATTACKERSTATEUPDATE {}
impl SMSG_ATTACKERSTATEUPDATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(24..=3176).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // hit_info: HitInfo
        let hit_info = HitInfo::new(crate::util::read_u32_le(&mut r)?);

        // attacker: PackedGuid
        let attacker = crate::util::read_packed_guid(&mut r)?;

        // target: PackedGuid
        let target = crate::util::read_packed_guid(&mut r)?;

        // total_damage: u32
        let total_damage = crate::util::read_u32_le(&mut r)?;

        // overkill: u32
        let overkill = crate::util::read_u32_le(&mut r)?;

        // amount_of_damages: u8
        let amount_of_damages = crate::util::read_u8_le(&mut r)?;

        // damage_infos: DamageInfo[amount_of_damages]
        let damage_infos = {
            let mut damage_infos = Vec::with_capacity(amount_of_damages as usize);
            for _ in 0..amount_of_damages {
                damage_infos.push(DamageInfo::read(&mut r)?);
            }
            damage_infos
        };

        let hit_info_all_absorb = if hit_info.is_all_absorb() {
            // absorb: u32
            let absorb = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb {
                absorb,
            })
        }
        else {
            None
        };

        let hit_info_all_resist = if hit_info.is_all_resist() {
            // resist: u32
            let resist = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist {
                resist,
            })
        }
        else {
            None
        };

        // victim_state: VictimState
        let victim_state = VictimState::new(crate::util::read_u8_le(&mut r)?);

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        let hit_info_block = if hit_info.is_block() {
            // blocked_amount: u32
            let blocked_amount = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_ATTACKERSTATEUPDATE_HitInfo_Block {
                blocked_amount,
            })
        }
        else {
            None
        };

        let hit_info_unk19 = if hit_info.is_unk19() {
            // unknown3: u32
            let unknown3 = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19 {
                unknown3,
            })
        }
        else {
            None
        };

        let hit_info_unk1 = if hit_info.is_unk1() {
            // unknown4: u32
            let unknown4 = crate::util::read_u32_le(&mut r)?;

            // unknown5: f32
            let unknown5 = crate::util::read_f32_le(&mut r)?;

            // unknown6: f32
            let unknown6 = crate::util::read_f32_le(&mut r)?;

            // unknown7: f32
            let unknown7 = crate::util::read_f32_le(&mut r)?;

            // unknown8: f32
            let unknown8 = crate::util::read_f32_le(&mut r)?;

            // unknown9: f32
            let unknown9 = crate::util::read_f32_le(&mut r)?;

            // unknown10: f32
            let unknown10 = crate::util::read_f32_le(&mut r)?;

            // unknown11: f32
            let unknown11 = crate::util::read_f32_le(&mut r)?;

            // unknown12: f32
            let unknown12 = crate::util::read_f32_le(&mut r)?;

            // unknown13: f32
            let unknown13 = crate::util::read_f32_le(&mut r)?;

            // unknown14: f32
            let unknown14 = crate::util::read_f32_le(&mut r)?;

            // unknown15: u32
            let unknown15 = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1 {
                unknown10,
                unknown11,
                unknown12,
                unknown13,
                unknown14,
                unknown15,
                unknown4,
                unknown5,
                unknown6,
                unknown7,
                unknown8,
                unknown9,
            })
        }
        else {
            None
        };

        let hit_info = SMSG_ATTACKERSTATEUPDATE_HitInfo {
            inner: hit_info.as_int(),
            unk1: hit_info_unk1,
            all_absorb: hit_info_all_absorb,
            all_resist: hit_info_all_resist,
            block: hit_info_block,
            unk19: hit_info_unk19,
        };

        Ok(Self {
            hit_info,
            attacker,
            target,
            total_damage,
            overkill,
            damage_infos,
            victim_state,
            unknown1,
            unknown2,
        })
    }

}

impl crate::Message for SMSG_ATTACKERSTATEUPDATE {
    const OPCODE: u32 = 0x014a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ATTACKERSTATEUPDATE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ATTACKERSTATEUPDATE {{").unwrap();
        // Members
        writeln!(s, "    hit_info = {};", HitInfo::new(self.hit_info.as_int()).as_test_case_value()).unwrap();
        writeln!(s, "    attacker = {};", self.attacker.guid()).unwrap();
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    total_damage = {};", self.total_damage).unwrap();
        writeln!(s, "    overkill = {};", self.overkill).unwrap();
        writeln!(s, "    amount_of_damages = {};", self.damage_infos.len()).unwrap();
        writeln!(s, "    damage_infos = [").unwrap();
        for v in self.damage_infos.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            spell_school_mask = {};", v.spell_school_mask).unwrap();
            writeln!(s, "            damage_float = {};", if v.damage_float.to_string().contains('.') { v.damage_float.to_string() } else { format!("{}.0", v.damage_float) }).unwrap();
            writeln!(s, "            damage_uint = {};", v.damage_uint).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        if let Some(if_statement) = &self.hit_info.get_all_absorb() {
            writeln!(s, "    absorb = {};", if_statement.absorb).unwrap();
        }

        if let Some(if_statement) = &self.hit_info.get_all_resist() {
            writeln!(s, "    resist = {};", if_statement.resist).unwrap();
        }

        writeln!(s, "    victim_state = {};", self.victim_state.as_test_case_value()).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        if let Some(if_statement) = &self.hit_info.get_block() {
            writeln!(s, "    blocked_amount = {};", if_statement.blocked_amount).unwrap();
        }

        if let Some(if_statement) = &self.hit_info.get_unk19() {
            writeln!(s, "    unknown3 = {};", if_statement.unknown3).unwrap();
        }

        if let Some(if_statement) = &self.hit_info.get_unk1() {
            writeln!(s, "    unknown4 = {};", if_statement.unknown4).unwrap();
            writeln!(s, "    unknown5 = {};", if if_statement.unknown5.to_string().contains('.') { if_statement.unknown5.to_string() } else { format!("{}.0", if_statement.unknown5) }).unwrap();
            writeln!(s, "    unknown6 = {};", if if_statement.unknown6.to_string().contains('.') { if_statement.unknown6.to_string() } else { format!("{}.0", if_statement.unknown6) }).unwrap();
            writeln!(s, "    unknown7 = {};", if if_statement.unknown7.to_string().contains('.') { if_statement.unknown7.to_string() } else { format!("{}.0", if_statement.unknown7) }).unwrap();
            writeln!(s, "    unknown8 = {};", if if_statement.unknown8.to_string().contains('.') { if_statement.unknown8.to_string() } else { format!("{}.0", if_statement.unknown8) }).unwrap();
            writeln!(s, "    unknown9 = {};", if if_statement.unknown9.to_string().contains('.') { if_statement.unknown9.to_string() } else { format!("{}.0", if_statement.unknown9) }).unwrap();
            writeln!(s, "    unknown10 = {};", if if_statement.unknown10.to_string().contains('.') { if_statement.unknown10.to_string() } else { format!("{}.0", if_statement.unknown10) }).unwrap();
            writeln!(s, "    unknown11 = {};", if if_statement.unknown11.to_string().contains('.') { if_statement.unknown11.to_string() } else { format!("{}.0", if_statement.unknown11) }).unwrap();
            writeln!(s, "    unknown12 = {};", if if_statement.unknown12.to_string().contains('.') { if_statement.unknown12.to_string() } else { format!("{}.0", if_statement.unknown12) }).unwrap();
            writeln!(s, "    unknown13 = {};", if if_statement.unknown13.to_string().contains('.') { if_statement.unknown13.to_string() } else { format!("{}.0", if_statement.unknown13) }).unwrap();
            writeln!(s, "    unknown14 = {};", if if_statement.unknown14.to_string().contains('.') { if_statement.unknown14.to_string() } else { format!("{}.0", if_statement.unknown14) }).unwrap();
            writeln!(s, "    unknown15 = {};", if_statement.unknown15).unwrap();
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 330_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "hit_info", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.attacker), "attacker", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.target), "target", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "total_damage", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "overkill", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_damages", "    ");
        if !self.damage_infos.is_empty() {
            writeln!(s, "    /* damage_infos: DamageInfo[amount_of_damages] start */").unwrap();
            for (i, v) in self.damage_infos.iter().enumerate() {
                writeln!(s, "    /* damage_infos: DamageInfo[amount_of_damages] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_school_mask", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "damage_float", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "damage_uint", "        ");
                writeln!(s, "    /* damage_infos: DamageInfo[amount_of_damages] {i} end */").unwrap();
            }
            writeln!(s, "    /* damage_infos: DamageInfo[amount_of_damages] end */").unwrap();
        }
        if let Some(if_statement) = &self.hit_info.get_all_absorb() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "absorb", "    ");
        }

        if let Some(if_statement) = &self.hit_info.get_all_resist() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "resist", "    ");
        }

        crate::util::write_bytes(&mut s, &mut bytes, 1, "victim_state", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "    ");
        if let Some(if_statement) = &self.hit_info.get_block() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "blocked_amount", "    ");
        }

        if let Some(if_statement) = &self.hit_info.get_unk19() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown3", "    ");
        }

        if let Some(if_statement) = &self.hit_info.get_unk1() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown4", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown5", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown6", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown7", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown8", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown9", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown10", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown11", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown12", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown13", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown14", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown15", "    ");
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // hit_info: HitInfo
        w.write_all(&(self.hit_info.as_int().to_le_bytes()))?;

        // attacker: PackedGuid
        crate::util::write_packed_guid(&self.attacker, &mut w)?;

        // target: PackedGuid
        crate::util::write_packed_guid(&self.target, &mut w)?;

        // total_damage: u32
        w.write_all(&self.total_damage.to_le_bytes())?;

        // overkill: u32
        w.write_all(&self.overkill.to_le_bytes())?;

        // amount_of_damages: u8
        w.write_all(&(self.damage_infos.len() as u8).to_le_bytes())?;

        // damage_infos: DamageInfo[amount_of_damages]
        for i in self.damage_infos.iter() {
            i.write_into_vec(&mut w)?;
        }

        if let Some(if_statement) = &self.hit_info.all_absorb {
            // absorb: u32
            w.write_all(&if_statement.absorb.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.hit_info.all_resist {
            // resist: u32
            w.write_all(&if_statement.resist.to_le_bytes())?;

        }

        // victim_state: VictimState
        w.write_all(&(self.victim_state.as_int().to_le_bytes()))?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        if let Some(if_statement) = &self.hit_info.block {
            // blocked_amount: u32
            w.write_all(&if_statement.blocked_amount.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.hit_info.unk19 {
            // unknown3: u32
            w.write_all(&if_statement.unknown3.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.hit_info.unk1 {
            // unknown4: u32
            w.write_all(&if_statement.unknown4.to_le_bytes())?;

            // unknown5: f32
            w.write_all(&if_statement.unknown5.to_le_bytes())?;

            // unknown6: f32
            w.write_all(&if_statement.unknown6.to_le_bytes())?;

            // unknown7: f32
            w.write_all(&if_statement.unknown7.to_le_bytes())?;

            // unknown8: f32
            w.write_all(&if_statement.unknown8.to_le_bytes())?;

            // unknown9: f32
            w.write_all(&if_statement.unknown9.to_le_bytes())?;

            // unknown10: f32
            w.write_all(&if_statement.unknown10.to_le_bytes())?;

            // unknown11: f32
            w.write_all(&if_statement.unknown11.to_le_bytes())?;

            // unknown12: f32
            w.write_all(&if_statement.unknown12.to_le_bytes())?;

            // unknown13: f32
            w.write_all(&if_statement.unknown13.to_le_bytes())?;

            // unknown14: f32
            w.write_all(&if_statement.unknown14.to_le_bytes())?;

            // unknown15: u32
            w.write_all(&if_statement.unknown15.to_le_bytes())?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(330, "SMSG_ATTACKERSTATEUPDATE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ATTACKERSTATEUPDATE {}

impl SMSG_ATTACKERSTATEUPDATE {
    pub(crate) fn size(&self) -> usize {
        self.hit_info.size() // hit_info: SMSG_ATTACKERSTATEUPDATE_HitInfo
        + crate::util::packed_guid_size(&self.attacker) // attacker: PackedGuid
        + crate::util::packed_guid_size(&self.target) // target: PackedGuid
        + 4 // total_damage: u32
        + 4 // overkill: u32
        + 1 // amount_of_damages: u8
        + self.damage_infos.len() * 12 // damage_infos: DamageInfo[amount_of_damages]
        + 1 // victim_state: VictimState
        + 4 // unknown1: u32
        + 4 // unknown2: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE_HitInfo {
    inner: u32,
    unk1: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1>,
    all_absorb: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb>,
    all_resist: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist>,
    block: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_Block>,
    unk19: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19>,
}

impl SMSG_ATTACKERSTATEUPDATE_HitInfo {
    pub const fn new(inner: u32, unk1: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1>,all_absorb: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb>,all_resist: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist>,block: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_Block>,unk19: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19>,) -> Self {
        Self {
            inner,
            unk1, 
            all_absorb, 
            all_resist, 
            block, 
            unk19, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.unk1.is_none()
        && self.all_absorb.is_none()
        && self.all_resist.is_none()
        && self.block.is_none()
        && self.unk19.is_none()
    }

    pub const fn new_unk1(unk1: SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1) -> Self {
        Self {
            inner: HitInfo::UNK1,
            unk1: Some(unk1),
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk1(mut self, unk1: SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1) -> Self {
        self.inner |= HitInfo::UNK1;
        self.unk1 = Some(unk1);
        self
    }

    pub const fn get_unk1(&self) -> Option<&SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1> {
        self.unk1.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk1(mut self) -> Self {
        self.inner &= HitInfo::UNK1.reverse_bits();
        self.unk1 = None;
        self
    }

    pub const fn new_affects_victim() -> Self {
        Self {
            inner: HitInfo::AFFECTS_VICTIM,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_affects_victim(mut self) -> Self {
        self.inner |= HitInfo::AFFECTS_VICTIM;
        self
    }

    pub const fn get_affects_victim(&self) -> bool {
        (self.inner & HitInfo::AFFECTS_VICTIM) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_affects_victim(mut self) -> Self {
        self.inner &= HitInfo::AFFECTS_VICTIM.reverse_bits();
        self
    }

    pub const fn new_offhand() -> Self {
        Self {
            inner: HitInfo::OFFHAND,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_offhand(mut self) -> Self {
        self.inner |= HitInfo::OFFHAND;
        self
    }

    pub const fn get_offhand(&self) -> bool {
        (self.inner & HitInfo::OFFHAND) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_offhand(mut self) -> Self {
        self.inner &= HitInfo::OFFHAND.reverse_bits();
        self
    }

    pub const fn new_unk2() -> Self {
        Self {
            inner: HitInfo::UNK2,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk2(mut self) -> Self {
        self.inner |= HitInfo::UNK2;
        self
    }

    pub const fn get_unk2(&self) -> bool {
        (self.inner & HitInfo::UNK2) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk2(mut self) -> Self {
        self.inner &= HitInfo::UNK2.reverse_bits();
        self
    }

    pub const fn new_miss() -> Self {
        Self {
            inner: HitInfo::MISS,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_miss(mut self) -> Self {
        self.inner |= HitInfo::MISS;
        self
    }

    pub const fn get_miss(&self) -> bool {
        (self.inner & HitInfo::MISS) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_miss(mut self) -> Self {
        self.inner &= HitInfo::MISS.reverse_bits();
        self
    }

    pub const fn new_full_absorb() -> Self {
        Self {
            inner: HitInfo::FULL_ABSORB,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_full_absorb(mut self) -> Self {
        self.inner |= HitInfo::FULL_ABSORB;
        self
    }

    pub const fn get_full_absorb(&self) -> bool {
        (self.inner & HitInfo::FULL_ABSORB) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_full_absorb(mut self) -> Self {
        self.inner &= HitInfo::FULL_ABSORB.reverse_bits();
        self
    }

    pub const fn new_partial_absorb() -> Self {
        Self {
            inner: HitInfo::PARTIAL_ABSORB,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_partial_absorb(mut self) -> Self {
        self.inner |= HitInfo::PARTIAL_ABSORB;
        self
    }

    pub const fn get_partial_absorb(&self) -> bool {
        (self.inner & HitInfo::PARTIAL_ABSORB) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_partial_absorb(mut self) -> Self {
        self.inner &= HitInfo::PARTIAL_ABSORB.reverse_bits();
        self
    }

    pub const fn new_all_absorb(all_absorb: SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb) -> Self {
        Self {
            inner: HitInfo::ALL_ABSORB,
            unk1: None,
            all_absorb: Some(all_absorb),
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_all_absorb(mut self, all_absorb: SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb) -> Self {
        self.inner |= HitInfo::ALL_ABSORB;
        self.all_absorb = Some(all_absorb);
        self
    }

    pub const fn get_all_absorb(&self) -> Option<&SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb> {
        self.all_absorb.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_all_absorb(mut self) -> Self {
        self.inner &= HitInfo::ALL_ABSORB.reverse_bits();
        self.all_absorb = None;
        self
    }

    pub const fn new_full_resist() -> Self {
        Self {
            inner: HitInfo::FULL_RESIST,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_full_resist(mut self) -> Self {
        self.inner |= HitInfo::FULL_RESIST;
        self
    }

    pub const fn get_full_resist(&self) -> bool {
        (self.inner & HitInfo::FULL_RESIST) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_full_resist(mut self) -> Self {
        self.inner &= HitInfo::FULL_RESIST.reverse_bits();
        self
    }

    pub const fn new_partial_resist() -> Self {
        Self {
            inner: HitInfo::PARTIAL_RESIST,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_partial_resist(mut self) -> Self {
        self.inner |= HitInfo::PARTIAL_RESIST;
        self
    }

    pub const fn get_partial_resist(&self) -> bool {
        (self.inner & HitInfo::PARTIAL_RESIST) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_partial_resist(mut self) -> Self {
        self.inner &= HitInfo::PARTIAL_RESIST.reverse_bits();
        self
    }

    pub const fn new_all_resist(all_resist: SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist) -> Self {
        Self {
            inner: HitInfo::ALL_RESIST,
            unk1: None,
            all_absorb: None,
            all_resist: Some(all_resist),
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_all_resist(mut self, all_resist: SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist) -> Self {
        self.inner |= HitInfo::ALL_RESIST;
        self.all_resist = Some(all_resist);
        self
    }

    pub const fn get_all_resist(&self) -> Option<&SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist> {
        self.all_resist.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_all_resist(mut self) -> Self {
        self.inner &= HitInfo::ALL_RESIST.reverse_bits();
        self.all_resist = None;
        self
    }

    pub const fn new_criticalhit() -> Self {
        Self {
            inner: HitInfo::CRITICALHIT,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_criticalhit(mut self) -> Self {
        self.inner |= HitInfo::CRITICALHIT;
        self
    }

    pub const fn get_criticalhit(&self) -> bool {
        (self.inner & HitInfo::CRITICALHIT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_criticalhit(mut self) -> Self {
        self.inner &= HitInfo::CRITICALHIT.reverse_bits();
        self
    }

    pub const fn new_unk10() -> Self {
        Self {
            inner: HitInfo::UNK10,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk10(mut self) -> Self {
        self.inner |= HitInfo::UNK10;
        self
    }

    pub const fn get_unk10(&self) -> bool {
        (self.inner & HitInfo::UNK10) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk10(mut self) -> Self {
        self.inner &= HitInfo::UNK10.reverse_bits();
        self
    }

    pub const fn new_unk11() -> Self {
        Self {
            inner: HitInfo::UNK11,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk11(mut self) -> Self {
        self.inner |= HitInfo::UNK11;
        self
    }

    pub const fn get_unk11(&self) -> bool {
        (self.inner & HitInfo::UNK11) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk11(mut self) -> Self {
        self.inner &= HitInfo::UNK11.reverse_bits();
        self
    }

    pub const fn new_unk12() -> Self {
        Self {
            inner: HitInfo::UNK12,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk12(mut self) -> Self {
        self.inner |= HitInfo::UNK12;
        self
    }

    pub const fn get_unk12(&self) -> bool {
        (self.inner & HitInfo::UNK12) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk12(mut self) -> Self {
        self.inner &= HitInfo::UNK12.reverse_bits();
        self
    }

    pub const fn new_block(block: SMSG_ATTACKERSTATEUPDATE_HitInfo_Block) -> Self {
        Self {
            inner: HitInfo::BLOCK,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: Some(block),
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_block(mut self, block: SMSG_ATTACKERSTATEUPDATE_HitInfo_Block) -> Self {
        self.inner |= HitInfo::BLOCK;
        self.block = Some(block);
        self
    }

    pub const fn get_block(&self) -> Option<&SMSG_ATTACKERSTATEUPDATE_HitInfo_Block> {
        self.block.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_block(mut self) -> Self {
        self.inner &= HitInfo::BLOCK.reverse_bits();
        self.block = None;
        self
    }

    pub const fn new_unk14() -> Self {
        Self {
            inner: HitInfo::UNK14,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk14(mut self) -> Self {
        self.inner |= HitInfo::UNK14;
        self
    }

    pub const fn get_unk14(&self) -> bool {
        (self.inner & HitInfo::UNK14) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk14(mut self) -> Self {
        self.inner &= HitInfo::UNK14.reverse_bits();
        self
    }

    pub const fn new_unk15() -> Self {
        Self {
            inner: HitInfo::UNK15,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk15(mut self) -> Self {
        self.inner |= HitInfo::UNK15;
        self
    }

    pub const fn get_unk15(&self) -> bool {
        (self.inner & HitInfo::UNK15) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk15(mut self) -> Self {
        self.inner &= HitInfo::UNK15.reverse_bits();
        self
    }

    pub const fn new_glancing() -> Self {
        Self {
            inner: HitInfo::GLANCING,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_glancing(mut self) -> Self {
        self.inner |= HitInfo::GLANCING;
        self
    }

    pub const fn get_glancing(&self) -> bool {
        (self.inner & HitInfo::GLANCING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_glancing(mut self) -> Self {
        self.inner &= HitInfo::GLANCING.reverse_bits();
        self
    }

    pub const fn new_crushing() -> Self {
        Self {
            inner: HitInfo::CRUSHING,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_crushing(mut self) -> Self {
        self.inner |= HitInfo::CRUSHING;
        self
    }

    pub const fn get_crushing(&self) -> bool {
        (self.inner & HitInfo::CRUSHING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_crushing(mut self) -> Self {
        self.inner &= HitInfo::CRUSHING.reverse_bits();
        self
    }

    pub const fn new_no_animation() -> Self {
        Self {
            inner: HitInfo::NO_ANIMATION,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_no_animation(mut self) -> Self {
        self.inner |= HitInfo::NO_ANIMATION;
        self
    }

    pub const fn get_no_animation(&self) -> bool {
        (self.inner & HitInfo::NO_ANIMATION) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_no_animation(mut self) -> Self {
        self.inner &= HitInfo::NO_ANIMATION.reverse_bits();
        self
    }

    pub const fn new_unk19(unk19: SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19) -> Self {
        Self {
            inner: HitInfo::UNK19,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: Some(unk19),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk19(mut self, unk19: SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19) -> Self {
        self.inner |= HitInfo::UNK19;
        self.unk19 = Some(unk19);
        self
    }

    pub const fn get_unk19(&self) -> Option<&SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19> {
        self.unk19.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk19(mut self) -> Self {
        self.inner &= HitInfo::UNK19.reverse_bits();
        self.unk19 = None;
        self
    }

    pub const fn new_unk20() -> Self {
        Self {
            inner: HitInfo::UNK20,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk20(mut self) -> Self {
        self.inner |= HitInfo::UNK20;
        self
    }

    pub const fn get_unk20(&self) -> bool {
        (self.inner & HitInfo::UNK20) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk20(mut self) -> Self {
        self.inner &= HitInfo::UNK20.reverse_bits();
        self
    }

    pub const fn new_swingnohitsound() -> Self {
        Self {
            inner: HitInfo::SWINGNOHITSOUND,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_swingnohitsound(mut self) -> Self {
        self.inner |= HitInfo::SWINGNOHITSOUND;
        self
    }

    pub const fn get_swingnohitsound(&self) -> bool {
        (self.inner & HitInfo::SWINGNOHITSOUND) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_swingnohitsound(mut self) -> Self {
        self.inner &= HitInfo::SWINGNOHITSOUND.reverse_bits();
        self
    }

    pub const fn new_unk22() -> Self {
        Self {
            inner: HitInfo::UNK22,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk22(mut self) -> Self {
        self.inner |= HitInfo::UNK22;
        self
    }

    pub const fn get_unk22(&self) -> bool {
        (self.inner & HitInfo::UNK22) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk22(mut self) -> Self {
        self.inner &= HitInfo::UNK22.reverse_bits();
        self
    }

    pub const fn new_rage_gain() -> Self {
        Self {
            inner: HitInfo::RAGE_GAIN,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_rage_gain(mut self) -> Self {
        self.inner |= HitInfo::RAGE_GAIN;
        self
    }

    pub const fn get_rage_gain(&self) -> bool {
        (self.inner & HitInfo::RAGE_GAIN) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_rage_gain(mut self) -> Self {
        self.inner &= HitInfo::RAGE_GAIN.reverse_bits();
        self
    }

    pub const fn new_fake_damage() -> Self {
        Self {
            inner: HitInfo::FAKE_DAMAGE,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_fake_damage(mut self) -> Self {
        self.inner |= HitInfo::FAKE_DAMAGE;
        self
    }

    pub const fn get_fake_damage(&self) -> bool {
        (self.inner & HitInfo::FAKE_DAMAGE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_fake_damage(mut self) -> Self {
        self.inner &= HitInfo::FAKE_DAMAGE.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_ATTACKERSTATEUPDATE_HitInfo {
    pub(crate) const fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.unk1 {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.all_absorb {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.all_resist {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.block {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.unk19 {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1 {
    pub unknown10: f32,
    pub unknown11: f32,
    pub unknown12: f32,
    pub unknown13: f32,
    pub unknown14: f32,
    pub unknown15: u32,
    pub unknown4: u32,
    pub unknown5: f32,
    pub unknown6: f32,
    pub unknown7: f32,
    pub unknown8: f32,
    pub unknown9: f32,
}

impl SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1 {
    pub(crate) const fn size(&self) -> usize {
        4 // unknown10: f32
        + 4 // unknown11: f32
        + 4 // unknown12: f32
        + 4 // unknown13: f32
        + 4 // unknown14: f32
        + 4 // unknown15: u32
        + 4 // unknown4: u32
        + 4 // unknown5: f32
        + 4 // unknown6: f32
        + 4 // unknown7: f32
        + 4 // unknown8: f32
        + 4 // unknown9: f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb {
    pub absorb: u32,
}

impl SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb {
    pub(crate) const fn size(&self) -> usize {
        4 // absorb: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist {
    pub resist: u32,
}

impl SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist {
    pub(crate) const fn size(&self) -> usize {
        4 // resist: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE_HitInfo_Block {
    pub blocked_amount: u32,
}

impl SMSG_ATTACKERSTATEUPDATE_HitInfo_Block {
    pub(crate) const fn size(&self) -> usize {
        4 // blocked_amount: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19 {
    pub unknown3: u32,
}

impl SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19 {
    pub(crate) const fn size(&self) -> usize {
        4 // unknown3: u32
    }
}

