use std::convert::{TryFrom, TryInto};
use crate::world::wrath::LogoutResult;
use crate::world::wrath::LogoutSpeed;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Reply to [`CMSG_LOGOUT_REQUEST`](crate::world::vanilla::CMSG_LOGOUT_REQUEST).
///
/// The client expects to get an `SMSG_LOGOUT_COMPLETE` when logout is complete.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm#L35):
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
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: LogoutResult
        w.write_all(&(self.result.as_int() as u16).to_le_bytes())?;

        // speed: LogoutSpeed
        w.write_all(&(self.speed.as_int() as u16).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // result: LogoutResult
        let result: LogoutResult = crate::util::read_u16_le(r)?.try_into()?;

        // speed: LogoutSpeed
        let speed: LogoutSpeed = crate::util::read_u16_le(r)?.try_into()?;

        Ok(Self {
            result,
            speed,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_LOGOUT_RESPONSE {}

