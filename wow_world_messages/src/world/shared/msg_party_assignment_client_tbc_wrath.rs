use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::shared::party_role_tbc_wrath::PartyRole;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_party_assignment.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_party_assignment.wowm#L8):
/// ```text
/// cmsg MSG_PARTY_ASSIGNMENT_Client = 0x038E {
///     PartyRole role;
///     Bool apply;
///     Guid player;
/// }
/// ```
pub struct MSG_PARTY_ASSIGNMENT_Client {
    pub role: PartyRole,
    pub apply: bool,
    pub player: Guid,
}

impl crate::Message for MSG_PARTY_ASSIGNMENT_Client {
    const OPCODE: u32 = 0x038e;

    fn size_without_header(&self) -> u32 {
        10
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // role: PartyRole
        w.write_all(&(self.role.as_int() as u8).to_le_bytes())?;

        // apply: Bool
        w.write_all(u8::from(self.apply).to_le_bytes().as_slice())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 10 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x038E, size: body_size as u32 });
        }

        // role: PartyRole
        let role: PartyRole = crate::util::read_u8_le(r)?.try_into()?;

        // apply: Bool
        let apply = crate::util::read_u8_le(r)? != 0;
        // player: Guid
        let player = Guid::read(r)?;

        Ok(Self {
            role,
            apply,
            player,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for MSG_PARTY_ASSIGNMENT_Client {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for MSG_PARTY_ASSIGNMENT_Client {}

