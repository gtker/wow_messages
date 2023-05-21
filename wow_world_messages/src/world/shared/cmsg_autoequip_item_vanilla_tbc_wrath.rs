use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autoequip_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autoequip_item.wowm#L1):
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

impl crate::private::Sealed for CMSG_AUTOEQUIP_ITEM {}
impl CMSG_AUTOEQUIP_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x010A, size: body_size });
        }

        // source_bag: u8
        let source_bag = crate::util::read_u8_le(&mut r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            source_bag,
            source_slot,
        })
    }

}

impl crate::Message for CMSG_AUTOEQUIP_ITEM {
    const OPCODE: u32 = 0x010a;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUTOEQUIP_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUTOEQUIP_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUTOEQUIP_ITEM {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_AUTOEQUIP_ITEM;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_AUTOEQUIP_ITEM, expected: &CMSG_AUTOEQUIP_ITEM) {
        assert_eq!(t.source_bag, expected.source_bag);
        assert_eq!(t.source_slot, expected.source_slot);
    }

    const RAW0: [u8; 8] = [ 0x00, 0x06, 0x0A, 0x01, 0x00, 0x00, 0xFF, 0x18, ];

    pub(crate) fn expected0() -> CMSG_AUTOEQUIP_ITEM {
        CMSG_AUTOEQUIP_ITEM {
            source_bag: 0xFF,
            source_slot: 0x18,
        }

    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_autoequip_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTOEQUIP_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_autoequip_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTOEQUIP_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_autoequip_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTOEQUIP_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_AUTOEQUIP_ITEM;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_AUTOEQUIP_ITEM, expected: &CMSG_AUTOEQUIP_ITEM) {
        assert_eq!(t.source_bag, expected.source_bag);
        assert_eq!(t.source_slot, expected.source_slot);
    }

    const RAW0: [u8; 8] = [ 0x00, 0x06, 0x0A, 0x01, 0x00, 0x00, 0xFF, 0x18, ];

    pub(crate) fn expected0() -> CMSG_AUTOEQUIP_ITEM {
        CMSG_AUTOEQUIP_ITEM {
            source_bag: 0xFF,
            source_slot: 0x18,
        }

    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_autoequip_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTOEQUIP_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_autoequip_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTOEQUIP_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_autoequip_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTOEQUIP_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_AUTOEQUIP_ITEM;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_AUTOEQUIP_ITEM, expected: &CMSG_AUTOEQUIP_ITEM) {
        assert_eq!(t.source_bag, expected.source_bag);
        assert_eq!(t.source_slot, expected.source_slot);
    }

    const RAW0: [u8; 8] = [ 0x00, 0x06, 0x0A, 0x01, 0x00, 0x00, 0xFF, 0x18, ];

    pub(crate) fn expected0() -> CMSG_AUTOEQUIP_ITEM {
        CMSG_AUTOEQUIP_ITEM {
            source_bag: 0xFF,
            source_slot: 0x18,
        }

    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_autoequip_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTOEQUIP_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_autoequip_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTOEQUIP_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_autoequip_item0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTOEQUIP_ITEM(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTOEQUIP_ITEM, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(2 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

