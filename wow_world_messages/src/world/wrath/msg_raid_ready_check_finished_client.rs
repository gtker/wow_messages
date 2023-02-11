use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// This MSG does not appear to have an SMSG version.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_raid_ready_check_finished.wowm:6`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_raid_ready_check_finished.wowm#L6):
/// ```text
/// cmsg MSG_RAID_READY_CHECK_FINISHED_Client = 0x03C6 {
/// }
/// ```
pub struct MSG_RAID_READY_CHECK_FINISHED_Client {
}

impl crate::Message for MSG_RAID_READY_CHECK_FINISHED_Client {
    const OPCODE: u32 = 0x03c6;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03C6, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_RAID_READY_CHECK_FINISHED_Client {}

