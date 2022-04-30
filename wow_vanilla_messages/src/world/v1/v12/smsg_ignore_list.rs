use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_IGNORE_LIST {
    pub ignored: Vec<u64>,
}

impl ServerMessageWrite for SMSG_IGNORE_LIST {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_IGNORE_LIST {
    const OPCODE: u16 = 0x006b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_ignored: u8
        let amount_of_ignored = crate::util::read_u8_le(r)?;

        // ignored: u64[amount_of_ignored]
        let mut ignored = Vec::with_capacity(amount_of_ignored as usize);
        for i in 0..amount_of_ignored {
            ignored.push(crate::util::read_u64_le(r)?);
        }

        Ok(Self {
            ignored,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_ignored: u8
        w.write_all(&(self.ignored.len() as u8).to_le_bytes())?;

        // ignored: u64[amount_of_ignored]
        for i in self.ignored.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_ignored: u8
        let amount_of_ignored = crate::util::tokio_read_u8_le(r).await?;

        // ignored: u64[amount_of_ignored]
        let mut ignored = Vec::with_capacity(amount_of_ignored as usize);
        for i in 0..amount_of_ignored {
            ignored.push(crate::util::tokio_read_u64_le(r).await?);
        }

        Ok(Self {
            ignored,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_ignored: u8
        w.write_all(&(self.ignored.len() as u8).to_le_bytes()).await?;

        // ignored: u64[amount_of_ignored]
        for i in self.ignored.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_ignored: u8
        let amount_of_ignored = crate::util::astd_read_u8_le(r).await?;

        // ignored: u64[amount_of_ignored]
        let mut ignored = Vec::with_capacity(amount_of_ignored as usize);
        for i in 0..amount_of_ignored {
            ignored.push(crate::util::astd_read_u64_le(r).await?);
        }

        Ok(Self {
            ignored,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_ignored: u8
        w.write_all(&(self.ignored.len() as u8).to_le_bytes()).await?;

        // ignored: u64[amount_of_ignored]
        for i in self.ignored.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }

}

impl VariableSized for SMSG_IGNORE_LIST {
    fn size(&self) -> usize {
        1 // amount_of_ignored: u8
        + self.ignored.len() * core::mem::size_of::<u64>() // ignored: u64[amount_of_ignored]
    }
}

impl MaximumPossibleSized for SMSG_IGNORE_LIST {
    fn maximum_possible_size() -> usize {
        1 // amount_of_ignored: u8
        + 255 * core::mem::size_of::<u64>() // ignored: u64[amount_of_ignored]
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::SMSG_IGNORE_LIST;
    use crate::VariableSized;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ServerOpcodeMessage;
    use crate::{MessageBody, ClientMessageWrite, ServerMessageWrite, OpcodeMessage};

    #[test]
    fn SMSG_IGNORE_LIST0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0B, 0x6B, 0x00, 0x01, 0xEF, 0xBE, 0xAD,
             0xDE, 0xFE, 0x0F, 0xDC, 0xBA, ];

        let expected = SMSG_IGNORE_LIST {
            ignored: vec![ 0xBADC0FFEDEADBEEF, ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.ignored, expected.ignored);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[test]
    fn SMSG_IGNORE_LIST1() {
        let raw: Vec<u8> = vec![ 0x00, 0x13, 0x6B, 0x00, 0x02, 0xEF, 0xBE, 0xAD,
             0xDE, 0xFE, 0x0F, 0xDC, 0xBA, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x00, 0x00,
             0x00, ];

        let expected = SMSG_IGNORE_LIST {
            ignored: vec![ 0xBADC0FFEDEADBEEF, 0xDEADBEEF, ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.ignored, expected.ignored);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
