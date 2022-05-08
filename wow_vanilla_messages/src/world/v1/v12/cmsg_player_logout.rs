use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_PLAYER_LOGOUT {
}

impl ClientMessageWrite for CMSG_PLAYER_LOGOUT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_PLAYER_LOGOUT {
    const OPCODE: u16 = 0x004a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }

}

impl ConstantSized for CMSG_PLAYER_LOGOUT {}

impl MaximumPossibleSized for CMSG_PLAYER_LOGOUT {
    fn maximum_possible_size() -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use super::CMSG_PLAYER_LOGOUT;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ClientOpcodeMessage;
    use crate::{MessageBody, ClientMessageWrite, ServerMessageWrite, OpcodeMessage};

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_PLAYER_LOGOUT0() {
        let raw: Vec<u8> = vec![ 0x00, 0x04, 0x4A, 0x00, 0x00, 0x00, ];

        let expected = CMSG_PLAYER_LOGOUT {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGOUT, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMSG_PLAYER_LOGOUT::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", async_std::test)]
    async fn tokio_CMSG_PLAYER_LOGOUT0() {
        let raw: Vec<u8> = vec![ 0x00, 0x04, 0x4A, 0x00, 0x00, 0x00, ];

        let expected = CMSG_PLAYER_LOGOUT {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGOUT, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMSG_PLAYER_LOGOUT::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", tokio::test)]
    async fn astd_CMSG_PLAYER_LOGOUT0() {
        let raw: Vec<u8> = vec![ 0x00, 0x04, 0x4A, 0x00, 0x00, 0x00, ];

        let expected = CMSG_PLAYER_LOGOUT {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGOUT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGOUT, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMSG_PLAYER_LOGOUT::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

}
