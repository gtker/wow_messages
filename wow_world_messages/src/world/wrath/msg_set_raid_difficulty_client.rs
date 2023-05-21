use std::io::{Read, Write};

use crate::wrath::RaidDifficulty;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_set_raid_difficulty.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_set_raid_difficulty.wowm#L1):
/// ```text
/// cmsg MSG_SET_RAID_DIFFICULTY_Client = 0x04EB {
///     (u32)RaidDifficulty difficulty;
/// }
/// ```
pub struct MSG_SET_RAID_DIFFICULTY_Client {
    pub difficulty: RaidDifficulty,
}

impl crate::private::Sealed for MSG_SET_RAID_DIFFICULTY_Client {}
impl MSG_SET_RAID_DIFFICULTY_Client {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04EB, size: body_size });
        }

        // difficulty: RaidDifficulty
        let difficulty = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            difficulty,
        })
    }

}

impl crate::Message for MSG_SET_RAID_DIFFICULTY_Client {
    const OPCODE: u32 = 0x04eb;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_SET_RAID_DIFFICULTY_Client {{").unwrap();
        // Members
        writeln!(s, "    difficulty = {};", self.difficulty.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1259_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "difficulty", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // difficulty: RaidDifficulty
        w.write_all(&u32::from(self.difficulty.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_SET_RAID_DIFFICULTY_Client {}

