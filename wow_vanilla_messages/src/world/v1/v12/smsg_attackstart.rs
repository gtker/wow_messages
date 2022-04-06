use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackstart.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackstart.wowm#L3):
/// ```text
/// smsg SMSG_ATTACKSTART = 0x143 {
///     u64 attacker_guid;
///     u64 victim_guid;
/// }
/// ```
pub struct SMSG_ATTACKSTART {
    pub attacker_guid: u64,
    pub victim_guid: u64,
}

impl WorldServerMessageWrite for SMSG_ATTACKSTART {
    const OPCODE: u16 = 0x143;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_ATTACKSTART {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // attacker_guid: u64
        let attacker_guid = crate::util::read_u64_le(r)?;

        // victim_guid: u64
        let victim_guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            attacker_guid,
            victim_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // attacker_guid: u64
        w.write_all(&self.attacker_guid.to_le_bytes())?;

        // victim_guid: u64
        w.write_all(&self.victim_guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_ATTACKSTART {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_ATTACKSTART {
    fn maximum_possible_size() -> usize {
        8 // attacker_guid: u64
        + 8 // victim_guid: u64
    }
}

