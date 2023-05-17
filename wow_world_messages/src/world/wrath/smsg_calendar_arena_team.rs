use std::io::{Read, Write};

use crate::wrath::CalendarMember;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_arena_team.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_arena_team.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_ARENA_TEAM = 0x0439 {
///     u32 amount_of_members;
///     CalendarMember[amount_of_members] members;
/// }
/// ```
pub struct SMSG_CALENDAR_ARENA_TEAM {
    pub members: Vec<CalendarMember>,
}

impl crate::private::Sealed for SMSG_CALENDAR_ARENA_TEAM {}
impl crate::Message for SMSG_CALENDAR_ARENA_TEAM {
    const OPCODE: u32 = 0x0439;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // members: CalendarMember[amount_of_members]
        for i in self.members.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0439, size: body_size });
        }

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(&mut r)?;

        // members: CalendarMember[amount_of_members]
        let members = {
            let mut members = Vec::with_capacity(amount_of_members as usize);
            for i in 0..amount_of_members {
                members.push(CalendarMember::read(&mut r)?);
            }
            members
        };

        Ok(Self {
            members,
        })
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

