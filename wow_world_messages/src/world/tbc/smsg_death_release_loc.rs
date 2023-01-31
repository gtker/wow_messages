use std::convert::{TryFrom, TryInto};
use crate::tbc::Vector3d;
use crate::tbc::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_death_release_loc.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_death_release_loc.wowm#L1):
/// ```text
/// smsg SMSG_DEATH_RELEASE_LOC = 0x0378 {
///     Map map;
///     Vector3d position;
/// }
/// ```
pub struct SMSG_DEATH_RELEASE_LOC {
    pub map: Map,
    pub position: Vector3d,
}

impl crate::Message for SMSG_DEATH_RELEASE_LOC {
    const OPCODE: u32 = 0x0378;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0378, size: body_size as u32 });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        Ok(Self {
            map,
            position,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DEATH_RELEASE_LOC {}

