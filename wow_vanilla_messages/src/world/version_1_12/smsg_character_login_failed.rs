use std::convert::{TryFrom, TryInto};
use crate::world::version_1_2::WorldResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm#L3):
/// ```text
/// smsg SMSG_CHARACTER_LOGIN_FAILED = 0x0041 {
///     WorldResult result;
/// }
/// ```
/// # Description
///
/// Response if [CMSG_PLAYER_LOGIN] fails. If successful it should instead be [SMSG_LOGIN_VERIFY_WORLD].
pub struct SMSG_CHARACTER_LOGIN_FAILED {
    pub result: WorldResult,
}

impl ServerMessage for SMSG_CHARACTER_LOGIN_FAILED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0041;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: WorldResult
        let result: WorldResult = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}

#[cfg(test)]
mod test {
    use super::SMSG_CHARACTER_LOGIN_FAILED;
    use crate::world::version_1_2::WorldResult;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 8] = [ 0x00, 0x06, 0x41, 0x00, 0x41, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_CHARACTER_LOGIN_FAILED0() {
        let expected = SMSG_CHARACTER_LOGIN_FAILED {
            result: WorldResult::CHAR_LOGIN_FAILED,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHARACTER_LOGIN_FAILED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_CHARACTER_LOGIN_FAILED0() {
        let expected = SMSG_CHARACTER_LOGIN_FAILED {
            result: WorldResult::CHAR_LOGIN_FAILED,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHARACTER_LOGIN_FAILED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_CHARACTER_LOGIN_FAILED0() {
        let expected = SMSG_CHARACTER_LOGIN_FAILED {
            result: WorldResult::CHAR_LOGIN_FAILED,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHARACTER_LOGIN_FAILED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
