use std::convert::{TryFrom, TryInto};
use crate::world::version_1_2::WorldResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Response to [`CMSG_CHAR_CREATE`](crate::world::version_1_12::CMSG_CHAR_CREATE).
///
/// Every [`WorldResult`](crate::world::version_1_2::WorldResult) except `CHAR_CREATE_SUCCESS` will lead to a popup showing. `CHAR_CREATE_SUCCESS` will cause the client to send a [`CMSG_CHAR_ENUM`](crate::world::version_1_2::CMSG_CHAR_ENUM).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm#L3):
/// ```text
/// smsg SMSG_CHAR_CREATE = 0x003A {
///     WorldResult result;
/// }
/// ```
pub struct SMSG_CHAR_CREATE {
    pub result: WorldResult,
}

impl ServerMessage for SMSG_CHAR_CREATE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x003a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: WorldResult
        let result: WorldResult = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}

#[cfg(test)]
mod test {
    use super::SMSG_CHAR_CREATE;
    use crate::world::version_1_2::WorldResult;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 5] = [ 0x00, 0x03, 0x3A, 0x00, 0x2F, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_CHAR_CREATE0() {
        let expected = SMSG_CHAR_CREATE {
            result: WorldResult::CHAR_CREATE_ERROR,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_CREATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_CREATE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_CHAR_CREATE0() {
        let expected = SMSG_CHAR_CREATE {
            result: WorldResult::CHAR_CREATE_ERROR,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_CREATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_CREATE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_CHAR_CREATE0() {
        let expected = SMSG_CHAR_CREATE {
            result: WorldResult::CHAR_CREATE_ERROR,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_CREATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_CREATE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
