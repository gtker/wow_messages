use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/5needs_endless_array/cmsg_auth_session.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/5needs_endless_array/cmsg_auth_session.wowm#L31):
/// ```text
/// smsg SMSG_SHOWTAXINODES = 0x1A9 {
///     u32 unknown1;
///     Guid guid;
///     u32 nearest_node;
///     u32[-] nodes;
/// }
/// ```
pub struct SMSG_SHOWTAXINODES {
    pub unknown1: u32,
    pub guid: Guid,
    pub nearest_node: u32,
    pub nodes: Vec<u32>,
}

impl WorldServerMessageWrite for SMSG_SHOWTAXINODES {
    const OPCODE: u16 = 0x1a9;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_SHOWTAXINODES {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        // nearest_node: u32
        let nearest_node = crate::util::read_u32_le(r)?;

        // nodes: u32[-]
        let mut current_size = {
            4 // unknown1: u32
            + 8 // guid: Guid
            + 4 // nearest_node: u32
        };
        let mut nodes = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            nodes.push(crate::util::read_u32_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            unknown1,
            guid,
            nearest_node,
            nodes,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // guid: Guid
        self.guid.write(w)?;

        // nearest_node: u32
        w.write_all(&self.nearest_node.to_le_bytes())?;

        // nodes: u32[-]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_SHOWTAXINODES {
    fn size(&self) -> usize {
        4 // unknown1: u32
        + 8 // guid: Guid
        + 4 // nearest_node: u32
        + self.nodes.len() * core::mem::size_of::<u32>() // nodes: u32[-]
    }
}

impl MaximumPossibleSized for SMSG_SHOWTAXINODES {
    fn maximum_possible_size() -> usize {
        4 // unknown1: u32
        + 8 // guid: Guid
        + 4 // nearest_node: u32
        + 65536 // nodes: u32[-]
    }
}

