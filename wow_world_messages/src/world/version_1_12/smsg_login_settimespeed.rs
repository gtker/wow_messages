use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Tells the client what the datetime is and how fast time passes.
///
/// The client also asks for the datetime with [`CMSG_QUERY_TIME`](crate::world::version_1_12::CMSG_QUERY_TIME) and gets a reply from [`SMSG_QUERY_TIME_RESPONSE`](crate::world::version_1_12::SMSG_QUERY_TIME_RESPONSE), but this does not appear to change anything in the client.
/// Despite sending this as the very first message after the client logs in it will still send a [`CMSG_QUERY_TIME`](crate::world::version_1_12::CMSG_QUERY_TIME).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm#L3):
/// ```text
/// smsg SMSG_LOGIN_SETTIMESPEED = 0x0042 {
///     u32 datetime;
///     f32 timescale;
/// }
/// ```
pub struct SMSG_LOGIN_SETTIMESPEED {
    /// Current server datetime.
    ///
    /// This is not just unix time but instead seems to be a custom bitfield.
    /// vmangos/cmangos/mangoszero uses the format `years_after_2000 << 24 | month << 20 | month_day << 14 | week_day << 11 | hours << 6 | minutes`. All values start at 0 and `week_day` starts on Sunday.
    /// Running the client with `-console` verifies that this message in this format sets the correct datetime. [`SMSG_QUERY_TIME_RESPONSE`](crate::world::version_1_12::SMSG_QUERY_TIME_RESPONSE) will not set this.
    ///
    pub datetime: u32,
    /// How many minutes should pass by every second.
    ///
    /// vmangos/cmangos/mangoszero set this to 0.01666667. This means that 1/60 minutes pass every second (one second passes every second). Setting this to 1.0 will make the client advance one minute every second.
    ///
    pub timescale: f32,
}

impl ServerMessage for SMSG_LOGIN_SETTIMESPEED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // datetime: u32
        w.write_all(&self.datetime.to_le_bytes())?;

        // timescale: f32
        w.write_all(&self.timescale.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0042;

    fn server_size(&self) -> u16 {
        12
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // datetime: u32
        let datetime = crate::util::read_u32_le(r)?;

        // timescale: f32
        let timescale = crate::util::read_f32_le(r)?;
        Ok(Self {
            datetime,
            timescale,
        })
    }

}

#[cfg(test)]
mod test {
    use super::SMSG_LOGIN_SETTIMESPEED;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 12] = [ 0x00, 0x0A, 0x42, 0x00, 0x0A, 0x1A, 0x73, 0x16, 0x89,
         0x88, 0x88, 0x3C, ];

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm` line 20.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGIN_SETTIMESPEED0() {
        let expected = SMSG_LOGIN_SETTIMESPEED {
            datetime: 0x16731A0A,
            timescale: 0.016666668_f32,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_SETTIMESPEED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.datetime, expected.datetime);
        assert_eq!(t.timescale, expected.timescale);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm` line 20.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_LOGIN_SETTIMESPEED0() {
        let expected = SMSG_LOGIN_SETTIMESPEED {
            datetime: 0x16731A0A,
            timescale: 0.016666668_f32,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_SETTIMESPEED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.datetime, expected.datetime);
        assert_eq!(t.timescale, expected.timescale);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm` line 20.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_LOGIN_SETTIMESPEED0() {
        let expected = SMSG_LOGIN_SETTIMESPEED {
            datetime: 0x16731A0A,
            timescale: 0.016666668_f32,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_SETTIMESPEED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.datetime, expected.datetime);
        assert_eq!(t.timescale, expected.timescale);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
