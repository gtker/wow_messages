use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_time_skipped.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_time_skipped.wowm#L1):
/// ```text
/// smsg MSG_MOVE_TIME_SKIPPED_Server = 0x0319 {
///     PackedGuid player;
///     u32 time_skipped;
/// }
/// ```
pub struct MSG_MOVE_TIME_SKIPPED_Server {
    pub player: Guid,
    pub time_skipped: u32,
}

impl crate::private::Sealed for MSG_MOVE_TIME_SKIPPED_Server {}
impl MSG_MOVE_TIME_SKIPPED_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=13).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: PackedGuid
        let player = crate::util::read_packed_guid(&mut r)?;

        // time_skipped: u32
        let time_skipped = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            player,
            time_skipped,
        })
    }

}

impl crate::Message for MSG_MOVE_TIME_SKIPPED_Server {
    const OPCODE: u32 = 0x0319;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_MOVE_TIME_SKIPPED_Server"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_MOVE_TIME_SKIPPED_Server {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    time_skipped = {};", self.time_skipped).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 793_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.player), "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_skipped", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        crate::util::write_packed_guid(&self.player, &mut w)?;

        // time_skipped: u32
        w.write_all(&self.time_skipped.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(793, "MSG_MOVE_TIME_SKIPPED_Server", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MOVE_TIME_SKIPPED_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_MOVE_TIME_SKIPPED_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_TIME_SKIPPED_Server {}

impl MSG_MOVE_TIME_SKIPPED_Server {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + 4 // time_skipped: u32
    }
}

