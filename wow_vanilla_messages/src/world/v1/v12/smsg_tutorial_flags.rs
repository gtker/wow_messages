use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TUTORIAL_FLAGS {
    pub tutorial_data0: u32,
    pub tutorial_data1: u32,
    pub tutorial_data2: u32,
    pub tutorial_data3: u32,
    pub tutorial_data4: u32,
    pub tutorial_data5: u32,
    pub tutorial_data6: u32,
    pub tutorial_data7: u32,
}

impl ServerMessageWrite for SMSG_TUTORIAL_FLAGS {}

impl MessageBody for SMSG_TUTORIAL_FLAGS {
    const OPCODE: u16 = 0x00fd;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // tutorial_data0: u32
        let tutorial_data0 = crate::util::read_u32_le(r)?;

        // tutorial_data1: u32
        let tutorial_data1 = crate::util::read_u32_le(r)?;

        // tutorial_data2: u32
        let tutorial_data2 = crate::util::read_u32_le(r)?;

        // tutorial_data3: u32
        let tutorial_data3 = crate::util::read_u32_le(r)?;

        // tutorial_data4: u32
        let tutorial_data4 = crate::util::read_u32_le(r)?;

        // tutorial_data5: u32
        let tutorial_data5 = crate::util::read_u32_le(r)?;

        // tutorial_data6: u32
        let tutorial_data6 = crate::util::read_u32_le(r)?;

        // tutorial_data7: u32
        let tutorial_data7 = crate::util::read_u32_le(r)?;

        Ok(Self {
            tutorial_data0,
            tutorial_data1,
            tutorial_data2,
            tutorial_data3,
            tutorial_data4,
            tutorial_data5,
            tutorial_data6,
            tutorial_data7,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // tutorial_data0: u32
        w.write_all(&self.tutorial_data0.to_le_bytes())?;

        // tutorial_data1: u32
        w.write_all(&self.tutorial_data1.to_le_bytes())?;

        // tutorial_data2: u32
        w.write_all(&self.tutorial_data2.to_le_bytes())?;

        // tutorial_data3: u32
        w.write_all(&self.tutorial_data3.to_le_bytes())?;

        // tutorial_data4: u32
        w.write_all(&self.tutorial_data4.to_le_bytes())?;

        // tutorial_data5: u32
        w.write_all(&self.tutorial_data5.to_le_bytes())?;

        // tutorial_data6: u32
        w.write_all(&self.tutorial_data6.to_le_bytes())?;

        // tutorial_data7: u32
        w.write_all(&self.tutorial_data7.to_le_bytes())?;

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
            // tutorial_data0: u32
            let tutorial_data0 = crate::util::tokio_read_u32_le(r).await?;

            // tutorial_data1: u32
            let tutorial_data1 = crate::util::tokio_read_u32_le(r).await?;

            // tutorial_data2: u32
            let tutorial_data2 = crate::util::tokio_read_u32_le(r).await?;

            // tutorial_data3: u32
            let tutorial_data3 = crate::util::tokio_read_u32_le(r).await?;

            // tutorial_data4: u32
            let tutorial_data4 = crate::util::tokio_read_u32_le(r).await?;

            // tutorial_data5: u32
            let tutorial_data5 = crate::util::tokio_read_u32_le(r).await?;

            // tutorial_data6: u32
            let tutorial_data6 = crate::util::tokio_read_u32_le(r).await?;

            // tutorial_data7: u32
            let tutorial_data7 = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                tutorial_data0,
                tutorial_data1,
                tutorial_data2,
                tutorial_data3,
                tutorial_data4,
                tutorial_data5,
                tutorial_data6,
                tutorial_data7,
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
            // tutorial_data0: u32
            w.write_all(&self.tutorial_data0.to_le_bytes()).await?;

            // tutorial_data1: u32
            w.write_all(&self.tutorial_data1.to_le_bytes()).await?;

            // tutorial_data2: u32
            w.write_all(&self.tutorial_data2.to_le_bytes()).await?;

            // tutorial_data3: u32
            w.write_all(&self.tutorial_data3.to_le_bytes()).await?;

            // tutorial_data4: u32
            w.write_all(&self.tutorial_data4.to_le_bytes()).await?;

            // tutorial_data5: u32
            w.write_all(&self.tutorial_data5.to_le_bytes()).await?;

            // tutorial_data6: u32
            w.write_all(&self.tutorial_data6.to_le_bytes()).await?;

            // tutorial_data7: u32
            w.write_all(&self.tutorial_data7.to_le_bytes()).await?;

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
            // tutorial_data0: u32
            let tutorial_data0 = crate::util::astd_read_u32_le(r).await?;

            // tutorial_data1: u32
            let tutorial_data1 = crate::util::astd_read_u32_le(r).await?;

            // tutorial_data2: u32
            let tutorial_data2 = crate::util::astd_read_u32_le(r).await?;

            // tutorial_data3: u32
            let tutorial_data3 = crate::util::astd_read_u32_le(r).await?;

            // tutorial_data4: u32
            let tutorial_data4 = crate::util::astd_read_u32_le(r).await?;

            // tutorial_data5: u32
            let tutorial_data5 = crate::util::astd_read_u32_le(r).await?;

            // tutorial_data6: u32
            let tutorial_data6 = crate::util::astd_read_u32_le(r).await?;

            // tutorial_data7: u32
            let tutorial_data7 = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                tutorial_data0,
                tutorial_data1,
                tutorial_data2,
                tutorial_data3,
                tutorial_data4,
                tutorial_data5,
                tutorial_data6,
                tutorial_data7,
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
            // tutorial_data0: u32
            w.write_all(&self.tutorial_data0.to_le_bytes()).await?;

            // tutorial_data1: u32
            w.write_all(&self.tutorial_data1.to_le_bytes()).await?;

            // tutorial_data2: u32
            w.write_all(&self.tutorial_data2.to_le_bytes()).await?;

            // tutorial_data3: u32
            w.write_all(&self.tutorial_data3.to_le_bytes()).await?;

            // tutorial_data4: u32
            w.write_all(&self.tutorial_data4.to_le_bytes()).await?;

            // tutorial_data5: u32
            w.write_all(&self.tutorial_data5.to_le_bytes()).await?;

            // tutorial_data6: u32
            w.write_all(&self.tutorial_data6.to_le_bytes()).await?;

            // tutorial_data7: u32
            w.write_all(&self.tutorial_data7.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl SMSG_TUTORIAL_FLAGS {
    pub(crate) fn size() -> usize {
        0
        + 4 // tutorial_data0: u32
        + 4 // tutorial_data1: u32
        + 4 // tutorial_data2: u32
        + 4 // tutorial_data3: u32
        + 4 // tutorial_data4: u32
        + 4 // tutorial_data5: u32
        + 4 // tutorial_data6: u32
        + 4 // tutorial_data7: u32
    }
}

