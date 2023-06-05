use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vehicle/cmsg_change_seats_on_controlled_vehicle.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vehicle/cmsg_change_seats_on_controlled_vehicle.wowm#L1):
/// ```text
/// cmsg CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE = 0x049B {
///     PackedGuid vehicle;
///     MovementInfo info;
///     PackedGuid accessory;
///     u8 seat;
/// }
/// ```
pub struct CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {
    pub vehicle: Guid,
    pub info: MovementInfo,
    pub accessory: Guid,
    pub seat: u8,
}

#[cfg(feature = "print-testcase")]
impl CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {{").unwrap();
        // Members
        writeln!(s, "    vehicle = {};", self.vehicle.guid()).unwrap();
        // info: MovementInfo
        writeln!(s, "    info = {{").unwrap();
        // Members
        writeln!(s, "    flags = {};", crate::wrath::MovementFlags::new(self.info.flags.as_int()).as_test_case_value()).unwrap();
        writeln!(s, "    timestamp = {};", self.info.timestamp).unwrap();
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.info.position.x.to_string().contains(".") { self.info.position.x.to_string() } else { format!("{}.0", self.info.position.x) }).unwrap();
        writeln!(s, "    {}", if self.info.position.y.to_string().contains(".") { self.info.position.y.to_string() } else { format!("{}.0", self.info.position.y) }).unwrap();
        writeln!(s, "    {}", if self.info.position.z.to_string().contains(".") { self.info.position.z.to_string() } else { format!("{}.0", self.info.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    {}", if self.info.orientation.to_string().contains(".") { self.info.orientation.to_string() } else { format!("{}.0", self.info.orientation) }).unwrap();
        if let Some(if_statement) = &self.info.flags.get_on_transport_and_interpolated_movement() {
            match if_statement {
                crate::wrath::MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                    transport_info,
                    transport_time,
                } => {
                    // transport_info: TransportInfo
                    writeln!(s, "    transport_info = {{").unwrap();
                    // Members
                    writeln!(s, "    guid = {};", transport_info.guid.guid()).unwrap();
                    // position: Vector3d
                    writeln!(s, "    position = {{").unwrap();
                    // Members
                    writeln!(s, "    {}", if transport_info.position.x.to_string().contains(".") { transport_info.position.x.to_string() } else { format!("{}.0", transport_info.position.x) }).unwrap();
                    writeln!(s, "    {}", if transport_info.position.y.to_string().contains(".") { transport_info.position.y.to_string() } else { format!("{}.0", transport_info.position.y) }).unwrap();
                    writeln!(s, "    {}", if transport_info.position.z.to_string().contains(".") { transport_info.position.z.to_string() } else { format!("{}.0", transport_info.position.z) }).unwrap();

                    writeln!(s, "    }};").unwrap();
                    writeln!(s, "    {}", if transport_info.orientation.to_string().contains(".") { transport_info.orientation.to_string() } else { format!("{}.0", transport_info.orientation) }).unwrap();
                    writeln!(s, "    timestamp = {};", transport_info.timestamp).unwrap();
                    writeln!(s, "    seat = {};", transport_info.seat).unwrap();

                    writeln!(s, "    }};").unwrap();
                    writeln!(s, "    transport_time = {};", transport_time).unwrap();
                }
                crate::wrath::MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                    transport,
                } => {
                    // transport: TransportInfo
                    writeln!(s, "    transport = {{").unwrap();
                    // Members
                    writeln!(s, "    guid = {};", transport.guid.guid()).unwrap();
                    // position: Vector3d
                    writeln!(s, "    position = {{").unwrap();
                    // Members
                    writeln!(s, "    {}", if transport.position.x.to_string().contains(".") { transport.position.x.to_string() } else { format!("{}.0", transport.position.x) }).unwrap();
                    writeln!(s, "    {}", if transport.position.y.to_string().contains(".") { transport.position.y.to_string() } else { format!("{}.0", transport.position.y) }).unwrap();
                    writeln!(s, "    {}", if transport.position.z.to_string().contains(".") { transport.position.z.to_string() } else { format!("{}.0", transport.position.z) }).unwrap();

                    writeln!(s, "    }};").unwrap();
                    writeln!(s, "    {}", if transport.orientation.to_string().contains(".") { transport.orientation.to_string() } else { format!("{}.0", transport.orientation) }).unwrap();
                    writeln!(s, "    timestamp = {};", transport.timestamp).unwrap();
                    writeln!(s, "    seat = {};", transport.seat).unwrap();

                    writeln!(s, "    }};").unwrap();
                }
            }
        }

        if let Some(if_statement) = &self.info.flags.get_swimming() {
            match if_statement {
                crate::wrath::MovementInfo_MovementFlags_Swimming::Swimming {
                    pitch1,
                } => {
                    writeln!(s, "    {}", if pitch1.to_string().contains(".") { pitch1.to_string() } else { format!("{}.0", pitch1) }).unwrap();
                }
                crate::wrath::MovementInfo_MovementFlags_Swimming::Flying {
                    pitch2,
                } => {
                    writeln!(s, "    {}", if pitch2.to_string().contains(".") { pitch2.to_string() } else { format!("{}.0", pitch2) }).unwrap();
                }
                crate::wrath::MovementInfo_MovementFlags_Swimming::AlwaysAllowPitching {
                    pitch3,
                } => {
                    writeln!(s, "    {}", if pitch3.to_string().contains(".") { pitch3.to_string() } else { format!("{}.0", pitch3) }).unwrap();
                }
            }
        }

        writeln!(s, "    {}", if self.info.fall_time.to_string().contains(".") { self.info.fall_time.to_string() } else { format!("{}.0", self.info.fall_time) }).unwrap();
        if let Some(if_statement) = &self.info.flags.get_falling() {
            writeln!(s, "    {}", if if_statement.z_speed.to_string().contains(".") { if_statement.z_speed.to_string() } else { format!("{}.0", if_statement.z_speed) }).unwrap();
            writeln!(s, "    {}", if if_statement.cos_angle.to_string().contains(".") { if_statement.cos_angle.to_string() } else { format!("{}.0", if_statement.cos_angle) }).unwrap();
            writeln!(s, "    {}", if if_statement.sin_angle.to_string().contains(".") { if_statement.sin_angle.to_string() } else { format!("{}.0", if_statement.sin_angle) }).unwrap();
            writeln!(s, "    {}", if if_statement.xy_speed.to_string().contains(".") { if_statement.xy_speed.to_string() } else { format!("{}.0", if_statement.xy_speed) }).unwrap();
        }

        if let Some(if_statement) = &self.info.flags.get_spline_elevation() {
            writeln!(s, "    {}", if if_statement.spline_elevation.to_string().contains(".") { if_statement.spline_elevation.to_string() } else { format!("{}.0", if_statement.spline_elevation) }).unwrap();
        }


        writeln!(s, "    }};").unwrap();
        writeln!(s, "    accessory = {};", self.accessory.guid()).unwrap();
        writeln!(s, "    seat = {};", self.seat).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 6).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1179_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
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
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {}
impl crate::Message for CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {
    const OPCODE: u32 = 0x049b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vehicle: PackedGuid
        crate::util::write_packed_guid(&self.vehicle, &mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // accessory: PackedGuid
        crate::util::write_packed_guid(&self.accessory, &mut w)?;

        // seat: u8
        w.write_all(&self.seat.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(35..=107).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x049B, size: body_size });
        }

        // vehicle: PackedGuid
        let vehicle = crate::util::read_packed_guid(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // accessory: PackedGuid
        let accessory = crate::util::read_packed_guid(&mut r)?;

        // seat: u8
        let seat = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            vehicle,
            info,
            accessory,
            seat,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {}

impl CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.vehicle) // vehicle: PackedGuid
        + self.info.size() // info: MovementInfo
        + crate::util::packed_guid_size(&self.accessory) // accessory: PackedGuid
        + 1 // seat: u8
    }
}

