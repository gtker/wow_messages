use std::io::{Read, Write};

use crate::wrath::LfgTeleportError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_teleport_denied.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_teleport_denied.wowm#L14):
/// ```text
/// smsg SMSG_LFG_TELEPORT_DENIED = 0x0200 {
///     LfgTeleportError error;
/// }
/// ```
pub struct SMSG_LFG_TELEPORT_DENIED {
    pub error: LfgTeleportError,
}

impl crate::private::Sealed for SMSG_LFG_TELEPORT_DENIED {}
impl crate::Message for SMSG_LFG_TELEPORT_DENIED {
    const OPCODE: u32 = 0x0200;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // error: LfgTeleportError
        w.write_all(&u32::from(self.error.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0200, size: body_size as u32 });
        }

        // error: LfgTeleportError
        let error: LfgTeleportError = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            error,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_TELEPORT_DENIED {}

