use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{NewItemChatAlert, NewItemChatAlertError};
use crate::world::v1::v12::{NewItemCreationType, NewItemCreationTypeError};
use crate::world::v1::v12::{NewItemSource, NewItemSourceError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_ITEM_PUSH_RESULT {
    pub guid: Guid,
    pub source: NewItemSource,
    pub creation_type: NewItemCreationType,
    pub alert_chat: NewItemChatAlert,
    pub bag_slot: u8,
    pub item_slot: u32,
    pub item_id: u32,
    pub item_suffix_factor: u32,
    pub item_random_property_id: u32,
    pub item_count: u32,
}

impl ServerMessageWrite for SMSG_ITEM_PUSH_RESULT {}

impl MessageBody for SMSG_ITEM_PUSH_RESULT {
    const OPCODE: u16 = 0x0166;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_ITEM_PUSH_RESULTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // source: NewItemSource
        let source = NewItemSource::read(r)?;

        // creation_type: NewItemCreationType
        let creation_type = NewItemCreationType::read(r)?;

        // alert_chat: NewItemChatAlert
        let alert_chat = NewItemChatAlert::read(r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(r)?;

        // item_slot: u32
        let item_slot = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            source,
            creation_type,
            alert_chat,
            bag_slot,
            item_slot,
            item_id,
            item_suffix_factor,
            item_random_property_id,
            item_count,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // source: NewItemSource
        crate::util::write_u32_le(w, self.source.as_int() as u32)?;

        // creation_type: NewItemCreationType
        crate::util::write_u32_le(w, self.creation_type.as_int() as u32)?;

        // alert_chat: NewItemChatAlert
        crate::util::write_u32_le(w, self.alert_chat.as_int() as u32)?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // item_slot: u32
        w.write_all(&self.item_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_suffix_factor: u32
        w.write_all(&self.item_suffix_factor.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // source: NewItemSource
            let source = NewItemSource::tokio_read(r).await?;

            // creation_type: NewItemCreationType
            let creation_type = NewItemCreationType::tokio_read(r).await?;

            // alert_chat: NewItemChatAlert
            let alert_chat = NewItemChatAlert::tokio_read(r).await?;

            // bag_slot: u8
            let bag_slot = crate::util::tokio_read_u8_le(r).await?;

            // item_slot: u32
            let item_slot = crate::util::tokio_read_u32_le(r).await?;

            // item_id: u32
            let item_id = crate::util::tokio_read_u32_le(r).await?;

            // item_suffix_factor: u32
            let item_suffix_factor = crate::util::tokio_read_u32_le(r).await?;

            // item_random_property_id: u32
            let item_random_property_id = crate::util::tokio_read_u32_le(r).await?;

            // item_count: u32
            let item_count = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                guid,
                source,
                creation_type,
                alert_chat,
                bag_slot,
                item_slot,
                item_id,
                item_suffix_factor,
                item_random_property_id,
                item_count,
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
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // source: NewItemSource
            crate::util::tokio_write_u32_le(w, self.source.as_int() as u32).await?;

            // creation_type: NewItemCreationType
            crate::util::tokio_write_u32_le(w, self.creation_type.as_int() as u32).await?;

            // alert_chat: NewItemChatAlert
            crate::util::tokio_write_u32_le(w, self.alert_chat.as_int() as u32).await?;

            // bag_slot: u8
            w.write_all(&self.bag_slot.to_le_bytes()).await?;

            // item_slot: u32
            w.write_all(&self.item_slot.to_le_bytes()).await?;

            // item_id: u32
            w.write_all(&self.item_id.to_le_bytes()).await?;

            // item_suffix_factor: u32
            w.write_all(&self.item_suffix_factor.to_le_bytes()).await?;

            // item_random_property_id: u32
            w.write_all(&self.item_random_property_id.to_le_bytes()).await?;

            // item_count: u32
            w.write_all(&self.item_count.to_le_bytes()).await?;

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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // source: NewItemSource
            let source = NewItemSource::astd_read(r).await?;

            // creation_type: NewItemCreationType
            let creation_type = NewItemCreationType::astd_read(r).await?;

            // alert_chat: NewItemChatAlert
            let alert_chat = NewItemChatAlert::astd_read(r).await?;

            // bag_slot: u8
            let bag_slot = crate::util::astd_read_u8_le(r).await?;

            // item_slot: u32
            let item_slot = crate::util::astd_read_u32_le(r).await?;

            // item_id: u32
            let item_id = crate::util::astd_read_u32_le(r).await?;

            // item_suffix_factor: u32
            let item_suffix_factor = crate::util::astd_read_u32_le(r).await?;

            // item_random_property_id: u32
            let item_random_property_id = crate::util::astd_read_u32_le(r).await?;

            // item_count: u32
            let item_count = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                guid,
                source,
                creation_type,
                alert_chat,
                bag_slot,
                item_slot,
                item_id,
                item_suffix_factor,
                item_random_property_id,
                item_count,
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
            // guid: Guid
            self.guid.astd_write(w).await?;

            // source: NewItemSource
            crate::util::astd_write_u32_le(w, self.source.as_int() as u32).await?;

            // creation_type: NewItemCreationType
            crate::util::astd_write_u32_le(w, self.creation_type.as_int() as u32).await?;

            // alert_chat: NewItemChatAlert
            crate::util::astd_write_u32_le(w, self.alert_chat.as_int() as u32).await?;

            // bag_slot: u8
            w.write_all(&self.bag_slot.to_le_bytes()).await?;

            // item_slot: u32
            w.write_all(&self.item_slot.to_le_bytes()).await?;

            // item_id: u32
            w.write_all(&self.item_id.to_le_bytes()).await?;

            // item_suffix_factor: u32
            w.write_all(&self.item_suffix_factor.to_le_bytes()).await?;

            // item_random_property_id: u32
            w.write_all(&self.item_random_property_id.to_le_bytes()).await?;

            // item_count: u32
            w.write_all(&self.item_count.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_ITEM_PUSH_RESULT {}

impl MaximumPossibleSized for SMSG_ITEM_PUSH_RESULT {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 4 // source: NewItemSource
        + 4 // creation_type: NewItemCreationType
        + 4 // alert_chat: NewItemChatAlert
        + 1 // bag_slot: u8
        + 4 // item_slot: u32
        + 4 // item_id: u32
        + 4 // item_suffix_factor: u32
        + 4 // item_random_property_id: u32
        + 4 // item_count: u32
    }
}

#[derive(Debug)]
pub enum SMSG_ITEM_PUSH_RESULTError {
    Io(std::io::Error),
    NewItemChatAlert(NewItemChatAlertError),
    NewItemCreationType(NewItemCreationTypeError),
    NewItemSource(NewItemSourceError),
}

impl std::error::Error for SMSG_ITEM_PUSH_RESULTError {}
impl std::fmt::Display for SMSG_ITEM_PUSH_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::NewItemChatAlert(i) => i.fmt(f),
            Self::NewItemCreationType(i) => i.fmt(f),
            Self::NewItemSource(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ITEM_PUSH_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<NewItemChatAlertError> for SMSG_ITEM_PUSH_RESULTError {
    fn from(e: NewItemChatAlertError) -> Self {
        Self::NewItemChatAlert(e)
    }
}

impl From<NewItemCreationTypeError> for SMSG_ITEM_PUSH_RESULTError {
    fn from(e: NewItemCreationTypeError) -> Self {
        Self::NewItemCreationType(e)
    }
}

impl From<NewItemSourceError> for SMSG_ITEM_PUSH_RESULTError {
    fn from(e: NewItemSourceError) -> Self {
        Self::NewItemSource(e)
    }
}

