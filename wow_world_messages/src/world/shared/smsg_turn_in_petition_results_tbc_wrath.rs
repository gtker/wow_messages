use std::io::{Read, Write};

use wow_world_base::shared::petition_result_tbc_wrath::PetitionResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_turn_in_petition_results.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_turn_in_petition_results.wowm#L7):
/// ```text
/// smsg SMSG_TURN_IN_PETITION_RESULTS = 0x01C5 {
///     PetitionResult result;
/// }
/// ```
pub struct SMSG_TURN_IN_PETITION_RESULTS {
    pub result: PetitionResult,
}

impl crate::private::Sealed for SMSG_TURN_IN_PETITION_RESULTS {}
impl SMSG_TURN_IN_PETITION_RESULTS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // result: PetitionResult
        let result = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}

impl crate::Message for SMSG_TURN_IN_PETITION_RESULTS {
    const OPCODE: u32 = 0x01c5;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_TURN_IN_PETITION_RESULTS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TURN_IN_PETITION_RESULTS {{").unwrap();
        // Members
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 453_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "result", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: PetitionResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(453, "SMSG_TURN_IN_PETITION_RESULTS", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TURN_IN_PETITION_RESULTS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TURN_IN_PETITION_RESULTS {}

