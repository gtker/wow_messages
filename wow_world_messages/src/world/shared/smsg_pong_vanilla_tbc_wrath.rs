use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/ping_pong/smsg_pong.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/ping_pong/smsg_pong.wowm#L2):
/// ```text
/// smsg SMSG_PONG = 0x01DD {
///     u32 sequence_id;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PONG {
    pub sequence_id: u32,
}

impl crate::private::Sealed for SMSG_PONG {}
impl SMSG_PONG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // sequence_id: u32
        let sequence_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            sequence_id,
        })
    }

}

impl crate::Message for SMSG_PONG {
    const OPCODE: u32 = 0x01dd;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PONG"
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // sequence_id: u32
        w.write_all(&self.sequence_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(477, "SMSG_PONG", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PONG {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PONG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PONG {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_PONG;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 8] = [ 0x00, 0x06, 0xDD, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, ];

    pub(crate) fn expected0() -> SMSG_PONG {
        SMSG_PONG {
            sequence_id: 0xDEADBEEF,
        }

    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/smsg_pong.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_pong0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/smsg_pong.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_pong0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/smsg_pong.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_pong0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_PONG;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 8] = [ 0x00, 0x06, 0xDD, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, ];

    pub(crate) fn expected0() -> SMSG_PONG {
        SMSG_PONG {
            sequence_id: 0xDEADBEEF,
        }

    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/smsg_pong.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_pong0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/smsg_pong.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_pong0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/smsg_pong.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_pong0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_PONG;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 8] = [ 0x00, 0x06, 0xDD, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, ];

    pub(crate) fn expected0() -> SMSG_PONG {
        SMSG_PONG {
            sequence_id: 0xDEADBEEF,
        }

    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/smsg_pong.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_pong0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/smsg_pong.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_pong0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/smsg_pong.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_pong0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PONG(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PONG, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

