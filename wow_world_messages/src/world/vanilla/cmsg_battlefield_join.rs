use std::io::{Read, Write};

use crate::vanilla::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm#L3):
/// ```text
/// cmsg CMSG_BATTLEFIELD_JOIN = 0x023E {
///     Map map;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_JOIN {
    pub map: Map,
}

impl crate::private::Sealed for CMSG_BATTLEFIELD_JOIN {}
impl crate::Message for CMSG_BATTLEFIELD_JOIN {
    const OPCODE: u32 = 0x023e;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x023E, size: body_size });
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            map,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BATTLEFIELD_JOIN {}

