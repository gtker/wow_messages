use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlemaster_join.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlemaster_join.wowm#L3):
/// ```text
/// cmsg CMSG_BATTLEMASTER_JOIN = 0x02EE {
///     Guid guid;
///     Map map;
///     u32 instance_id;
///     u8 join_as_group;
/// }
/// ```
pub struct CMSG_BATTLEMASTER_JOIN {
    pub guid: Guid,
    pub map: Map,
    pub instance_id: u32,
    pub join_as_group: u8,
}

impl crate::Message for CMSG_BATTLEMASTER_JOIN {
    const OPCODE: u32 = 0x02ee;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        // join_as_group: u8
        w.write_all(&self.join_as_group.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // instance_id: u32
        let instance_id = crate::util::read_u32_le(r)?;

        // join_as_group: u8
        let join_as_group = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            map,
            instance_id,
            join_as_group,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_BATTLEMASTER_JOIN {}

