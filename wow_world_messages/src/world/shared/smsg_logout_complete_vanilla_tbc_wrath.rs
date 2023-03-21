use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Immediately logs out the client of the world and makes it send [`CMSG_CHAR_ENUM`](crate::vanilla::CMSG_CHAR_ENUM).
/// Normally the client will send [`CMSG_LOGOUT_REQUEST`](crate::vanilla::CMSG_LOGOUT_REQUEST) and the server will reply with an [`SMSG_LOGOUT_RESPONSE`](crate::vanilla::SMSG_LOGOUT_RESPONSE) before this message, but sending it unprompted will also immediately send the client to the character screen.
///
/// The client always seems to send 2 [`CMSG_CANCEL_TRADE`](crate::vanilla::CMSG_CANCEL_TRADE) immediately after receiving this mesage, but before sending [`CMSG_CHAR_ENUM`](crate::vanilla::CMSG_CHAR_ENUM).
/// Even if 'Exit Game' is selected the client will still send a [`CMSG_CHAR_ENUM`](crate::vanilla::CMSG_CHAR_ENUM) immediately before closing the connection, despite it not needing to see the character list.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm#L3):
/// ```text
/// smsg SMSG_LOGOUT_COMPLETE = 0x004D {
/// }
/// ```
pub struct SMSG_LOGOUT_COMPLETE {
}

impl crate::Message for SMSG_LOGOUT_COMPLETE {
    const OPCODE: u32 = 0x004d;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x004D, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOGOUT_COMPLETE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOGOUT_COMPLETE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOGOUT_COMPLETE {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    use super::SMSG_LOGOUT_COMPLETE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 4] = [ 0x00, 0x02, 0x4D, 0x00, ];

    pub(crate) fn expected0() -> SMSG_LOGOUT_COMPLETE {
        SMSG_LOGOUT_COMPLETE {
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGOUT_COMPLETE0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE => {}
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_LOGOUT_COMPLETE0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE => {}
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_LOGOUT_COMPLETE0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE => {}
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    use super::SMSG_LOGOUT_COMPLETE;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 4] = [ 0x00, 0x02, 0x4D, 0x00, ];

    pub(crate) fn expected0() -> SMSG_LOGOUT_COMPLETE {
        SMSG_LOGOUT_COMPLETE {
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGOUT_COMPLETE0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE => {}
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_LOGOUT_COMPLETE0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE => {}
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_LOGOUT_COMPLETE0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE => {}
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    use super::SMSG_LOGOUT_COMPLETE;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 4] = [ 0x00, 0x02, 0x4D, 0x00, ];

    pub(crate) fn expected0() -> SMSG_LOGOUT_COMPLETE {
        SMSG_LOGOUT_COMPLETE {
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGOUT_COMPLETE0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE => {}
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_LOGOUT_COMPLETE0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE => {}
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_LOGOUT_COMPLETE0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE => {}
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

