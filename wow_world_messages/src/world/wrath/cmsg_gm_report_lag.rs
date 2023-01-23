use std::convert::{TryFrom, TryInto};
use crate::world::wrath::Vector3d;
use crate::world::wrath::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_gm_report_lag.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_gm_report_lag.wowm#L1):
/// ```text
/// cmsg CMSG_GM_REPORT_LAG = 0x0502 {
///     u32 lag_type;
///     Map map;
///     Vector3d position;
/// }
/// ```
pub struct CMSG_GM_REPORT_LAG {
    pub lag_type: u32,
    pub map: Map,
    pub position: Vector3d,
}

impl crate::Message for CMSG_GM_REPORT_LAG {
    const OPCODE: u32 = 0x0502;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // lag_type: u32
        w.write_all(&self.lag_type.to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0502, size: body_size as u32 });
        }

        // lag_type: u32
        let lag_type = crate::util::read_u32_le(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        Ok(Self {
            lag_type,
            map,
            position,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_GM_REPORT_LAG {}

