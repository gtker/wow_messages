use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_play_music.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_play_music.wowm#L3):
/// ```text
/// smsg SMSG_PLAY_MUSIC = 0x0277 {
///     u32 sound_id;
/// }
/// ```
pub struct SMSG_PLAY_MUSIC {
    pub sound_id: u32,
}

impl ServerMessage for SMSG_PLAY_MUSIC {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0277;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // sound_id: u32
        let sound_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            sound_id,
        })
    }

}

