use std::io::{Read, Write};

use crate::vanilla::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_heartbeat.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_heartbeat.wowm#L1):
/// ```text
/// cmsg MSG_MOVE_HEARTBEAT_Client = 0x00EE {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_HEARTBEAT_Client {
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_HEARTBEAT_Client {
    const OPCODE: u32 = 0x00ee;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00EE, size: body_size as u32 });
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MOVE_HEARTBEAT_Client {}

impl MSG_MOVE_HEARTBEAT_Client {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    use super::MSG_MOVE_HEARTBEAT_Client;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &MSG_MOVE_HEARTBEAT_Client, expected: &MSG_MOVE_HEARTBEAT_Client) {
        assert_eq!(t.info, expected.info);
    }

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, 0x46, 0x31, 0x7A, 0x01, 0x19, 0xC7, 0x0B, 0xC6, 0xFE, 0x6E, 0xE0,
         0xC2, 0x1A, 0xF5, 0xA5, 0x42, 0x03, 0x51, 0x24, 0x40, 0x85, 0x03, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> MSG_MOVE_HEARTBEAT_Client {
        MSG_MOVE_HEARTBEAT_Client {
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_FORWARD()
                    ,
                timestamp: 0x17A3146,
                position: Vector3d {
                    x: -8945.774_f32,
                    y: -112.21678_f32,
                    z: 82.978714_f32,
                },
                orientation: 2.5674446_f32,
                fall_time: 0.000000000000000000000000000000000000000001263_f32,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_heartbeat.wowm` line 7.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_MOVE_HEARTBEAT_Client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_HEARTBEAT(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_HEARTBEAT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_heartbeat.wowm` line 7.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_MOVE_HEARTBEAT_Client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_HEARTBEAT(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_HEARTBEAT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_heartbeat.wowm` line 7.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_MOVE_HEARTBEAT_Client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_HEARTBEAT(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_HEARTBEAT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

