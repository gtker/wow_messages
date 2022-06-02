use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm#L3):
/// ```text
/// smsg SMSG_LOGIN_SETTIMESPEED = 0x0042 {
///     u32 secs_to_time_bit_field;
///     f32 game_speed;
/// }
/// ```
pub struct SMSG_LOGIN_SETTIMESPEED {
    pub secs_to_time_bit_field: u32,
    /// # Comment
    ///
    /// Set to 0.01666667f in cmangos.
    ///
    pub game_speed: f32,
}

impl ServerMessage for SMSG_LOGIN_SETTIMESPEED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // secs_to_time_bit_field: u32
        w.write_all(&self.secs_to_time_bit_field.to_le_bytes())?;

        // game_speed: f32
        w.write_all(&self.game_speed.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0042;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // secs_to_time_bit_field: u32
        let secs_to_time_bit_field = crate::util::read_u32_le(r)?;

        // game_speed: f32
        let game_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            secs_to_time_bit_field,
            game_speed,
        })
    }

}

