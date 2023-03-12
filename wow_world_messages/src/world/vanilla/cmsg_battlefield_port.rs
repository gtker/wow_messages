use std::io::{Read, Write};

use crate::vanilla::{
    BattlefieldPortAction,
    Map,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_port.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_port.wowm#L8):
/// ```text
/// cmsg CMSG_BATTLEFIELD_PORT = 0x02D5 {
///     Map map;
///     BattlefieldPortAction action;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_PORT {
    pub map: Map,
    pub action: BattlefieldPortAction,
}

impl crate::Message for CMSG_BATTLEFIELD_PORT {
    const OPCODE: u32 = 0x02d5;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // action: BattlefieldPortAction
        w.write_all(&u8::from(self.action.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02D5, size: body_size as u32 });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // action: BattlefieldPortAction
        let action: BattlefieldPortAction = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            map,
            action,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BATTLEFIELD_PORT {}

