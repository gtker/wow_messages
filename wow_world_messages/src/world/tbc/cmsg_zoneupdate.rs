use crate::tbc::Area;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent by the client whenever it reaches a new area.
///
/// The client does not send an accurate area. For example when going to Sen'jin Village, the client will send `DUROTAR` (0x0E) and not `SENJIN_VILLAGE` (0x16F).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm#L1):
/// ```text
/// cmsg CMSG_ZONEUPDATE = 0x01F4 {
///     Area area;
/// }
/// ```
pub struct CMSG_ZONEUPDATE {
    pub area: Area,
}

impl crate::Message for CMSG_ZONEUPDATE {
    const OPCODE: u32 = 0x01f4;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F4, size: body_size as u32 });
        }

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            area,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ZONEUPDATE {}

