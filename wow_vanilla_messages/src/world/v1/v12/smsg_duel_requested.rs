use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_requested.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_requested.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_REQUESTED = 0x167 {
///     Guid initiator_guid;
///     Guid target_guid;
/// }
/// ```
pub struct SMSG_DUEL_REQUESTED {
    pub initiator_guid: Guid,
    pub target_guid: Guid,
}

impl WorldServerMessageWrite for SMSG_DUEL_REQUESTED {
    const OPCODE: u16 = 0x167;

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
impl WorldMessageBody for SMSG_DUEL_REQUESTED {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // initiator_guid: Guid
        let initiator_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            initiator_guid,
            target_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // initiator_guid: Guid
        self.initiator_guid.write(w)?;

        // target_guid: Guid
        self.target_guid.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_DUEL_REQUESTED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_DUEL_REQUESTED {
    fn maximum_possible_size() -> usize {
        8 // initiator_guid: Guid
        + 8 // target_guid: Guid
    }
}

