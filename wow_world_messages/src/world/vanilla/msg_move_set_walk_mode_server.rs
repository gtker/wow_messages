use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_set_walk_mode.wowm:46`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_set_walk_mode.wowm#L46):
/// ```text
/// smsg MSG_MOVE_SET_WALK_MODE_Server = 0x00C3 {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_SET_WALK_MODE_Server {
    pub guid: Guid,
    pub info: MovementInfo,
}

impl crate::private::Sealed for MSG_MOVE_SET_WALK_MODE_Server {}
impl MSG_MOVE_SET_WALK_MODE_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(30..=90).contains(&body_size) {
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

impl crate::Message for MSG_MOVE_SET_WALK_MODE_Server {
    const OPCODE: u32 = 0x00c3;

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
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(195, "MSG_MOVE_SET_WALK_MODE_Server", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MOVE_SET_WALK_MODE_Server {}

impl MSG_MOVE_SET_WALK_MODE_Server {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_MOVE_SET_WALK_MODE_Server;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &MSG_MOVE_SET_WALK_MODE_Server, expected: &MSG_MOVE_SET_WALK_MODE_Server) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);
    }

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xC3, 0x00, 0x01, 0x05, 0x01, 0x01, 0x00,
         0x00, 0x9A, 0x17, 0x5B, 0x02, 0x02, 0xBD, 0x0B, 0xC6, 0x4E, 0x58, 0x01,
         0xC3, 0x26, 0x29, 0xA7, 0x42, 0x2E, 0x0E, 0xC3, 0x40, 0x00, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> MSG_MOVE_SET_WALK_MODE_Server {
        MSG_MOVE_SET_WALK_MODE_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_walk_mode()
                    .set_forward()
                    ,
                timestamp: 0x25B179A,
                position: Vector3d {
                    x: -8943.252_f32,
                    y: -129.34494_f32,
                    z: 83.58037_f32,
                },
                orientation: 6.095481_f32,
                fall_time: 0_f32,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_walk_mode.wowm` line 60.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_move_set_walk_mode_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_SET_WALK_MODE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_WALK_MODE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_walk_mode.wowm` line 60.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_move_set_walk_mode_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_SET_WALK_MODE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_WALK_MODE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_walk_mode.wowm` line 60.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_move_set_walk_mode_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_SET_WALK_MODE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_WALK_MODE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

