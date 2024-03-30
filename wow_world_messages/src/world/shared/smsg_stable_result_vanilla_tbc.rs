use std::io::{Read, Write};

use wow_world_base::shared::stable_result_vanilla_tbc::StableResult;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_stable_result.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_stable_result.wowm#L16):
/// ```text
/// smsg SMSG_STABLE_RESULT = 0x0273 {
///     StableResult result;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_STABLE_RESULT {
    pub result: StableResult,
}

impl crate::private::Sealed for SMSG_STABLE_RESULT {}
impl SMSG_STABLE_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // result: StableResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}

impl crate::Message for SMSG_STABLE_RESULT {
    const OPCODE: u32 = 0x0273;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_STABLE_RESULT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_STABLE_RESULT {{").unwrap();
        // Members
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 3_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 627_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: StableResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(627, "SMSG_STABLE_RESULT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_STABLE_RESULT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_STABLE_RESULT {}

