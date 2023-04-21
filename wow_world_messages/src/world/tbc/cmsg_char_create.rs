use std::io::{Read, Write};

use crate::tbc::{
    Class, Gender, Race,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent after the client presses 'Create Character'. The client will then wait for [`SMSG_CHAR_CREATE`](crate::tbc::SMSG_CHAR_CREATE).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_char_create.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_char_create.wowm#L1):
/// ```text
/// cmsg CMSG_CHAR_CREATE = 0x0036 {
///     CString name;
///     Race race;
///     Class class;
///     Gender gender;
///     u8 skin_color;
///     u8 face;
///     u8 hair_style;
///     u8 hair_color;
///     u8 facial_hair;
///     u8 outfit_id = 0;
/// }
/// ```
pub struct CMSG_CHAR_CREATE {
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub gender: Gender,
    pub skin_color: u8,
    pub face: u8,
    pub hair_style: u8,
    pub hair_color: u8,
    pub facial_hair: u8,
}

impl CMSG_CHAR_CREATE {
    /// The field `outfit_id` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const OUTFIT_ID_VALUE: u8 = 0x00;

}

impl crate::private::Sealed for CMSG_CHAR_CREATE {}
impl crate::Message for CMSG_CHAR_CREATE {
    const OPCODE: u32 = 0x0036;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&u8::from(self.race.as_int()).to_le_bytes())?;

        // class: Class
        w.write_all(&u8::from(self.class.as_int()).to_le_bytes())?;

        // gender: Gender
        w.write_all(&u8::from(self.gender.as_int()).to_le_bytes())?;

        // skin_color: u8
        w.write_all(&self.skin_color.to_le_bytes())?;

        // face: u8
        w.write_all(&self.face.to_le_bytes())?;

        // hair_style: u8
        w.write_all(&self.hair_style.to_le_bytes())?;

        // hair_color: u8
        w.write_all(&self.hair_color.to_le_bytes())?;

        // facial_hair: u8
        w.write_all(&self.facial_hair.to_le_bytes())?;

        // outfit_id: u8
        w.write_all(&Self::OUTFIT_ID_VALUE.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=265).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0036, size: body_size as u32 });
        }

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // race: Race
        let race: Race = crate::util::read_u8_le(&mut r)?.try_into()?;

        // class: Class
        let class: Class = crate::util::read_u8_le(&mut r)?.try_into()?;

        // gender: Gender
        let gender: Gender = crate::util::read_u8_le(&mut r)?.try_into()?;

        // skin_color: u8
        let skin_color = crate::util::read_u8_le(&mut r)?;

        // face: u8
        let face = crate::util::read_u8_le(&mut r)?;

        // hair_style: u8
        let hair_style = crate::util::read_u8_le(&mut r)?;

        // hair_color: u8
        let hair_color = crate::util::read_u8_le(&mut r)?;

        // facial_hair: u8
        let facial_hair = crate::util::read_u8_le(&mut r)?;

        // outfit_id: u8
        let _outfit_id = crate::util::read_u8_le(&mut r)?;
        // outfit_id is expected to always be 0 (0)

        Ok(Self {
            name,
            race,
            class,
            gender,
            skin_color,
            face,
            hair_style,
            hair_color,
            facial_hair,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CHAR_CREATE {}

impl CMSG_CHAR_CREATE {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + 1 // race: Race
        + 1 // class: Class
        + 1 // gender: Gender
        + 1 // skin_color: u8
        + 1 // face: u8
        + 1 // hair_style: u8
        + 1 // hair_color: u8
        + 1 // facial_hair: u8
        + 1 // outfit_id: u8
    }
}

