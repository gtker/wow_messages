use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::Gender;

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

#[cfg(feature = "print-testcase")]
impl CMSG_CHAR_CUSTOMIZE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CHAR_CUSTOMIZE {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    new_name = \"{}\";", self.new_name).unwrap();
        writeln!(s, "    gender = {};", self.gender.as_test_case_value()).unwrap();
        writeln!(s, "    skin_color = {};", self.skin_color).unwrap();
        writeln!(s, "    hair_color = {};", self.hair_color).unwrap();
        writeln!(s, "    hair_style = {};", self.hair_style).unwrap();
        writeln!(s, "    facial_hair = {};", self.facial_hair).unwrap();
        writeln!(s, "    face = {};", self.face).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1139_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.new_name.len() + 1, "new_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "gender", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "skin_color", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "hair_color", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "hair_style", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "facial_hair", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "face", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_CHAR_CUSTOMIZE {}
impl crate::Message for CMSG_CHAR_CUSTOMIZE {
    const OPCODE: u32 = 0x0473;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_CHAR_CUSTOMIZE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // new_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.new_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `new_name` must not be null-terminated.");
        w.write_all(self.new_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // gender: Gender
        w.write_all(&(self.gender.as_int().to_le_bytes()))?;

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

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(15..=270).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0473, size: body_size });
        }

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // new_name: CString
        let new_name = {
            let new_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(new_name)?
        };

        // gender: Gender
        let gender = crate::util::read_u8_le(&mut r)?.try_into()?;

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

