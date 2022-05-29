use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PONG {
    pub sequence_id: u32,
}

impl ServerMessage for SMSG_PONG {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // sequence_id: u32
        w.write_all(&self.sequence_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01dd;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // sequence_id: u32
        let sequence_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            sequence_id,
        })
    }

}

#[cfg(test)]
mod test {
    use super::SMSG_PONG;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_PONG0() {
        let raw: Vec<u8> = vec![ 0x00, 0x06, 0xDD, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, ];

        let expected = SMSG_PONG {
            sequence_id: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);

        assert_eq!(4 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_PONG0() {
        let raw: Vec<u8> = vec![ 0x00, 0x06, 0xDD, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, ];

        let expected = SMSG_PONG {
            sequence_id: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);

        assert_eq!(4 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_PONG0() {
        let raw: Vec<u8> = vec![ 0x00, 0x06, 0xDD, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, ];

        let expected = SMSG_PONG {
            sequence_id: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);

        assert_eq!(4 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

}
