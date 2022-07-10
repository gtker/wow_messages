use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::MovementInfo;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_set_facing.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_set_facing.wowm#L3):
/// ```text
/// cmsg MSG_MOVE_SET_FACING_Client = 0x00DA {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_SET_FACING_Client {
    pub info: MovementInfo,
}

impl ClientMessage for MSG_MOVE_SET_FACING_Client {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    const OPCODE: u16 = 0x00da;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}

impl MSG_MOVE_SET_FACING_Client {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    use super::MSG_MOVE_SET_FACING_Client;
    use crate::world::version_1_12::MovementFlags;
    use crate::world::version_1_12::MovementInfo;
    use crate::world::version_1_12::TransportInfo;
    use crate::world::version_1_12::Vector3d;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ClientOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xDA, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, 0x5E, 0x2D, 0x7A, 0x01, 0x97, 0xAF, 0x0B, 0xC6, 0x42, 0x0A, 0xE8,
         0xC2, 0xE3, 0x25, 0xA5, 0x42, 0xA7, 0x4F, 0x23, 0x40, 0x85, 0x03, 0x00,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_facing.wowm` line 7.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_MOVE_SET_FACING_Client0() {
        let expected = MSG_MOVE_SET_FACING_Client {
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_FORWARD()
                    ,
                timestamp: 0x17A2D5E,
                position: Vector3d {
                    x: -8939.897_f32,
                    y: -116.020035_f32,
                    z: 82.574_f32,
                },
                orientation: 2.5517366_f32,
                fall_time: 0.000000000000000000000000000000000000000001263_f32,
            },
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_SET_FACING(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_FACING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_facing.wowm` line 7.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_MOVE_SET_FACING_Client0() {
        let expected = MSG_MOVE_SET_FACING_Client {
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_FORWARD()
                    ,
                timestamp: 0x17A2D5E,
                position: Vector3d {
                    x: -8939.897_f32,
                    y: -116.020035_f32,
                    z: 82.574_f32,
                },
                orientation: 2.5517366_f32,
                fall_time: 0.000000000000000000000000000000000000000001263_f32,
            },
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_SET_FACING(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_FACING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_facing.wowm` line 7.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_MOVE_SET_FACING_Client0() {
        let expected = MSG_MOVE_SET_FACING_Client {
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_FORWARD()
                    ,
                timestamp: 0x17A2D5E,
                position: Vector3d {
                    x: -8939.897_f32,
                    y: -116.020035_f32,
                    z: 82.574_f32,
                },
                orientation: 2.5517366_f32,
                fall_time: 0.000000000000000000000000000000000000000001263_f32,
            },
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_SET_FACING(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_FACING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
