use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:3361`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L3361):
/// ```text
/// struct GmSurveyQuestion {
///     u32 question_id;
///     u8 answer;
/// }
/// ```
pub struct GmSurveyQuestion {
    pub question_id: u32,
    pub answer: u8,
}

impl ReadableAndWritable for GmSurveyQuestion {
    type Error = std::io::Error;

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

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // question_id: u32
        w.write_all(&self.question_id.to_le_bytes())?;

        // answer: u8
        w.write_all(&self.answer.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for GmSurveyQuestion {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for GmSurveyQuestion {
    fn maximum_possible_size() -> usize {
        4 // question_id: u32
        + 1 // answer: u8
    }
}

