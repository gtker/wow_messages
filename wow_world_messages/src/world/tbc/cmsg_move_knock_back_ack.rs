use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_knock_back_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_knock_back_ack.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_KNOCK_BACK_ACK = 0x00F0 {
///     Guid guid;
///     u32 counter;
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_MOVE_KNOCK_BACK_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub info: MovementInfo,
}

#[cfg(feature = "print-testcase")]
impl CMSG_MOVE_KNOCK_BACK_ACK {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_MOVE_KNOCK_BACK_ACK {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    counter = {};", self.counter).unwrap();
        // info: MovementInfo
        writeln!(s, "    info = {{").unwrap();
        // Members
        writeln!(s, "    flags = {};", crate::tbc::MovementFlags::new(self.info.flags.as_int()).as_test_case_value()).unwrap();
        writeln!(s, "    extra_flags = {};", self.info.extra_flags).unwrap();
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
            match if_statement {
                crate::tbc::MovementInfo_MovementFlags_Swimming::Swimming {
                    pitch1,
                } => {
                    writeln!(s, "    {}", if pitch1.to_string().contains(".") { pitch1.to_string() } else { format!("{}.0", pitch1) }).unwrap();
                }
                crate::tbc::MovementInfo_MovementFlags_Swimming::Ontransport {
                    pitch2,
                } => {
                    writeln!(s, "    {}", if pitch2.to_string().contains(".") { pitch2.to_string() } else { format!("{}.0", pitch2) }).unwrap();
                }
            }
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
        let [a, b] = (u16::try_from(self.size() + 6).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 240_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_MOVE_KNOCK_BACK_ACK {}
impl crate::Message for CMSG_MOVE_KNOCK_BACK_ACK {
    const OPCODE: u32 = 0x00f0;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(41..=94).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00F0, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            guid,
            counter,
            info,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_MOVE_KNOCK_BACK_ACK {}

impl CMSG_MOVE_KNOCK_BACK_ACK {
    pub(crate) const fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + self.info.size() // info: MovementInfo
    }
}

