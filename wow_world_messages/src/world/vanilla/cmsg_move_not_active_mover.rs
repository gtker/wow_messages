use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    MovementFlags, MovementInfo, TransportInfo, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_not_active_mover.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_not_active_mover.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_NOT_ACTIVE_MOVER = 0x02D1 {
///     Guid old_mover;
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub old_mover: Guid,
    pub info: MovementInfo,
}

impl crate::private::Sealed for CMSG_MOVE_NOT_ACTIVE_MOVER {}
impl CMSG_MOVE_NOT_ACTIVE_MOVER {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(36..=89).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // old_mover: Guid
        let old_mover = crate::util::read_guid(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            old_mover,
            info,
        })
    }

}

impl crate::Message for CMSG_MOVE_NOT_ACTIVE_MOVER {
    const OPCODE: u32 = 0x02d1;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_MOVE_NOT_ACTIVE_MOVER"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_MOVE_NOT_ACTIVE_MOVER {{").unwrap();
        // Members
        writeln!(s, "    old_mover = {};", self.old_mover.guid()).unwrap();
        // info: MovementInfo
        writeln!(s, "    info = {{").unwrap();
        // Members
        writeln!(s, "        flags = {};", MovementFlags::new(self.info.flags.as_int()).as_test_case_value()).unwrap();
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
            writeln!(s, "    {}", if if_statement.pitch.to_string().contains('.') { if_statement.pitch.to_string() } else { format!("{}.0", if_statement.pitch) }).unwrap();
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
        let [a, b, c, d] = 721_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "old_mover", "    ");
        writeln!(s, "    /* info: MovementInfo start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "        ");
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
            crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch", "        ");
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
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // old_mover: Guid
        w.write_all(&self.old_mover.guid().to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(721, "CMSG_MOVE_NOT_ACTIVE_MOVER", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MOVE_NOT_ACTIVE_MOVER {}

impl CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub(crate) const fn size(&self) -> usize {
        8 // old_mover: Guid
        + self.info.size() // info: MovementInfo
    }
}

