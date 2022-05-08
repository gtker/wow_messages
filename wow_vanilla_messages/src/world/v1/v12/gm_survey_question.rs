use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct GmSurveyQuestion {
    pub question_id: u32,
    pub answer: u8,
}

impl ReadableAndWritable for GmSurveyQuestion {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // question_id: u32
        let question_id = crate::util::read_u32_le(r)?;

        // answer: u8
        let answer = crate::util::read_u8_le(r)?;

        Ok(Self {
            question_id,
            answer,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // question_id: u32
        w.write_all(&self.question_id.to_le_bytes())?;

        // answer: u8
        w.write_all(&self.answer.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // question_id: u32
            let question_id = crate::util::tokio_read_u32_le(r).await?;

            // answer: u8
            let answer = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                question_id,
                answer,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
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
            // question_id: u32
            w.write_all(&self.question_id.to_le_bytes()).await?;

            // answer: u8
            w.write_all(&self.answer.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // question_id: u32
            let question_id = crate::util::astd_read_u32_le(r).await?;

            // answer: u8
            let answer = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                question_id,
                answer,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
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
            // question_id: u32
            w.write_all(&self.question_id.to_le_bytes()).await?;

            // answer: u8
            w.write_all(&self.answer.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for GmSurveyQuestion {}

impl MaximumPossibleSized for GmSurveyQuestion {
    fn maximum_possible_size() -> usize {
        0
        + 4 // question_id: u32
        + 1 // answer: u8
    }
}

