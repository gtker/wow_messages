use crate::vanilla::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_leave_battlefield.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_leave_battlefield.wowm#L1):
/// ```text
/// cmsg CMSG_LEAVE_BATTLEFIELD = 0x02E1 {
///     Map map;
/// }
/// ```
pub struct CMSG_LEAVE_BATTLEFIELD {
    pub map: Map,
}

impl crate::Message for CMSG_LEAVE_BATTLEFIELD {
    const OPCODE: u32 = 0x02e1;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02E1, size: body_size as u32 });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            map,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LEAVE_BATTLEFIELD {}

