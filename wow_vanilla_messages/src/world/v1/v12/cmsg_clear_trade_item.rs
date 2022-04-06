use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/cmsg_clear_trade_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/cmsg_clear_trade_item.wowm#L3):
/// ```text
/// cmsg CMSG_CLEAR_TRADE_ITEM = 0x11E {
///     u8 trade_slot;
/// }
/// ```
pub struct CMSG_CLEAR_TRADE_ITEM {
    pub trade_slot: u8,
}

impl WorldClientMessageWrite for CMSG_CLEAR_TRADE_ITEM {
    const OPCODE: u32 = 0x11e;

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
impl WorldMessageBody for CMSG_CLEAR_TRADE_ITEM {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // trade_slot: u8
        let trade_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            trade_slot,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // trade_slot: u8
        w.write_all(&self.trade_slot.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_CLEAR_TRADE_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_CLEAR_TRADE_ITEM {
    fn maximum_possible_size() -> usize {
        1 // trade_slot: u8
    }
}

