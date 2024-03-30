use std::io::{Read, Write};

use crate::Guid;
use crate::shared::level_vanilla_tbc_wrath::Level;
use crate::wrath::CalendarMember;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_arena_team.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_arena_team.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_ARENA_TEAM = 0x0439 {
///     u32 amount_of_members;
///     CalendarMember[amount_of_members] members;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_CALENDAR_ARENA_TEAM {
    pub members: Vec<CalendarMember>,
}

impl crate::private::Sealed for SMSG_CALENDAR_ARENA_TEAM {}
impl SMSG_CALENDAR_ARENA_TEAM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(&mut r)?;

        // members: CalendarMember[amount_of_members]
        let members = {
            let mut members = Vec::with_capacity(amount_of_members as usize);

            let allocation_size = u64::from(amount_of_members) * 2;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_members {
                members.push(CalendarMember::read(&mut r)?);
            }
            members
        };

        Ok(Self {
            members,
        })
    }

}

impl crate::Message for SMSG_CALENDAR_ARENA_TEAM {
    const OPCODE: u32 = 0x0439;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CALENDAR_ARENA_TEAM"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_ARENA_TEAM {{").unwrap();
        // Members
        writeln!(s, "    amount_of_members = {};", self.members.len()).unwrap();
        writeln!(s, "    members = [").unwrap();
        for v in self.members.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            member = {};", v.member.guid()).unwrap();
            writeln!(s, "            level = {};", v.level.as_int()).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1081_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_members", "    ");
        if !self.members.is_empty() {
            writeln!(s, "    /* members: CalendarMember[amount_of_members] start */").unwrap();
            for (i, v) in self.members.iter().enumerate() {
                writeln!(s, "    /* members: CalendarMember[amount_of_members] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&v.member), "member", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "level", "        ");
                writeln!(s, "    /* members: CalendarMember[amount_of_members] {i} end */").unwrap();
            }
            writeln!(s, "    /* members: CalendarMember[amount_of_members] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // members: CalendarMember[amount_of_members]
        for i in self.members.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1081, "SMSG_CALENDAR_ARENA_TEAM", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_ARENA_TEAM {}

impl SMSG_CALENDAR_ARENA_TEAM {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_members: u32
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: CalendarMember[amount_of_members]
    }
}

