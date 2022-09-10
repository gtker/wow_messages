use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_start_strafe_left.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_start_strafe_left.wowm#L31):
/// ```text
/// smsg MSG_MOVE_START_STRAFE_LEFT_Server = 0x00B8 {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_START_STRAFE_LEFT_Server {
    pub guid: Guid,
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_START_STRAFE_LEFT_Server {
    const OPCODE: u32 = 0x00b8;

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
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for MSG_MOVE_START_STRAFE_LEFT_Server {}

impl MSG_MOVE_START_STRAFE_LEFT_Server {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    use super::MSG_MOVE_START_STRAFE_LEFT_Server;
    use crate::world::vanilla::MovementFlags;
    use crate::world::vanilla::MovementInfo;
    use crate::world::vanilla::TransportInfo;
    use crate::world::vanilla::Vector3d;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xB8, 0x00, 0x01, 0x05, 0x05, 0x00, 0x00,
         0x00, 0x9F, 0xD2, 0x79, 0x01, 0xEE, 0xC1, 0x0B, 0xC6, 0xFD, 0x44, 0x08,
         0xC3, 0x24, 0x7E, 0xA7, 0x42, 0xB8, 0x9D, 0xC2, 0x3E, 0x7F, 0x03, 0x00,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_strafe_left.wowm` line 36.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_MOVE_START_STRAFE_LEFT_Server0() {
        let expected = MSG_MOVE_START_STRAFE_LEFT_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_FORWARD()
                    .set_STRAFE_LEFT()
                    ,
                timestamp: 0x179D29F,
                position: Vector3d {
                    x: -8944.482_f32,
                    y: -136.26949_f32,
                    z: 83.74637_f32,
                },
                orientation: 0.38010955_f32,
                fall_time: 0.000000000000000000000000000000000000000001254_f32,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_START_STRAFE_LEFT(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_STRAFE_LEFT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_strafe_left.wowm` line 36.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_MOVE_START_STRAFE_LEFT_Server0() {
        let expected = MSG_MOVE_START_STRAFE_LEFT_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_FORWARD()
                    .set_STRAFE_LEFT()
                    ,
                timestamp: 0x179D29F,
                position: Vector3d {
                    x: -8944.482_f32,
                    y: -136.26949_f32,
                    z: 83.74637_f32,
                },
                orientation: 0.38010955_f32,
                fall_time: 0.000000000000000000000000000000000000000001254_f32,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_START_STRAFE_LEFT(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_STRAFE_LEFT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_strafe_left.wowm` line 36.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_MOVE_START_STRAFE_LEFT_Server0() {
        let expected = MSG_MOVE_START_STRAFE_LEFT_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_FORWARD()
                    .set_STRAFE_LEFT()
                    ,
                timestamp: 0x179D29F,
                position: Vector3d {
                    x: -8944.482_f32,
                    y: -136.26949_f32,
                    z: 83.74637_f32,
                },
                orientation: 0.38010955_f32,
                fall_time: 0.000000000000000000000000000000000000000001254_f32,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_START_STRAFE_LEFT(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_STRAFE_LEFT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
