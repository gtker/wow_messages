use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Area;
use crate::world::version_1_12::Map;
use crate::world::version_1_12::Vector3d;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Set new hearthstone location.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_bindpointupdate.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_bindpointupdate.wowm#L5):
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

impl ServerMessage for SMSG_BINDPOINTUPDATE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // position: Vector3d
        self.position.write_into_vec(w)?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0155;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        20
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            position,
            map,
            area,
        })
    }

}

