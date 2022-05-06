use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMD_XFER_DATA {
    pub data: Vec<u8>,
}

impl ServerMessage for CMD_XFER_DATA {
    const OPCODE: u8 = 0x31;
}
#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for CMD_XFER_DATA {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // size: u16
        let size = crate::util::read_u16_le(r)?;

        // data: u8[size]
        let mut data = Vec::with_capacity(size as usize);
        for i in 0..size {
            data.push(crate::util::read_u8_le(r)?);
        }

        Ok(Self {
            data,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // size: u16
        w.write_all(&(self.data.len() as u16).to_le_bytes())?;

        // data: u8[size]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // size: u16
        let size = crate::util::tokio_read_u16_le(r).await?;

        // data: u8[size]
        let mut data = Vec::with_capacity(size as usize);
        for i in 0..size {
            data.push(crate::util::tokio_read_u8_le(r).await?);
        }

        Ok(Self {
            data,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes()).await?;

        // size: u16
        w.write_all(&(self.data.len() as u16).to_le_bytes()).await?;

        // data: u8[size]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // size: u16
        let size = crate::util::astd_read_u16_le(r).await?;

        // data: u8[size]
        let mut data = Vec::with_capacity(size as usize);
        for i in 0..size {
            data.push(crate::util::astd_read_u8_le(r).await?);
        }

        Ok(Self {
            data,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes()).await?;

        // size: u16
        w.write_all(&(self.data.len() as u16).to_le_bytes()).await?;

        // data: u8[size]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }

}

impl VariableSized for CMD_XFER_DATA {
    fn size(&self) -> usize {
        0
        + 2 // size: u16
        + self.data.len() * core::mem::size_of::<u8>() // data: u8[size]
    }
}

impl MaximumPossibleSized for CMD_XFER_DATA {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_XFER_DATA;
    use crate::VariableSized;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    #[test]
    #[cfg(feature = "sync")]
    fn CMD_XFER_DATA0() {
        let raw: Vec<u8> = vec![ 0x31, 0x01, 0x00, 0xFF, ];

        let expected = CMD_XFER_DATA {
            data: vec![ 0xFF, ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[test]
    #[cfg(feature = "sync")]
    fn CMD_XFER_DATA1() {
        let raw: Vec<u8> = vec![ 0x31, 0x00, 0x00, ];

        let expected = CMD_XFER_DATA {
            data: vec![ ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
