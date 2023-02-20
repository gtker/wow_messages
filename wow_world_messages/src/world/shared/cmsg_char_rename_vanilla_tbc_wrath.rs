use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Request of new name for character. This is only sent by the client if RENAME is set in the `CharacterFlags` of [`SMSG_CHAR_ENUM`](crate::vanilla::SMSG_CHAR_ENUM) and the client tries to login.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm#L3):
/// ```text
/// cmsg CMSG_CHAR_RENAME = 0x02C7 {
///     Guid character;
///     CString new_name;
/// }
/// ```
pub struct CMSG_CHAR_RENAME {
    pub character: Guid,
    pub new_name: String,
}

impl crate::Message for CMSG_CHAR_RENAME {
    const OPCODE: u32 = 0x02c7;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // character: Guid
        w.write_all(&self.character.guid().to_le_bytes())?;

        // new_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.new_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `new_name` must not be null-terminated.");
        w.write_all(self.new_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02C7, size: body_size as u32 });
        }

        // character: Guid
        let character = Guid::read(r)?;

        // new_name: CString
        let new_name = {
            let new_name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(new_name)?
        };

        Ok(Self {
            character,
            new_name,
        })
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
    use super::CMSG_CHAR_RENAME;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 23] = [ 0x00, 0x15, 0xC7, 0x02, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61, 0x64, 0x62, 0x65, 0x65,
         0x66, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_CHAR_RENAME0() {
        let expected = CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.character, expected.character);
        assert_eq!(t.new_name, expected.new_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_CHAR_RENAME0() {
        let expected = CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.character, expected.character);
        assert_eq!(t.new_name, expected.new_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_CHAR_RENAME0() {
        let expected = CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.character, expected.character);
        assert_eq!(t.new_name, expected.new_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    use super::CMSG_CHAR_RENAME;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const RAW0: [u8; 23] = [ 0x00, 0x15, 0xC7, 0x02, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61, 0x64, 0x62, 0x65, 0x65,
         0x66, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_CHAR_RENAME0() {
        let expected = CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.character, expected.character);
        assert_eq!(t.new_name, expected.new_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_CHAR_RENAME0() {
        let expected = CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.character, expected.character);
        assert_eq!(t.new_name, expected.new_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_CHAR_RENAME0() {
        let expected = CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.character, expected.character);
        assert_eq!(t.new_name, expected.new_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    use super::CMSG_CHAR_RENAME;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const RAW0: [u8; 23] = [ 0x00, 0x15, 0xC7, 0x02, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61, 0x64, 0x62, 0x65, 0x65,
         0x66, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_CHAR_RENAME0() {
        let expected = CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.character, expected.character);
        assert_eq!(t.new_name, expected.new_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_CHAR_RENAME0() {
        let expected = CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.character, expected.character);
        assert_eq!(t.new_name, expected.new_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_CHAR_RENAME0() {
        let expected = CMSG_CHAR_RENAME {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.character, expected.character);
        assert_eq!(t.new_name, expected.new_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

