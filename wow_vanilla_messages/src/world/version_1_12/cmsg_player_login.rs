use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Command to log into the specified character.
///
/// This is sent after the client has been authenticated and served the character list with [`SMSG_CHAR_ENUM`](crate::world::version_1_12::SMSG_CHAR_ENUM).
/// If the player receives a [`SMSG_CHARACTER_LOGIN_FAILED`](crate::world::version_1_12::SMSG_CHARACTER_LOGIN_FAILED) it will return to the character screen and send a [`CMSG_CHAR_ENUM`](crate::world::version_1_2::CMSG_CHAR_ENUM).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_player_login.wow:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_player_login.wow#L3):
/// ```text
/// cmsg CMSG_PLAYER_LOGIN = 0x003D {
///     Guid guid;
/// }
/// ```
pub struct CMSG_PLAYER_LOGIN {
    pub guid: Guid,
}

impl ClientMessage for CMSG_PLAYER_LOGIN {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x003d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMSG_PLAYER_LOGIN;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ClientOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x3D, 0x00, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wow` line 11.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_PLAYER_LOGIN0() {
        let expected = CMSG_PLAYER_LOGIN {
            guid: Guid::new(0xDEADBEEF),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wow` line 11.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_PLAYER_LOGIN0() {
        let expected = CMSG_PLAYER_LOGIN {
            guid: Guid::new(0xDEADBEEF),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_player_login.wow` line 11.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_PLAYER_LOGIN0() {
        let expected = CMSG_PLAYER_LOGIN {
            guid: Guid::new(0xDEADBEEF),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PLAYER_LOGIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGIN, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
