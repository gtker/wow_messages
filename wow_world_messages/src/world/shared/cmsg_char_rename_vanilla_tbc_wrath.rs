use std::io::{Read, Write};

use crate::Guid;

/// Request of new name for character. This is only sent by the client if RENAME is set in the `CharacterFlags` of [`SMSG_CHAR_ENUM`](crate::vanilla::SMSG_CHAR_ENUM) and the client tries to login.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm:4`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm#L4):
/// ```text
/// cmsg CMSG_CHAR_RENAME = 0x02C7 {
///     Guid character;
///     CString new_name;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_CHAR_RENAME {
    pub character: Guid,
    pub new_name: String,
}

impl crate::private::Sealed for CMSG_CHAR_RENAME {}
impl CMSG_CHAR_RENAME {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // character: Guid
        let character = crate::util::read_guid(&mut r)?;

        // new_name: CString
        let new_name = {
            let new_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(new_name)?
        };

        Ok(Self {
            character,
            new_name,
        })
    }

}

impl crate::Message for CMSG_CHAR_RENAME {
    const OPCODE: u32 = 0x02c7;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_CHAR_RENAME"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // character: Guid
        w.write_all(&self.character.guid().to_le_bytes())?;

        // new_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.new_name.as_bytes().iter().next_back(), Some(&0_u8), "String `new_name` must not be null-terminated.");
        w.write_all(self.new_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(711, "CMSG_CHAR_RENAME", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_CHAR_RENAME {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CHAR_RENAME {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CHAR_RENAME {}

impl CMSG_CHAR_RENAME {
    pub(crate) fn size(&self) -> usize {
        8 // character: Guid
        + self.new_name.len() + 1 // new_name: CString
    }
}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_CHAR_RENAME;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 23] = [ 0x00, 0x15, 0xC7, 0x02, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61, 0x64, 0x62, 0x65, 0x65,
         0x66, 0x00, ];

    pub(crate) fn expected0() -> CMSG_CHAR_RENAME {
        CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        }

    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_char_rename0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_char_rename0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_char_rename0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_CHAR_RENAME;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 23] = [ 0x00, 0x15, 0xC7, 0x02, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61, 0x64, 0x62, 0x65, 0x65,
         0x66, 0x00, ];

    pub(crate) fn expected0() -> CMSG_CHAR_RENAME {
        CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        }

    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_char_rename0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_char_rename0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_char_rename0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_CHAR_RENAME;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 23] = [ 0x00, 0x15, 0xC7, 0x02, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61, 0x64, 0x62, 0x65, 0x65,
         0x66, 0x00, ];

    pub(crate) fn expected0() -> CMSG_CHAR_RENAME {
        CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        }

    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_char_rename0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_char_rename0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_char_rename0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

