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
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm#L3):
/// ```text
/// cmsg CMSG_CAST_SPELL = 0x012E {
///     u32 spell;
///     SpellCastTargets targets;
/// }
/// ```
pub struct CMSG_CAST_SPELL {
    pub spell: u32,
    pub targets: SpellCastTargets,
}

impl ClientMessage for CMSG_CAST_SPELL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(w)?;

        Ok(())
    }
    const OPCODE: u16 = 0x012e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        Ok(Self {
            spell,
            targets,
        })
    }

}

impl CMSG_CAST_SPELL {
    pub(crate) fn size(&self) -> usize {
        4 // spell: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}

