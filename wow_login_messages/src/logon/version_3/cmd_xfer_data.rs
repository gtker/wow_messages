use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMD_XFER_DATA {
    pub data: Vec<u8>,
}

impl CMD_XFER_DATA {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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
}

impl ServerMessage for CMD_XFER_DATA {
    const OPCODE: u8 = 49;

    type Error = std::io::Error;

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
        let mut v = Vec::with_capacity(self.size() + 1);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
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
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
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
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

impl CMD_XFER_DATA {
    pub(crate) fn size(&self) -> usize {
        2 // size: u16
        + self.data.len() * core::mem::size_of::<u8>() // data: u8[size]
    }
}

#[cfg(test)]
mod test {
    use super::CMD_XFER_DATA;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    const RAW0: [u8; 4] = [ 0x31, 0x01, 0x00, 0xFF, ];

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_XFER_DATA0() {
        let expected = CMD_XFER_DATA {
            data: vec![ 0xFF, ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_XFER_DATA0() {
        let expected = CMD_XFER_DATA {
            data: vec![ 0xFF, ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_XFER_DATA0() {
        let expected = CMD_XFER_DATA {
            data: vec![ 0xFF, ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 3] = [ 0x31, 0x00, 0x00, ];

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_XFER_DATA1() {
        let expected = CMD_XFER_DATA {
            data: vec![ ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_XFER_DATA1() {
        let expected = CMD_XFER_DATA {
            data: vec![ ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_XFER_DATA1() {
        let expected = CMD_XFER_DATA {
            data: vec![ ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}
