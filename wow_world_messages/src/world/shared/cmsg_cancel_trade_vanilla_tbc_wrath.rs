use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent twice by the client when teleporting and logging out.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm#L3):
/// ```text
/// cmsg CMSG_CANCEL_TRADE = 0x011C {
/// }
/// ```
pub struct CMSG_CANCEL_TRADE {
}

impl crate::private::Sealed for CMSG_CANCEL_TRADE {}
impl CMSG_CANCEL_TRADE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        Ok(Self {
        })
    }

}

impl crate::Message for CMSG_CANCEL_TRADE {
    const OPCODE: u32 = 0x011c;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_CANCEL_TRADE"
    }

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(284, "CMSG_CANCEL_TRADE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_CANCEL_TRADE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CANCEL_TRADE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CANCEL_TRADE {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_CANCEL_TRADE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x1C, 0x01, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_CANCEL_TRADE {
        CMSG_CANCEL_TRADE {
        }

    }

    // Generated from `wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm` line 7.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_cancel_trade0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::CMSG_CANCEL_TRADE => {}
            opcode => panic!("incorrect opcode. Expected CMSG_CANCEL_TRADE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm` line 7.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_cancel_trade0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_CANCEL_TRADE => {}
            opcode => panic!("incorrect opcode. Expected CMSG_CANCEL_TRADE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm` line 7.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_cancel_trade0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_CANCEL_TRADE => {}
            opcode => panic!("incorrect opcode. Expected CMSG_CANCEL_TRADE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_CANCEL_TRADE;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x1C, 0x01, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_CANCEL_TRADE {
        CMSG_CANCEL_TRADE {
        }

    }

    // Generated from `wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm` line 7.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_cancel_trade0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::CMSG_CANCEL_TRADE => {}
            opcode => panic!("incorrect opcode. Expected CMSG_CANCEL_TRADE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm` line 7.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_cancel_trade0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_CANCEL_TRADE => {}
            opcode => panic!("incorrect opcode. Expected CMSG_CANCEL_TRADE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm` line 7.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_cancel_trade0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_CANCEL_TRADE => {}
            opcode => panic!("incorrect opcode. Expected CMSG_CANCEL_TRADE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_CANCEL_TRADE;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x1C, 0x01, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_CANCEL_TRADE {
        CMSG_CANCEL_TRADE {
        }

    }

    // Generated from `wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm` line 7.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_cancel_trade0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::CMSG_CANCEL_TRADE => {}
            opcode => panic!("incorrect opcode. Expected CMSG_CANCEL_TRADE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm` line 7.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_cancel_trade0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_CANCEL_TRADE => {}
            opcode => panic!("incorrect opcode. Expected CMSG_CANCEL_TRADE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm` line 7.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_cancel_trade0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_CANCEL_TRADE => {}
            opcode => panic!("incorrect opcode. Expected CMSG_CANCEL_TRADE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

