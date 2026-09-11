use std::io::{Read, Write};

use crate::wrath::ItemSlot;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm#L15):
/// ```text
/// cmsg CMSG_SWAP_INV_ITEM = 0x010D {
///     ItemSlot destination_slot;
///     ItemSlot source_slot;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_SWAP_INV_ITEM {
    pub destination_slot: ItemSlot,
    pub source_slot: ItemSlot,
}

impl crate::private::Sealed for CMSG_SWAP_INV_ITEM {}
impl CMSG_SWAP_INV_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 2 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // destination_slot: ItemSlot
        let destination_slot = crate::util::read_u8_le(&mut r)?.try_into()?;

        // source_slot: ItemSlot
        let source_slot = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            destination_slot,
            source_slot,
        })
    }

}

impl crate::Message for CMSG_SWAP_INV_ITEM {
    const OPCODE: u32 = 0x010d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SWAP_INV_ITEM"
    }

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // destination_slot: ItemSlot
        w.write_all(&(self.destination_slot.as_int().to_le_bytes()))?;

        // source_slot: ItemSlot
        w.write_all(&(self.source_slot.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(269, "CMSG_SWAP_INV_ITEM", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SWAP_INV_ITEM {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_SWAP_INV_ITEM;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 8] = [ 0x00, 0x06, 0x0D, 0x01, 0x00, 0x00, 0x18, 0x17, ];

    pub(crate) fn expected0() -> CMSG_SWAP_INV_ITEM {
        CMSG_SWAP_INV_ITEM {
            destination_slot: ItemSlot::Inventory1,
            source_slot: ItemSlot::Inventory0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm` line 436.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_swap_inv_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SWAP_INV_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SWAP_INV_ITEM, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm` line 436.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_swap_inv_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SWAP_INV_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SWAP_INV_ITEM, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_swap_inv_item.wowm` line 436.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_swap_inv_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SWAP_INV_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SWAP_INV_ITEM, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

