use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/cmsg_turn_in_petition.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/cmsg_turn_in_petition.wowm#L3):
/// ```text
/// cmsg CMSG_TURN_IN_PETITION = 0x1C4 {
///     u64 petition_guid;
/// }
/// ```
pub struct CMSG_TURN_IN_PETITION {
    pub petition_guid: u64,
}

impl WorldClientMessageWrite for CMSG_TURN_IN_PETITION {
    const OPCODE: u32 = 0x1c4;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_TURN_IN_PETITION {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition_guid: u64
        let petition_guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            petition_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // petition_guid: u64
        w.write_all(&self.petition_guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_TURN_IN_PETITION {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_TURN_IN_PETITION {
    fn maximum_possible_size() -> usize {
        8 // petition_guid: u64
    }
}

