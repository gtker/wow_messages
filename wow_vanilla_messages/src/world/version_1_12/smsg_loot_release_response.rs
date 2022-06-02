use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_release_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_release_response.wowm#L3):
/// ```text
/// smsg SMSG_LOOT_RELEASE_RESPONSE = 0x0161 {
///     Guid guid;
///     u8 unknown1;
/// }
/// ```
pub struct SMSG_LOOT_RELEASE_RESPONSE {
    pub guid: Guid,
    /// # Comment
    ///
    /// Set to 1 on mangoszero/vmangos/cmangos
    pub unknown1: u8,
}

impl ServerMessage for SMSG_LOOT_RELEASE_RESPONSE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0161;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            unknown1,
        })
    }

}

