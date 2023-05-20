use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Only exists as comment in azerothcore/trinitycore.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_state_change.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_state_change.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_STATE_CHANGE = 0x04E8 {
///     u32 unknown1;
///     u32 unknown2;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_MGR_STATE_CHANGE {
    pub unknown1: u32,
    pub unknown2: u32,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_MGR_STATE_CHANGE {}
impl crate::Message for SMSG_BATTLEFIELD_MGR_STATE_CHANGE {
    const OPCODE: u32 = 0x04e8;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04E8, size: body_size });
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unknown1,
            unknown2,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_STATE_CHANGE {}

