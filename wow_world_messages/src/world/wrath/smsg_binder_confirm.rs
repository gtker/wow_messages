use crate::Guid;
use crate::wrath::Area;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_binder_confirm.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_binder_confirm.wowm#L7):
/// ```text
/// smsg SMSG_BINDER_CONFIRM = 0x02EB {
///     Guid guid;
///     Area area;
/// }
/// ```
pub struct SMSG_BINDER_CONFIRM {
    pub guid: Guid,
    /// arcemu has this field while other emus do not.
    ///
    pub area: Area,
}

impl crate::Message for SMSG_BINDER_CONFIRM {
    const OPCODE: u32 = 0x02eb;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02EB, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            guid,
            area,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BINDER_CONFIRM {}

