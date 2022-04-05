use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new2.wowm:460`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new2.wowm#L460):
/// ```text
/// cmsg CMSG_UNLEARN_SKILL = 0x202 {
///     u32 skill_id;
/// }
/// ```
pub struct CMSG_UNLEARN_SKILL {
    pub skill_id: u32,
}

impl WorldClientMessageWrite for CMSG_UNLEARN_SKILL {
    const OPCODE: u32 = 0x202;

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
impl WorldMessageBody for CMSG_UNLEARN_SKILL {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // skill_id: u32
        let skill_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            skill_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // skill_id: u32
        w.write_all(&self.skill_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_UNLEARN_SKILL {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_UNLEARN_SKILL {
    fn maximum_possible_size() -> usize {
        4 // skill_id: u32
    }
}

