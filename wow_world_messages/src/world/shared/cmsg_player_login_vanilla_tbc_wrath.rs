use std::io::{Read, Write};

use crate::Guid;

/// Command to log into the specified character.
/// This is sent after the client has been authenticated and served the character list with [`SMSG_CHAR_ENUM`](crate::vanilla::SMSG_CHAR_ENUM).
/// If the player receives a [`SMSG_CHARACTER_LOGIN_FAILED`](crate::vanilla::SMSG_CHARACTER_LOGIN_FAILED) it will return to the character screen and send a [`CMSG_CHAR_ENUM`](crate::vanilla::CMSG_CHAR_ENUM).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_player_login.wowm:6`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_player_login.wowm#L6):
/// ```text
/// cmsg CMSG_PLAYER_LOGIN = 0x003D {
///     Guid guid;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_PLAYER_LOGIN {
    pub guid: Guid,
}

impl crate::private::Sealed for CMSG_PLAYER_LOGIN {}
impl CMSG_PLAYER_LOGIN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}

impl crate::Message for CMSG_PLAYER_LOGIN {
    const OPCODE: u32 = 0x003d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_PLAYER_LOGIN"
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(61, "CMSG_PLAYER_LOGIN", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PLAYER_LOGIN {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PLAYER_LOGIN {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PLAYER_LOGIN {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_PLAYER_LOGIN;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x3D, 0x00, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_PLAYER_LOGIN {
        CMSG_PLAYER_LOGIN {
            guid: Guid::new(0xDEADBEEF),
        }

    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_player_login0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_player_login0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_player_login0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_PLAYER_LOGIN;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x3D, 0x00, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_PLAYER_LOGIN {
        CMSG_PLAYER_LOGIN {
            guid: Guid::new(0xDEADBEEF),
        }

    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_player_login0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_player_login0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_player_login0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_PLAYER_LOGIN;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x3D, 0x00, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_PLAYER_LOGIN {
        CMSG_PLAYER_LOGIN {
            guid: Guid::new(0xDEADBEEF),
        }

    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_player_login0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_player_login0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_player_login0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

