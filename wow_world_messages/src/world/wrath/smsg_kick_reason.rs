use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// All that exists of this is an implementation in cmangos-tbc.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_kick_reason.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_kick_reason.wowm#L9):
/// ```text
/// smsg SMSG_KICK_REASON = 0x03C5 {
///     u8 reason;
///     CString text;
/// }
/// ```
pub struct SMSG_KICK_REASON {
    pub reason: u8,
    pub text: String,
}

impl crate::private::Sealed for SMSG_KICK_REASON {}
impl SMSG_KICK_REASON {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(2..=257).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // reason: u8
        let reason = crate::util::read_u8_le(&mut r)?;

        // text: CString
        let text = {
            let text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(text)?
        };

        Ok(Self {
            reason,
            text,
        })
    }

}

impl crate::Message for SMSG_KICK_REASON {
    const OPCODE: u32 = 0x03c5;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_KICK_REASON"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_KICK_REASON {{").unwrap();
        // Members
        writeln!(s, "    reason = {};", self.reason).unwrap();
        writeln!(s, "    text = \"{}\";", self.text).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 965_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "reason", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.text.len() + 1, "text", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // reason: u8
        w.write_all(&self.reason.to_le_bytes())?;

        // text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.text.as_bytes().iter().next_back(), Some(&0_u8), "String `text` must not be null-terminated.");
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(965, "SMSG_KICK_REASON", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_KICK_REASON {}

impl SMSG_KICK_REASON {
    pub(crate) fn size(&self) -> usize {
        1 // reason: u8
        + self.text.len() + 1 // text: CString
    }
}

