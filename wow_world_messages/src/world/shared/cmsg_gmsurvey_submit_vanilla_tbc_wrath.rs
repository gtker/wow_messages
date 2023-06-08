use std::io::{Read, Write};

use crate::shared::gm_survey_question_vanilla_tbc_wrath::GmSurveyQuestion;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmsurvey_submit.wowm#L17):
/// ```text
/// cmsg CMSG_GMSURVEY_SUBMIT = 0x032A {
///     u32 survey_id;
///     GmSurveyQuestion[10] questions;
///     CString answer_comment;
/// }
/// ```
pub struct CMSG_GMSURVEY_SUBMIT {
    /// cmangos: Survey ID: found in GMSurveySurveys.dbc
    pub survey_id: u32,
    pub questions: [GmSurveyQuestion; 10],
    /// cmangos: Answer comment: Unused in stock UI, can be only set by calling Lua function
    /// cmangos: Answer comment max sizes in bytes: Vanilla - 8106:8110, TBC - 11459:11463, Wrath - 582:586
    pub answer_comment: String,
}

#[cfg(feature = "print-testcase")]
impl CMSG_GMSURVEY_SUBMIT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GMSURVEY_SUBMIT {{").unwrap();
        // Members
        writeln!(s, "    survey_id = {};", self.survey_id).unwrap();
        write!(s, "    questions = [").unwrap();
        for v in self.questions.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        question_id = {};", v.question_id).unwrap();
            writeln!(s, "        answer = {};", v.answer).unwrap();
            writeln!(s, "        comment = \"{}\";", v.comment).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    answer_comment = \"{}\";", self.answer_comment).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 810_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "survey_id", "    ");
        writeln!(s, "    /* questions: GmSurveyQuestion[10] start */").unwrap();
        for (i, v) in self.questions.iter().enumerate() {
            writeln!(s, "    /* questions: GmSurveyQuestion[10] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "question_id", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "answer", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, v.comment.len() + 1, "comment", "        ");
            writeln!(s, "    /* questions: GmSurveyQuestion[10] {i} end */").unwrap();
        }
        writeln!(s, "    /* questions: GmSurveyQuestion[10] end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, self.answer_comment.len() + 1, "answer_comment", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_GMSURVEY_SUBMIT {}
impl crate::Message for CMSG_GMSURVEY_SUBMIT {
    const OPCODE: u32 = 0x032a;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_GMSURVEY_SUBMIT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // survey_id: u32
        w.write_all(&self.survey_id.to_le_bytes())?;

        // questions: GmSurveyQuestion[10]
        for i in self.questions.iter() {
            i.write_into_vec(&mut w)?;
        }

        // answer_comment: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.answer_comment.as_bytes().iter().rev().next(), Some(&0_u8), "String `answer_comment` must not be null-terminated.");
        w.write_all(self.answer_comment.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(65..=2870).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x032A, size: body_size });
        }

        // survey_id: u32
        let survey_id = crate::util::read_u32_le(&mut r)?;

        // questions: GmSurveyQuestion[10]
        let questions = {
            let mut questions = [(); 10].map(|_| GmSurveyQuestion::default());
            for i in questions.iter_mut() {
                *i = GmSurveyQuestion::read(&mut r)?;
            }
            questions
        };

        // answer_comment: CString
        let answer_comment = {
            let answer_comment = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(answer_comment)?
        };

        Ok(Self {
            survey_id,
            questions,
            answer_comment,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GMSURVEY_SUBMIT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GMSURVEY_SUBMIT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GMSURVEY_SUBMIT {}

impl CMSG_GMSURVEY_SUBMIT {
    pub(crate) fn size(&self) -> usize {
        4 // survey_id: u32
        + self.questions.iter().fold(0, |acc, x| acc + x.size()) // questions: GmSurveyQuestion[10]
        + self.answer_comment.len() + 1 // answer_comment: CString
    }
}

