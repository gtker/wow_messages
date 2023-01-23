use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// cmangos/vmangos/mangoszero: deliver undelivered mail
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_received_mail.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_received_mail.wowm#L3):
/// ```text
/// smsg SMSG_RECEIVED_MAIL = 0x0285 {
///     u32 unknown1;
/// }
/// ```
pub struct SMSG_RECEIVED_MAIL {
    /// cmangos/vmangos sends 0 as u32, mangoszero sends 0 as f32
    ///
    pub unknown1: u32,
}

impl crate::Message for SMSG_RECEIVED_MAIL {
    const OPCODE: u32 = 0x0285;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0285, size: body_size as u32 });
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown1,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_RECEIVED_MAIL {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_RECEIVED_MAIL {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_RECEIVED_MAIL {}

