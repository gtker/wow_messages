use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_set_walk_mode.wowm:46`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_set_walk_mode.wowm#L46):
/// ```text
/// smsg MSG_MOVE_SET_WALK_MODE_Server = 0x00C3 {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_SET_WALK_MODE_Server {
    pub guid: Guid,
    pub info: MovementInfo,
}

#[cfg(feature = "print-testcase")]
impl MSG_MOVE_SET_WALK_MODE_Server {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_MOVE_SET_WALK_MODE_Server {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        // info: MovementInfo
        writeln!(s, "    info = {{").unwrap();
        // Members
        writeln!(s, "    flags = {};", crate::vanilla::MovementFlags::new(self.info.flags.as_int()).as_test_case_value()).unwrap();
        writeln!(s, "    timestamp = {};", self.info.timestamp).unwrap();
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.info.position.x.to_string().contains(".") { self.info.position.x.to_string() } else { format!("{}.0", self.info.position.x) }).unwrap();
        writeln!(s, "    {}", if self.info.position.y.to_string().contains(".") { self.info.position.y.to_string() } else { format!("{}.0", self.info.position.y) }).unwrap();
        writeln!(s, "    {}", if self.info.position.z.to_string().contains(".") { self.info.position.z.to_string() } else { format!("{}.0", self.info.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    {}", if self.info.orientation.to_string().contains(".") { self.info.orientation.to_string() } else { format!("{}.0", self.info.orientation) }).unwrap();
        if let Some(if_statement) = &self.info.flags.get_on_transport() {
            // transport: TransportInfo
            writeln!(s, "    transport = {{").unwrap();
            // Members
            writeln!(s, "    guid = {};", if_statement.transport.guid.guid()).unwrap();
            // position: Vector3d
            writeln!(s, "    position = {{").unwrap();
            // Members
            writeln!(s, "    {}", if if_statement.transport.position.x.to_string().contains(".") { if_statement.transport.position.x.to_string() } else { format!("{}.0", if_statement.transport.position.x) }).unwrap();
            writeln!(s, "    {}", if if_statement.transport.position.y.to_string().contains(".") { if_statement.transport.position.y.to_string() } else { format!("{}.0", if_statement.transport.position.y) }).unwrap();
            writeln!(s, "    {}", if if_statement.transport.position.z.to_string().contains(".") { if_statement.transport.position.z.to_string() } else { format!("{}.0", if_statement.transport.position.z) }).unwrap();

            writeln!(s, "    }};").unwrap();
            writeln!(s, "    {}", if if_statement.transport.orientation.to_string().contains(".") { if_statement.transport.orientation.to_string() } else { format!("{}.0", if_statement.transport.orientation) }).unwrap();
            writeln!(s, "    timestamp = {};", if_statement.transport.timestamp).unwrap();

            writeln!(s, "    }};").unwrap();
        }

        if let Some(if_statement) = &self.info.flags.get_swimming() {
            writeln!(s, "    {}", if if_statement.pitch.to_string().contains(".") { if_statement.pitch.to_string() } else { format!("{}.0", if_statement.pitch) }).unwrap();
        }

        writeln!(s, "    {}", if self.info.fall_time.to_string().contains(".") { self.info.fall_time.to_string() } else { format!("{}.0", self.info.fall_time) }).unwrap();
        if let Some(if_statement) = &self.info.flags.get_jumping() {
            writeln!(s, "    {}", if if_statement.z_speed.to_string().contains(".") { if_statement.z_speed.to_string() } else { format!("{}.0", if_statement.z_speed) }).unwrap();
            writeln!(s, "    {}", if if_statement.cos_angle.to_string().contains(".") { if_statement.cos_angle.to_string() } else { format!("{}.0", if_statement.cos_angle) }).unwrap();
            writeln!(s, "    {}", if if_statement.sin_angle.to_string().contains(".") { if_statement.sin_angle.to_string() } else { format!("{}.0", if_statement.sin_angle) }).unwrap();
            writeln!(s, "    {}", if if_statement.xy_speed.to_string().contains(".") { if_statement.xy_speed.to_string() } else { format!("{}.0", if_statement.xy_speed) }).unwrap();
        }

        if let Some(if_statement) = &self.info.flags.get_spline_elevation() {
            writeln!(s, "    {}", if if_statement.spline_elevation.to_string().contains(".") { if_statement.spline_elevation.to_string() } else { format!("{}.0", if_statement.spline_elevation) }).unwrap();
        }


        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 195_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for MSG_MOVE_SET_WALK_MODE_Server {}
impl crate::Message for MSG_MOVE_SET_WALK_MODE_Server {
    const OPCODE: u32 = 0x00c3;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(30..=90).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00C3, size: body_size });
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
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MOVE_SET_WALK_MODE_Server {}

impl MSG_MOVE_SET_WALK_MODE_Server {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + self.info.size() // info: MovementInfo
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_MOVE_SET_WALK_MODE_Server;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &MSG_MOVE_SET_WALK_MODE_Server, expected: &MSG_MOVE_SET_WALK_MODE_Server) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.info, expected.info);
    }

    const RAW0: [u8; 34] = [ 0x00, 0x20, 0xC3, 0x00, 0x01, 0x05, 0x01, 0x01, 0x00,
         0x00, 0x9A, 0x17, 0x5B, 0x02, 0x02, 0xBD, 0x0B, 0xC6, 0x4E, 0x58, 0x01,
         0xC3, 0x26, 0x29, 0xA7, 0x42, 0x2E, 0x0E, 0xC3, 0x40, 0x00, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> MSG_MOVE_SET_WALK_MODE_Server {
        MSG_MOVE_SET_WALK_MODE_Server {
            guid: Guid::new(0x5),
            info: MovementInfo {
                flags: MovementInfo_MovementFlags::empty()
                    .set_walk_mode()
                    .set_forward()
                    ,
                timestamp: 0x25B179A,
                position: Vector3d {
                    x: -8943.252_f32,
                    y: -129.34494_f32,
                    z: 83.58037_f32,
                },
                orientation: 6.095481_f32,
                fall_time: 0_f32,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_walk_mode.wowm` line 60.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_move_set_walk_mode_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_SET_WALK_MODE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_WALK_MODE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_walk_mode.wowm` line 60.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_move_set_walk_mode_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_SET_WALK_MODE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_WALK_MODE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/msg/msg_move_set_walk_mode.wowm` line 60.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_move_set_walk_mode_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_MOVE_SET_WALK_MODE(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_MOVE_SET_WALK_MODE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

