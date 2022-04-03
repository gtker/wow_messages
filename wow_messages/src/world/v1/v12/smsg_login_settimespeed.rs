use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/add_messages.wowm:51`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/add_messages.wowm#L51):
/// ```text
/// smsg SMSG_LOGIN_SETTIMESPEED = 0x42 {
///     u32 secs_to_time_bit_field;
///     f32 game_speed;
/// }
/// ```
pub struct SMSG_LOGIN_SETTIMESPEED {
    pub secs_to_time_bit_field: u32,
    pub game_speed: f32,
}

impl WorldServerMessageWrite for SMSG_LOGIN_SETTIMESPEED {
    const OPCODE: u16 = 0x42;

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
impl WorldMessageBody for SMSG_LOGIN_SETTIMESPEED {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // secs_to_time_bit_field: u32
        let secs_to_time_bit_field = crate::util::read_u32_le(r)?;

        // game_speed: f32
        let game_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            secs_to_time_bit_field,
            game_speed,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // secs_to_time_bit_field: u32
        w.write_all(&self.secs_to_time_bit_field.to_le_bytes())?;

        // game_speed: f32
        w.write_all(&self.game_speed.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_LOGIN_SETTIMESPEED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_LOGIN_SETTIMESPEED {
    fn maximum_possible_size() -> usize {
        4 // secs_to_time_bit_field: u32
        + 4 // game_speed: f32
    }
}

