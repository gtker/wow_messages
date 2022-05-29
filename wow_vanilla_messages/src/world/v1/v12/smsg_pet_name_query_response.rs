use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_PET_NAME_QUERY_RESPONSE {
    pub pet_number: u32,
    pub name: String,
    pub pet_name_timestamp: u32,
}

impl ServerMessage for SMSG_PET_NAME_QUERY_RESPONSE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // pet_name_timestamp: u32
        w.write_all(&self.pet_name_timestamp.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0053;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_number: u32
        let pet_number = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // pet_name_timestamp: u32
        let pet_name_timestamp = crate::util::read_u32_le(r)?;

        Ok(Self {
            pet_number,
            name,
            pet_name_timestamp,
        })
    }

}

impl SMSG_PET_NAME_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        0
        + 4 // pet_number: u32
        + self.name.len() + 1 // name: CString
        + 4 // pet_name_timestamp: u32
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_PET_NAME_QUERY_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 19] = [ 0x00, 0x11, 0x53, 0x00, 0xEF, 0xBE, 0xAD, 0xDE, 0x41,
         0x42, 0x43, 0x44, 0x45, 0x46, 0x00, 0xDE, 0xCA, 0xFA, 0x00, ];

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_PET_NAME_QUERY_RESPONSE0() {
        let expected = SMSG_PET_NAME_QUERY_RESPONSE {
            pet_number: 0xDEADBEEF,
            name: String::from("ABCDEF"),
            pet_name_timestamp: 0xFACADE,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PET_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PET_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.name, expected.name);
        assert_eq!(t.pet_name_timestamp, expected.pet_name_timestamp);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_PET_NAME_QUERY_RESPONSE0() {
        let expected = SMSG_PET_NAME_QUERY_RESPONSE {
            pet_number: 0xDEADBEEF,
            name: String::from("ABCDEF"),
            pet_name_timestamp: 0xFACADE,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PET_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PET_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.name, expected.name);
        assert_eq!(t.pet_name_timestamp, expected.pet_name_timestamp);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_PET_NAME_QUERY_RESPONSE0() {
        let expected = SMSG_PET_NAME_QUERY_RESPONSE {
            pet_number: 0xDEADBEEF,
            name: String::from("ABCDEF"),
            pet_name_timestamp: 0xFACADE,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PET_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PET_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.name, expected.name);
        assert_eq!(t.pet_name_timestamp, expected.pet_name_timestamp);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
