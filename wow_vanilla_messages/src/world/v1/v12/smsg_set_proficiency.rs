use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{ItemClass, ItemClassError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_SET_PROFICIENCY {
    pub class: ItemClass,
    pub item_sub_class_mask: u32,
}

impl ServerMessageWrite for SMSG_SET_PROFICIENCY {}

impl MessageBody for SMSG_SET_PROFICIENCY {
    const OPCODE: u16 = 0x0127;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_SET_PROFICIENCYError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // class: ItemClass
        let class: ItemClass = crate::util::read_u8_le(r)?.try_into()?;

        // item_sub_class_mask: u32
        let item_sub_class_mask = crate::util::read_u32_le(r)?;

        Ok(Self {
            class,
            item_sub_class_mask,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // class: ItemClass
        crate::util::write_u8_le(w, self.class.as_int() as u8)?;

        // item_sub_class_mask: u32
        w.write_all(&self.item_sub_class_mask.to_le_bytes())?;

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
            // class: ItemClass
            let class: ItemClass = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // item_sub_class_mask: u32
            let item_sub_class_mask = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                class,
                item_sub_class_mask,
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
            // class: ItemClass
            crate::util::tokio_write_u8_le(w, self.class.as_int() as u8).await?;

            // item_sub_class_mask: u32
            w.write_all(&self.item_sub_class_mask.to_le_bytes()).await?;

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
            // class: ItemClass
            let class: ItemClass = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // item_sub_class_mask: u32
            let item_sub_class_mask = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                class,
                item_sub_class_mask,
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
            // class: ItemClass
            crate::util::astd_write_u8_le(w, self.class.as_int() as u8).await?;

            // item_sub_class_mask: u32
            w.write_all(&self.item_sub_class_mask.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_SET_PROFICIENCY {}

impl MaximumPossibleSized for SMSG_SET_PROFICIENCY {
    fn maximum_possible_size() -> usize {
        0
        + 1 // class: ItemClass
        + 4 // item_sub_class_mask: u32
    }
}

#[derive(Debug)]
pub enum SMSG_SET_PROFICIENCYError {
    Io(std::io::Error),
    ItemClass(ItemClassError),
}

impl std::error::Error for SMSG_SET_PROFICIENCYError {}
impl std::fmt::Display for SMSG_SET_PROFICIENCYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::ItemClass(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SET_PROFICIENCYError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ItemClassError> for SMSG_SET_PROFICIENCYError {
    fn from(e: ItemClassError) -> Self {
        Self::ItemClass(e)
    }
}

