use std::io::{Read, Write};

use wow_world_base::shared::sheath_state_vanilla_tbc_wrath::SheathState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Says which weapon the client pulls out.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm#L11):
/// ```text
/// cmsg CMSG_SETSHEATHED = 0x01E0 {
///     (u32)SheathState sheathed;
/// }
/// ```
pub struct CMSG_SETSHEATHED {
    pub sheathed: SheathState,
}

impl crate::private::Sealed for CMSG_SETSHEATHED {}
impl CMSG_SETSHEATHED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x01E0, size: body_size });
        }

        // sheathed: SheathState
        let sheathed = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            sheathed,
        })
    }

}

impl crate::Message for CMSG_SETSHEATHED {
    const OPCODE: u32 = 0x01e0;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // sheathed: SheathState
        w.write_all(&u32::from(self.sheathed.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SETSHEATHED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SETSHEATHED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SETSHEATHED {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_SETSHEATHED;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_SETSHEATHED, expected: &CMSG_SETSHEATHED) {
        assert_eq!(t.sheathed, expected.sheathed);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0xE0, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_SETSHEATHED {
        CMSG_SETSHEATHED {
            sheathed: SheathState::Melee,
        }

    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 18.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_setsheathed0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 18.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_setsheathed0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 18.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_setsheathed0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}"),
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
    use super::CMSG_SETSHEATHED;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_SETSHEATHED, expected: &CMSG_SETSHEATHED) {
        assert_eq!(t.sheathed, expected.sheathed);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0xE0, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_SETSHEATHED {
        CMSG_SETSHEATHED {
            sheathed: SheathState::Melee,
        }

    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 18.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_setsheathed0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 18.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_setsheathed0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 18.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_setsheathed0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}"),
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
    use super::CMSG_SETSHEATHED;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_SETSHEATHED, expected: &CMSG_SETSHEATHED) {
        assert_eq!(t.sheathed, expected.sheathed);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0xE0, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_SETSHEATHED {
        CMSG_SETSHEATHED {
            sheathed: SheathState::Melee,
        }

    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 18.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_setsheathed0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 18.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_setsheathed0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 18.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_setsheathed0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

