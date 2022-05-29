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
#[derive(Copy)]
pub struct CMSG_PET_NAME_QUERY {
    pub pet_number: u32,
    pub guid: Guid,
}

impl ClientMessage for CMSG_PET_NAME_QUERY {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0052;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_number: u32
        let pet_number = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            pet_number,
            guid,
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMSG_PET_NAME_QUERY;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ClientOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_PET_NAME_QUERY0() {
        let raw: Vec<u8> = vec![ 0x00, 0x10, 0x52, 0x00, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xEF, 0xBE, 0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

        let expected = CMSG_PET_NAME_QUERY {
            pet_number: 0xDEADBEEF,
            guid: Guid::new(0xFACADEDEADBEEF),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.guid, expected.guid);

        assert_eq!(12 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_PET_NAME_QUERY0() {
        let raw: Vec<u8> = vec![ 0x00, 0x10, 0x52, 0x00, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xEF, 0xBE, 0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

        let expected = CMSG_PET_NAME_QUERY {
            pet_number: 0xDEADBEEF,
            guid: Guid::new(0xFACADEDEADBEEF),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.guid, expected.guid);

        assert_eq!(12 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_PET_NAME_QUERY0() {
        let raw: Vec<u8> = vec![ 0x00, 0x10, 0x52, 0x00, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xEF, 0xBE, 0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

        let expected = CMSG_PET_NAME_QUERY {
            pet_number: 0xDEADBEEF,
            guid: Guid::new(0xFACADEDEADBEEF),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.guid, expected.guid);

        assert_eq!(12 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

}
