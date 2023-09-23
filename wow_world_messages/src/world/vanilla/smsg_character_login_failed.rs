use std::io::{Read, Write};

use crate::vanilla::WorldResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Response if [`CMSG_PLAYER_LOGIN`](crate::vanilla::CMSG_PLAYER_LOGIN) fails. If successful it should instead be [`SMSG_LOGIN_VERIFY_WORLD`](crate::vanilla::SMSG_LOGIN_VERIFY_WORLD).
/// Client seems to always send a [`CMSG_CANCEL_TRADE`](crate::vanilla::CMSG_CANCEL_TRADE) after receiving this message, for unknown reasons.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm#L3):
/// ```text
/// smsg SMSG_CHARACTER_LOGIN_FAILED = 0x0041 {
///     WorldResult result;
/// }
/// ```
pub struct SMSG_CHARACTER_LOGIN_FAILED {
    pub result: WorldResult,
}

impl crate::private::Sealed for SMSG_CHARACTER_LOGIN_FAILED {}
impl SMSG_CHARACTER_LOGIN_FAILED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // result: WorldResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}

impl crate::Message for SMSG_CHARACTER_LOGIN_FAILED {
    const OPCODE: u32 = 0x0041;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CHARACTER_LOGIN_FAILED"
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(65, "SMSG_CHARACTER_LOGIN_FAILED", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CHARACTER_LOGIN_FAILED {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_CHARACTER_LOGIN_FAILED;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_CHARACTER_LOGIN_FAILED, expected: &SMSG_CHARACTER_LOGIN_FAILED) {
        assert_eq!(t.result, expected.result);
    }

    const RAW0: [u8; 5] = [ 0x00, 0x03, 0x41, 0x00, 0x41, ];

    pub(crate) fn expected0() -> SMSG_CHARACTER_LOGIN_FAILED {
        SMSG_CHARACTER_LOGIN_FAILED {
            result: WorldResult::CharLoginFailed,
        }

    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_character_login_failed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHARACTER_LOGIN_FAILED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(1 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_character_login_failed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHARACTER_LOGIN_FAILED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(1 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_character_login_failed.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_character_login_failed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHARACTER_LOGIN_FAILED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHARACTER_LOGIN_FAILED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(1 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

