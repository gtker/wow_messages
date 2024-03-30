use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::MovementInfo;

/// Can be response to [`CMSG_TELEPORT_TO_UNIT`](crate::vanilla::CMSG_TELEPORT_TO_UNIT).
/// Can also be a response to [`MSG_MOVE_TELEPORT_ACK_Client`](crate::vanilla::MSG_MOVE_TELEPORT_ACK_Client) after being sent.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm#L12):
/// ```text
/// smsg MSG_MOVE_TELEPORT_ACK_Server = 0x00C7 {
///     PackedGuid guid;
///     u32 movement_counter;
///     MovementInfo info;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MSG_MOVE_TELEPORT_ACK_Server {
    pub guid: Guid,
    pub movement_counter: u32,
    pub info: MovementInfo,
}

impl crate::private::Sealed for MSG_MOVE_TELEPORT_ACK_Server {}
impl MSG_MOVE_TELEPORT_ACK_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(33..=94).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            guid,
            movement_counter,
            info,
        })
    }

}

impl crate::Message for MSG_MOVE_TELEPORT_ACK_Server {
    const OPCODE: u32 = 0x00c7;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_MOVE_TELEPORT_ACK_Server"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(199, "MSG_MOVE_TELEPORT_ACK_Server", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MOVE_TELEPORT_ACK_Server {}

impl MSG_MOVE_TELEPORT_ACK_Server {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // movement_counter: u32
        + self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_MOVE_TELEPORT_ACK_Server;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 38] = [ 0x00, 0x24, 0xC7, 0x00, 0x01, 0x17, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x87,
         0x45, 0x00, 0xA0, 0x25, 0xC5, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> MSG_MOVE_TELEPORT_ACK_Server {
        MSG_MOVE_TELEPORT_ACK_Server {
            guid: Guid::new(0x17),
            movement_counter: 0x0,
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    ,
                timestamp: 0x0,
                position: Vector3d {
                    x: 4320_f32,
                    y: -2650_f32,
                    z: 0_f32,
                },
                orientation: 0_f32,
                fall_time: 0_f32,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 36.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_move_teleport_ack_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 36.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_move_teleport_ack_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm` line 36.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_move_teleport_ack_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_TELEPORT_ACK(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_TELEPORT_ACK, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

