use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_spell_autocast.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_spell_autocast.wowm#L3):
/// ```text
/// cmsg CMSG_PET_SPELL_AUTOCAST = 0x2F3 {
///     Guid guid;
///     u32 spell_id;
///     u8 enabled;
/// }
/// ```
pub struct CMSG_PET_SPELL_AUTOCAST {
    pub guid: Guid,
    pub spell_id: u32,
    pub enabled: u8,
}

impl WorldClientMessageWrite for CMSG_PET_SPELL_AUTOCAST {
    const OPCODE: u32 = 0x2f3;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_PET_SPELL_AUTOCAST {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // spell_id: u32
        let spell_id = crate::util::read_u32_le(r)?;

        // enabled: u8
        let enabled = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            spell_id,
            enabled,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

        // enabled: u8
        w.write_all(&self.enabled.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_PET_SPELL_AUTOCAST {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_PET_SPELL_AUTOCAST {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // spell_id: u32
        + 1 // enabled: u8
    }
}

