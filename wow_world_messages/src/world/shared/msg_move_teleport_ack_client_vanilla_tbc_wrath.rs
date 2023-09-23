use std::io::{Read, Write};

use crate::Guid;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Response to [`MSG_MOVE_TELEPORT_ACK_Server`](crate::vanilla::MSG_MOVE_TELEPORT_ACK_Server), at which point [`MSG_MOVE_TELEPORT_ACK_Server`](crate::vanilla::MSG_MOVE_TELEPORT_ACK_Server) should be sent to observing players.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm#L2):
/// ```text
/// cmsg MSG_MOVE_TELEPORT_ACK_Client = 0x00C7 {
///     PackedGuid guid;
///     u32 movement_counter;
///     Milliseconds time;
/// }
/// ```
pub struct MSG_MOVE_TELEPORT_ACK_Client {
    pub guid: Guid,
    pub movement_counter: u32,
    pub time: Duration,
}

impl crate::private::Sealed for MSG_MOVE_TELEPORT_ACK_Client {}
impl MSG_MOVE_TELEPORT_ACK_Client {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=17).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        // time: Milliseconds
        let time = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            guid,
            movement_counter,
            time,
        })
    }

}

impl crate::Message for MSG_MOVE_TELEPORT_ACK_Client {
    const OPCODE: u32 = 0x00c7;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_MOVE_TELEPORT_ACK_Client"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // time: Milliseconds
        w.write_all((self.time.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(199, "MSG_MOVE_TELEPORT_ACK_Client", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MOVE_TELEPORT_ACK_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_MOVE_TELEPORT_ACK_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_MOVE_TELEPORT_ACK_Client {}

impl MSG_MOVE_TELEPORT_ACK_Client {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // movement_counter: u32
        + 4 // time: Milliseconds
    }
}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_MOVE_TELEPORT_ACK_Client;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &MSG_MOVE_TELEPORT_ACK_Client, expected: &MSG_MOVE_TELEPORT_ACK_Client) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.movement_counter, expected.movement_counter);
        assert_eq!(t.time, expected.time);
    }

    const RAW0: [u8; 15] = [ 0x00, 0x0D, 0xC7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x26, ];

    pub(crate) fn expected0() -> MSG_MOVE_TELEPORT_ACK_Client {
        MSG_MOVE_TELEPORT_ACK_Client {
            guid: Guid::new(0x0),
            movement_counter: 0x0,
            time: Duration::from_millis(0x26000000),
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 21.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_move_teleport_ack_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 21.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_move_teleport_ack_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 21.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_move_teleport_ack_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_MOVE_TELEPORT_ACK_Client;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &MSG_MOVE_TELEPORT_ACK_Client, expected: &MSG_MOVE_TELEPORT_ACK_Client) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.movement_counter, expected.movement_counter);
        assert_eq!(t.time, expected.time);
    }

    const RAW0: [u8; 15] = [ 0x00, 0x0D, 0xC7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x26, ];

    pub(crate) fn expected0() -> MSG_MOVE_TELEPORT_ACK_Client {
        MSG_MOVE_TELEPORT_ACK_Client {
            guid: Guid::new(0x0),
            movement_counter: 0x0,
            time: Duration::from_millis(0x26000000),
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 21.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_move_teleport_ack_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 21.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_move_teleport_ack_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 21.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_move_teleport_ack_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_MOVE_TELEPORT_ACK_Client;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &MSG_MOVE_TELEPORT_ACK_Client, expected: &MSG_MOVE_TELEPORT_ACK_Client) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.movement_counter, expected.movement_counter);
        assert_eq!(t.time, expected.time);
    }

    const RAW0: [u8; 15] = [ 0x00, 0x0D, 0xC7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x26, ];

    pub(crate) fn expected0() -> MSG_MOVE_TELEPORT_ACK_Client {
        MSG_MOVE_TELEPORT_ACK_Client {
            guid: Guid::new(0x0),
            movement_counter: 0x0,
            time: Duration::from_millis(0x26000000),
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 21.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_move_teleport_ack_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 21.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_move_teleport_ack_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 21.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_move_teleport_ack_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

