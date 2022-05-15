use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::GmSurveyQuestion;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GMSURVEY_SUBMIT {
    pub survey_id: u32,
    pub questions: [GmSurveyQuestion; 10],
    pub answer_comment: String,
}

impl ClientMessageWrite for CMSG_GMSURVEY_SUBMIT {}

impl MessageBody for CMSG_GMSURVEY_SUBMIT {
    const OPCODE: u16 = 0x032a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GMSURVEY_SUBMITError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // survey_id: u32
        let survey_id = crate::util::read_u32_le(r)?;

        // questions: GmSurveyQuestion[10]
        let mut questions = Vec::with_capacity(10 as usize);
        for i in 0..10 {
            questions.push(GmSurveyQuestion::read(r)?);
        }
        let questions = questions.try_into().unwrap();

        // answer_comment: CString
        let answer_comment = crate::util::read_c_string_to_vec(r)?;
        let answer_comment = String::from_utf8(answer_comment)?;

        Ok(Self {
            survey_id,
            questions,
            answer_comment,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // survey_id: u32
        w.write_all(&self.survey_id.to_le_bytes())?;

        // questions: GmSurveyQuestion[10]
        for i in self.questions.iter() {
            i.write(w)?;
        }

        // answer_comment: CString
        w.write_all(self.answer_comment.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

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
            // survey_id: u32
            let survey_id = crate::util::tokio_read_u32_le(r).await?;

            // questions: GmSurveyQuestion[10]
            let mut questions = Vec::with_capacity(10 as usize);
            for i in 0..10 {
                questions.push(GmSurveyQuestion::tokio_read(r).await?);
            }
            let questions = questions.try_into().unwrap();

            // answer_comment: CString
            let answer_comment = crate::util::tokio_read_c_string_to_vec(r).await?;
            let answer_comment = String::from_utf8(answer_comment)?;

            Ok(Self {
                survey_id,
                questions,
                answer_comment,
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
            // survey_id: u32
            w.write_all(&self.survey_id.to_le_bytes()).await?;

            // questions: GmSurveyQuestion[10]
            for i in self.questions.iter() {
                i.tokio_write(w).await?;
            }

            // answer_comment: CString
            w.write_all(self.answer_comment.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

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
            // survey_id: u32
            let survey_id = crate::util::astd_read_u32_le(r).await?;

            // questions: GmSurveyQuestion[10]
            let mut questions = Vec::with_capacity(10 as usize);
            for i in 0..10 {
                questions.push(GmSurveyQuestion::astd_read(r).await?);
            }
            let questions = questions.try_into().unwrap();

            // answer_comment: CString
            let answer_comment = crate::util::astd_read_c_string_to_vec(r).await?;
            let answer_comment = String::from_utf8(answer_comment)?;

            Ok(Self {
                survey_id,
                questions,
                answer_comment,
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
            // survey_id: u32
            w.write_all(&self.survey_id.to_le_bytes()).await?;

            // questions: GmSurveyQuestion[10]
            for i in self.questions.iter() {
                i.astd_write(w).await?;
            }

            // answer_comment: CString
            w.write_all(self.answer_comment.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

}

impl CMSG_GMSURVEY_SUBMIT {
    pub fn size(&self) -> usize {
        0
        + 4 // survey_id: u32
        + 10 * GmSurveyQuestion::size() // questions: GmSurveyQuestion[10]
        + self.answer_comment.len() + 1 // answer_comment: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GMSURVEY_SUBMITError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GMSURVEY_SUBMITError {}
impl std::fmt::Display for CMSG_GMSURVEY_SUBMITError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GMSURVEY_SUBMITError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GMSURVEY_SUBMITError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

