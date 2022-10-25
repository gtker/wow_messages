use std::convert::{TryFrom, TryInto};
use crate::world::shared::logout_result_vanilla_tbc_wrath::LogoutResult;
use crate::world::shared::logout_speed_vanilla_tbc_wrath::LogoutSpeed;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Reply to [`CMSG_LOGOUT_REQUEST`](crate::world::vanilla::CMSG_LOGOUT_REQUEST).
///
/// The client expects to get an [`SMSG_LOGOUT_COMPLETE`](crate::world::vanilla::SMSG_LOGOUT_COMPLETE) when logout is complete.
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

impl crate::Message for SMSG_LOGOUT_RESPONSE {
    const OPCODE: u32 = 0x004c;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: LogoutResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        // speed: LogoutSpeed
        w.write_all(&(self.speed.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // result: LogoutResult
        let result: LogoutResult = crate::util::read_u32_le(r)?.try_into()?;

        // speed: LogoutSpeed
        let speed: LogoutSpeed = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            result,
            speed,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_LOGOUT_RESPONSE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_LOGOUT_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_LOGOUT_RESPONSE {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    use super::SMSG_LOGOUT_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 9] = [ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, ];

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGOUT_RESPONSE0() {
        let expected = SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_LOGOUT_RESPONSE0() {
        let expected = SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_LOGOUT_RESPONSE0() {
        let expected = SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    use super::SMSG_LOGOUT_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::world::tbc::opcodes::ServerOpcodeMessage;
    use crate::world::tbc::{ClientMessage, ServerMessage};

    const RAW0: [u8; 9] = [ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, ];

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGOUT_RESPONSE0() {
        let expected = SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_LOGOUT_RESPONSE0() {
        let expected = SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_LOGOUT_RESPONSE0() {
        let expected = SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    use super::SMSG_LOGOUT_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::world::wrath::opcodes::ServerOpcodeMessage;
    use crate::world::wrath::{ClientMessage, ServerMessage};

    const RAW0: [u8; 9] = [ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, ];

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGOUT_RESPONSE0() {
        let expected = SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_LOGOUT_RESPONSE0() {
        let expected = SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm` line 25.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_LOGOUT_RESPONSE0() {
        let expected = SMSG_LOGOUT_RESPONSE {
            result: LogoutResult::Success,
            speed: LogoutSpeed::Instant,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

