use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Object;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_UPDATE_OBJECT {
    pub has_transport: u8,
    pub objects: Vec<Object>,
}

impl SMSG_UPDATE_OBJECT {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // amount_of_objects: u32
        w.write_all(&(self.objects.len() as u32).to_le_bytes())?;

        // has_transport: u8
        w.write_all(&self.has_transport.to_le_bytes())?;

        // objects: Object[amount_of_objects]
        for i in self.objects.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ClientMessage for SMSG_UPDATE_OBJECT {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_objects: u32
        w.write_all(&(self.objects.len() as u32).to_le_bytes())?;

        // has_transport: u8
        w.write_all(&self.has_transport.to_le_bytes())?;

        // objects: Object[amount_of_objects]
        for i in self.objects.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x00a9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_objects: u32
        let amount_of_objects = crate::util::read_u32_le(r)?;

        // has_transport: u8
        let has_transport = crate::util::read_u8_le(r)?;

        // objects: Object[amount_of_objects]
        let mut objects = Vec::with_capacity(amount_of_objects as usize);
        for i in 0..amount_of_objects {
            objects.push(Object::read(r)?);
        }

        Ok(Self {
            has_transport,
            objects,
        })
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
            // amount_of_objects: u32
            let amount_of_objects = crate::util::tokio_read_u32_le(r).await?;

            // has_transport: u8
            let has_transport = crate::util::tokio_read_u8_le(r).await?;

            // objects: Object[amount_of_objects]
            let mut objects = Vec::with_capacity(amount_of_objects as usize);
            for i in 0..amount_of_objects {
                objects.push(Object::tokio_read(r).await?);
            }

            Ok(Self {
                has_transport,
                objects,
            })
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
            // amount_of_objects: u32
            let amount_of_objects = crate::util::astd_read_u32_le(r).await?;

            // has_transport: u8
            let has_transport = crate::util::astd_read_u8_le(r).await?;

            // objects: Object[amount_of_objects]
            let mut objects = Vec::with_capacity(amount_of_objects as usize);
            for i in 0..amount_of_objects {
                objects.push(Object::astd_read(r).await?);
            }

            Ok(Self {
                has_transport,
                objects,
            })
        })
    }

}

impl SMSG_UPDATE_OBJECT {
    pub fn size(&self) -> usize {
        0
        + 4 // amount_of_objects: u32
        + 1 // has_transport: u8
        + self.objects.iter().fold(0, |acc, x| acc + x.size()) // objects: Object[amount_of_objects]
    }
}

