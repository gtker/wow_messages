use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new4.wowm:449`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new4.wowm#L449):
/// ```text
/// cmsg CMSG_SET_FACTION_INACTIVE = 0x317 {
///     u32 reputation_list_id;
///     u8 inactive;
/// }
/// ```
pub struct CMSG_SET_FACTION_INACTIVE {
    pub reputation_list_id: u32,
    pub inactive: u8,
}

impl WorldClientMessageWrite for CMSG_SET_FACTION_INACTIVE {
    const OPCODE: u32 = 0x317;

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
impl WorldMessageBody for CMSG_SET_FACTION_INACTIVE {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // inactive: u8
        let inactive = crate::util::read_u8_le(r)?;

        Ok(Self {
            reputation_list_id,
            inactive,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // inactive: u8
        w.write_all(&self.inactive.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_SET_FACTION_INACTIVE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_SET_FACTION_INACTIVE {
    fn maximum_possible_size() -> usize {
        4 // reputation_list_id: u32
        + 1 // inactive: u8
    }
}

