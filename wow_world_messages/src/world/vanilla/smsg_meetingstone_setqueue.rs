use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::Area;
use crate::world::vanilla::MeetingStoneStatus;
use crate::world::vanilla::ServerMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_setqueue.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_setqueue.wowm#L12):
/// ```text
/// smsg SMSG_MEETINGSTONE_SETQUEUE = 0x0295 {
///     Area area;
///     MeetingStoneStatus status;
/// }
/// ```
pub struct SMSG_MEETINGSTONE_SETQUEUE {
    pub area: Area,
    pub status: MeetingStoneStatus,
}

impl crate::Message for SMSG_MEETINGSTONE_SETQUEUE {
    const OPCODE: u32 = 0x0295;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // status: MeetingStoneStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        // status: MeetingStoneStatus
        let status: MeetingStoneStatus = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            area,
            status,
        })
    }

}
impl ServerMessage for SMSG_MEETINGSTONE_SETQUEUE {}

