use std::io::{Read, Write};

use wow_world_base::shared::logout_result_vanilla_tbc_wrath::LogoutResult;
use wow_world_base::shared::logout_speed_vanilla_tbc_wrath::LogoutSpeed;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Reply to [`CMSG_LOGOUT_REQUEST`](crate::vanilla::CMSG_LOGOUT_REQUEST).
///
/// The client expects to get an [`SMSG_LOGOUT_COMPLETE`](crate::vanilla::SMSG_LOGOUT_COMPLETE) when logout is complete.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm#L17):
/// ```text
/// smsg SMSG_LOGOUT_RESPONSE = 0x004C {
///     LogoutResult result;
///     LogoutSpeed speed;
/// }
/// ```
pub struct SMSG_LOGOUT_RESPONSE {
    pub result: LogoutResult,
    pub speed: LogoutSpeed,
}

#[cfg(feature = "print-testcase")]
impl SMSG_LOGOUT_RESPONSE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOGOUT_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();
        writeln!(s, "    speed = {};", self.speed.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 9_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 76_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "result");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_LOGOUT_RESPONSE {}
impl crate::Message for SMSG_LOGOUT_RESPONSE {
    const OPCODE: u32 = 0x004c;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: LogoutResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        // speed: LogoutSpeed
        w.write_all(&(self.speed.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x004C, size: body_size });
        }

        // result: LogoutResult
        let result = crate::util::read_u32_le(&mut r)?.try_into()?;

        // speed: LogoutSpeed
        let speed = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
            speed,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOGOUT_RESPONSE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOGOUT_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOGOUT_RESPONSE {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_LOGOUT_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_LOGOUT_RESPONSE, expected: &SMSG_LOGOUT_RESPONSE) {
        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);
    }

    const RAW0: [u8; 9] = [ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, ];

    pub(crate) fn expected0() -> SMSG_LOGOUT_RESPONSE {
        SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_logout_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_logout_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_logout_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_LOGOUT_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_LOGOUT_RESPONSE, expected: &SMSG_LOGOUT_RESPONSE) {
        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);
    }

    const RAW0: [u8; 9] = [ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, ];

    pub(crate) fn expected0() -> SMSG_LOGOUT_RESPONSE {
        SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_logout_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_logout_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_logout_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_LOGOUT_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_LOGOUT_RESPONSE, expected: &SMSG_LOGOUT_RESPONSE) {
        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);
    }

    const RAW0: [u8; 9] = [ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, ];

    pub(crate) fn expected0() -> SMSG_LOGOUT_RESPONSE {
        SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_logout_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_logout_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_logout_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(5 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

