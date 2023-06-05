use std::io::{Read, Write};

use crate::wrath::Character;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_enum_3_3_5.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_enum_3_3_5.wowm#L3):
/// ```text
/// smsg SMSG_CHAR_ENUM = 0x003B {
///     u8 amount_of_characters;
///     Character[amount_of_characters] characters;
/// }
/// ```
pub struct SMSG_CHAR_ENUM {
    pub characters: Vec<Character>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CHAR_ENUM {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CHAR_ENUM {{").unwrap();
        // Members
        writeln!(s, "    amount_of_characters = {};", self.characters.len()).unwrap();
        write!(s, "    characters = [").unwrap();
        for v in self.characters.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "    name = \"{}\";", v.name).unwrap();
            writeln!(s, "    race = {};", v.race.as_test_case_value()).unwrap();
            writeln!(s, "    class = {};", v.class.as_test_case_value()).unwrap();
            writeln!(s, "    gender = {};", v.gender.as_test_case_value()).unwrap();
            writeln!(s, "    skin = {};", v.skin).unwrap();
            writeln!(s, "    face = {};", v.face).unwrap();
            writeln!(s, "    hair_style = {};", v.hair_style).unwrap();
            writeln!(s, "    hair_color = {};", v.hair_color).unwrap();
            writeln!(s, "    facial_hair = {};", v.facial_hair).unwrap();
            writeln!(s, "    level = {};", v.level.as_int()).unwrap();
            writeln!(s, "    area = {};", v.area.as_test_case_value()).unwrap();
            writeln!(s, "    map = {};", v.map.as_test_case_value()).unwrap();
            // position: Vector3d
            writeln!(s, "    position = {{").unwrap();
            // Members
            writeln!(s, "    {}", if v.position.x.to_string().contains(".") { v.position.x.to_string() } else { format!("{}.0", v.position.x) }).unwrap();
            writeln!(s, "    {}", if v.position.y.to_string().contains(".") { v.position.y.to_string() } else { format!("{}.0", v.position.y) }).unwrap();
            writeln!(s, "    {}", if v.position.z.to_string().contains(".") { v.position.z.to_string() } else { format!("{}.0", v.position.z) }).unwrap();

            writeln!(s, "    }};").unwrap();
            writeln!(s, "    guild_id = {};", v.guild_id).unwrap();
            writeln!(s, "    flags = {};", v.flags).unwrap();
            writeln!(s, "    recustomization_flags = {};", v.recustomization_flags).unwrap();
            writeln!(s, "    first_login = {};", if v.first_login { "TRUE" } else { "FALSE" }).unwrap();
            writeln!(s, "    pet_display_id = {};", v.pet_display_id).unwrap();
            writeln!(s, "    pet_level = {};", v.pet_level.as_int()).unwrap();
            writeln!(s, "    pet_family = {};", v.pet_family.as_test_case_value()).unwrap();
            write!(s, "    equipment = [").unwrap();
            for v in v.equipment.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "    equipment_display_id = {};", v.equipment_display_id).unwrap();
                writeln!(s, "    inventory_type = {};", v.inventory_type.as_test_case_value()).unwrap();
                writeln!(s, "    enchantment = {};", v.enchantment).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 59_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_characters");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_CHAR_ENUM {}
impl crate::Message for SMSG_CHAR_ENUM {
    const OPCODE: u32 = 0x003b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_characters: u8
        w.write_all(&(self.characters.len() as u8).to_le_bytes())?;

        // characters: Character[amount_of_characters]
        for i in self.characters.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=134401).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x003B, size: body_size });
        }

        // amount_of_characters: u8
        let amount_of_characters = crate::util::read_u8_le(&mut r)?;

        // characters: Character[amount_of_characters]
        let characters = {
            let mut characters = Vec::with_capacity(amount_of_characters as usize);
            for _ in 0..amount_of_characters {
                characters.push(Character::read(&mut r)?);
            }
            characters
        };

        Ok(Self {
            characters,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CHAR_ENUM {}

impl SMSG_CHAR_ENUM {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_characters: u8
        + self.characters.iter().fold(0, |acc, x| acc + x.size()) // characters: Character[amount_of_characters]
    }
}

