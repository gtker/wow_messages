use std::convert::{TryFrom, TryInto};
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

impl GmSurveyQuestion {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
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
    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // question_id: u32
        w.write_all(&self.question_id.to_le_bytes())?;

        // answer: u8
        w.write_all(&self.answer.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // question_id: u32
        let question_id = crate::util::tokio_read_u32_le(r).await?;

        // answer: u8
        let answer = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            question_id,
            answer,
        })
    }

    #[cfg(feature = "async_tokio")]
    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // question_id: u32
        w.write_all(&self.question_id.to_le_bytes()).await?;

        // answer: u8
        w.write_all(&self.answer.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // question_id: u32
        let question_id = crate::util::astd_read_u32_le(r).await?;

        // answer: u8
        let answer = crate::util::astd_read_u8_le(r).await?;

        Ok(Self {
            question_id,
            answer,
        })
    }

    #[cfg(feature = "async_std")]
    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // question_id: u32
        w.write_all(&self.question_id.to_le_bytes()).await?;

        // answer: u8
        w.write_all(&self.answer.to_le_bytes()).await?;

        Ok(())
    }

}

impl GmSurveyQuestion {
    pub(crate) fn size() -> usize {
        0
        + 4 // question_id: u32
        + 1 // answer: u8
    }
}

