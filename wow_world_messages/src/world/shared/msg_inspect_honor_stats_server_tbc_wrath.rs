use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_inspect_honor_stats_server.wowm:28`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_inspect_honor_stats_server.wowm#L28):
/// ```text
/// smsg MSG_INSPECT_HONOR_STATS_Server = 0x02D6 {
///     Guid guid;
///     u8 amount_of_honor;
///     u32 kills;
///     u32 honor_today;
///     u32 honor_yesterday;
///     u32 lifetime_honorable_kills;
/// }
/// ```
pub struct MSG_INSPECT_HONOR_STATS_Server {
    pub guid: Guid,
    pub amount_of_honor: u8,
    pub kills: u32,
    pub honor_today: u32,
    pub honor_yesterday: u32,
    pub lifetime_honorable_kills: u32,
}

impl crate::private::Sealed for MSG_INSPECT_HONOR_STATS_Server {}
impl MSG_INSPECT_HONOR_STATS_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 25 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02D6, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // amount_of_honor: u8
        let amount_of_honor = crate::util::read_u8_le(&mut r)?;

        // kills: u32
        let kills = crate::util::read_u32_le(&mut r)?;

        // honor_today: u32
        let honor_today = crate::util::read_u32_le(&mut r)?;

        // honor_yesterday: u32
        let honor_yesterday = crate::util::read_u32_le(&mut r)?;

        // lifetime_honorable_kills: u32
        let lifetime_honorable_kills = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            amount_of_honor,
            kills,
            honor_today,
            honor_yesterday,
            lifetime_honorable_kills,
        })
    }

}

impl crate::Message for MSG_INSPECT_HONOR_STATS_Server {
    const OPCODE: u32 = 0x02d6;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_INSPECT_HONOR_STATS_Server {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    amount_of_honor = {};", self.amount_of_honor).unwrap();
        writeln!(s, "    kills = {};", self.kills).unwrap();
        writeln!(s, "    honor_today = {};", self.honor_today).unwrap();
        writeln!(s, "    honor_yesterday = {};", self.honor_yesterday).unwrap();
        writeln!(s, "    lifetime_honorable_kills = {};", self.lifetime_honorable_kills).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 27_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 726_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_honor", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "kills", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_today", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_yesterday", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "lifetime_honorable_kills", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        25
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // amount_of_honor: u8
        w.write_all(&self.amount_of_honor.to_le_bytes())?;

        // kills: u32
        w.write_all(&self.kills.to_le_bytes())?;

        // honor_today: u32
        w.write_all(&self.honor_today.to_le_bytes())?;

        // honor_yesterday: u32
        w.write_all(&self.honor_yesterday.to_le_bytes())?;

        // lifetime_honorable_kills: u32
        w.write_all(&self.lifetime_honorable_kills.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_INSPECT_HONOR_STATS_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_INSPECT_HONOR_STATS_Server {}

