use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Character;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm:43`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm#L43):
/// ```text
/// smsg SMSG_CHAR_ENUM = 0x003B {
///     u8 amount_of_characters;
///     Character[amount_of_characters] characters;
/// }
/// ```
/// # Description
///
/// Response to [CMSG_CHAR_ENUM].
pub struct SMSG_CHAR_ENUM {
    pub characters: Vec<Character>,
}

impl ServerMessage for SMSG_CHAR_ENUM {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_characters: u8
        w.write_all(&(self.characters.len() as u8).to_le_bytes())?;

        // characters: Character[amount_of_characters]
        for i in self.characters.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x003b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // amount_of_characters: u8
        let amount_of_characters = crate::util::read_u8_le(r)?;

        // characters: Character[amount_of_characters]
        let mut characters = Vec::with_capacity(amount_of_characters as usize);
        for i in 0..amount_of_characters {
            characters.push(Character::read(r)?);
        }

        Ok(Self {
            characters,
        })
    }

}

impl SMSG_CHAR_ENUM {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_characters: u8
        + self.characters.iter().fold(0, |acc, x| acc + x.size()) // characters: Character[amount_of_characters]
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_CHAR_ENUM;
    use crate::world::version_1_12::Area;
    use crate::world::version_1_12::Character;
    use crate::world::version_1_12::CharacterFlags;
    use crate::world::version_1_12::CharacterGear;
    use crate::world::version_1_12::Class;
    use crate::world::version_1_12::Gender;
    use crate::world::version_1_12::InventoryType;
    use crate::world::version_1_12::Map;
    use crate::world::version_1_12::Race;
    use crate::world::version_1_12::Vector3d;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 5] = [ 0x00, 0x03, 0x3B, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm` line 53.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_CHAR_ENUM0() {
        let expected = SMSG_CHAR_ENUM {
            characters: vec![ ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_ENUM(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_ENUM, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.characters, expected.characters);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm` line 53.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_CHAR_ENUM0() {
        let expected = SMSG_CHAR_ENUM {
            characters: vec![ ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_ENUM(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_ENUM, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.characters, expected.characters);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm` line 53.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_CHAR_ENUM0() {
        let expected = SMSG_CHAR_ENUM {
            characters: vec![ ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_ENUM(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_ENUM, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.characters, expected.characters);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
