use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    MovementFlags, MovementInfo, TransportInfo, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_set_pitch.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_set_pitch.wowm#L7):
/// ```text
/// cmsg MSG_MOVE_SET_PITCH_Client = 0x00DB {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_SET_PITCH_Client {
    pub info: MovementInfo,
}

impl crate::private::Sealed for MSG_MOVE_SET_PITCH_Client {}
impl crate::Message for MSG_MOVE_SET_PITCH_Client {
    const OPCODE: u32 = 0x00db;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_MOVE_SET_PITCH_Client {{").unwrap();
        // Members
        // info: MovementInfo
        writeln!(s, "    info = {{").unwrap();
        // Members
        writeln!(s, "        flags = {};", MovementFlags::new(self.info.flags.as_int()).as_test_case_value()).unwrap();
        writeln!(s, "        extra_flags = {};", self.info.extra_flags).unwrap();
        writeln!(s, "        timestamp = {};", self.info.timestamp).unwrap();
        // position: Vector3d
        writeln!(s, "        position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.info.position.x.to_string().contains('.') { self.info.position.x.to_string() } else { format!("{}.0", self.info.position.x) }).unwrap();
        writeln!(s, "    {}", if self.info.position.y.to_string().contains('.') { self.info.position.y.to_string() } else { format!("{}.0", self.info.position.y) }).unwrap();
        writeln!(s, "    {}", if self.info.position.z.to_string().contains('.') { self.info.position.z.to_string() } else { format!("{}.0", self.info.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    {}", if self.info.orientation.to_string().contains('.') { self.info.orientation.to_string() } else { format!("{}.0", self.info.orientation) }).unwrap();
        if let Some(if_statement) = &self.info.flags.get_on_transport() {
            // transport: TransportInfo
            writeln!(s, "        transport = {{").unwrap();
            // Members
            writeln!(s, "            guid = {};", if_statement.transport.guid.guid()).unwrap();
            // position: Vector3d
            writeln!(s, "            position = {{").unwrap();
            // Members
            writeln!(s, "    {}", if if_statement.transport.position.x.to_string().contains('.') { if_statement.transport.position.x.to_string() } else { format!("{}.0", if_statement.transport.position.x) }).unwrap();
            writeln!(s, "    {}", if if_statement.transport.position.y.to_string().contains('.') { if_statement.transport.position.y.to_string() } else { format!("{}.0", if_statement.transport.position.y) }).unwrap();
            writeln!(s, "    {}", if if_statement.transport.position.z.to_string().contains('.') { if_statement.transport.position.z.to_string() } else { format!("{}.0", if_statement.transport.position.z) }).unwrap();

            writeln!(s, "    }};").unwrap();
            writeln!(s, "    {}", if if_statement.transport.orientation.to_string().contains('.') { if_statement.transport.orientation.to_string() } else { format!("{}.0", if_statement.transport.orientation) }).unwrap();
            writeln!(s, "            timestamp = {};", if_statement.transport.timestamp).unwrap();

            writeln!(s, "    }};").unwrap();
        }

        if let Some(if_statement) = &self.info.flags.get_swimming() {
            match if_statement {
                crate::tbc::MovementInfo_MovementFlags_Swimming::Swimming {
                    pitch1,
                } => {
                    writeln!(s, "    {}", if pitch1.to_string().contains('.') { pitch1.to_string() } else { format!("{}.0", pitch1) }).unwrap();
                }
                crate::tbc::MovementInfo_MovementFlags_Swimming::Ontransport {
                    pitch2,
                } => {
                    writeln!(s, "    {}", if pitch2.to_string().contains('.') { pitch2.to_string() } else { format!("{}.0", pitch2) }).unwrap();
                }
            }
        }

        writeln!(s, "    {}", if self.info.fall_time.to_string().contains('.') { self.info.fall_time.to_string() } else { format!("{}.0", self.info.fall_time) }).unwrap();
        if let Some(if_statement) = &self.info.flags.get_jumping() {
            writeln!(s, "    {}", if if_statement.z_speed.to_string().contains('.') { if_statement.z_speed.to_string() } else { format!("{}.0", if_statement.z_speed) }).unwrap();
            writeln!(s, "    {}", if if_statement.cos_angle.to_string().contains('.') { if_statement.cos_angle.to_string() } else { format!("{}.0", if_statement.cos_angle) }).unwrap();
            writeln!(s, "    {}", if if_statement.sin_angle.to_string().contains('.') { if_statement.sin_angle.to_string() } else { format!("{}.0", if_statement.sin_angle) }).unwrap();
            writeln!(s, "    {}", if if_statement.xy_speed.to_string().contains('.') { if_statement.xy_speed.to_string() } else { format!("{}.0", if_statement.xy_speed) }).unwrap();
        }

        if let Some(if_statement) = &self.info.flags.get_spline_elevation() {
            writeln!(s, "    {}", if if_statement.spline_elevation.to_string().contains('.') { if_statement.spline_elevation.to_string() } else { format!("{}.0", if_statement.spline_elevation) }).unwrap();
        }


        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 219_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    /* info: MovementInfo start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "extra_flags", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "        ");
        writeln!(s, "    /* position: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "            ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "            ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "            ");
        writeln!(s, "    /* position: Vector3d end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "        ");
        if let Some(if_statement) = &self.info.flags.get_on_transport() {
            writeln!(s, "    /* transport: TransportInfo start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.transport.guid), "guid", "            ");
            writeln!(s, "    /* position: Vector3d start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
            writeln!(s, "    /* position: Vector3d end */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "            ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "            ");
            writeln!(s, "    /* transport: TransportInfo end */").unwrap();
        }

        if let Some(if_statement) = &self.info.flags.get_swimming() {
            match if_statement {
                crate::tbc::MovementInfo_MovementFlags_Swimming::Swimming {
                    pitch1,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch1", "        ");
                }
                crate::tbc::MovementInfo_MovementFlags_Swimming::Ontransport {
                    pitch2,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch2", "        ");
                }
            }
        }

        crate::util::write_bytes(&mut s, &mut bytes, 4, "fall_time", "        ");
        if let Some(if_statement) = &self.info.flags.get_jumping() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "z_speed", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "cos_angle", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "sin_angle", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "xy_speed", "        ");
        }

        if let Some(if_statement) = &self.info.flags.get_spline_elevation() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_elevation", "        ");
        }

        writeln!(s, "    /* info: MovementInfo end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(29..=82).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00DB, size: body_size });
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            info,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_MOVE_SET_PITCH_Client {}

impl MSG_MOVE_SET_PITCH_Client {
    pub(crate) const fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

