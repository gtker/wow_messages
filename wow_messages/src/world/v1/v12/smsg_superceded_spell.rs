use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new.wowm:62`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new.wowm#L62):
/// ```text
/// smsg SMSG_SUPERCEDED_SPELL = 0x12C {
///     u16 new_spell_id;
///     u16 old_spell_id;
/// }
/// ```
pub struct SMSG_SUPERCEDED_SPELL {
    pub new_spell_id: u16,
    pub old_spell_id: u16,
}

impl WorldServerMessageWrite for SMSG_SUPERCEDED_SPELL {
    const OPCODE: u16 = 0x12c;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_SUPERCEDED_SPELL {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // new_spell_id: u16
        let new_spell_id = crate::util::read_u16_le(r)?;

        // old_spell_id: u16
        let old_spell_id = crate::util::read_u16_le(r)?;

        Ok(Self {
            new_spell_id,
            old_spell_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // new_spell_id: u16
        w.write_all(&self.new_spell_id.to_le_bytes())?;

        // old_spell_id: u16
        w.write_all(&self.old_spell_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SUPERCEDED_SPELL {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SUPERCEDED_SPELL {
    fn maximum_possible_size() -> usize {
        2 // new_spell_id: u16
        + 2 // old_spell_id: u16
    }
}

