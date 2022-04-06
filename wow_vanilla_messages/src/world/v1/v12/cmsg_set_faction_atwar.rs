use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm#L3):
/// ```text
/// cmsg CMSG_SET_FACTION_ATWAR = 0x125 {
///     u32 reputation_list_id;
///     u8 flags;
/// }
/// ```
pub struct CMSG_SET_FACTION_ATWAR {
    pub reputation_list_id: u32,
    pub flags: u8,
}

impl WorldClientMessageWrite for CMSG_SET_FACTION_ATWAR {
    const OPCODE: u32 = 0x125;

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
impl WorldMessageBody for CMSG_SET_FACTION_ATWAR {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // flags: u8
        let flags = crate::util::read_u8_le(r)?;

        Ok(Self {
            reputation_list_id,
            flags,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_SET_FACTION_ATWAR {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_SET_FACTION_ATWAR {
    fn maximum_possible_size() -> usize {
        4 // reputation_list_id: u32
        + 1 // flags: u8
    }
}

