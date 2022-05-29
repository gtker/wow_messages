use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::SpellCastTargets;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_USE_ITEM {
    pub bag_index: u8,
    pub bag_slot: u8,
    pub spell_index: u8,
    pub targets: SpellCastTargets,
}

impl ClientMessage for CMSG_USE_ITEM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // spell_index: u8
        w.write_all(&self.spell_index.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(w)?;

        Ok(())
    }
    const OPCODE: u16 = 0x00ab;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // bag_index: u8
        let bag_index = crate::util::read_u8_le(r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(r)?;

        // spell_index: u8
        let spell_index = crate::util::read_u8_le(r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        Ok(Self {
            bag_index,
            bag_slot,
            spell_index,
            targets,
        })
    }

}

impl CMSG_USE_ITEM {
    pub(crate) fn size(&self) -> usize {
        1 // bag_index: u8
        + 1 // bag_slot: u8
        + 1 // spell_index: u8
        + self.targets.size() // targets: SpellCastTargets
    }
}

