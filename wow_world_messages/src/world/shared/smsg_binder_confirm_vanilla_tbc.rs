use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_binder_confirm.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_binder_confirm.wowm#L1):
/// ```text
/// smsg SMSG_BINDER_CONFIRM = 0x02EB {
///     Guid guid;
/// }
/// ```
pub struct SMSG_BINDER_CONFIRM {
    pub guid: Guid,
}

impl crate::Message for SMSG_BINDER_CONFIRM {
    const OPCODE: u32 = 0x02eb;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02EB, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_BINDER_CONFIRM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_BINDER_CONFIRM {}

