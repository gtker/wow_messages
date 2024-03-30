use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::MovementInfo;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_stop_strafe.wowm:46`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_stop_strafe.wowm#L46):
/// ```text
/// smsg MSG_MOVE_STOP_STRAFE_Server = 0x00BA {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MSG_MOVE_STOP_STRAFE_Server {
    pub guid: Guid,
    pub info: MovementInfo,
}

impl crate::private::Sealed for MSG_MOVE_STOP_STRAFE_Server {}
impl MSG_MOVE_STOP_STRAFE_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(29..=90).contains(&body_size) {
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

impl crate::Message for MSG_MOVE_STOP_STRAFE_Server {
    const OPCODE: u32 = 0x00ba;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_MOVE_STOP_STRAFE_Server"
    }

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
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(186, "MSG_MOVE_STOP_STRAFE_Server", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MOVE_STOP_STRAFE_Server {}

impl MSG_MOVE_STOP_STRAFE_Server {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_MOVE_STOP_STRAFE_Server;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &MSG_MOVE_STOP_STRAFE_Server, expected: &MSG_MOVE_STOP_STRAFE_Server) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);
    }

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xBA, 0x00, 0x01, 0x05, 0x01, 0x00, 0x00,
         0x00, 0x46, 0xD3, 0x79, 0x01, 0x16, 0xC0, 0x0B, 0xC6, 0xF8, 0x31, 0x07,
         0xC3, 0x73, 0x7F, 0xA7, 0x42, 0xB8, 0x9D, 0xC2, 0x3E, 0x7F, 0x03, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> MSG_MOVE_STOP_STRAFE_Server {
        MSG_MOVE_STOP_STRAFE_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_forward()
                    ,
                timestamp: 0x179D346,
                position: Vector3d {
                    x: -8944.021_f32,
                    y: -135.19519_f32,
                    z: 83.748924_f32,
                },
                orientation: 0.38010955_f32,
                fall_time: 0.000000000000000000000000000000000000000001254_f32,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop_strafe.wowm` line 60.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_move_stop_strafe_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_STOP_STRAFE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP_STRAFE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop_strafe.wowm` line 60.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_move_stop_strafe_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_STOP_STRAFE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP_STRAFE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_stop_strafe.wowm` line 60.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_move_stop_strafe_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_STOP_STRAFE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_STOP_STRAFE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

