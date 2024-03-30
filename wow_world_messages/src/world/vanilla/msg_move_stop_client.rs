use std::io::{Read, Write};

use crate::vanilla::MovementInfo;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_stop.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_stop.wowm#L1):
/// ```text
/// cmsg MSG_MOVE_STOP_Client = 0x00B7 {
///     MovementInfo info;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MSG_MOVE_STOP_Client {
    pub info: MovementInfo,
}

impl crate::private::Sealed for MSG_MOVE_STOP_Client {}
impl MSG_MOVE_STOP_Client {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(28..=81).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            info,
        })
    }

}

impl crate::Message for MSG_MOVE_STOP_Client {
    const OPCODE: u32 = 0x00b7;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_MOVE_STOP_Client"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(183, "MSG_MOVE_STOP_Client", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MOVE_STOP_Client {}

impl MSG_MOVE_STOP_Client {
    pub(crate) const fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_MOVE_STOP_Client;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xB7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0xF2, 0x31, 0x7A, 0x01, 0x24, 0xCB, 0x0B, 0xC6, 0x30, 0x20, 0xDF,
         0xC2, 0x3D, 0x17, 0xA6, 0x42, 0x03, 0x51, 0x24, 0x40, 0x85, 0x03, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> MSG_MOVE_STOP_Client {
        MSG_MOVE_STOP_Client {
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

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop.wowm` line 20.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_move_stop_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_STOP(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop.wowm` line 20.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_move_stop_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_STOP(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop.wowm` line 20.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_move_stop_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_MOVE_STOP(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

