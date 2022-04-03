use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new.wowm:67`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new.wowm#L67):
/// ```text
/// cmsg CMSG_CANCEL_CAST = 0x12F {
///     u32 spell_id;
/// }
/// ```
pub struct CMSG_CANCEL_CAST {
    pub spell_id: u32,
}

impl WorldClientMessageWrite for CMSG_CANCEL_CAST {
    const OPCODE: u32 = 0x12f;

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
impl WorldMessageBody for CMSG_CANCEL_CAST {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // spell_id: u32
        let spell_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            spell_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_CANCEL_CAST {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_CANCEL_CAST {
    fn maximum_possible_size() -> usize {
        4 // spell_id: u32
    }
}

