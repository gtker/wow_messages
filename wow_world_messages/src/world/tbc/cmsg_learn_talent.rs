use std::io::{Read, Write};

use crate::tbc::Talent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm#L1):
/// ```text
/// cmsg CMSG_LEARN_TALENT = 0x0251 {
///     Talent talent;
///     u32 requested_rank;
/// }
/// ```
pub struct CMSG_LEARN_TALENT {
    pub talent: Talent,
    pub requested_rank: u32,
}

impl crate::private::Sealed for CMSG_LEARN_TALENT {}
impl CMSG_LEARN_TALENT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // talent: Talent
        let talent = crate::util::read_u32_le(&mut r)?.try_into()?;

        // requested_rank: u32
        let requested_rank = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            talent,
            requested_rank,
        })
    }

}

impl crate::Message for CMSG_LEARN_TALENT {
    const OPCODE: u32 = 0x0251;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LEARN_TALENT {{").unwrap();
        // Members
        writeln!(s, "    talent = {};", self.talent.as_test_case_value()).unwrap();
        writeln!(s, "    requested_rank = {};", self.requested_rank).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 593_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "talent", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "requested_rank", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // talent: Talent
        w.write_all(&(self.talent.as_int().to_le_bytes()))?;

        // requested_rank: u32
        w.write_all(&self.requested_rank.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(593, "CMSG_LEARN_TALENT", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LEARN_TALENT {}

