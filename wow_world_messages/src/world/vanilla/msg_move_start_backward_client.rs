use std::convert::{TryFrom, TryInto};
use crate::vanilla::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm#L1):
/// ```text
/// cmsg MSG_MOVE_START_BACKWARD_Client = 0x00B6 {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_START_BACKWARD_Client {
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_START_BACKWARD_Client {
    const OPCODE: u32 = 0x00b6;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(28..=81).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00B6, size: body_size as u32 });
        }

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MOVE_START_BACKWARD_Client {}

impl MSG_MOVE_START_BACKWARD_Client {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    use super::MSG_MOVE_START_BACKWARD_Client;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xB6, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
         0x00, 0x10, 0x57, 0x5B, 0x02, 0x75, 0xA5, 0x0B, 0xC6, 0x6F, 0xF4, 0xF4,
         0xC2, 0xBD, 0x0D, 0xA5, 0x42, 0x6B, 0x6C, 0x92, 0x40, 0x00, 0x00, 0x00,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm` line 20.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_MOVE_START_BACKWARD_Client0() {
        let expected = MSG_MOVE_START_BACKWARD_Client {
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_BACKWARD()
                    ,
                timestamp: 0x25B5710,
                position: Vector3d {
                    x: -8937.364_f32,
                    y: -122.47741_f32,
                    z: 82.52683_f32,
                },
                orientation: 4.5757346_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_START_BACKWARD(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_BACKWARD, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm` line 20.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_MOVE_START_BACKWARD_Client0() {
        let expected = MSG_MOVE_START_BACKWARD_Client {
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_BACKWARD()
                    ,
                timestamp: 0x25B5710,
                position: Vector3d {
                    x: -8937.364_f32,
                    y: -122.47741_f32,
                    z: 82.52683_f32,
                },
                orientation: 4.5757346_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_START_BACKWARD(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_BACKWARD, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_backward.wowm` line 20.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_MOVE_START_BACKWARD_Client0() {
        let expected = MSG_MOVE_START_BACKWARD_Client {
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_BACKWARD()
                    ,
                timestamp: 0x25B5710,
                position: Vector3d {
                    x: -8937.364_f32,
                    y: -122.47741_f32,
                    z: 82.52683_f32,
                },
                orientation: 4.5757346_f32,
                fall_time: 0_f32,
            },
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_START_BACKWARD(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_BACKWARD, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.info, expected.info);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

