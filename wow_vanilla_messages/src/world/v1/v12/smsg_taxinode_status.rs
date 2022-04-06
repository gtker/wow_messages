use std::convert::{TryFrom, TryInto};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_taxinode_status.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_taxinode_status.wowm#L3):
/// ```text
/// smsg SMSG_TAXINODE_STATUS = 0x1AB {
///     u64 guid;
///     u8 taxi_mask_node_known;
/// }
/// ```
pub struct SMSG_TAXINODE_STATUS {
    pub guid: u64,
    pub taxi_mask_node_known: u8,
}

impl WorldServerMessageWrite for SMSG_TAXINODE_STATUS {
    const OPCODE: u16 = 0x1ab;

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
impl WorldMessageBody for SMSG_TAXINODE_STATUS {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // taxi_mask_node_known: u8
        let taxi_mask_node_known = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            taxi_mask_node_known,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // taxi_mask_node_known: u8
        w.write_all(&self.taxi_mask_node_known.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_TAXINODE_STATUS {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_TAXINODE_STATUS {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + 1 // taxi_mask_node_known: u8
    }
}

