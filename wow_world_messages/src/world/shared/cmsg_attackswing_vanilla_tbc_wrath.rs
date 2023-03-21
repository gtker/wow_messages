use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Signals that client has right clicked an opponent and is in the attack stance.
/// Server should reply with [`SMSG_ATTACKSTART`](crate::vanilla::SMSG_ATTACKSTART).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm#L3):
/// ```text
/// cmsg CMSG_ATTACKSWING = 0x0141 {
///     Guid guid;
/// }
/// ```
pub struct CMSG_ATTACKSWING {
    pub guid: Guid,
}

impl crate::Message for CMSG_ATTACKSWING {
    const OPCODE: u32 = 0x0141;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0141, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ATTACKSWING {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ATTACKSWING {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ATTACKSWING {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    use super::CMSG_ATTACKSWING;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x41, 0x01, 0x00, 0x00, 0x64, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_ATTACKSWING {
        CMSG_ATTACKSWING {
            guid: Guid::new(0x64),
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_ATTACKSWING0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_ATTACKSWING0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_ATTACKSWING0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    use super::CMSG_ATTACKSWING;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x41, 0x01, 0x00, 0x00, 0x64, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_ATTACKSWING {
        CMSG_ATTACKSWING {
            guid: Guid::new(0x64),
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_ATTACKSWING0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_ATTACKSWING0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_ATTACKSWING0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    use super::CMSG_ATTACKSWING;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x41, 0x01, 0x00, 0x00, 0x64, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_ATTACKSWING {
        CMSG_ATTACKSWING {
            guid: Guid::new(0x64),
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_ATTACKSWING0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_ATTACKSWING0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_ATTACKSWING0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

