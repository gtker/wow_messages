use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm#L3):
/// ```text
/// struct GmSurveyQuestion {
///     u32 question_id;
///     u8 answer;
/// }
/// ```
pub struct GmSurveyQuestion {
    /// cmangos: questions found in GMSurveyQuestions.dbc
    ///
    pub question_id: u32,
    /// Rating: hardcoded limit of 0-5 in pre-Wrath, ranges defined in GMSurveyAnswers.dbc Wrath+
    ///
    pub answer: u8,
}

impl GmSurveyQuestion {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // question_id: u32
        w.write_all(&self.question_id.to_le_bytes())?;

        // answer: u8
        w.write_all(&self.answer.to_le_bytes())?;

        Ok(())
    }
}

impl GmSurveyQuestion {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // question_id: u32
        let question_id = crate::util::read_u32_le(r)?;

        // answer: u8
        let answer = crate::util::read_u8_le(r)?;

        Ok(Self {
            question_id,
            answer,
        })
    }

}

