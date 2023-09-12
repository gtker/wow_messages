use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_summon_response.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_summon_response.wowm#L7):
/// ```text
/// cmsg CMSG_SUMMON_RESPONSE = 0x02AC {
///     Guid summoner;
///     Bool agree;
/// }
/// ```
pub struct CMSG_SUMMON_RESPONSE {
    pub summoner: Guid,
    pub agree: bool,
}

impl crate::private::Sealed for CMSG_SUMMON_RESPONSE {}
impl CMSG_SUMMON_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // summoner: Guid
        let summoner = crate::util::read_guid(&mut r)?;

        // agree: Bool
        let agree = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            summoner,
            agree,
        })
    }

}

impl crate::Message for CMSG_SUMMON_RESPONSE {
    const OPCODE: u32 = 0x02ac;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SUMMON_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SUMMON_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    summoner = {};", self.summoner.guid()).unwrap();
        writeln!(s, "    agree = {};", if self.agree { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 13_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 684_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "summoner", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "agree", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // summoner: Guid
        w.write_all(&self.summoner.guid().to_le_bytes())?;

        // agree: Bool
        w.write_all(u8::from(self.agree).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(684, "CMSG_SUMMON_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SUMMON_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SUMMON_RESPONSE {}

