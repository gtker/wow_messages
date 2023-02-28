use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/ping_pong/cmsg_ping.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/ping_pong/cmsg_ping.wowm#L7):
/// ```text
/// cmsg CMSG_PING = 0x01DC {
///     u32 sequence_id;
///     u32 round_time_in_ms;
/// }
/// ```
pub struct CMSG_PING {
    pub sequence_id: u32,
    pub round_time_in_ms: u32,
}

impl crate::Message for CMSG_PING {
    const OPCODE: u32 = 0x01dc;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // sequence_id: u32
        w.write_all(&self.sequence_id.to_le_bytes())?;

        // round_time_in_ms: u32
        w.write_all(&self.round_time_in_ms.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01DC, size: body_size as u32 });
        }

        // sequence_id: u32
        let sequence_id = crate::util::read_u32_le(&mut r)?;

        // round_time_in_ms: u32
        let round_time_in_ms = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            sequence_id,
            round_time_in_ms,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PING {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PING {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PING {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    use super::CMSG_PING;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0xDC, 0x01, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/ping_pong/cmsg_ping.wowm` line 14.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_PING0() {
        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/cmsg_ping.wowm` line 14.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_PING0() {
        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/cmsg_ping.wowm` line 14.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_PING0() {
        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    use super::CMSG_PING;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0xDC, 0x01, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/ping_pong/cmsg_ping.wowm` line 14.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_PING0() {
        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/cmsg_ping.wowm` line 14.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_PING0() {
        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/cmsg_ping.wowm` line 14.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_PING0() {
        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    use super::CMSG_PING;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0xDC, 0x01, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/ping_pong/cmsg_ping.wowm` line 14.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_PING0() {
        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/cmsg_ping.wowm` line 14.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_PING0() {
        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/ping_pong/cmsg_ping.wowm` line 14.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_PING0() {
        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

