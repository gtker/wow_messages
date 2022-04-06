use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_dispel_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_dispel_failed.wowm#L3):
/// ```text
/// smsg SMSG_DISPEL_FAILED = 0x262 {
///     u64 caster_guid;
///     u64 target_guid;
///     u32[-] spells;
/// }
/// ```
pub struct SMSG_DISPEL_FAILED {
    pub caster_guid: u64,
    pub target_guid: u64,
    pub spells: Vec<u32>,
}

impl WorldServerMessageWrite for SMSG_DISPEL_FAILED {
    const OPCODE: u16 = 0x262;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_DISPEL_FAILED {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster_guid: u64
        let caster_guid = crate::util::read_u64_le(r)?;

        // target_guid: u64
        let target_guid = crate::util::read_u64_le(r)?;

        // spells: u32[-]
        let mut current_size = {
            8 // caster_guid: u64
            + 8 // target_guid: u64
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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // caster_guid: u64
        w.write_all(&self.caster_guid.to_le_bytes())?;

        // target_guid: u64
        w.write_all(&self.target_guid.to_le_bytes())?;

        // spells: u32[-]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_DISPEL_FAILED {
    fn size(&self) -> usize {
        8 // caster_guid: u64
        + 8 // target_guid: u64
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[-]
    }
}

impl MaximumPossibleSized for SMSG_DISPEL_FAILED {
    fn maximum_possible_size() -> usize {
        8 // caster_guid: u64
        + 8 // target_guid: u64
        + 65536 // spells: u32[-]
    }
}

