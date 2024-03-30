use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_bug.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_bug.wowm#L3):
/// ```text
/// cmsg CMSG_BUG = 0x01CA {
///     u32 suggestion;
///     SizedCString content;
///     SizedCString bug_type;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_BUG {
    /// cmangos/vmangos/mangoszero: If 0 received bug report, else received suggestion
    pub suggestion: u32,
    pub content: String,
    pub bug_type: String,
}

impl crate::private::Sealed for CMSG_BUG {}
impl CMSG_BUG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(14..=16012).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // suggestion: u32
        let suggestion = crate::util::read_u32_le(&mut r)?;

        // content: SizedCString
        let content = {
            let content = crate::util::read_u32_le(&mut r)?;
            let content = crate::util::read_sized_c_string_to_vec(&mut r, content)?;
            String::from_utf8(content)?
        };

        // bug_type: SizedCString
        let bug_type = {
            let bug_type = crate::util::read_u32_le(&mut r)?;
            let bug_type = crate::util::read_sized_c_string_to_vec(&mut r, bug_type)?;
            String::from_utf8(bug_type)?
        };

        Ok(Self {
            suggestion,
            content,
            bug_type,
        })
    }

}

impl crate::Message for CMSG_BUG {
    const OPCODE: u32 = 0x01ca;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_BUG"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_BUG {{").unwrap();
        // Members
        writeln!(s, "    suggestion = {};", self.suggestion).unwrap();
        writeln!(s, "    content = \"{}\";", self.content).unwrap();
        writeln!(s, "    bug_type = \"{}\";", self.bug_type).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 458_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "suggestion", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.content.len() + 5, "content", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.bug_type.len() + 5, "bug_type", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // suggestion: u32
        w.write_all(&self.suggestion.to_le_bytes())?;

        // content: SizedCString
        w.write_all(&((self.content.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.content.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // bug_type: SizedCString
        w.write_all(&((self.bug_type.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.bug_type.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(458, "CMSG_BUG", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BUG {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BUG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BUG {}

impl CMSG_BUG {
    pub(crate) fn size(&self) -> usize {
        4 // suggestion: u32
        + self.content.len() + 5 // content: SizedCString
        + self.bug_type.len() + 5 // bug_type: SizedCString
    }
}

