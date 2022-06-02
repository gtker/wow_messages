use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Area;
use crate::world::version_1_12::MeetingStoneStatus;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

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

impl ServerMessage for SMSG_MEETINGSTONE_SETQUEUE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // status: MeetingStoneStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0295;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        5
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
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

