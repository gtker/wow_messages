use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfg_set_roles.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfg_set_roles.wowm#L1):
/// ```text
/// cmsg CMSG_LFG_SET_ROLES = 0x036A {
///     u8 roles;
/// }
/// ```
pub struct CMSG_LFG_SET_ROLES {
    pub roles: u8,
}

impl crate::Message for CMSG_LFG_SET_ROLES {
    const OPCODE: u32 = 0x036a;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // roles: u8
        w.write_all(&self.roles.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036A, size: body_size as u32 });
        }

        // roles: u8
        let roles = crate::util::read_u8_le(r)?;

        Ok(Self {
            roles,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LFG_SET_ROLES {}

