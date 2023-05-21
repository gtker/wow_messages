use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_TIME_SKIPPED = 0x02CE {
///     Guid guid;
///     u32 lag;
/// }
/// ```
pub struct CMSG_MOVE_TIME_SKIPPED {
    pub guid: Guid,
    pub lag: u32,
}

impl crate::private::Sealed for CMSG_MOVE_TIME_SKIPPED {}
impl CMSG_MOVE_TIME_SKIPPED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // lag: u32
        let lag = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            lag,
        })
    }

}

impl crate::Message for CMSG_MOVE_TIME_SKIPPED {
    const OPCODE: u32 = 0x02ce;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // lag: u32
        w.write_all(&self.lag.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(718, "CMSG_MOVE_TIME_SKIPPED", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MOVE_TIME_SKIPPED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_MOVE_TIME_SKIPPED {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_MOVE_TIME_SKIPPED;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_MOVE_TIME_SKIPPED, expected: &CMSG_MOVE_TIME_SKIPPED) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.lag, expected.lag);
    }

    const RAW0: [u8; 18] = [ 0x00, 0x10, 0xCE, 0x02, 0x00, 0x00, 0x17, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_MOVE_TIME_SKIPPED {
        CMSG_MOVE_TIME_SKIPPED {
            guid: Guid::new(0x17),
            lag: 0x20,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_move_time_skipped0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_MOVE_TIME_SKIPPED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_MOVE_TIME_SKIPPED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_move_time_skipped0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_MOVE_TIME_SKIPPED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_MOVE_TIME_SKIPPED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_move_time_skipped0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_MOVE_TIME_SKIPPED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_MOVE_TIME_SKIPPED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_MOVE_TIME_SKIPPED;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_MOVE_TIME_SKIPPED, expected: &CMSG_MOVE_TIME_SKIPPED) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.lag, expected.lag);
    }

    const RAW0: [u8; 18] = [ 0x00, 0x10, 0xCE, 0x02, 0x00, 0x00, 0x17, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_MOVE_TIME_SKIPPED {
        CMSG_MOVE_TIME_SKIPPED {
            guid: Guid::new(0x17),
            lag: 0x20,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_move_time_skipped0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_MOVE_TIME_SKIPPED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_MOVE_TIME_SKIPPED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_move_time_skipped0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_MOVE_TIME_SKIPPED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_MOVE_TIME_SKIPPED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_move_time_skipped.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_move_time_skipped0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_MOVE_TIME_SKIPPED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_MOVE_TIME_SKIPPED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

