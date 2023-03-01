use crate:: {
    Guid,
};
use crate::wrath::DamageInfo;
use crate::wrath::HitInfo;
use crate::wrath::VictimState;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm:76`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm#L76):
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
pub struct SMSG_ATTACKERSTATEUPDATE {
    pub hit_info: SMSG_ATTACKERSTATEUPDATE_HitInfo,
    pub attacker: Guid,
    pub target: Guid,
    pub total_damage: u32,
    pub overkill: u32,
    pub damage_infos: Vec<DamageInfo>,
    pub victim_state: VictimState,
    /// arcemu: can be 0,1000 or -1
    ///
    pub unknown1: u32,
    pub unknown2: u32,
}

impl crate::Message for SMSG_ATTACKERSTATEUPDATE {
    const OPCODE: u32 = 0x014a;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // hit_info: HitInfo
        w.write_all(&u32::from(self.hit_info.as_int()).to_le_bytes())?;

        // attacker: PackedGuid
        self.attacker.write_packed_guid_into_vec(&mut w)?;

        // target: PackedGuid
        self.target.write_packed_guid_into_vec(&mut w)?;

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
        w.write_all(&u8::from(self.victim_state.as_int()).to_le_bytes())?;

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
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(26..=3176).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x014A, size: body_size as u32 });
        }

        // hit_info: HitInfo
        let hit_info = HitInfo::new(crate::util::read_u32_le(&mut r)?);

        // attacker: PackedGuid
        let attacker = Guid::read_packed(&mut r)?;

        // target: PackedGuid
        let target = Guid::read_packed(&mut r)?;

        // total_damage: u32
        let total_damage = crate::util::read_u32_le(&mut r)?;

        // overkill: u32
        let overkill = crate::util::read_u32_le(&mut r)?;

        // amount_of_damages: u8
        let amount_of_damages = crate::util::read_u8_le(&mut r)?;

        // damage_infos: DamageInfo[amount_of_damages]
        let damage_infos = {
            let mut damage_infos = Vec::with_capacity(amount_of_damages as usize);
            for i in 0..amount_of_damages {
                damage_infos.push(DamageInfo::read(&mut r)?);
            }
            damage_infos
        };

        let hit_info_ALL_ABSORB = if hit_info.is_ALL_ABSORB() {
            // absorb: u32
            let absorb = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb {
                absorb,
            })
        }
        else {
            None
        };

        let hit_info_ALL_RESIST = if hit_info.is_ALL_RESIST() {
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

        let hit_info_BLOCK = if hit_info.is_BLOCK() {
            // blocked_amount: u32
            let blocked_amount = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_ATTACKERSTATEUPDATE_HitInfo_Block {
                blocked_amount,
            })
        }
        else {
            None
        };

        let hit_info_UNK19 = if hit_info.is_UNK19() {
            // unknown3: u32
            let unknown3 = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19 {
                unknown3,
            })
        }
        else {
            None
        };

        let hit_info_UNK1 = if hit_info.is_UNK1() {
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
            unk1: hit_info_UNK1,
            all_absorb: hit_info_ALL_ABSORB,
            all_resist: hit_info_ALL_RESIST,
            block: hit_info_BLOCK,
            unk19: hit_info_UNK19,
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ATTACKERSTATEUPDATE {}

impl SMSG_ATTACKERSTATEUPDATE {
    pub(crate) fn size(&self) -> usize {
        self.hit_info.size() // hit_info: SMSG_ATTACKERSTATEUPDATE_HitInfo
        + self.attacker.size() // attacker: Guid
        + self.target.size() // target: Guid
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

    pub const fn new_UNK1(unk1: SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1) -> Self {
        Self {
            inner: HitInfo::UNK1,
            unk1: Some(unk1),
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_UNK1(mut self, unk1: SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1) -> Self {
        self.inner |= HitInfo::UNK1;
        self.unk1 = Some(unk1);
        self
    }

    pub const fn get_UNK1(&self) -> Option<&SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1> {
        self.unk1.as_ref()
    }

    pub fn clear_UNK1(mut self) -> Self {
        self.inner &= HitInfo::UNK1.reverse_bits();
        self.unk1 = None;
        self
    }

    pub const fn new_AFFECTS_VICTIM() -> Self {
        Self {
            inner: HitInfo::AFFECTS_VICTIM,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_AFFECTS_VICTIM(mut self) -> Self {
        self.inner |= HitInfo::AFFECTS_VICTIM;
        self
    }

    pub const fn get_AFFECTS_VICTIM(&self) -> bool {
        (self.inner & HitInfo::AFFECTS_VICTIM) != 0
    }

    pub fn clear_AFFECTS_VICTIM(mut self) -> Self {
        self.inner &= HitInfo::AFFECTS_VICTIM.reverse_bits();
        self
    }

    pub const fn new_OFFHAND() -> Self {
        Self {
            inner: HitInfo::OFFHAND,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_OFFHAND(mut self) -> Self {
        self.inner |= HitInfo::OFFHAND;
        self
    }

    pub const fn get_OFFHAND(&self) -> bool {
        (self.inner & HitInfo::OFFHAND) != 0
    }

    pub fn clear_OFFHAND(mut self) -> Self {
        self.inner &= HitInfo::OFFHAND.reverse_bits();
        self
    }

    pub const fn new_UNK2() -> Self {
        Self {
            inner: HitInfo::UNK2,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_UNK2(mut self) -> Self {
        self.inner |= HitInfo::UNK2;
        self
    }

    pub const fn get_UNK2(&self) -> bool {
        (self.inner & HitInfo::UNK2) != 0
    }

    pub fn clear_UNK2(mut self) -> Self {
        self.inner &= HitInfo::UNK2.reverse_bits();
        self
    }

    pub const fn new_MISS() -> Self {
        Self {
            inner: HitInfo::MISS,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_MISS(mut self) -> Self {
        self.inner |= HitInfo::MISS;
        self
    }

    pub const fn get_MISS(&self) -> bool {
        (self.inner & HitInfo::MISS) != 0
    }

    pub fn clear_MISS(mut self) -> Self {
        self.inner &= HitInfo::MISS.reverse_bits();
        self
    }

    pub const fn new_FULL_ABSORB() -> Self {
        Self {
            inner: HitInfo::FULL_ABSORB,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_FULL_ABSORB(mut self) -> Self {
        self.inner |= HitInfo::FULL_ABSORB;
        self
    }

    pub const fn get_FULL_ABSORB(&self) -> bool {
        (self.inner & HitInfo::FULL_ABSORB) != 0
    }

    pub fn clear_FULL_ABSORB(mut self) -> Self {
        self.inner &= HitInfo::FULL_ABSORB.reverse_bits();
        self
    }

    pub const fn new_PARTIAL_ABSORB() -> Self {
        Self {
            inner: HitInfo::PARTIAL_ABSORB,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_PARTIAL_ABSORB(mut self) -> Self {
        self.inner |= HitInfo::PARTIAL_ABSORB;
        self
    }

    pub const fn get_PARTIAL_ABSORB(&self) -> bool {
        (self.inner & HitInfo::PARTIAL_ABSORB) != 0
    }

    pub fn clear_PARTIAL_ABSORB(mut self) -> Self {
        self.inner &= HitInfo::PARTIAL_ABSORB.reverse_bits();
        self
    }

    pub const fn new_ALL_ABSORB(all_absorb: SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb) -> Self {
        Self {
            inner: HitInfo::ALL_ABSORB,
            unk1: None,
            all_absorb: Some(all_absorb),
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_ALL_ABSORB(mut self, all_absorb: SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb) -> Self {
        self.inner |= HitInfo::ALL_ABSORB;
        self.all_absorb = Some(all_absorb);
        self
    }

    pub const fn get_ALL_ABSORB(&self) -> Option<&SMSG_ATTACKERSTATEUPDATE_HitInfo_AllAbsorb> {
        self.all_absorb.as_ref()
    }

    pub fn clear_ALL_ABSORB(mut self) -> Self {
        self.inner &= HitInfo::ALL_ABSORB.reverse_bits();
        self.all_absorb = None;
        self
    }

    pub const fn new_FULL_RESIST() -> Self {
        Self {
            inner: HitInfo::FULL_RESIST,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_FULL_RESIST(mut self) -> Self {
        self.inner |= HitInfo::FULL_RESIST;
        self
    }

    pub const fn get_FULL_RESIST(&self) -> bool {
        (self.inner & HitInfo::FULL_RESIST) != 0
    }

    pub fn clear_FULL_RESIST(mut self) -> Self {
        self.inner &= HitInfo::FULL_RESIST.reverse_bits();
        self
    }

    pub const fn new_PARTIAL_RESIST() -> Self {
        Self {
            inner: HitInfo::PARTIAL_RESIST,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_PARTIAL_RESIST(mut self) -> Self {
        self.inner |= HitInfo::PARTIAL_RESIST;
        self
    }

    pub const fn get_PARTIAL_RESIST(&self) -> bool {
        (self.inner & HitInfo::PARTIAL_RESIST) != 0
    }

    pub fn clear_PARTIAL_RESIST(mut self) -> Self {
        self.inner &= HitInfo::PARTIAL_RESIST.reverse_bits();
        self
    }

    pub const fn new_ALL_RESIST(all_resist: SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist) -> Self {
        Self {
            inner: HitInfo::ALL_RESIST,
            unk1: None,
            all_absorb: None,
            all_resist: Some(all_resist),
            block: None,
            unk19: None,
        }
    }

    pub fn set_ALL_RESIST(mut self, all_resist: SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist) -> Self {
        self.inner |= HitInfo::ALL_RESIST;
        self.all_resist = Some(all_resist);
        self
    }

    pub const fn get_ALL_RESIST(&self) -> Option<&SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist> {
        self.all_resist.as_ref()
    }

    pub fn clear_ALL_RESIST(mut self) -> Self {
        self.inner &= HitInfo::ALL_RESIST.reverse_bits();
        self.all_resist = None;
        self
    }

    pub const fn new_CRITICALHIT() -> Self {
        Self {
            inner: HitInfo::CRITICALHIT,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_CRITICALHIT(mut self) -> Self {
        self.inner |= HitInfo::CRITICALHIT;
        self
    }

    pub const fn get_CRITICALHIT(&self) -> bool {
        (self.inner & HitInfo::CRITICALHIT) != 0
    }

    pub fn clear_CRITICALHIT(mut self) -> Self {
        self.inner &= HitInfo::CRITICALHIT.reverse_bits();
        self
    }

    pub const fn new_UNK10() -> Self {
        Self {
            inner: HitInfo::UNK10,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_UNK10(mut self) -> Self {
        self.inner |= HitInfo::UNK10;
        self
    }

    pub const fn get_UNK10(&self) -> bool {
        (self.inner & HitInfo::UNK10) != 0
    }

    pub fn clear_UNK10(mut self) -> Self {
        self.inner &= HitInfo::UNK10.reverse_bits();
        self
    }

    pub const fn new_UNK11() -> Self {
        Self {
            inner: HitInfo::UNK11,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_UNK11(mut self) -> Self {
        self.inner |= HitInfo::UNK11;
        self
    }

    pub const fn get_UNK11(&self) -> bool {
        (self.inner & HitInfo::UNK11) != 0
    }

    pub fn clear_UNK11(mut self) -> Self {
        self.inner &= HitInfo::UNK11.reverse_bits();
        self
    }

    pub const fn new_UNK12() -> Self {
        Self {
            inner: HitInfo::UNK12,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_UNK12(mut self) -> Self {
        self.inner |= HitInfo::UNK12;
        self
    }

    pub const fn get_UNK12(&self) -> bool {
        (self.inner & HitInfo::UNK12) != 0
    }

    pub fn clear_UNK12(mut self) -> Self {
        self.inner &= HitInfo::UNK12.reverse_bits();
        self
    }

    pub const fn new_BLOCK(block: SMSG_ATTACKERSTATEUPDATE_HitInfo_Block) -> Self {
        Self {
            inner: HitInfo::BLOCK,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: Some(block),
            unk19: None,
        }
    }

    pub fn set_BLOCK(mut self, block: SMSG_ATTACKERSTATEUPDATE_HitInfo_Block) -> Self {
        self.inner |= HitInfo::BLOCK;
        self.block = Some(block);
        self
    }

    pub const fn get_BLOCK(&self) -> Option<&SMSG_ATTACKERSTATEUPDATE_HitInfo_Block> {
        self.block.as_ref()
    }

    pub fn clear_BLOCK(mut self) -> Self {
        self.inner &= HitInfo::BLOCK.reverse_bits();
        self.block = None;
        self
    }

    pub const fn new_UNK14() -> Self {
        Self {
            inner: HitInfo::UNK14,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_UNK14(mut self) -> Self {
        self.inner |= HitInfo::UNK14;
        self
    }

    pub const fn get_UNK14(&self) -> bool {
        (self.inner & HitInfo::UNK14) != 0
    }

    pub fn clear_UNK14(mut self) -> Self {
        self.inner &= HitInfo::UNK14.reverse_bits();
        self
    }

    pub const fn new_UNK15() -> Self {
        Self {
            inner: HitInfo::UNK15,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_UNK15(mut self) -> Self {
        self.inner |= HitInfo::UNK15;
        self
    }

    pub const fn get_UNK15(&self) -> bool {
        (self.inner & HitInfo::UNK15) != 0
    }

    pub fn clear_UNK15(mut self) -> Self {
        self.inner &= HitInfo::UNK15.reverse_bits();
        self
    }

    pub const fn new_GLANCING() -> Self {
        Self {
            inner: HitInfo::GLANCING,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_GLANCING(mut self) -> Self {
        self.inner |= HitInfo::GLANCING;
        self
    }

    pub const fn get_GLANCING(&self) -> bool {
        (self.inner & HitInfo::GLANCING) != 0
    }

    pub fn clear_GLANCING(mut self) -> Self {
        self.inner &= HitInfo::GLANCING.reverse_bits();
        self
    }

    pub const fn new_CRUSHING() -> Self {
        Self {
            inner: HitInfo::CRUSHING,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_CRUSHING(mut self) -> Self {
        self.inner |= HitInfo::CRUSHING;
        self
    }

    pub const fn get_CRUSHING(&self) -> bool {
        (self.inner & HitInfo::CRUSHING) != 0
    }

    pub fn clear_CRUSHING(mut self) -> Self {
        self.inner &= HitInfo::CRUSHING.reverse_bits();
        self
    }

    pub const fn new_NO_ANIMATION() -> Self {
        Self {
            inner: HitInfo::NO_ANIMATION,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_NO_ANIMATION(mut self) -> Self {
        self.inner |= HitInfo::NO_ANIMATION;
        self
    }

    pub const fn get_NO_ANIMATION(&self) -> bool {
        (self.inner & HitInfo::NO_ANIMATION) != 0
    }

    pub fn clear_NO_ANIMATION(mut self) -> Self {
        self.inner &= HitInfo::NO_ANIMATION.reverse_bits();
        self
    }

    pub const fn new_UNK19(unk19: SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19) -> Self {
        Self {
            inner: HitInfo::UNK19,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: Some(unk19),
        }
    }

    pub fn set_UNK19(mut self, unk19: SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19) -> Self {
        self.inner |= HitInfo::UNK19;
        self.unk19 = Some(unk19);
        self
    }

    pub const fn get_UNK19(&self) -> Option<&SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19> {
        self.unk19.as_ref()
    }

    pub fn clear_UNK19(mut self) -> Self {
        self.inner &= HitInfo::UNK19.reverse_bits();
        self.unk19 = None;
        self
    }

    pub const fn new_UNK20() -> Self {
        Self {
            inner: HitInfo::UNK20,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_UNK20(mut self) -> Self {
        self.inner |= HitInfo::UNK20;
        self
    }

    pub const fn get_UNK20(&self) -> bool {
        (self.inner & HitInfo::UNK20) != 0
    }

    pub fn clear_UNK20(mut self) -> Self {
        self.inner &= HitInfo::UNK20.reverse_bits();
        self
    }

    pub const fn new_SWINGNOHITSOUND() -> Self {
        Self {
            inner: HitInfo::SWINGNOHITSOUND,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_SWINGNOHITSOUND(mut self) -> Self {
        self.inner |= HitInfo::SWINGNOHITSOUND;
        self
    }

    pub const fn get_SWINGNOHITSOUND(&self) -> bool {
        (self.inner & HitInfo::SWINGNOHITSOUND) != 0
    }

    pub fn clear_SWINGNOHITSOUND(mut self) -> Self {
        self.inner &= HitInfo::SWINGNOHITSOUND.reverse_bits();
        self
    }

    pub const fn new_UNK22() -> Self {
        Self {
            inner: HitInfo::UNK22,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_UNK22(mut self) -> Self {
        self.inner |= HitInfo::UNK22;
        self
    }

    pub const fn get_UNK22(&self) -> bool {
        (self.inner & HitInfo::UNK22) != 0
    }

    pub fn clear_UNK22(mut self) -> Self {
        self.inner &= HitInfo::UNK22.reverse_bits();
        self
    }

    pub const fn new_RAGE_GAIN() -> Self {
        Self {
            inner: HitInfo::RAGE_GAIN,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_RAGE_GAIN(mut self) -> Self {
        self.inner |= HitInfo::RAGE_GAIN;
        self
    }

    pub const fn get_RAGE_GAIN(&self) -> bool {
        (self.inner & HitInfo::RAGE_GAIN) != 0
    }

    pub fn clear_RAGE_GAIN(mut self) -> Self {
        self.inner &= HitInfo::RAGE_GAIN.reverse_bits();
        self
    }

    pub const fn new_FAKE_DAMAGE() -> Self {
        Self {
            inner: HitInfo::FAKE_DAMAGE,
            unk1: None,
            all_absorb: None,
            all_resist: None,
            block: None,
            unk19: None,
        }
    }

    pub fn set_FAKE_DAMAGE(mut self) -> Self {
        self.inner |= HitInfo::FAKE_DAMAGE;
        self
    }

    pub const fn get_FAKE_DAMAGE(&self) -> bool {
        (self.inner & HitInfo::FAKE_DAMAGE) != 0
    }

    pub fn clear_FAKE_DAMAGE(mut self) -> Self {
        self.inner &= HitInfo::FAKE_DAMAGE.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_ATTACKERSTATEUPDATE_HitInfo {
    pub(crate) fn size(&self) -> usize {
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
    pub(crate) fn size(&self) -> usize {
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
    pub(crate) fn size(&self) -> usize {
        4 // absorb: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist {
    pub resist: u32,
}

impl SMSG_ATTACKERSTATEUPDATE_HitInfo_AllResist {
    pub(crate) fn size(&self) -> usize {
        4 // resist: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE_HitInfo_Block {
    pub blocked_amount: u32,
}

impl SMSG_ATTACKERSTATEUPDATE_HitInfo_Block {
    pub(crate) fn size(&self) -> usize {
        4 // blocked_amount: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19 {
    pub unknown3: u32,
}

impl SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk19 {
    pub(crate) fn size(&self) -> usize {
        4 // unknown3: u32
    }
}

