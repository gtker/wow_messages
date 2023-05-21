use std::io::{Read, Write};

use wow_world_base::shared::dungeon_difficulty_tbc_wrath::DungeonDifficulty;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/msg_set_dungeon_difficulty.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/msg_set_dungeon_difficulty.wowm#L1):
/// ```text
/// smsg MSG_SET_DUNGEON_DIFFICULTY_Server = 0x0329 {
///     (u32)DungeonDifficulty difficulty;
///     u32 unknown1;
///     Bool32 is_in_group;
/// }
/// ```
pub struct MSG_SET_DUNGEON_DIFFICULTY_Server {
    pub difficulty: DungeonDifficulty,
    /// ArcEmu hardcodes this to 1
    pub unknown1: u32,
    pub is_in_group: bool,
}

impl crate::private::Sealed for MSG_SET_DUNGEON_DIFFICULTY_Server {}
impl MSG_SET_DUNGEON_DIFFICULTY_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0329, size: body_size });
        }

        // difficulty: DungeonDifficulty
        let difficulty = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // is_in_group: Bool32
        let is_in_group = crate::util::read_u32_le(&mut r)? != 0;

        Ok(Self {
            difficulty,
            unknown1,
            is_in_group,
        })
    }

}

impl crate::Message for MSG_SET_DUNGEON_DIFFICULTY_Server {
    const OPCODE: u32 = 0x0329;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_SET_DUNGEON_DIFFICULTY_Server {{").unwrap();
        // Members
        writeln!(s, "    difficulty = {};", self.difficulty.as_test_case_value()).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    is_in_group = {};", if self.is_in_group { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 809_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "difficulty", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "is_in_group", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // difficulty: DungeonDifficulty
        w.write_all(&u32::from(self.difficulty.as_int()).to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // is_in_group: Bool32
        w.write_all(u32::from(self.is_in_group).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_SET_DUNGEON_DIFFICULTY_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_SET_DUNGEON_DIFFICULTY_Server {}

