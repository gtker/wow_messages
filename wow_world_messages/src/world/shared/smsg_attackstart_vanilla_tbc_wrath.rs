use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackstart.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackstart.wowm#L1):
/// ```text
/// smsg SMSG_ATTACKSTART = 0x0143 {
///     Guid attacker;
///     Guid victim;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ATTACKSTART {
    pub attacker: Guid,
    pub victim: Guid,
}

impl crate::private::Sealed for SMSG_ATTACKSTART {}
impl SMSG_ATTACKSTART {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // attacker: Guid
        let attacker = crate::util::read_guid(&mut r)?;

        // victim: Guid
        let victim = crate::util::read_guid(&mut r)?;

        Ok(Self {
            attacker,
            victim,
        })
    }

}

impl crate::Message for SMSG_ATTACKSTART {
    const OPCODE: u32 = 0x0143;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ATTACKSTART"
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // attacker: Guid
        w.write_all(&self.attacker.guid().to_le_bytes())?;

        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(323, "SMSG_ATTACKSTART", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ATTACKSTART {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ATTACKSTART {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ATTACKSTART {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_ATTACKSTART;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 20] = [ 0x00, 0x12, 0x43, 0x01, 0x17, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ATTACKSTART {
        SMSG_ATTACKSTART {
            attacker: Guid::new(0x17),
            victim: Guid::new(0x64),
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstart.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_attackstart0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTART(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTART, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstart.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_attackstart0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTART(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTART, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstart.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_attackstart0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTART(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTART, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_ATTACKSTART;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 20] = [ 0x00, 0x12, 0x43, 0x01, 0x17, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ATTACKSTART {
        SMSG_ATTACKSTART {
            attacker: Guid::new(0x17),
            victim: Guid::new(0x64),
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstart.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_attackstart0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTART(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTART, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstart.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_attackstart0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTART(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTART, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstart.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_attackstart0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTART(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTART, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_ATTACKSTART;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 20] = [ 0x00, 0x12, 0x43, 0x01, 0x17, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ATTACKSTART {
        SMSG_ATTACKSTART {
            attacker: Guid::new(0x17),
            victim: Guid::new(0x64),
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstart.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_attackstart0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTART(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTART, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstart.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_attackstart0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTART(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTART, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstart.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_attackstart0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTART(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTART, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

