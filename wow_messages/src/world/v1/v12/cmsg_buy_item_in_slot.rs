use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new2.wowm:272`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new2.wowm#L272):
/// ```text
/// cmsg CMSG_BUY_ITEM_IN_SLOT = 0x1A3 {
///     u64 vendor_guid;
///     u32 item_id;
///     u64 bag_guid;
///     u8 bag_slot;
///     u8 amount;
/// }
/// ```
pub struct CMSG_BUY_ITEM_IN_SLOT {
    pub vendor_guid: u64,
    pub item_id: u32,
    pub bag_guid: u64,
    pub bag_slot: u8,
    pub amount: u8,
}

impl WorldClientMessageWrite for CMSG_BUY_ITEM_IN_SLOT {
    const OPCODE: u32 = 0x1a3;

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
impl WorldMessageBody for CMSG_BUY_ITEM_IN_SLOT {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // vendor_guid: u64
        let vendor_guid = crate::util::read_u64_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // bag_guid: u64
        let bag_guid = crate::util::read_u64_le(r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(r)?;

        Ok(Self {
            vendor_guid,
            item_id,
            bag_guid,
            bag_slot,
            amount,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // vendor_guid: u64
        w.write_all(&self.vendor_guid.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // bag_guid: u64
        w.write_all(&self.bag_guid.to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_BUY_ITEM_IN_SLOT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_BUY_ITEM_IN_SLOT {
    fn maximum_possible_size() -> usize {
        8 // vendor_guid: u64
        + 4 // item_id: u32
        + 8 // bag_guid: u64
        + 1 // bag_slot: u8
        + 1 // amount: u8
    }
}

