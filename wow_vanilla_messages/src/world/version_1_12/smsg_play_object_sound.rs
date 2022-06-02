use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_player_object_sound.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_player_object_sound.wowm#L5):
/// ```text
/// smsg SMSG_PLAY_OBJECT_SOUND = 0x0278 {
///     u32 sound_id;
///     Guid guid;
/// }
/// ```
/// # Comment
///
/// vmangos: Nostalrius: ignored by client if unit is not loaded
pub struct SMSG_PLAY_OBJECT_SOUND {
    pub sound_id: u32,
    pub guid: Guid,
}

impl ServerMessage for SMSG_PLAY_OBJECT_SOUND {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0278;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // sound_id: u32
        let sound_id = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            sound_id,
            guid,
        })
    }

}

