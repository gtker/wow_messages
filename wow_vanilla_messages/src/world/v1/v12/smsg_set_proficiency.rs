use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::ItemClass;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_SET_PROFICIENCY {
    pub class: ItemClass,
    pub item_sub_class_mask: u32,
}

impl SMSG_SET_PROFICIENCY {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 5], std::io::Error> {
        let mut array_w = [0u8; 5];
        let mut w = array_w.as_mut_slice();
        // class: ItemClass
        w.write_all(&(self.class.as_int() as u8).to_le_bytes())?;

        // item_sub_class_mask: u32
        w.write_all(&self.item_sub_class_mask.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_SET_PROFICIENCY {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // class: ItemClass
        w.write_all(&(self.class.as_int() as u8).to_le_bytes())?;

        // item_sub_class_mask: u32
        w.write_all(&self.item_sub_class_mask.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0127;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        5
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

}

#[derive(Debug)]
pub enum SMSG_SET_PROFICIENCYError {
    Io(std::io::Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for SMSG_SET_PROFICIENCYError {}
impl std::fmt::Display for SMSG_SET_PROFICIENCYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SET_PROFICIENCYError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for SMSG_SET_PROFICIENCYError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

