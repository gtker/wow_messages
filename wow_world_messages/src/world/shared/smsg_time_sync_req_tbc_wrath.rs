use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_time_sync_req.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_time_sync_req.wowm#L3):
/// ```text
/// smsg SMSG_TIME_SYNC_REQ = 0x0390 {
///     u32 time_sync;
/// }
/// ```
pub struct SMSG_TIME_SYNC_REQ {
    pub time_sync: u32,
}

impl crate::private::Sealed for SMSG_TIME_SYNC_REQ {}
impl crate::Message for SMSG_TIME_SYNC_REQ {
    const OPCODE: u32 = 0x0390;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // time_sync: u32
        w.write_all(&self.time_sync.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0390, size: body_size });
        }

        // time_sync: u32
        let time_sync = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            time_sync,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TIME_SYNC_REQ {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TIME_SYNC_REQ {}

