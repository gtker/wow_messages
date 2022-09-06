use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_play_music.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_play_music.wowm#L3):
/// ```text
/// smsg SMSG_PLAY_MUSIC = 0x0277 {
///     u32 sound_id;
/// }
/// ```
pub struct SMSG_PLAY_MUSIC {
    pub sound_id: u32,
}

impl crate::Message for SMSG_PLAY_MUSIC {
    const OPCODE: u32 = 0x0277;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // sound_id: u32
        let sound_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            sound_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_PLAY_MUSIC {}

