use std::convert::{TryFrom, TryInto};
use crate::world::tbc::LfgData;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_set_looking_for_group.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_set_looking_for_group.wowm#L1):
/// ```text
/// cmsg CMSG_SET_LOOKING_FOR_GROUP = 0x0200 {
///     u32 slot;
///     LfgData data;
/// }
/// ```
pub struct CMSG_SET_LOOKING_FOR_GROUP {
    pub slot: u32,
    pub data: LfgData,
}

impl crate::Message for CMSG_SET_LOOKING_FOR_GROUP {
    const OPCODE: u32 = 0x0200;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        // data: LfgData
        self.data.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0200, size: body_size as u32 });
        }

        // slot: u32
        let slot = crate::util::read_u32_le(r)?;

        // data: LfgData
        let data = LfgData::read(r)?;

        Ok(Self {
            slot,
            data,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_SET_LOOKING_FOR_GROUP {}

