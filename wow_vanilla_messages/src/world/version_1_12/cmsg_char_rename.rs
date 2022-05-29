use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_CHAR_RENAME {
    pub character: Guid,
    pub new_name: String,
}

impl ClientMessage for CMSG_CHAR_RENAME {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // character: Guid
        w.write_all(&self.character.guid().to_le_bytes())?;

        // new_name: CString
        w.write_all(self.new_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x02c7;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // character: Guid
        let character = Guid::read(r)?;

        // new_name: CString
        let new_name = crate::util::read_c_string_to_vec(r)?;
        let new_name = String::from_utf8(new_name)?;

        Ok(Self {
            character,
            new_name,
        })
    }

}

impl CMSG_CHAR_RENAME {
    pub(crate) fn size(&self) -> usize {
        8 // character: Guid
        + self.new_name.len() + 1 // new_name: CString
    }
}

#[cfg(test)]
mod test {
    use super::CMSG_CHAR_RENAME;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ClientOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 23] = [ 0x00, 0x15, 0xC7, 0x02, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61, 0x64, 0x62, 0x65, 0x65,
         0x66, 0x00, ];

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
