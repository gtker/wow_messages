use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new2.wowm:459`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new2.wowm#L459):
/// ```text
/// cmsg CMSG_PETITION_QUERY = 0x1C6 {
///     u32 guild_guid;
///     u64 petition_guid;
/// }
/// ```
pub struct CMSG_PETITION_QUERY {
    pub guild_guid: u32,
    pub petition_guid: u64,
}

impl WorldClientMessageWrite for CMSG_PETITION_QUERY {
    const OPCODE: u32 = 0x1c6;

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
impl WorldMessageBody for CMSG_PETITION_QUERY {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guild_guid: u32
        let guild_guid = crate::util::read_u32_le(r)?;

        // petition_guid: u64
        let petition_guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            guild_guid,
            petition_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guild_guid: u32
        w.write_all(&self.guild_guid.to_le_bytes())?;

        // petition_guid: u64
        w.write_all(&self.petition_guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_PETITION_QUERY {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_PETITION_QUERY {
    fn maximum_possible_size() -> usize {
        4 // guild_guid: u32
        + 8 // petition_guid: u64
    }
}

