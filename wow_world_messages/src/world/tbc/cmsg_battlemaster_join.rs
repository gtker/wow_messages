use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlemaster_join.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlemaster_join.wowm#L1):
/// ```text
/// cmsg CMSG_BATTLEMASTER_JOIN = 0x02EE {
///     Guid guid;
///     Map map;
///     u32 instance_id;
///     Bool join_as_group;
/// }
/// ```
pub struct CMSG_BATTLEMASTER_JOIN {
    /// vmangos: battlemaster guid, or player guid if joining queue from BG portal
    ///
    pub guid: Guid,
    pub map: Map,
    /// vmangos: 0 if First Available selected
    ///
    pub instance_id: u32,
    pub join_as_group: bool,
}

impl crate::private::Sealed for CMSG_BATTLEMASTER_JOIN {}
impl crate::Message for CMSG_BATTLEMASTER_JOIN {
    const OPCODE: u32 = 0x02ee;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        // join_as_group: Bool
        w.write_all(u8::from(self.join_as_group).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02EE, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // instance_id: u32
        let instance_id = crate::util::read_u32_le(&mut r)?;

        // join_as_group: Bool
        let join_as_group = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            guid,
            map,
            instance_id,
            join_as_group,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BATTLEMASTER_JOIN {}

