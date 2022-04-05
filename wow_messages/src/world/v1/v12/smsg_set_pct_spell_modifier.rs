use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:752`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L752):
/// ```text
/// smsg SMSG_SET_PCT_SPELL_MODIFIER = 0x267 {
///     u8 eff;
///     u8 op;
///     u32 value;
/// }
/// ```
pub struct SMSG_SET_PCT_SPELL_MODIFIER {
    pub eff: u8,
    pub op: u8,
    pub value: u32,
}

impl WorldServerMessageWrite for SMSG_SET_PCT_SPELL_MODIFIER {
    const OPCODE: u16 = 0x267;

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
impl WorldMessageBody for SMSG_SET_PCT_SPELL_MODIFIER {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // eff: u8
        let eff = crate::util::read_u8_le(r)?;

        // op: u8
        let op = crate::util::read_u8_le(r)?;

        // value: u32
        let value = crate::util::read_u32_le(r)?;

        Ok(Self {
            eff,
            op,
            value,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // eff: u8
        w.write_all(&self.eff.to_le_bytes())?;

        // op: u8
        w.write_all(&self.op.to_le_bytes())?;

        // value: u32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SET_PCT_SPELL_MODIFIER {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SET_PCT_SPELL_MODIFIER {
    fn maximum_possible_size() -> usize {
        1 // eff: u8
        + 1 // op: u8
        + 4 // value: u32
    }
}

