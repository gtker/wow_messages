use std::convert::{TryFrom, TryInto};
use crate::world::wrath::Map;
use crate::world::wrath::RaidDifficulty;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_set_saved_instance_extend.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_set_saved_instance_extend.wowm#L1):
/// ```text
/// cmsg CMSG_SET_SAVED_INSTANCE_EXTEND = 0x0292 {
///     Map map;
///     RaidDifficulty difficulty;
///     Bool toggle_extend;
/// }
/// ```
pub struct CMSG_SET_SAVED_INSTANCE_EXTEND {
    pub map: Map,
    pub difficulty: RaidDifficulty,
    pub toggle_extend: bool,
}

impl crate::Message for CMSG_SET_SAVED_INSTANCE_EXTEND {
    const OPCODE: u32 = 0x0292;

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // difficulty: RaidDifficulty
        w.write_all(&(self.difficulty.as_int() as u8).to_le_bytes())?;

        // toggle_extend: Bool
        w.write_all(u8::from(self.toggle_extend).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0292, size: body_size as u32 });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // difficulty: RaidDifficulty
        let difficulty: RaidDifficulty = crate::util::read_u8_le(r)?.try_into()?;

        // toggle_extend: Bool
        let toggle_extend = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            map,
            difficulty,
            toggle_extend,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_SET_SAVED_INSTANCE_EXTEND {}

