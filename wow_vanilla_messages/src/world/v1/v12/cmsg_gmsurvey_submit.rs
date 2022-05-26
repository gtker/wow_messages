use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::GmSurveyQuestion;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GMSURVEY_SUBMIT {
    pub survey_id: u32,
    pub questions: [GmSurveyQuestion; 10],
    pub answer_comment: String,
}

impl ClientMessage for CMSG_GMSURVEY_SUBMIT {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // survey_id: u32
        w.write_all(&self.survey_id.to_le_bytes())?;

        // questions: GmSurveyQuestion[10]
        for i in self.questions.iter() {
            i.as_bytes(w)?;
        }

        // answer_comment: CString
        w.write_all(self.answer_comment.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x032a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

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

}

impl CMSG_GMSURVEY_SUBMIT {
    pub fn size(&self) -> usize {
        0
        + 4 // survey_id: u32
        + 10 * 5 // questions: GmSurveyQuestion[10]
        + self.answer_comment.len() + 1 // answer_comment: CString
    }
}

