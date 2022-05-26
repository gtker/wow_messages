use std::convert::{TryFrom, TryInto};
use crate::{ServerMessage, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_IGNORE_LIST {
    pub ignored: Vec<u64>,
}

impl ServerMessage for SMSG_IGNORE_LIST {}

impl SMSG_IGNORE_LIST {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // amount_of_ignored: u8
        w.write_all(&(self.ignored.len() as u8).to_le_bytes())?;

        // ignored: u64[amount_of_ignored]
        for i in self.ignored.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(w)
    }
}

impl MessageBody for SMSG_IGNORE_LIST {
    const OPCODE: u16 = 0x006b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
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
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
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
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
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
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
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
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_IGNORE_LIST {
    pub fn size(&self) -> usize {
        0
        + 1 // amount_of_ignored: u8
        + self.ignored.len() * core::mem::size_of::<u64>() // ignored: u64[amount_of_ignored]
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_IGNORE_LIST;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ServerOpcodeMessage;
    use crate::{MessageBody, ClientMessage, ServerMessage};

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_IGNORE_LIST0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0B, 0x6B, 0x00, 0x01, 0xEF, 0xBE, 0xAD,
             0xDE, 0xFE, 0x0F, 0xDC, 0xBA, ];

        let expected = SMSG_IGNORE_LIST {
            ignored: vec![ 0xBADC0FFEDEADBEEF, ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.ignored, expected.ignored);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_IGNORE_LIST0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0B, 0x6B, 0x00, 0x01, 0xEF, 0xBE, 0xAD,
             0xDE, 0xFE, 0x0F, 0xDC, 0xBA, ];

        let expected = SMSG_IGNORE_LIST {
            ignored: vec![ 0xBADC0FFEDEADBEEF, ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.ignored, expected.ignored);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_IGNORE_LIST0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0B, 0x6B, 0x00, 0x01, 0xEF, 0xBE, 0xAD,
             0xDE, 0xFE, 0x0F, 0xDC, 0xBA, ];

        let expected = SMSG_IGNORE_LIST {
            ignored: vec![ 0xBADC0FFEDEADBEEF, ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.ignored, expected.ignored);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_IGNORE_LIST1() {
        let raw: Vec<u8> = vec![ 0x00, 0x13, 0x6B, 0x00, 0x02, 0xEF, 0xBE, 0xAD,
             0xDE, 0xFE, 0x0F, 0xDC, 0xBA, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x00, 0x00,
             0x00, ];

        let expected = SMSG_IGNORE_LIST {
            ignored: vec![ 0xBADC0FFEDEADBEEF, 0xDEADBEEF, ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.ignored, expected.ignored);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_IGNORE_LIST1() {
        let raw: Vec<u8> = vec![ 0x00, 0x13, 0x6B, 0x00, 0x02, 0xEF, 0xBE, 0xAD,
             0xDE, 0xFE, 0x0F, 0xDC, 0xBA, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x00, 0x00,
             0x00, ];

        let expected = SMSG_IGNORE_LIST {
            ignored: vec![ 0xBADC0FFEDEADBEEF, 0xDEADBEEF, ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.ignored, expected.ignored);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_IGNORE_LIST1() {
        let raw: Vec<u8> = vec![ 0x00, 0x13, 0x6B, 0x00, 0x02, 0xEF, 0xBE, 0xAD,
             0xDE, 0xFE, 0x0F, 0xDC, 0xBA, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x00, 0x00,
             0x00, ];

        let expected = SMSG_IGNORE_LIST {
            ignored: vec![ 0xBADC0FFEDEADBEEF, 0xDEADBEEF, ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.ignored, expected.ignored);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

}
