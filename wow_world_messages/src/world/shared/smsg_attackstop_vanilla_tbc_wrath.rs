use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackstop.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackstop.wowm#L10):
/// ```text
/// smsg SMSG_ATTACKSTOP = 0x0144 {
///     PackedGuid player;
///     PackedGuid enemy;
///     u32 unknown1;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ATTACKSTOP {
    pub player: Guid,
    pub enemy: Guid,
    /// cmangos/vmangos/mangoszero/arcemu/azerothcore/mangostwo: set to 0 with comment: unk, can be 1 also
    pub unknown1: u32,
}

impl crate::private::Sealed for SMSG_ATTACKSTOP {}
impl SMSG_ATTACKSTOP {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(6..=22).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: PackedGuid
        let player = crate::util::read_packed_guid(&mut r)?;

        // enemy: PackedGuid
        let enemy = crate::util::read_packed_guid(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            player,
            enemy,
            unknown1,
        })
    }

}

impl crate::Message for SMSG_ATTACKSTOP {
    const OPCODE: u32 = 0x0144;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ATTACKSTOP"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        crate::util::write_packed_guid(&self.player, &mut w)?;

        // enemy: PackedGuid
        crate::util::write_packed_guid(&self.enemy, &mut w)?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(324, "SMSG_ATTACKSTOP", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ATTACKSTOP {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ATTACKSTOP {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ATTACKSTOP {}

impl SMSG_ATTACKSTOP {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + crate::util::packed_guid_size(&self.enemy) // enemy: PackedGuid
        + 4 // unknown1: u32
    }
}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_ATTACKSTOP;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_ATTACKSTOP, expected: &SMSG_ATTACKSTOP) {
        assert_eq!(t.player, expected.player);
        assert_eq!(t.enemy, expected.enemy);
        assert_eq!(t.unknown1, expected.unknown1);
    }

    const RAW0: [u8; 12] = [ 0x00, 0x0A, 0x44, 0x01, 0x01, 0x17, 0x01, 0x64, 0x00,
         0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ATTACKSTOP {
        SMSG_ATTACKSTOP {
            player: Guid::new(0x17),
            enemy: Guid::new(0x64),
            unknown1: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstop.wowm` line 20.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_attackstop0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTOP(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstop.wowm` line 20.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_attackstop0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTOP(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstop.wowm` line 20.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_attackstop0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTOP(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_ATTACKSTOP;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_ATTACKSTOP, expected: &SMSG_ATTACKSTOP) {
        assert_eq!(t.player, expected.player);
        assert_eq!(t.enemy, expected.enemy);
        assert_eq!(t.unknown1, expected.unknown1);
    }

    const RAW0: [u8; 12] = [ 0x00, 0x0A, 0x44, 0x01, 0x01, 0x17, 0x01, 0x64, 0x00,
         0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ATTACKSTOP {
        SMSG_ATTACKSTOP {
            player: Guid::new(0x17),
            enemy: Guid::new(0x64),
            unknown1: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstop.wowm` line 20.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_attackstop0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTOP(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstop.wowm` line 20.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_attackstop0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTOP(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstop.wowm` line 20.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_attackstop0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTOP(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_ATTACKSTOP;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_ATTACKSTOP, expected: &SMSG_ATTACKSTOP) {
        assert_eq!(t.player, expected.player);
        assert_eq!(t.enemy, expected.enemy);
        assert_eq!(t.unknown1, expected.unknown1);
    }

    const RAW0: [u8; 12] = [ 0x00, 0x0A, 0x44, 0x01, 0x01, 0x17, 0x01, 0x64, 0x00,
         0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ATTACKSTOP {
        SMSG_ATTACKSTOP {
            player: Guid::new(0x17),
            enemy: Guid::new(0x64),
            unknown1: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstop.wowm` line 20.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_attackstop0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTOP(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstop.wowm` line 20.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_attackstop0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTOP(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackstop.wowm` line 20.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_attackstop0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKSTOP(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKSTOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

