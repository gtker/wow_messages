use std::io::{Read, Write};

use crate::tbc::{
    Map, Vector3d,
};

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

impl crate::private::Sealed for SMSG_DEATH_RELEASE_LOC {}
impl crate::Message for SMSG_DEATH_RELEASE_LOC {
    const OPCODE: u32 = 0x0378;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0378, size: body_size });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        Ok(Self {
            map,
            position,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DEATH_RELEASE_LOC {}

