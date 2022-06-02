use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellordamage_immune.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellordamage_immune.wowm#L3):
/// ```text
/// smsg SMSG_SPELLORDAMAGE_IMMUNE = 0x0263 {
///     Guid caster_guid;
///     Guid target_guid;
///     u32 id;
///     u8 unknown1;
/// }
/// ```
pub struct SMSG_SPELLORDAMAGE_IMMUNE {
    pub caster_guid: Guid,
    pub target_guid: Guid,
    pub id: u32,
    /// # Comment
    ///
    /// vmangos/cmangos sets to 0
    pub unknown1: u8,
}

impl ServerMessage for SMSG_SPELLORDAMAGE_IMMUNE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0263;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        21
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        Ok(Self {
            caster_guid,
            target_guid,
            id,
            unknown1,
        })
    }

}

