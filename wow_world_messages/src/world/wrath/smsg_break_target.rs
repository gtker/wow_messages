use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_break_target.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_break_target.wowm#L1):
/// ```text
/// smsg SMSG_BREAK_TARGET = 0x0152 {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_BREAK_TARGET {
    pub guid: Guid,
}

impl crate::Message for SMSG_BREAK_TARGET {
    const OPCODE: u32 = 0x0152;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0152, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BREAK_TARGET {}

impl SMSG_BREAK_TARGET {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
    }
}

