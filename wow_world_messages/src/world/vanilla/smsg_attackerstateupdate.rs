use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    DamageInfo, HitInfo,
};

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
    pub spell_id: u32,
    pub blocked_amount: u32,
}

impl crate::private::Sealed for SMSG_ATTACKERSTATEUPDATE {}
impl SMSG_ATTACKERSTATEUPDATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(27..=5163).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // hit_info: HitInfo
        let hit_info = crate::util::read_u32_le(&mut r)?.try_into()?;

        // attacker: PackedGuid
        let attacker = crate::util::read_packed_guid(&mut r)?;

        // target: PackedGuid
        let target = crate::util::read_packed_guid(&mut r)?;

        // total_damage: u32
        let total_damage = crate::util::read_u32_le(&mut r)?;

        // amount_of_damages: u8
        let amount_of_damages = crate::util::read_u8_le(&mut r)?;

        // damages: DamageInfo[amount_of_damages]
        let damages = {
            let mut damages = Vec::with_capacity(amount_of_damages as usize);
            for _ in 0..amount_of_damages {
                damages.push(DamageInfo::read(&mut r)?);
            }
            damages
        };

        // damage_state: u32
        let damage_state = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // spell_id: u32
        let spell_id = crate::util::read_u32_le(&mut r)?;

        // blocked_amount: u32
        let blocked_amount = crate::util::read_u32_le(&mut r)?;

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

impl crate::Message for SMSG_ATTACKERSTATEUPDATE {
    const OPCODE: u32 = 0x014a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ATTACKERSTATEUPDATE"
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

        // amount_of_damages: u8
        w.write_all(&(self.damages.len() as u8).to_le_bytes())?;

        // damages: DamageInfo[amount_of_damages]
        for i in self.damages.iter() {
            i.write_into_vec(&mut w)?;
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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(330, "SMSG_ATTACKERSTATEUPDATE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ATTACKERSTATEUPDATE {}

impl SMSG_ATTACKERSTATEUPDATE {
    pub(crate) fn size(&self) -> usize {
        4 // hit_info: HitInfo
        + crate::util::packed_guid_size(&self.attacker) // attacker: PackedGuid
        + crate::util::packed_guid_size(&self.target) // target: PackedGuid
        + 4 // total_damage: u32
        + 1 // amount_of_damages: u8
        + self.damages.len() * 20 // damages: DamageInfo[amount_of_damages]
        + 4 // damage_state: u32
        + 4 // unknown1: u32
        + 4 // spell_id: u32
        + 4 // blocked_amount: u32
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_ATTACKERSTATEUPDATE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_ATTACKERSTATEUPDATE, expected: &SMSG_ATTACKERSTATEUPDATE) {
        assert_eq!(t.hit_info, expected.hit_info);
        assert_eq!(t.attacker, expected.attacker);
        assert_eq!(t.target, expected.target);
        assert_eq!(t.total_damage, expected.total_damage);
        assert_eq!(t.damages, expected.damages);
        assert_eq!(t.damage_state, expected.damage_state);
        assert_eq!(t.unknown1, expected.unknown1);
        assert_eq!(t.spell_id, expected.spell_id);
        assert_eq!(t.blocked_amount, expected.blocked_amount);
    }

    const RAW0: [u8; 53] = [ 0x00, 0x33, 0x4A, 0x01, 0x80, 0x00, 0x00, 0x00, 0x01,
         0x17, 0x01, 0x64, 0x39, 0x05, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x80, 0xA6, 0x44, 0x34, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ATTACKERSTATEUPDATE {
        SMSG_ATTACKERSTATEUPDATE {
            hit_info: HitInfo::CriticalHit,
            attacker: Guid::new(0x17),
            target: Guid::new(0x64),
            total_damage: 0x539,
            damages: vec![
                DamageInfo {
                    spell_school_mask: 0x0,
                    damage_float: 1332_f32,
                    damage_uint: 0x534,
                    absorb: 0x0,
                    resist: 0x0,
                },
            ],
            damage_state: 0x0,
            unknown1: 0x0,
            spell_id: 0x0,
            blocked_amount: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 68.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_attackerstateupdate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 68.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_attackerstateupdate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 68.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_attackerstateupdate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

