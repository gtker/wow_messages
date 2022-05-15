use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Object, ObjectError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_UPDATE_OBJECT {
    pub has_transport: u8,
    pub objects: Vec<Object>,
}

impl ClientMessageWrite for SMSG_UPDATE_OBJECT {}

impl MessageBody for SMSG_UPDATE_OBJECT {
    const OPCODE: u16 = 0x00a9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_UPDATE_OBJECTError;

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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_objects: u32
        w.write_all(&(self.objects.len() as u32).to_le_bytes())?;

        // has_transport: u8
        w.write_all(&self.has_transport.to_le_bytes())?;

        // objects: Object[amount_of_objects]
        for i in self.objects.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // amount_of_objects: u32
            w.write_all(&(self.objects.len() as u32).to_le_bytes()).await?;

            // has_transport: u8
            w.write_all(&self.has_transport.to_le_bytes()).await?;

            // objects: Object[amount_of_objects]
            for i in self.objects.iter() {
                i.tokio_write(w).await?;
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // amount_of_objects: u32
            w.write_all(&(self.objects.len() as u32).to_le_bytes()).await?;

            // has_transport: u8
            w.write_all(&self.has_transport.to_le_bytes()).await?;

            // objects: Object[amount_of_objects]
            for i in self.objects.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_UPDATE_OBJECT {
    fn size(&self) -> usize {
        0
        + 4 // amount_of_objects: u32
        + 1 // has_transport: u8
        + self.objects.iter().fold(0, |acc, x| acc + x.size()) // objects: Object[amount_of_objects]
    }
}

impl MaximumPossibleSized for SMSG_UPDATE_OBJECT {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

#[derive(Debug)]
pub enum SMSG_UPDATE_OBJECTError {
    Io(std::io::Error),
    Object(ObjectError),
}

impl std::error::Error for SMSG_UPDATE_OBJECTError {}
impl std::fmt::Display for SMSG_UPDATE_OBJECTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Object(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_UPDATE_OBJECTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ObjectError> for SMSG_UPDATE_OBJECTError {
    fn from(e: ObjectError) -> Self {
        Self::Object(e)
    }
}

