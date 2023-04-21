use std::io::{Read, Write};

use crate::vanilla::{
    Area, MeetingStoneStatus,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_setqueue.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_setqueue.wowm#L14):
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

impl crate::private::Sealed for SMSG_MEETINGSTONE_SETQUEUE {}
impl crate::Message for SMSG_MEETINGSTONE_SETQUEUE {
    const OPCODE: u32 = 0x0295;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&u32::from(self.area.as_int()).to_le_bytes())?;

        // status: MeetingStoneStatus
        w.write_all(&u8::from(self.status.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0295, size: body_size as u32 });
        }

        // area: Area
        let area: Area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // status: MeetingStoneStatus
        let status: MeetingStoneStatus = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            area,
            status,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MEETINGSTONE_SETQUEUE {}

