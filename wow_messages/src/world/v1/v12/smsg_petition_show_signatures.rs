use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new2.wowm:410`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new2.wowm#L410):
/// ```text
/// smsg SMSG_PETITION_SHOW_SIGNATURES = 0x1BF {
///     u64 item_guid;
///     u64 owner_guid;
///     u64 petition_guid;
///     u8 amount_of_signatures;
/// }
/// ```
pub struct SMSG_PETITION_SHOW_SIGNATURES {
    pub item_guid: u64,
    pub owner_guid: u64,
    pub petition_guid: u64,
    pub amount_of_signatures: u8,
}

impl WorldServerMessageWrite for SMSG_PETITION_SHOW_SIGNATURES {
    const OPCODE: u16 = 0x1bf;

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
impl WorldMessageBody for SMSG_PETITION_SHOW_SIGNATURES {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_guid: u64
        let item_guid = crate::util::read_u64_le(r)?;

        // owner_guid: u64
        let owner_guid = crate::util::read_u64_le(r)?;

        // petition_guid: u64
        let petition_guid = crate::util::read_u64_le(r)?;

        // amount_of_signatures: u8
        let amount_of_signatures = crate::util::read_u8_le(r)?;

        Ok(Self {
            item_guid,
            owner_guid,
            petition_guid,
            amount_of_signatures,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_guid: u64
        w.write_all(&self.item_guid.to_le_bytes())?;

        // owner_guid: u64
        w.write_all(&self.owner_guid.to_le_bytes())?;

        // petition_guid: u64
        w.write_all(&self.petition_guid.to_le_bytes())?;

        // amount_of_signatures: u8
        w.write_all(&self.amount_of_signatures.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PETITION_SHOW_SIGNATURES {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PETITION_SHOW_SIGNATURES {
    fn maximum_possible_size() -> usize {
        8 // item_guid: u64
        + 8 // owner_guid: u64
        + 8 // petition_guid: u64
        + 1 // amount_of_signatures: u8
    }
}

