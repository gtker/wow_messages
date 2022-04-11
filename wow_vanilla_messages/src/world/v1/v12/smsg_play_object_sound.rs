use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_player_object_sound.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_player_object_sound.wowm#L5):
/// ```text
/// smsg SMSG_PLAY_OBJECT_SOUND = 0x278 {
///     u32 sound_id;
///     Guid guid;
/// }
/// ```
pub struct SMSG_PLAY_OBJECT_SOUND {
    pub sound_id: u32,
    pub guid: Guid,
}

impl WorldServerMessageWrite for SMSG_PLAY_OBJECT_SOUND {
    const OPCODE: u16 = 0x278;

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
impl WorldMessageBody for SMSG_PLAY_OBJECT_SOUND {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // sound_id: u32
        let sound_id = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            sound_id,
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PLAY_OBJECT_SOUND {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PLAY_OBJECT_SOUND {
    fn maximum_possible_size() -> usize {
        4 // sound_id: u32
        + 8 // guid: Guid
    }
}

