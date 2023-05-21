use std::io::{Read, Write};

use crate::vanilla::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_start_turn_left.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_start_turn_left.wowm#L1):
/// ```text
/// cmsg MSG_MOVE_START_TURN_LEFT_Client = 0x00BC {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_START_TURN_LEFT_Client {
    pub info: MovementInfo,
}

impl crate::private::Sealed for MSG_MOVE_START_TURN_LEFT_Client {}
impl MSG_MOVE_START_TURN_LEFT_Client {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(28..=81).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x00BC, size: body_size });
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            info,
        })
    }

}

impl crate::Message for MSG_MOVE_START_TURN_LEFT_Client {
    const OPCODE: u32 = 0x00bc;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MOVE_START_TURN_LEFT_Client {}

impl MSG_MOVE_START_TURN_LEFT_Client {
    pub(crate) const fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_MOVE_START_TURN_LEFT_Client;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &MSG_MOVE_START_TURN_LEFT_Client, expected: &MSG_MOVE_START_TURN_LEFT_Client) {
        assert_eq!(t.info, expected.info);
    }

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xBC, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
         0x00, 0xFB, 0xBE, 0x79, 0x01, 0xCD, 0xD7, 0x0B, 0xC6, 0x35, 0x7E, 0x04,
         0xC3, 0xF9, 0x0F, 0xA7, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> MSG_MOVE_START_TURN_LEFT_Client {
        MSG_MOVE_START_TURN_LEFT_Client {
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_turn_left()
                    ,
                timestamp: 0x179BEFB,
                position: Vector3d {
                    x: -8949.95_f32,
                    y: -132.493_f32,
                    z: 83.5312_f32,
                },
                orientation: 0_f32,
                fall_time: 0_f32,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_turn_left.wowm` line 20.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_move_start_turn_left_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_START_TURN_LEFT(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_TURN_LEFT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_turn_left.wowm` line 20.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_move_start_turn_left_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_START_TURN_LEFT(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_TURN_LEFT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_start_turn_left.wowm` line 20.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_move_start_turn_left_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_START_TURN_LEFT(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_START_TURN_LEFT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

