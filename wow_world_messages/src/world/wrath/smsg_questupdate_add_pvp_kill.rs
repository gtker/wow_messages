use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_add_pvp_kill.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_add_pvp_kill.wowm#L1):
/// ```text
/// smsg SMSG_QUESTUPDATE_ADD_PVP_KILL = 0x046F {
///     u32 quest_id;
///     u32 count;
///     u32 players_slain;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_QUESTUPDATE_ADD_PVP_KILL {
    pub quest_id: u32,
    pub count: u32,
    pub players_slain: u32,
}

impl crate::private::Sealed for SMSG_QUESTUPDATE_ADD_PVP_KILL {}
impl SMSG_QUESTUPDATE_ADD_PVP_KILL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // count: u32
        let count = crate::util::read_u32_le(&mut r)?;

        // players_slain: u32
        let players_slain = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            quest_id,
            count,
            players_slain,
        })
    }

}

impl crate::Message for SMSG_QUESTUPDATE_ADD_PVP_KILL {
    const OPCODE: u32 = 0x046f;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_QUESTUPDATE_ADD_PVP_KILL"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTUPDATE_ADD_PVP_KILL {{").unwrap();
        // Members
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();
        writeln!(s, "    count = {};", self.count).unwrap();
        writeln!(s, "    players_slain = {};", self.players_slain).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1135_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "count", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "players_slain", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // count: u32
        w.write_all(&self.count.to_le_bytes())?;

        // players_slain: u32
        w.write_all(&self.players_slain.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1135, "SMSG_QUESTUPDATE_ADD_PVP_KILL", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTUPDATE_ADD_PVP_KILL {}

