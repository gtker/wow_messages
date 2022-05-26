use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_PING {
    pub sequence_id: u32,
    pub round_time_in_ms: u32,
}

impl ClientMessage for CMSG_PING {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // sequence_id: u32
        w.write_all(&self.sequence_id.to_le_bytes())?;

        // round_time_in_ms: u32
        w.write_all(&self.round_time_in_ms.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01dc;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // sequence_id: u32
        let sequence_id = crate::util::read_u32_le(r)?;

        // round_time_in_ms: u32
        let round_time_in_ms = crate::util::read_u32_le(r)?;

        Ok(Self {
            sequence_id,
            round_time_in_ms,
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMSG_PING;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ClientOpcodeMessage;
    use crate::{ClientMessage, ServerMessage};

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_PING0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0C, 0xDC, 0x01, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_PING0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0C, 0xDC, 0x01, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_PING0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0C, 0xDC, 0x01, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

}
