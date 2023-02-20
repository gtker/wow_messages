use crate::Guid;
use crate::wrath::Gender;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_char_customize.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_char_customize.wowm#L1):
/// ```text
/// cmsg CMSG_CHAR_CUSTOMIZE = 0x0473 {
///     Guid player;
///     CString new_name;
///     Gender gender;
///     u8 skin_color;
///     u8 hair_color;
///     u8 hair_style;
///     u8 facial_hair;
///     u8 face;
/// }
/// ```
pub struct CMSG_CHAR_CUSTOMIZE {
    pub player: Guid,
    pub new_name: String,
    pub gender: Gender,
    pub skin_color: u8,
    pub hair_color: u8,
    pub hair_style: u8,
    pub facial_hair: u8,
    pub face: u8,
}

impl crate::Message for CMSG_CHAR_CUSTOMIZE {
    const OPCODE: u32 = 0x0473;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // new_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.new_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `new_name` must not be null-terminated.");
        w.write_all(self.new_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // gender: Gender
        w.write_all(&(self.gender.as_int() as u8).to_le_bytes())?;

        // skin_color: u8
        w.write_all(&self.skin_color.to_le_bytes())?;

        // hair_color: u8
        w.write_all(&self.hair_color.to_le_bytes())?;

        // hair_style: u8
        w.write_all(&self.hair_style.to_le_bytes())?;

        // facial_hair: u8
        w.write_all(&self.facial_hair.to_le_bytes())?;

        // face: u8
        w.write_all(&self.face.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(15..=270).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0473, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(r)?;

        // new_name: CString
        let new_name = {
            let new_name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(new_name)?
        };

        // gender: Gender
        let gender: Gender = crate::util::read_u8_le(r)?.try_into()?;

        // skin_color: u8
        let skin_color = crate::util::read_u8_le(r)?;

        // hair_color: u8
        let hair_color = crate::util::read_u8_le(r)?;

        // hair_style: u8
        let hair_style = crate::util::read_u8_le(r)?;

        // facial_hair: u8
        let facial_hair = crate::util::read_u8_le(r)?;

        // face: u8
        let face = crate::util::read_u8_le(r)?;

        Ok(Self {
            player,
            new_name,
            gender,
            skin_color,
            hair_color,
            hair_style,
            facial_hair,
            face,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CHAR_CUSTOMIZE {}

impl CMSG_CHAR_CUSTOMIZE {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + self.new_name.len() + 1 // new_name: CString
        + 1 // gender: Gender
        + 1 // skin_color: u8
        + 1 // hair_color: u8
        + 1 // hair_style: u8
        + 1 // facial_hair: u8
        + 1 // face: u8
    }
}

