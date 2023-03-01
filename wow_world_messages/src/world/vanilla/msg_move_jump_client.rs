use crate:: {
};
use crate::vanilla::MovementInfo;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_jump.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_jump.wowm#L1):
/// ```text
/// cmsg MSG_MOVE_JUMP_Client = 0x00BB {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_JUMP_Client {
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_JUMP_Client {
    const OPCODE: u32 = 0x00bb;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(28..=81).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00BB, size: body_size as u32 });
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MOVE_JUMP_Client {}

impl MSG_MOVE_JUMP_Client {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    use super::MSG_MOVE_JUMP_Client;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 50] = [ 0x00, 0x30, 0xBB, 0x00, 0x00, 0x00, 0x01, 0x20, 0x00,
         0x00, 0x20, 0xD6, 0x79, 0x01, 0x1B, 0xAD, 0x0B, 0xC6, 0x9D, 0x4C, 0x05,
         0xC3, 0xD1, 0x4A, 0xA7, 0x42, 0xB8, 0x9D, 0xC2, 0x3E, 0x00, 0x00, 0x00,
         0x00, 0xD8, 0x93, 0xFE, 0xC0, 0x4D, 0xBA, 0x6D, 0x3F, 0x9F, 0xF6, 0xBD,
         0x3E, 0x00, 0x00, 0xE0, 0x40, ];

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_jump.wowm` line 7.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_MOVE_JUMP_Client0() {
        let expected = MSG_MOVE_JUMP_Client {
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_JUMPING(MovementInfo_MovementFlags_Jumping {
                        cos_angle: 0.928624_f32,
                        sin_angle: 0.3710222_f32,
                        xy_speed: 7_f32,
                        z_speed: -7.9555473_f32,
                    })
                    .set_FORWARD()
                    ,
                timestamp: 0x179D620,
                position: Vector3d {
                    x: -8939.276_f32,
                    y: -133.29927_f32,
                    z: 83.646126_f32,
                },
                orientation: 0.38010955_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_JUMP(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_JUMP, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_jump.wowm` line 7.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_MOVE_JUMP_Client0() {
        let expected = MSG_MOVE_JUMP_Client {
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_JUMPING(MovementInfo_MovementFlags_Jumping {
                        cos_angle: 0.928624_f32,
                        sin_angle: 0.3710222_f32,
                        xy_speed: 7_f32,
                        z_speed: -7.9555473_f32,
                    })
                    .set_FORWARD()
                    ,
                timestamp: 0x179D620,
                position: Vector3d {
                    x: -8939.276_f32,
                    y: -133.29927_f32,
                    z: 83.646126_f32,
                },
                orientation: 0.38010955_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_JUMP(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_JUMP, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_jump.wowm` line 7.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_MOVE_JUMP_Client0() {
        let expected = MSG_MOVE_JUMP_Client {
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_JUMPING(MovementInfo_MovementFlags_Jumping {
                        cos_angle: 0.928624_f32,
                        sin_angle: 0.3710222_f32,
                        xy_speed: 7_f32,
                        z_speed: -7.9555473_f32,
                    })
                    .set_FORWARD()
                    ,
                timestamp: 0x179D620,
                position: Vector3d {
                    x: -8939.276_f32,
                    y: -133.29927_f32,
                    z: 83.646126_f32,
                },
                orientation: 0.38010955_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_JUMP(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_JUMP, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

