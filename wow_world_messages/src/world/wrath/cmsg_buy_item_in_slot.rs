use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm#L11):
/// ```text
/// cmsg CMSG_BUY_ITEM_IN_SLOT = 0x01A3 {
///     Guid vendor;
///     Item item;
///     u32 vendor_slot;
///     Guid bag;
///     u8 bag_slot;
///     u32 amount;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_BUY_ITEM_IN_SLOT {
    pub vendor: Guid,
    pub item: u32,
    /// arcemu: VLack: 3.1.2 This is the slot's number on the vendor's panel, starts from 1
    pub vendor_slot: u32,
    pub bag: Guid,
    pub bag_slot: u8,
    pub amount: u32,
}

impl crate::private::Sealed for CMSG_BUY_ITEM_IN_SLOT {}
impl CMSG_BUY_ITEM_IN_SLOT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 29 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // vendor: Guid
        let vendor = crate::util::read_guid(&mut r)?;

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // vendor_slot: u32
        let vendor_slot = crate::util::read_u32_le(&mut r)?;

        // bag: Guid
        let bag = crate::util::read_guid(&mut r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(&mut r)?;

        // amount: u32
        let amount = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            vendor,
            item,
            vendor_slot,
            bag,
            bag_slot,
            amount,
        })
    }

}

impl crate::Message for CMSG_BUY_ITEM_IN_SLOT {
    const OPCODE: u32 = 0x01a3;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_BUY_ITEM_IN_SLOT"
    }

    fn size_without_header(&self) -> u32 {
        29
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // vendor_slot: u32
        w.write_all(&self.vendor_slot.to_le_bytes())?;

        // bag: Guid
        w.write_all(&self.bag.guid().to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(419, "CMSG_BUY_ITEM_IN_SLOT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BUY_ITEM_IN_SLOT {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_BUY_ITEM_IN_SLOT;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 35] = [ 0x00, 0x21, 0xA3, 0x01, 0x00, 0x00, 0x64, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0xC8, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, 0x2C, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x03, 0x00,
         0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_BUY_ITEM_IN_SLOT {
        CMSG_BUY_ITEM_IN_SLOT {
            vendor: Guid::new(0x64),
            item: 0xC8,
            vendor_slot: 0x1,
            bag: Guid::new(0x12C),
            bag_slot: 0x7,
            amount: 0x3,
        }

    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm` line 23.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_buy_item_in_slot0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BUY_ITEM_IN_SLOT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BUY_ITEM_IN_SLOT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(29 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm` line 23.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_buy_item_in_slot0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BUY_ITEM_IN_SLOT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BUY_ITEM_IN_SLOT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(29 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_buy_item_in_slot.wowm` line 23.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_buy_item_in_slot0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BUY_ITEM_IN_SLOT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BUY_ITEM_IN_SLOT, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(29 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

