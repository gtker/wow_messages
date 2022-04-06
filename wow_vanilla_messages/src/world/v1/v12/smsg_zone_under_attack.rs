use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm#L3):
/// ```text
/// smsg SMSG_ZONE_UNDER_ATTACK = 0x254 {
///     u32 zone_id;
/// }
/// ```
pub struct SMSG_ZONE_UNDER_ATTACK {
    pub zone_id: u32,
}

impl WorldServerMessageWrite for SMSG_ZONE_UNDER_ATTACK {
    const OPCODE: u16 = 0x254;

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
impl WorldMessageBody for SMSG_ZONE_UNDER_ATTACK {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // zone_id: u32
        let zone_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            zone_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_ZONE_UNDER_ATTACK {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_ZONE_UNDER_ATTACK {
    fn maximum_possible_size() -> usize {
        4 // zone_id: u32
    }
}

