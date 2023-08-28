use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_confirm_accept.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_confirm_accept.wowm#L3):
/// ```text
/// smsg SMSG_QUEST_CONFIRM_ACCEPT = 0x019C {
///     u32 quest_id;
///     CString quest_title;
///     Guid guid;
/// }
/// ```
pub struct SMSG_QUEST_CONFIRM_ACCEPT {
    pub quest_id: u32,
    pub quest_title: String,
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_QUEST_CONFIRM_ACCEPT {}
impl SMSG_QUEST_CONFIRM_ACCEPT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(13..=268).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // quest_title: CString
        let quest_title = {
            let quest_title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(quest_title)?
        };

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            quest_id,
            quest_title,
            guid,
        })
    }

}

impl crate::Message for SMSG_QUEST_CONFIRM_ACCEPT {
    const OPCODE: u32 = 0x019c;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUEST_CONFIRM_ACCEPT {{").unwrap();
        // Members
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();
        writeln!(s, "    quest_title = \"{}\";", self.quest_title).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 412_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.quest_title.len() + 1, "quest_title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // quest_title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.quest_title.as_bytes().iter().next_back(), Some(&0_u8), "String `quest_title` must not be null-terminated.");
        w.write_all(self.quest_title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(412, "SMSG_QUEST_CONFIRM_ACCEPT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_QUEST_CONFIRM_ACCEPT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUEST_CONFIRM_ACCEPT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUEST_CONFIRM_ACCEPT {}

impl SMSG_QUEST_CONFIRM_ACCEPT {
    pub(crate) fn size(&self) -> usize {
        4 // quest_id: u32
        + self.quest_title.len() + 1 // quest_title: CString
        + 8 // guid: Guid
    }
}

