use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_role_chosen.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_role_chosen.wowm#L1):
/// ```text
/// smsg SMSG_LFG_ROLE_CHOSEN = 0x02BB {
///     Guid guid;
///     Bool ready;
///     u32 roles;
/// }
/// ```
pub struct SMSG_LFG_ROLE_CHOSEN {
    pub guid: Guid,
    pub ready: bool,
    pub roles: u32,
}

impl crate::private::Sealed for SMSG_LFG_ROLE_CHOSEN {}
impl crate::Message for SMSG_LFG_ROLE_CHOSEN {
    const OPCODE: u32 = 0x02bb;

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // ready: Bool
        w.write_all(u8::from(self.ready).to_le_bytes().as_slice())?;

        // roles: u32
        w.write_all(&self.roles.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02BB, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // ready: Bool
        let ready = crate::util::read_u8_le(&mut r)? != 0;

        // roles: u32
        let roles = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            ready,
            roles,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_ROLE_CHOSEN {}

