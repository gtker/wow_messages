use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buy_item.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buy_item.wowm#L11):
/// ```text
/// cmsg CMSG_BUY_ITEM = 0x01A2 {
///     Guid vendor;
///     Item item;
///     u32 slot;
///     u32 amount;
///     u8 unknown1;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_BUY_ITEM {
    pub vendor: Guid,
    pub item: u32,
    pub slot: u32,
    pub amount: u32,
    pub unknown1: u8,
}

impl crate::private::Sealed for CMSG_BUY_ITEM {}
impl CMSG_BUY_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 21 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // vendor: Guid
        let vendor = crate::util::read_guid(&mut r)?;

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // slot: u32
        let slot = crate::util::read_u32_le(&mut r)?;

        // amount: u32
        let amount = crate::util::read_u32_le(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            vendor,
            item,
            slot,
            amount,
            unknown1,
        })
    }

}

impl crate::Message for CMSG_BUY_ITEM {
    const OPCODE: u32 = 0x01a2;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_BUY_ITEM"
    }

    fn size_without_header(&self) -> u32 {
        21
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(418, "CMSG_BUY_ITEM", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BUY_ITEM {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_BUY_ITEM;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 27] = [ 0x00, 0x19, 0xA2, 0x01, 0x00, 0x00, 0x64, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0xC8, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, 0x03, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_BUY_ITEM {
        CMSG_BUY_ITEM {
            vendor: Guid::new(0x64),
            item: 0xC8,
            slot: 0x1,
            amount: 0x3,
            unknown1: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_buy_item.wowm` line 21.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_buy_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BUY_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BUY_ITEM, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(21 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_buy_item.wowm` line 21.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_buy_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BUY_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BUY_ITEM, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(21 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_buy_item.wowm` line 21.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_buy_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BUY_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BUY_ITEM, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(21 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

