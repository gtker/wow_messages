use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_set_contact_notes.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_set_contact_notes.wowm#L1):
/// ```text
/// cmsg CMSG_SET_CONTACT_NOTES = 0x006B {
///     Guid player;
///     CString note;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_SET_CONTACT_NOTES {
    pub player: Guid,
    pub note: String,
}

impl crate::private::Sealed for CMSG_SET_CONTACT_NOTES {}
impl CMSG_SET_CONTACT_NOTES {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // note: CString
        let note = {
            let note = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(note)?
        };

        Ok(Self {
            player,
            note,
        })
    }

}

impl crate::Message for CMSG_SET_CONTACT_NOTES {
    const OPCODE: u32 = 0x006b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SET_CONTACT_NOTES"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_CONTACT_NOTES {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    note = \"{}\";", self.note).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 107_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.note.len() + 1, "note", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // note: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.note.as_bytes().iter().next_back(), Some(&0_u8), "String `note` must not be null-terminated.");
        w.write_all(self.note.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(107, "CMSG_SET_CONTACT_NOTES", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_CONTACT_NOTES {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_CONTACT_NOTES {}

impl CMSG_SET_CONTACT_NOTES {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + self.note.len() + 1 // note: CString
    }
}

