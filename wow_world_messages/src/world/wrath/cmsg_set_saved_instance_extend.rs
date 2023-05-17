use std::io::{Read, Write};

use crate::wrath::{
    Map, RaidDifficulty,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/cmsg_set_saved_instance_extend.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/cmsg_set_saved_instance_extend.wowm#L1):
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

impl crate::private::Sealed for CMSG_SET_SAVED_INSTANCE_EXTEND {}
impl crate::Message for CMSG_SET_SAVED_INSTANCE_EXTEND {
    const OPCODE: u32 = 0x0292;

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // difficulty: RaidDifficulty
        w.write_all(&(self.difficulty.as_int().to_le_bytes()))?;

        // toggle_extend: Bool
        w.write_all(u8::from(self.toggle_extend).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0292, size: body_size });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // difficulty: RaidDifficulty
        let difficulty: RaidDifficulty = crate::util::read_u8_le(&mut r)?.try_into()?;

        // toggle_extend: Bool
        let toggle_extend = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            map,
            difficulty,
            toggle_extend,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_SAVED_INSTANCE_EXTEND {}

