use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{ItemClass, ItemClassError};
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
#[derive(Copy)]
pub struct SMSG_SET_PROFICIENCY {
    pub class: ItemClass,
    pub item_sub_class_mask: u32,
}

impl ServerMessageWrite for SMSG_SET_PROFICIENCY {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_SET_PROFICIENCY {
    const OPCODE: u16 = 0x0127;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_SET_PROFICIENCYError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // class: ItemClass
        let class = ItemClass::read(r)?;

        // item_sub_class_mask: u32
        let item_sub_class_mask = crate::util::read_u32_le(r)?;

        Ok(Self {
            class,
            item_sub_class_mask,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // class: ItemClass
        self.class.write(w)?;

        // item_sub_class_mask: u32
        w.write_all(&self.item_sub_class_mask.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // class: ItemClass
        let class = ItemClass::tokio_read(r).await?;

        // item_sub_class_mask: u32
        let item_sub_class_mask = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            class,
            item_sub_class_mask,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // class: ItemClass
        self.class.tokio_write(w).await?;

        // item_sub_class_mask: u32
        w.write_all(&self.item_sub_class_mask.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // class: ItemClass
        let class = ItemClass::astd_read(r).await?;

        // item_sub_class_mask: u32
        let item_sub_class_mask = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            class,
            item_sub_class_mask,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // class: ItemClass
        self.class.astd_write(w).await?;

        // item_sub_class_mask: u32
        w.write_all(&self.item_sub_class_mask.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_SET_PROFICIENCY {}

impl MaximumPossibleSized for SMSG_SET_PROFICIENCY {
    fn maximum_possible_size() -> usize {
        ItemClass::size() // class: ItemClass
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

