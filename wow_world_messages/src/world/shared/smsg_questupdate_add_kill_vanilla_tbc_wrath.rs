use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_add_kill.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_add_kill.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_ADD_KILL = 0x0199 {
///     u32 quest_id;
///     u32 creature_id;
///     u32 kill_count;
///     u32 required_kill_count;
///     Guid guid;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_ADD_KILL {
    pub quest_id: u32,
    /// Unsure of name
    ///
    pub creature_id: u32,
    pub kill_count: u32,
    pub required_kill_count: u32,
    pub guid: Guid,
}

#[cfg(feature = "print-testcase")]
impl SMSG_QUESTUPDATE_ADD_KILL {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTUPDATE_ADD_KILL {{").unwrap();
        // Members
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();
        writeln!(s, "    creature_id = {};", self.creature_id).unwrap();
        writeln!(s, "    kill_count = {};", self.kill_count).unwrap();
        writeln!(s, "    required_kill_count = {};", self.required_kill_count).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 26_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 409_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "creature_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "kill_count", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "required_kill_count", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_QUESTUPDATE_ADD_KILL {}
impl crate::Message for SMSG_QUESTUPDATE_ADD_KILL {
    const OPCODE: u32 = 0x0199;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_QUESTUPDATE_ADD_KILL::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // creature_id: u32
        w.write_all(&self.creature_id.to_le_bytes())?;

        // kill_count: u32
        w.write_all(&self.kill_count.to_le_bytes())?;

        // required_kill_count: u32
        w.write_all(&self.required_kill_count.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0199, size: body_size });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // creature_id: u32
        let creature_id = crate::util::read_u32_le(&mut r)?;

        // kill_count: u32
        let kill_count = crate::util::read_u32_le(&mut r)?;

        // required_kill_count: u32
        let required_kill_count = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            quest_id,
            creature_id,
            kill_count,
            required_kill_count,
            guid,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_QUESTUPDATE_ADD_KILL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTUPDATE_ADD_KILL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTUPDATE_ADD_KILL {}

