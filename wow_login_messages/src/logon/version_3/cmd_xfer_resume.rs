use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMD_XFER_RESUME {
    pub offset: u64,
}

impl ClientMessage for CMD_XFER_RESUME {
    const OPCODE: u8 = 0x33;
}
#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for CMD_XFER_RESUME {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // offset: u64
        let offset = crate::util::read_u64_le(r)?;

        Ok(Self {
            offset,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // offset: u64
        w.write_all(&self.offset.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // offset: u64
        let offset = crate::util::tokio_read_u64_le(r).await?;

        Ok(Self {
            offset,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes()).await?;

        // offset: u64
        w.write_all(&self.offset.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // offset: u64
        let offset = crate::util::astd_read_u64_le(r).await?;

        Ok(Self {
            offset,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes()).await?;

        // offset: u64
        w.write_all(&self.offset.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for CMD_XFER_RESUME {}

impl MaximumPossibleSized for CMD_XFER_RESUME {
    fn maximum_possible_size() -> usize {
        8 // offset: u64
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_XFER_RESUME;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    #[test]
    #[cfg(feature = "sync")]
    fn CMD_XFER_RESUME0() {
        let raw: Vec<u8> = vec![ 0x33, 0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x00, ];

        let expected = CMD_XFER_RESUME {
            offset: 0xDEAD,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.offset, expected.offset);

        assert_eq!(CMD_XFER_RESUME::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
