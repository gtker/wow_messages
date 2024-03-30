use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_set_lfg_comment.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_set_lfg_comment.wowm#L1):
/// ```text
/// cmsg CMSG_SET_LFG_COMMENT = 0x0366 {
///     CString comment;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_SET_LFG_COMMENT {
    pub comment: String,
}

impl crate::private::Sealed for CMSG_SET_LFG_COMMENT {}
impl CMSG_SET_LFG_COMMENT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // comment: CString
        let comment = {
            let comment = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(comment)?
        };

        Ok(Self {
            comment,
        })
    }

}

impl crate::Message for CMSG_SET_LFG_COMMENT {
    const OPCODE: u32 = 0x0366;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SET_LFG_COMMENT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_LFG_COMMENT {{").unwrap();
        // Members
        writeln!(s, "    comment = \"{}\";", self.comment).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 870_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.comment.len() + 1, "comment", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // comment: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.comment.as_bytes().iter().next_back(), Some(&0_u8), "String `comment` must not be null-terminated.");
        w.write_all(self.comment.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(870, "CMSG_SET_LFG_COMMENT", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_LFG_COMMENT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_LFG_COMMENT {}

impl CMSG_SET_LFG_COMMENT {
    pub(crate) fn size(&self) -> usize {
        self.comment.len() + 1 // comment: CString
    }
}

