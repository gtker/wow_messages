use std::io::{Read, Write};

use crate::vanilla::WorldResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Response to [`CMSG_CHAR_CREATE`](crate::vanilla::CMSG_CHAR_CREATE).
///
/// Every `WorldResult` except `CHAR_CREATE_SUCCESS` will lead to a popup showing. `CHAR_CREATE_SUCCESS` will cause the client to send a [`CMSG_CHAR_ENUM`](crate::vanilla::CMSG_CHAR_ENUM).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm#L1):
/// ```text
/// smsg SMSG_CHAR_CREATE = 0x003A {
///     WorldResult result;
/// }
/// ```
pub struct SMSG_CHAR_CREATE {
    pub result: WorldResult,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CHAR_CREATE {
    pub fn to_test_case_string(&self) -> Option<String> {
        None
    }

}

impl crate::private::Sealed for SMSG_CHAR_CREATE {}
impl crate::Message for SMSG_CHAR_CREATE {
    const OPCODE: u32 = 0x003a;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_CHAR_CREATE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x003A, size: body_size });
        }

        // result: WorldResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CHAR_CREATE {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_CHAR_CREATE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_CHAR_CREATE, expected: &SMSG_CHAR_CREATE) {
        assert_eq!(t.result, expected.result);
    }

    const RAW0: [u8; 5] = [ 0x00, 0x03, 0x3A, 0x00, 0x2F, ];

    pub(crate) fn expected0() -> SMSG_CHAR_CREATE {
        SMSG_CHAR_CREATE {
            result: WorldResult::CharCreateError,
        }

    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_char_create0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_CREATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_CREATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(1 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_char_create0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_CREATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_CREATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(1 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_create.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_char_create0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_CREATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_CREATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(1 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

