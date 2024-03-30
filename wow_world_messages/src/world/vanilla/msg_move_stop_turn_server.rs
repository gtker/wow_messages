use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::MovementInfo;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_stop_turn.wowm:46`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_stop_turn.wowm#L46):
/// ```text
/// smsg MSG_MOVE_STOP_TURN_Server = 0x00BE {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MSG_MOVE_STOP_TURN_Server {
    pub guid: Guid,
    pub info: MovementInfo,
}

impl crate::private::Sealed for MSG_MOVE_STOP_TURN_Server {}
impl MSG_MOVE_STOP_TURN_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(29..=90).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            guid,
            info,
        })
    }

}

impl crate::Message for MSG_MOVE_STOP_TURN_Server {
    const OPCODE: u32 = 0x00be;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_MOVE_STOP_TURN_Server"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(190, "MSG_MOVE_STOP_TURN_Server", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MOVE_STOP_TURN_Server {}

impl MSG_MOVE_STOP_TURN_Server {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_MOVE_STOP_TURN_Server;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &MSG_MOVE_STOP_TURN_Server, expected: &MSG_MOVE_STOP_TURN_Server) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);
    }

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xBE, 0x00, 0x01, 0x05, 0x00, 0x00, 0x00,
         0x00, 0x97, 0xA6, 0x5B, 0x02, 0x25, 0xA2, 0x0B, 0xC6, 0x39, 0x82, 0xF8,
         0xC2, 0xDE, 0x48, 0xA5, 0x42, 0x10, 0x13, 0x9C, 0x40, 0x00, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> MSG_MOVE_STOP_TURN_Server {
        MSG_MOVE_STOP_TURN_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    ,
                timestamp: 0x25BA697,
                position: Vector3d {
                    x: -8936.536_f32,
                    y: -124.25434_f32,
                    z: 82.64232_f32,
                },
                orientation: 4.877327_f32,
                fall_time: 0_f32,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop_turn.wowm` line 53.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_move_stop_turn_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_STOP_TURN(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP_TURN, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop_turn.wowm` line 53.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_move_stop_turn_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_STOP_TURN(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP_TURN, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop_turn.wowm` line 53.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_move_stop_turn_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_STOP_TURN(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP_TURN, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

