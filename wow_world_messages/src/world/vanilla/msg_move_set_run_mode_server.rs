use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_set_run_mode.wowm:45`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_set_run_mode.wowm#L45):
/// ```text
/// smsg MSG_MOVE_SET_RUN_MODE_Server = 0x00C2 {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_SET_RUN_MODE_Server {
    pub guid: Guid,
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_SET_RUN_MODE_Server {
    const OPCODE: u32 = 0x00c2;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            guid,
            info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for MSG_MOVE_SET_RUN_MODE_Server {}

impl MSG_MOVE_SET_RUN_MODE_Server {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    use super::MSG_MOVE_SET_RUN_MODE_Server;
    use crate::world::vanilla::MovementFlags;
    use crate::world::vanilla::MovementInfo;
    use crate::world::vanilla::TransportInfo;
    use crate::world::vanilla::Vector3d;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xC2, 0x00, 0x01, 0x05, 0x01, 0x00, 0x00,
         0x00, 0x41, 0x1B, 0x5B, 0x02, 0x55, 0xB9, 0x0B, 0xC6, 0xF8, 0x84, 0x01,
         0xC3, 0xAD, 0x31, 0xA7, 0x42, 0x2E, 0x0E, 0xC3, 0x40, 0x00, 0x00, 0x00,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_run_mode.wowm` line 66.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_MOVE_SET_RUN_MODE_Server0() {
        let expected = MSG_MOVE_SET_RUN_MODE_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_FORWARD()
                    ,
                timestamp: 0x25B1B41,
                position: Vector3d {
                    x: -8942.333_f32,
                    y: -129.51941_f32,
                    z: 83.59702_f32,
                },
                orientation: 6.095481_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_SET_RUN_MODE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_RUN_MODE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_run_mode.wowm` line 66.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_MOVE_SET_RUN_MODE_Server0() {
        let expected = MSG_MOVE_SET_RUN_MODE_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_FORWARD()
                    ,
                timestamp: 0x25B1B41,
                position: Vector3d {
                    x: -8942.333_f32,
                    y: -129.51941_f32,
                    z: 83.59702_f32,
                },
                orientation: 6.095481_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_SET_RUN_MODE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_RUN_MODE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_run_mode.wowm` line 66.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_MOVE_SET_RUN_MODE_Server0() {
        let expected = MSG_MOVE_SET_RUN_MODE_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_FORWARD()
                    ,
                timestamp: 0x25B1B41,
                position: Vector3d {
                    x: -8942.333_f32,
                    y: -129.51941_f32,
                    z: 83.59702_f32,
                },
                orientation: 6.095481_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_SET_RUN_MODE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_RUN_MODE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
