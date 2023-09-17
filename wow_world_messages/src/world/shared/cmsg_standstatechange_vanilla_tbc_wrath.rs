use std::io::{Read, Write};

use wow_world_base::shared::unit_stand_state_vanilla_tbc_wrath::UnitStandState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Automatically sent by the client when it goes AFK.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm#L3):
/// ```text
/// cmsg CMSG_STANDSTATECHANGE = 0x0101 {
///     (u32)UnitStandState animation_state;
/// }
/// ```
pub struct CMSG_STANDSTATECHANGE {
    pub animation_state: UnitStandState,
}

impl crate::private::Sealed for CMSG_STANDSTATECHANGE {}
impl CMSG_STANDSTATECHANGE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // animation_state: UnitStandState
        let animation_state = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            animation_state,
        })
    }

}

impl crate::Message for CMSG_STANDSTATECHANGE {
    const OPCODE: u32 = 0x0101;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_STANDSTATECHANGE"
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // animation_state: UnitStandState
        w.write_all(&u32::from(self.animation_state.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(257, "CMSG_STANDSTATECHANGE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_STANDSTATECHANGE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_STANDSTATECHANGE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_STANDSTATECHANGE {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_STANDSTATECHANGE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_STANDSTATECHANGE, expected: &CMSG_STANDSTATECHANGE) {
        assert_eq!(t.animation_state, expected.animation_state);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_STANDSTATECHANGE {
        CMSG_STANDSTATECHANGE {
            animation_state: UnitStandState::Sit,
        }

    }

    // Generated from `wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_standstatechange0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_STANDSTATECHANGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_STANDSTATECHANGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_standstatechange0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_STANDSTATECHANGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_STANDSTATECHANGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_standstatechange0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_STANDSTATECHANGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_STANDSTATECHANGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_STANDSTATECHANGE;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_STANDSTATECHANGE, expected: &CMSG_STANDSTATECHANGE) {
        assert_eq!(t.animation_state, expected.animation_state);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_STANDSTATECHANGE {
        CMSG_STANDSTATECHANGE {
            animation_state: UnitStandState::Sit,
        }

    }

    // Generated from `wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_standstatechange0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_STANDSTATECHANGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_STANDSTATECHANGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_standstatechange0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_STANDSTATECHANGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_STANDSTATECHANGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_standstatechange0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_STANDSTATECHANGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_STANDSTATECHANGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_STANDSTATECHANGE;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_STANDSTATECHANGE, expected: &CMSG_STANDSTATECHANGE) {
        assert_eq!(t.animation_state, expected.animation_state);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_STANDSTATECHANGE {
        CMSG_STANDSTATECHANGE {
            animation_state: UnitStandState::Sit,
        }

    }

    // Generated from `wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_standstatechange0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_STANDSTATECHANGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_STANDSTATECHANGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_standstatechange0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_STANDSTATECHANGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_STANDSTATECHANGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_standstatechange0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_STANDSTATECHANGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_STANDSTATECHANGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

