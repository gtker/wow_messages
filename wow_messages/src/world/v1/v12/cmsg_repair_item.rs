use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new4.wowm:237`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new4.wowm#L237):
/// ```text
/// cmsg CMSG_REPAIR_ITEM = 0x2A8 {
///     u64 npc_guid;
///     u64 item_guid;
/// }
/// ```
pub struct CMSG_REPAIR_ITEM {
    pub npc_guid: u64,
    pub item_guid: u64,
}

impl WorldClientMessageWrite for CMSG_REPAIR_ITEM {
    const OPCODE: u32 = 0x2a8;

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
impl WorldMessageBody for CMSG_REPAIR_ITEM {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc_guid: u64
        let npc_guid = crate::util::read_u64_le(r)?;

        // item_guid: u64
        let item_guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            npc_guid,
            item_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc_guid: u64
        w.write_all(&self.npc_guid.to_le_bytes())?;

        // item_guid: u64
        w.write_all(&self.item_guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_REPAIR_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_REPAIR_ITEM {
    fn maximum_possible_size() -> usize {
        8 // npc_guid: u64
        + 8 // item_guid: u64
    }
}

