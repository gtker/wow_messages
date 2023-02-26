use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_time_sync_resp.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_time_sync_resp.wowm#L3):
/// ```text
/// cmsg CMSG_TIME_SYNC_RESP = 0x0391 {
///     u32 time_sync;
///     u32 client_ticks;
/// }
/// ```
pub struct CMSG_TIME_SYNC_RESP {
    /// Can be used to check if the client is still properly in sync
    /// This should be the same as the counter sent in [`SMSG_TIME_SYNC_REQ`](crate::tbc::SMSG_TIME_SYNC_REQ).
    ///
    pub time_sync: u32,
    /// You can check this against expected values to estimate client latency
    ///
    pub client_ticks: u32,
}

impl crate::Message for CMSG_TIME_SYNC_RESP {
    const OPCODE: u32 = 0x0391;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // time_sync: u32
        w.write_all(&self.time_sync.to_le_bytes())?;

        // client_ticks: u32
        w.write_all(&self.client_ticks.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0391, size: body_size as u32 });
        }

        // time_sync: u32
        let time_sync = crate::util::read_u32_le(r)?;

        // client_ticks: u32
        let client_ticks = crate::util::read_u32_le(r)?;

        Ok(Self {
            time_sync,
            client_ticks,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_TIME_SYNC_RESP {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_TIME_SYNC_RESP {}

