use std::io::{Read, Write};

use crate::tbc::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_reset_failed_notify.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_reset_failed_notify.wowm#L1):
/// ```text
/// smsg SMSG_RESET_FAILED_NOTIFY = 0x0396 {
///     Map map;
/// }
/// ```
pub struct SMSG_RESET_FAILED_NOTIFY {
    pub map: Map,
}

impl crate::private::Sealed for SMSG_RESET_FAILED_NOTIFY {}
impl crate::Message for SMSG_RESET_FAILED_NOTIFY {
    const OPCODE: u32 = 0x0396;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0396, size: body_size });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            map,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_RESET_FAILED_NOTIFY {}

