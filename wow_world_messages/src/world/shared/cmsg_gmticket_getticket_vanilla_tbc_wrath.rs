use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent when the client enters the world.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm#L1):
/// ```text
/// cmsg CMSG_GMTICKET_GETTICKET = 0x0211 {
/// }
/// ```
pub struct CMSG_GMTICKET_GETTICKET {
}

impl crate::private::Sealed for CMSG_GMTICKET_GETTICKET {}
impl CMSG_GMTICKET_GETTICKET {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        Ok(Self {
        })
    }

}

impl crate::Message for CMSG_GMTICKET_GETTICKET {
    const OPCODE: u32 = 0x0211;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_GMTICKET_GETTICKET"
    }

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(529, "CMSG_GMTICKET_GETTICKET", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GMTICKET_GETTICKET {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GMTICKET_GETTICKET {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GMTICKET_GETTICKET {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_GMTICKET_GETTICKET;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x11, 0x02, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_GMTICKET_GETTICKET {
        CMSG_GMTICKET_GETTICKET {
        }

    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm` line 6.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_gmticket_getticket0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET => {}
            opcode => panic!("incorrect opcode. Expected CMSG_GMTICKET_GETTICKET, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm` line 6.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_gmticket_getticket0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET => {}
            opcode => panic!("incorrect opcode. Expected CMSG_GMTICKET_GETTICKET, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm` line 6.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_gmticket_getticket0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET => {}
            opcode => panic!("incorrect opcode. Expected CMSG_GMTICKET_GETTICKET, got {opcode:#?}"),
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
    use super::CMSG_GMTICKET_GETTICKET;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x11, 0x02, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_GMTICKET_GETTICKET {
        CMSG_GMTICKET_GETTICKET {
        }

    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm` line 6.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_gmticket_getticket0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET => {}
            opcode => panic!("incorrect opcode. Expected CMSG_GMTICKET_GETTICKET, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm` line 6.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_gmticket_getticket0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET => {}
            opcode => panic!("incorrect opcode. Expected CMSG_GMTICKET_GETTICKET, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm` line 6.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_gmticket_getticket0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET => {}
            opcode => panic!("incorrect opcode. Expected CMSG_GMTICKET_GETTICKET, got {opcode:#?}"),
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
    use super::CMSG_GMTICKET_GETTICKET;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x11, 0x02, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_GMTICKET_GETTICKET {
        CMSG_GMTICKET_GETTICKET {
        }

    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm` line 6.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_gmticket_getticket0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET => {}
            opcode => panic!("incorrect opcode. Expected CMSG_GMTICKET_GETTICKET, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm` line 6.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_gmticket_getticket0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET => {}
            opcode => panic!("incorrect opcode. Expected CMSG_GMTICKET_GETTICKET, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_getticket.wowm` line 6.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_gmticket_getticket0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_GMTICKET_GETTICKET => {}
            opcode => panic!("incorrect opcode. Expected CMSG_GMTICKET_GETTICKET, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

