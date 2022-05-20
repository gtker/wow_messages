use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct GmSurveyQuestion {
    pub question_id: u32,
    pub answer: u8,
}

impl GmSurveyQuestion {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 5], std::io::Error> {
        let mut array_w = [0u8; 5];
        let mut w = array_w.as_mut_slice();
        // question_id: u32
        w.write_all(&self.question_id.to_le_bytes())?;

        // answer: u8
        w.write_all(&self.answer.to_le_bytes())?;

        Ok(array_w)
    }
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

    #[cfg(feature = "tokio")]
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

    #[cfg(feature = "async-std")]
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

}

