use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::GmSurveyQuestion;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm#L12):
/// ```text
/// cmsg CMSG_GMSURVEY_SUBMIT = 0x032A {
///     u32 survey_id;
///     GmSurveyQuestion[10] questions;
///     CString answer_comment;
/// }
/// ```
pub struct CMSG_GMSURVEY_SUBMIT {
    /// cmangos: Survey ID: found in GMSurveySurveys.dbc
    ///
    pub survey_id: u32,
    pub questions: [GmSurveyQuestion; 10],
    /// cmangos: Answer comment: Unused in stock UI, can be only set by calling Lua function
    /// cmangos: Answer comment max sizes in bytes: Vanilla - 8106:8110, TBC - 11459:11463, Wrath - 582:586
    ///
    pub answer_comment: String,
}

impl ClientMessage for CMSG_GMSURVEY_SUBMIT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // survey_id: u32
        w.write_all(&self.survey_id.to_le_bytes())?;

        // questions: GmSurveyQuestion[10]
        for i in self.questions.iter() {
            i.write_into_vec(w)?;
        }

        // answer_comment: CString
        w.write_all(self.answer_comment.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x032a;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // survey_id: u32
        let survey_id = crate::util::read_u32_le(r)?;

        // questions: GmSurveyQuestion[10]
        let mut questions = [GmSurveyQuestion::default(); 10];
        for i in questions.iter_mut() {
            *i = GmSurveyQuestion::read(r)?;
        }

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
    pub(crate) fn size(&self) -> usize {
        4 // survey_id: u32
        + 10 * 5 // questions: GmSurveyQuestion[10]
        + self.answer_comment.len() + 1 // answer_comment: CString
    }
}

