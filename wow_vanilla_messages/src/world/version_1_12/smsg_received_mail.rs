use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_received_mail.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_received_mail.wowm#L3):
/// ```text
/// smsg SMSG_RECEIVED_MAIL = 0x0285 {
///     u32 unknown1;
/// }
/// ```
/// # Comment
///
/// cmangos/vmangos/mangoszero: deliver undelivered mail
pub struct SMSG_RECEIVED_MAIL {
    /// # Comment
    ///
    /// cmangos/vmangos sends 0 as u32, mangoszero sends 0 as f32
    pub unknown1: u32,
}

impl ServerMessage for SMSG_RECEIVED_MAIL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0285;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown1,
        })
    }

}

