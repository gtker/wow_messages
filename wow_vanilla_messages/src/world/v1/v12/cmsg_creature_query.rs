use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_creature_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_creature_query.wowm#L3):
/// ```text
/// cmsg CMSG_CREATURE_QUERY = 0x60 {
///     u32 creature;
///     u64 guid;
/// }
/// ```
pub struct CMSG_CREATURE_QUERY {
    pub creature: u32,
    pub guid: u64,
}

impl WorldClientMessageWrite for CMSG_CREATURE_QUERY {
    const OPCODE: u32 = 0x60;

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
impl WorldMessageBody for CMSG_CREATURE_QUERY {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // creature: u32
        let creature = crate::util::read_u32_le(r)?;

        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            creature,
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // creature: u32
        w.write_all(&self.creature.to_le_bytes())?;

        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_CREATURE_QUERY {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_CREATURE_QUERY {
    fn maximum_possible_size() -> usize {
        4 // creature: u32
        + 8 // guid: u64
    }
}

