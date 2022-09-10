use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_time_sync_req.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_time_sync_req.wowm#L3):
/// ```text
/// smsg SMSG_TIME_SYNC_REQ = 0x0390 {
///     u32 millisecond_counter;
/// }
/// ```
pub struct SMSG_TIME_SYNC_REQ {
    pub millisecond_counter: u32,
}

impl crate::Message for SMSG_TIME_SYNC_REQ {
    const OPCODE: u32 = 0x0390;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // millisecond_counter: u32
        w.write_all(&self.millisecond_counter.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // millisecond_counter: u32
        let millisecond_counter = crate::util::read_u32_le(r)?;

        Ok(Self {
            millisecond_counter,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_TIME_SYNC_REQ {}

