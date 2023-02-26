use crate::Guid;
use crate::vanilla::DamageInfo;
use crate::vanilla::HitInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm:50`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm#L50):
/// ```text
/// smsg SMSG_ATTACKERSTATEUPDATE = 0x014A {
///     HitInfo hit_info;
///     PackedGuid attacker;
///     PackedGuid target;
///     u32 total_damage;
///     u8 amount_of_damages;
///     DamageInfo[amount_of_damages] damages;
///     u32 damage_state;
///     u32 unknown1;
///     u32 spell_id;
///     u32 blocked_amount;
/// }
/// ```
pub struct SMSG_ATTACKERSTATEUPDATE {
    pub hit_info: HitInfo,
    pub attacker: Guid,
    pub target: Guid,
    pub total_damage: u32,
    pub damages: Vec<DamageInfo>,
    pub damage_state: u32,
    pub unknown1: u32,
    /// vmangos: spell id, seen with heroic strike and disarm as examples
    ///
    pub spell_id: u32,
    pub blocked_amount: u32,
}

impl crate::Message for SMSG_ATTACKERSTATEUPDATE {
    const OPCODE: u32 = 0x014a;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // hit_info: HitInfo
        w.write_all(&(self.hit_info.as_int() as u32).to_le_bytes())?;

        // attacker: PackedGuid
        self.attacker.write_packed_guid_into_vec(w)?;

        // target: PackedGuid
        self.target.write_packed_guid_into_vec(w)?;

        // total_damage: u32
        w.write_all(&self.total_damage.to_le_bytes())?;

        // amount_of_damages: u8
        w.write_all(&(self.damages.len() as u8).to_le_bytes())?;

        // damages: DamageInfo[amount_of_damages]
        for i in self.damages.iter() {
            i.write_into_vec(w)?;
        }

        // damage_state: u32
        w.write_all(&self.damage_state.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

        // blocked_amount: u32
        w.write_all(&self.blocked_amount.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(29..=5163).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x014A, size: body_size as u32 });
        }

        // hit_info: HitInfo
        let hit_info: HitInfo = crate::util::read_u32_le(r)?.try_into()?;

        // attacker: PackedGuid
        let attacker = Guid::read_packed(r)?;

        // target: PackedGuid
        let target = Guid::read_packed(r)?;

        // total_damage: u32
        let total_damage = crate::util::read_u32_le(r)?;

        // amount_of_damages: u8
        let amount_of_damages = crate::util::read_u8_le(r)?;

        // damages: DamageInfo[amount_of_damages]
        let damages = {
            let mut damages = Vec::with_capacity(amount_of_damages as usize);
            for i in 0..amount_of_damages {
                damages.push(DamageInfo::read(r)?);
            }
            damages
        };

        // damage_state: u32
        let damage_state = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // spell_id: u32
        let spell_id = crate::util::read_u32_le(r)?;

        // blocked_amount: u32
        let blocked_amount = crate::util::read_u32_le(r)?;

        Ok(Self {
            hit_info,
            attacker,
            target,
            total_damage,
            damages,
            damage_state,
            unknown1,
            spell_id,
            blocked_amount,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ATTACKERSTATEUPDATE {}

impl SMSG_ATTACKERSTATEUPDATE {
    pub(crate) fn size(&self) -> usize {
        4 // hit_info: HitInfo
        + self.attacker.size() // attacker: Guid
        + self.target.size() // target: Guid
        + 4 // total_damage: u32
        + 1 // amount_of_damages: u8
        + self.damages.len() * 20 // damages: DamageInfo[amount_of_damages]
        + 4 // damage_state: u32
        + 4 // unknown1: u32
        + 4 // spell_id: u32
        + 4 // blocked_amount: u32
    }
}

