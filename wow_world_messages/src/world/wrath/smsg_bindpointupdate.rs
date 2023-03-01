use crate:: {
};
use crate::wrath::Vector3d;
use crate::wrath::Area;
use crate::wrath::Map;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Inform the client of a their hearthstone location.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_bindpointupdate.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_bindpointupdate.wowm#L1):
/// ```text
/// smsg SMSG_BINDPOINTUPDATE = 0x0155 {
///     Vector3d position;
///     Map map;
///     Area area;
/// }
/// ```
pub struct SMSG_BINDPOINTUPDATE {
    pub position: Vector3d,
    pub map: Map,
    pub area: Area,
}

impl crate::Message for SMSG_BINDPOINTUPDATE {
    const OPCODE: u32 = 0x0155;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // area: Area
        w.write_all(&u32::from(self.area.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0155, size: body_size as u32 });
        }

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // area: Area
        let area: Area = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            position,
            map,
            area,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BINDPOINTUPDATE {}

