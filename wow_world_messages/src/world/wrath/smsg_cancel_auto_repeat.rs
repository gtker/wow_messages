use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_cancel_auto_repeat.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_cancel_auto_repeat.wowm#L5):
/// ```text
/// smsg SMSG_CANCEL_AUTO_REPEAT = 0x029C {
///     PackedGuid target;
/// }
/// ```
pub struct SMSG_CANCEL_AUTO_REPEAT {
    pub target: Guid,
}

impl crate::Message for SMSG_CANCEL_AUTO_REPEAT {
    const OPCODE: u32 = 0x029c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // target: PackedGuid
        self.target.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x029C, size: body_size as u32 });
        }

        // target: PackedGuid
        let target = Guid::read_packed(&mut r)?;

        Ok(Self {
            target,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CANCEL_AUTO_REPEAT {}

impl SMSG_CANCEL_AUTO_REPEAT {
    pub(crate) fn size(&self) -> usize {
        self.target.size() // target: Guid
    }
}

