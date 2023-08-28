use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    Gender, Race,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_char_faction_change.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_char_faction_change.wowm#L1):
/// ```text
/// cmsg CMSG_CHAR_FACTION_CHANGE = 0x04D9 {
///     Guid guid;
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
pub struct CMSG_CHAR_FACTION_CHANGE {
    pub guid: Guid,
    pub name: String,
    pub gender: Gender,
    pub skin_color: u8,
    pub hair_color: u8,
    pub hair_style: u8,
    pub facial_hair: u8,
    pub face: u8,
    pub race: Race,
}

impl crate::private::Sealed for CMSG_CHAR_FACTION_CHANGE {}
impl CMSG_CHAR_FACTION_CHANGE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(16..=271).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
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

        // race: Race
        let race = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
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

impl crate::Message for CMSG_CHAR_FACTION_CHANGE {
    const OPCODE: u32 = 0x04d9;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CHAR_FACTION_CHANGE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        writeln!(s, "    gender = {};", self.gender.as_test_case_value()).unwrap();
        writeln!(s, "    skin_color = {};", self.skin_color).unwrap();
        writeln!(s, "    hair_color = {};", self.hair_color).unwrap();
        writeln!(s, "    hair_style = {};", self.hair_style).unwrap();
        writeln!(s, "    facial_hair = {};", self.facial_hair).unwrap();
        writeln!(s, "    face = {};", self.face).unwrap();
        writeln!(s, "    race = {};", self.race.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1241_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 1, "name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "gender", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "skin_color", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "hair_color", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "hair_style", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "facial_hair", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "face", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "race", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().next_back(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
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

        // race: Race
        w.write_all(&(self.race.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1241, "CMSG_CHAR_FACTION_CHANGE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CHAR_FACTION_CHANGE {}

impl CMSG_CHAR_FACTION_CHANGE {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
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

