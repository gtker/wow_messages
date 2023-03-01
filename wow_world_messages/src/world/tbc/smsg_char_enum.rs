use crate:: {
};
use crate::tbc::Character;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_enum_2_4_3.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_enum_2_4_3.wowm#L3):
/// ```text
/// smsg SMSG_CHAR_ENUM = 0x003B {
///     u8 amount_of_characters;
///     Character[amount_of_characters] characters;
/// }
/// ```
pub struct SMSG_CHAR_ENUM {
    pub characters: Vec<Character>,
}

impl crate::Message for SMSG_CHAR_ENUM {
    const OPCODE: u32 = 0x003b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // amount_of_characters: u8
        w.write_all(&(self.characters.len() as u8).to_le_bytes())?;

        // characters: Character[amount_of_characters]
        for i in self.characters.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=126465).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x003B, size: body_size as u32 });
        }

        // amount_of_characters: u8
        let amount_of_characters = crate::util::read_u8_le(&mut r)?;

        // characters: Character[amount_of_characters]
        let characters = {
            let mut characters = Vec::with_capacity(amount_of_characters as usize);
            for i in 0..amount_of_characters {
                characters.push(Character::read(&mut r)?);
            }
            characters
        };

        Ok(Self {
            characters,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CHAR_ENUM {}

impl SMSG_CHAR_ENUM {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_characters: u8
        + self.characters.iter().fold(0, |acc, x| acc + x.size()) // characters: Character[amount_of_characters]
    }
}

