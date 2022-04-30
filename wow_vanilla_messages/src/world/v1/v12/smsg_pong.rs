use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PONG {
    pub sequence_id: u32,
}

impl ServerMessageWrite for SMSG_PONG {
    const OPCODE: u16 = 0x1dd;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_PONG {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // sequence_id: u32
        let sequence_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            sequence_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // sequence_id: u32
        w.write_all(&self.sequence_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PONG {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PONG {
    fn maximum_possible_size() -> usize {
        4 // sequence_id: u32
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::SMSG_PONG;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::WorldServerOpcodeMessage;
    use crate::{MessageBody, ClientMessageWrite, ServerMessageWrite, OpcodeMessage};

    #[test]
    fn SMSG_PONG0() {
        let raw: Vec<u8> = vec![ 0x00, 0x06, 0xDD, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, ];

        let expected = SMSG_PONG {
            sequence_id: 0xDEADBEEF,
        };

        let header_size = 2 + 2;
        let t = WorldServerOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            WorldServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);

        assert_eq!(SMSG_PONG::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
