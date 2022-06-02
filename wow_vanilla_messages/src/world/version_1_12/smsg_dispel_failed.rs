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
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_dispel_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_dispel_failed.wowm#L3):
/// ```text
/// smsg SMSG_DISPEL_FAILED = 0x0262 {
///     Guid caster_guid;
///     Guid target_guid;
///     u32[-] spells;
/// }
/// ```
pub struct SMSG_DISPEL_FAILED {
    pub caster_guid: Guid,
    pub target_guid: Guid,
    pub spells: Vec<u32>,
}

impl ServerMessage for SMSG_DISPEL_FAILED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // spells: u32[-]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0262;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // spells: u32[-]
        let mut current_size = {
            8 // caster_guid: Guid
            + 8 // target_guid: Guid
        };
        let mut spells = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            spells.push(crate::util::read_u32_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            caster_guid,
            target_guid,
            spells,
        })
    }

}

impl SMSG_DISPEL_FAILED {
    pub(crate) fn size(&self) -> usize {
        8 // caster_guid: Guid
        + 8 // target_guid: Guid
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[-]
    }
}

