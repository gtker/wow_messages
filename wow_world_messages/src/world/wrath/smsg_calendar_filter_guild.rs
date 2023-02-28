use crate::wrath::CalendarMember;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_filter_guild.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_filter_guild.wowm#L8):
/// ```text
/// smsg SMSG_CALENDAR_FILTER_GUILD = 0x0438 {
///     u32 amount_of_members;
///     CalendarMember[amount_of_members] members;
/// }
/// ```
pub struct SMSG_CALENDAR_FILTER_GUILD {
    pub members: Vec<CalendarMember>,
}

impl crate::Message for SMSG_CALENDAR_FILTER_GUILD {
    const OPCODE: u32 = 0x0438;

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
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0438, size: body_size as u32 });
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
impl crate::wrath::ServerMessage for SMSG_CALENDAR_FILTER_GUILD {}

impl SMSG_CALENDAR_FILTER_GUILD {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_members: u32
        + self.members.iter().fold(0, |acc, x| acc + x.size()) // members: CalendarMember[amount_of_members]
    }
}

