use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm#L3):
/// ```text
/// struct GmSurveyQuestion {
///     u32 question_id;
///     u8 answer;
///     CString comment;
/// }
/// ```
pub struct GmSurveyQuestion {
    /// cmangos: questions found in GMSurveyQuestions.dbc
    /// ref to i'th GMSurveySurveys.dbc field (all fields in that dbc point to fields in GMSurveyQuestions.dbc)
    ///
    pub question_id: u32,
    /// Rating: hardcoded limit of 0-5 in pre-Wrath, ranges defined in GMSurveyAnswers.dbc Wrath+
    ///
    pub answer: u8,
    /// Usage: `GMSurveyAnswerSubmit(question, rank, comment)`
    /// cmangos: Unused in stock UI, can be only set by calling Lua function
    ///
    pub comment: String,
}

impl GmSurveyQuestion {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // question_id: u32
        w.write_all(&self.question_id.to_le_bytes())?;

        // answer: u8
        w.write_all(&self.answer.to_le_bytes())?;

        // comment: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.comment.as_bytes().iter().rev().next(), Some(&0_u8), "String `comment` must not be null-terminated.");
        w.write_all(self.comment.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl GmSurveyQuestion {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // question_id: u32
        let question_id = crate::util::read_u32_le(&mut r)?;

        // answer: u8
        let answer = crate::util::read_u8_le(&mut r)?;

        // comment: CString
        let comment = {
            let comment = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(comment)?
        };

        Ok(Self {
            question_id,
            answer,
            comment,
        })
    }

}

impl GmSurveyQuestion {
    pub(crate) fn size(&self) -> usize {
        4 // question_id: u32
        + 1 // answer: u8
        + self.comment.len() + 1 // comment: CString
    }
}

