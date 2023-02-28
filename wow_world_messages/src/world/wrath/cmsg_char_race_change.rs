use crate::Guid;
use crate::wrath::Gender;
use crate::wrath::Race;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_char_race_change.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_char_race_change.wowm#L1):
/// ```text
/// cmsg CMSG_CHAR_RACE_CHANGE = 0x04F8 {
///     Guid player;
///     CString name;
///     Gender gender;
///     u8 skin_color;
///     u8 hair_color;
///     u8 hair_style;
///     u8 facial_hair;
///     u8 face;
///     Race race;
/// }
/// ```
pub struct CMSG_CHAR_RACE_CHANGE {
    pub player: Guid,
    pub name: String,
    pub gender: Gender,
    pub skin_color: u8,
    pub hair_color: u8,
    pub hair_style: u8,
    pub facial_hair: u8,
    pub face: u8,
    pub race: Race,
}

impl crate::Message for CMSG_CHAR_RACE_CHANGE {
    const OPCODE: u32 = 0x04f8;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // gender: Gender
        w.write_all(&u8::from(self.gender.as_int()).to_le_bytes())?;

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

        // race: Race
        w.write_all(&u8::from(self.race.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(16..=271).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04F8, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // gender: Gender
        let gender: Gender = crate::util::read_u8_le(&mut r)?.try_into()?;

        // skin_color: u8
        let skin_color = crate::util::read_u8_le(&mut r)?;

        // hair_color: u8
        let hair_color = crate::util::read_u8_le(&mut r)?;

        // hair_style: u8
        let hair_style = crate::util::read_u8_le(&mut r)?;

        // facial_hair: u8
        let facial_hair = crate::util::read_u8_le(&mut r)?;

        // face: u8
        let face = crate::util::read_u8_le(&mut r)?;

        // race: Race
        let race: Race = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            player,
            name,
            gender,
            skin_color,
            hair_color,
            hair_style,
            facial_hair,
            face,
            race,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CHAR_RACE_CHANGE {}

impl CMSG_CHAR_RACE_CHANGE {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + self.name.len() + 1 // name: CString
        + 1 // gender: Gender
        + 1 // skin_color: u8
        + 1 // hair_color: u8
        + 1 // hair_style: u8
        + 1 // facial_hair: u8
        + 1 // face: u8
        + 1 // race: Race
    }
}

