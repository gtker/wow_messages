use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autoequip_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autoequip_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOEQUIP_ITEM = 0x010A {
///     u8 source_bag;
///     u8 source_slot;
/// }
/// ```
pub struct CMSG_AUTOEQUIP_ITEM {
    pub source_bag: u8,
    pub source_slot: u8,
}

impl ClientMessage for CMSG_AUTOEQUIP_ITEM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x010a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        2
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // source_bag: u8
        let source_bag = crate::util::read_u8_le(r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            source_bag,
            source_slot,
        })
    }

}

