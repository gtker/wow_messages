use std::io::{Read, Write};

use crate::DateTime;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Tells the client what the datetime is and how fast time passes.
///
/// The client also asks for the datetime with [`CMSG_QUERY_TIME`](crate::vanilla::CMSG_QUERY_TIME) and gets a reply from [`SMSG_QUERY_TIME_RESPONSE`](crate::vanilla::SMSG_QUERY_TIME_RESPONSE), but this does not appear to change anything in the client.
/// Despite sending this as the very first message after the client logs in it will still send a [`CMSG_QUERY_TIME`](crate::vanilla::CMSG_QUERY_TIME).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm#L1):
/// ```text
/// smsg SMSG_LOGIN_SETTIMESPEED = 0x0042 {
///     DateTime datetime;
///     f32 timescale;
/// }
/// ```
pub struct SMSG_LOGIN_SETTIMESPEED {
    /// Current server datetime.
    ///
    /// Running the client with `-console` verifies that this message in this format sets the correct datetime. [`SMSG_QUERY_TIME_RESPONSE`](crate::vanilla::SMSG_QUERY_TIME_RESPONSE) will not set this.
    pub datetime: DateTime,
    /// How many minutes should pass by every second.
    ///
    /// vmangos/cmangos/mangoszero set this to 0.01666667. This means that 1/60 minutes pass every second (one second passes every second). Setting this to 1.0 will make the client advance one minute every second.
    pub timescale: f32,
}

impl crate::private::Sealed for SMSG_LOGIN_SETTIMESPEED {}
impl SMSG_LOGIN_SETTIMESPEED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // datetime: DateTime
        let datetime = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // timescale: f32
        let timescale = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            datetime,
            timescale,
        })
    }

}

impl crate::Message for SMSG_LOGIN_SETTIMESPEED {
    const OPCODE: u32 = 0x0042;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // datetime: DateTime
        w.write_all(&self.datetime.as_int().to_le_bytes())?;

        // timescale: f32
        w.write_all(&self.timescale.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(66, "SMSG_LOGIN_SETTIMESPEED", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOGIN_SETTIMESPEED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOGIN_SETTIMESPEED {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_LOGIN_SETTIMESPEED;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_LOGIN_SETTIMESPEED, expected: &SMSG_LOGIN_SETTIMESPEED) {
        assert_eq!(t.datetime, expected.datetime);
        assert_eq!(t.timescale, expected.timescale);
    }

    const RAW0: [u8; 12] = [ 0x00, 0x0A, 0x42, 0x00, 0x0A, 0x1A, 0x73, 0x16, 0x89,
         0x88, 0x88, 0x3C, ];

    pub(crate) fn expected0() -> SMSG_LOGIN_SETTIMESPEED {
        SMSG_LOGIN_SETTIMESPEED {
            datetime: DateTime::try_from(0x16731A0A).unwrap(),
            timescale: 0.016666668_f32,
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm` line 17.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_login_settimespeed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_SETTIMESPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm` line 17.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_login_settimespeed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_SETTIMESPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm` line 17.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_login_settimespeed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_SETTIMESPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_LOGIN_SETTIMESPEED;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_LOGIN_SETTIMESPEED, expected: &SMSG_LOGIN_SETTIMESPEED) {
        assert_eq!(t.datetime, expected.datetime);
        assert_eq!(t.timescale, expected.timescale);
    }

    const RAW0: [u8; 12] = [ 0x00, 0x0A, 0x42, 0x00, 0x0A, 0x1A, 0x73, 0x16, 0x89,
         0x88, 0x88, 0x3C, ];

    pub(crate) fn expected0() -> SMSG_LOGIN_SETTIMESPEED {
        SMSG_LOGIN_SETTIMESPEED {
            datetime: DateTime::try_from(0x16731A0A).unwrap(),
            timescale: 0.016666668_f32,
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm` line 17.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_login_settimespeed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_SETTIMESPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm` line 17.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_login_settimespeed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_SETTIMESPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm` line 17.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_login_settimespeed0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_SETTIMESPEED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_SETTIMESPEED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

