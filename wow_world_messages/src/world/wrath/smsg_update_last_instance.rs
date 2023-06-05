use std::io::{Read, Write};

use crate::wrath::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_update_last_instance.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_last_instance.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_LAST_INSTANCE = 0x0320 {
///     Map map;
/// }
/// ```
pub struct SMSG_UPDATE_LAST_INSTANCE {
    pub map: Map,
}

impl crate::private::Sealed for SMSG_UPDATE_LAST_INSTANCE {}
impl crate::Message for SMSG_UPDATE_LAST_INSTANCE {
    const OPCODE: u32 = 0x0320;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0320, size: body_size });
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            map,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_LAST_INSTANCE {}

