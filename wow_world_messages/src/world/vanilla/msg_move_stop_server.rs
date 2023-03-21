use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_stop.wowm:46`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_stop.wowm#L46):
/// ```text
/// smsg MSG_MOVE_STOP_Server = 0x00B7 {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_STOP_Server {
    pub guid: Guid,
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_STOP_Server {
    const OPCODE: u32 = 0x00b7;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(30..=90).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00B7, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            guid,
            info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MOVE_STOP_Server {}

impl MSG_MOVE_STOP_Server {
    pub(crate) const fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_MOVE_STOP_Server;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &MSG_MOVE_STOP_Server, expected: &MSG_MOVE_STOP_Server) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);
    }

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xB7, 0x00, 0x01, 0x05, 0x00, 0x00, 0x00,
         0x00, 0xF2, 0x31, 0x7A, 0x01, 0x24, 0xCB, 0x0B, 0xC6, 0x30, 0x20, 0xDF,
         0xC2, 0x3D, 0x17, 0xA6, 0x42, 0x03, 0x51, 0x24, 0x40, 0x85, 0x03, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> MSG_MOVE_STOP_Server {
        MSG_MOVE_STOP_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    ,
                timestamp: 0x17A31F2,
                position: Vector3d {
                    x: -8946.785_f32,
                    y: -111.56287_f32,
                    z: 83.04539_f32,
                },
                orientation: 2.5674446_f32,
                fall_time: 0.000000000000000000000000000000000000000001263_f32,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop.wowm` line 53.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_MOVE_STOP_Server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_STOP(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop.wowm` line 53.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_MOVE_STOP_Server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_STOP(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop.wowm` line 53.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_MOVE_STOP_Server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_STOP(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

