use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    MovementBlock, MovementFlags, Object, ObjectType, SplineFlag, TransportInfo, 
    UpdateFlag, UpdateMask, UpdateType, Vector3d,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm:201`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm#L201):
/// ```text
/// smsg SMSG_UPDATE_OBJECT = 0x00A9 {
///     u32 amount_of_objects;
///     Object[amount_of_objects] objects;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_UPDATE_OBJECT {
    pub objects: Vec<Object>,
}

impl crate::private::Sealed for SMSG_UPDATE_OBJECT {}
impl SMSG_UPDATE_OBJECT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_objects: u32
        let amount_of_objects = crate::util::read_u32_le(&mut r)?;

        // objects: Object[amount_of_objects]
        let objects = {
            let mut objects = Vec::with_capacity(amount_of_objects as usize);

            let allocation_size = u64::from(amount_of_objects);
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_objects {
                objects.push(Object::read(&mut r)?);
            }
            objects
        };

        Ok(Self {
            objects,
        })
    }

}

impl crate::Message for SMSG_UPDATE_OBJECT {
    const OPCODE: u32 = 0x00a9;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_UPDATE_OBJECT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_OBJECT {{").unwrap();
        // Members
        writeln!(s, "    amount_of_objects = {};", self.objects.len()).unwrap();
        writeln!(s, "    objects = [").unwrap();
        for v in self.objects.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            update_type = {};", UpdateType::try_from(v.update_type.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.update_type {
                crate::wrath::Object_UpdateType::Values {
                    guid1,
                    mask1,
                } => {
                    writeln!(s, "            guid1 = {};", guid1.guid()).unwrap();
                    panic!("unsupported type for test case printing: 'UpdateMask' for variable 'mask1'");
                }
                crate::wrath::Object_UpdateType::Movement {
                    guid2,
                    movement1,
                } => {
                    writeln!(s, "            guid2 = {};", guid2.guid()).unwrap();
                    // movement1: MovementBlock
                    writeln!(s, "            movement1 = {{").unwrap();
                    // Members
                    writeln!(s, "                update_flag = {};", UpdateFlag::new(movement1.update_flag.as_int()).as_test_case_value()).unwrap();
                    if let Some(if_statement) = &movement1.update_flag.get_living() {
                        match if_statement {
                            crate::wrath::MovementBlock_UpdateFlag_Living::Living {
                                backwards_flight_speed,
                                backwards_running_speed,
                                backwards_swimming_speed,
                                fall_time,
                                flags,
                                flight_speed,
                                orientation,
                                pitch_rate,
                                position,
                                running_speed,
                                swimming_speed,
                                timestamp,
                                turn_rate,
                                walking_speed,
                            } => {
                                writeln!(s, "                flags = {};", MovementFlags::new(flags.as_int()).as_test_case_value()).unwrap();
                                writeln!(s, "                timestamp = {};", timestamp).unwrap();
                                // position: Vector3d
                                writeln!(s, "                position = {{").unwrap();
                                // Members
                                writeln!(s, "                    x = {};", if position.x.to_string().contains('.') { position.x.to_string() } else { format!("{}.0", position.x) }).unwrap();
                                writeln!(s, "                    y = {};", if position.y.to_string().contains('.') { position.y.to_string() } else { format!("{}.0", position.y) }).unwrap();
                                writeln!(s, "                    z = {};", if position.z.to_string().contains('.') { position.z.to_string() } else { format!("{}.0", position.z) }).unwrap();

                                writeln!(s, "                }};").unwrap();
                                writeln!(s, "                orientation = {};", if orientation.to_string().contains('.') { orientation.to_string() } else { format!("{}.0", orientation) }).unwrap();
                                if let Some(if_statement) = &flags.get_on_transport_and_interpolated_movement() {
                                    match if_statement {
                                        crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                                            transport_info,
                                            transport_time,
                                        } => {
                                            // transport_info: TransportInfo
                                            writeln!(s, "                transport_info = {{").unwrap();
                                            // Members
                                            writeln!(s, "                    guid = {};", transport_info.guid.guid()).unwrap();
                                            // position: Vector3d
                                            writeln!(s, "                    position = {{").unwrap();
                                            // Members
                                            writeln!(s, "                        x = {};", if transport_info.position.x.to_string().contains('.') { transport_info.position.x.to_string() } else { format!("{}.0", transport_info.position.x) }).unwrap();
                                            writeln!(s, "                        y = {};", if transport_info.position.y.to_string().contains('.') { transport_info.position.y.to_string() } else { format!("{}.0", transport_info.position.y) }).unwrap();
                                            writeln!(s, "                        z = {};", if transport_info.position.z.to_string().contains('.') { transport_info.position.z.to_string() } else { format!("{}.0", transport_info.position.z) }).unwrap();

                                            writeln!(s, "                    }};").unwrap();
                                            writeln!(s, "                    orientation = {};", if transport_info.orientation.to_string().contains('.') { transport_info.orientation.to_string() } else { format!("{}.0", transport_info.orientation) }).unwrap();
                                            writeln!(s, "                    timestamp = {};", transport_info.timestamp).unwrap();
                                            writeln!(s, "                    seat = {};", transport_info.seat).unwrap();

                                            writeln!(s, "                }};").unwrap();
                                            writeln!(s, "                transport_time = {};", transport_time).unwrap();
                                        }
                                        crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                                            transport,
                                        } => {
                                            // transport: TransportInfo
                                            writeln!(s, "                transport = {{").unwrap();
                                            // Members
                                            writeln!(s, "                    guid = {};", transport.guid.guid()).unwrap();
                                            // position: Vector3d
                                            writeln!(s, "                    position = {{").unwrap();
                                            // Members
                                            writeln!(s, "                        x = {};", if transport.position.x.to_string().contains('.') { transport.position.x.to_string() } else { format!("{}.0", transport.position.x) }).unwrap();
                                            writeln!(s, "                        y = {};", if transport.position.y.to_string().contains('.') { transport.position.y.to_string() } else { format!("{}.0", transport.position.y) }).unwrap();
                                            writeln!(s, "                        z = {};", if transport.position.z.to_string().contains('.') { transport.position.z.to_string() } else { format!("{}.0", transport.position.z) }).unwrap();

                                            writeln!(s, "                    }};").unwrap();
                                            writeln!(s, "                    orientation = {};", if transport.orientation.to_string().contains('.') { transport.orientation.to_string() } else { format!("{}.0", transport.orientation) }).unwrap();
                                            writeln!(s, "                    timestamp = {};", transport.timestamp).unwrap();
                                            writeln!(s, "                    seat = {};", transport.seat).unwrap();

                                            writeln!(s, "                }};").unwrap();
                                        }
                                    }
                                }

                                if let Some(if_statement) = &flags.get_swimming() {
                                    match if_statement {
                                        crate::wrath::MovementBlock_MovementFlags_Swimming::Swimming {
                                            pitch1,
                                        } => {
                                            writeln!(s, "                pitch1 = {};", if pitch1.to_string().contains('.') { pitch1.to_string() } else { format!("{}.0", pitch1) }).unwrap();
                                        }
                                        crate::wrath::MovementBlock_MovementFlags_Swimming::Flying {
                                            pitch2,
                                        } => {
                                            writeln!(s, "                pitch2 = {};", if pitch2.to_string().contains('.') { pitch2.to_string() } else { format!("{}.0", pitch2) }).unwrap();
                                        }
                                        crate::wrath::MovementBlock_MovementFlags_Swimming::AlwaysAllowPitching {
                                            pitch3,
                                        } => {
                                            writeln!(s, "                pitch3 = {};", if pitch3.to_string().contains('.') { pitch3.to_string() } else { format!("{}.0", pitch3) }).unwrap();
                                        }
                                    }
                                }

                                writeln!(s, "                fall_time = {};", if fall_time.to_string().contains('.') { fall_time.to_string() } else { format!("{}.0", fall_time) }).unwrap();
                                if let Some(if_statement) = &flags.get_falling() {
                                    writeln!(s, "                z_speed = {};", if if_statement.z_speed.to_string().contains('.') { if_statement.z_speed.to_string() } else { format!("{}.0", if_statement.z_speed) }).unwrap();
                                    writeln!(s, "                cos_angle = {};", if if_statement.cos_angle.to_string().contains('.') { if_statement.cos_angle.to_string() } else { format!("{}.0", if_statement.cos_angle) }).unwrap();
                                    writeln!(s, "                sin_angle = {};", if if_statement.sin_angle.to_string().contains('.') { if_statement.sin_angle.to_string() } else { format!("{}.0", if_statement.sin_angle) }).unwrap();
                                    writeln!(s, "                xy_speed = {};", if if_statement.xy_speed.to_string().contains('.') { if_statement.xy_speed.to_string() } else { format!("{}.0", if_statement.xy_speed) }).unwrap();
                                }

                                if let Some(if_statement) = &flags.get_spline_elevation() {
                                    writeln!(s, "                spline_elevation = {};", if if_statement.spline_elevation.to_string().contains('.') { if_statement.spline_elevation.to_string() } else { format!("{}.0", if_statement.spline_elevation) }).unwrap();
                                }

                                writeln!(s, "                walking_speed = {};", if walking_speed.to_string().contains('.') { walking_speed.to_string() } else { format!("{}.0", walking_speed) }).unwrap();
                                writeln!(s, "                running_speed = {};", if running_speed.to_string().contains('.') { running_speed.to_string() } else { format!("{}.0", running_speed) }).unwrap();
                                writeln!(s, "                backwards_running_speed = {};", if backwards_running_speed.to_string().contains('.') { backwards_running_speed.to_string() } else { format!("{}.0", backwards_running_speed) }).unwrap();
                                writeln!(s, "                swimming_speed = {};", if swimming_speed.to_string().contains('.') { swimming_speed.to_string() } else { format!("{}.0", swimming_speed) }).unwrap();
                                writeln!(s, "                backwards_swimming_speed = {};", if backwards_swimming_speed.to_string().contains('.') { backwards_swimming_speed.to_string() } else { format!("{}.0", backwards_swimming_speed) }).unwrap();
                                writeln!(s, "                flight_speed = {};", if flight_speed.to_string().contains('.') { flight_speed.to_string() } else { format!("{}.0", flight_speed) }).unwrap();
                                writeln!(s, "                backwards_flight_speed = {};", if backwards_flight_speed.to_string().contains('.') { backwards_flight_speed.to_string() } else { format!("{}.0", backwards_flight_speed) }).unwrap();
                                writeln!(s, "                turn_rate = {};", if turn_rate.to_string().contains('.') { turn_rate.to_string() } else { format!("{}.0", turn_rate) }).unwrap();
                                writeln!(s, "                pitch_rate = {};", if pitch_rate.to_string().contains('.') { pitch_rate.to_string() } else { format!("{}.0", pitch_rate) }).unwrap();
                                if let Some(if_statement) = &flags.get_spline_enabled() {
                                    writeln!(s, "                spline_flags = {};", SplineFlag::new(if_statement.spline_flags.as_int()).as_test_case_value()).unwrap();
                                    if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                        match if_statement {
                                            crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                angle,
                                            } => {
                                                writeln!(s, "                angle = {};", if angle.to_string().contains('.') { angle.to_string() } else { format!("{}.0", angle) }).unwrap();
                                            }
                                            crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                target,
                                            } => {
                                                writeln!(s, "                target = {};", target).unwrap();
                                            }
                                            crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
                                                spline_final_point,
                                            } => {
                                                // spline_final_point: Vector3d
                                                writeln!(s, "                spline_final_point = {{").unwrap();
                                                // Members
                                                writeln!(s, "                    x = {};", if spline_final_point.x.to_string().contains('.') { spline_final_point.x.to_string() } else { format!("{}.0", spline_final_point.x) }).unwrap();
                                                writeln!(s, "                    y = {};", if spline_final_point.y.to_string().contains('.') { spline_final_point.y.to_string() } else { format!("{}.0", spline_final_point.y) }).unwrap();
                                                writeln!(s, "                    z = {};", if spline_final_point.z.to_string().contains('.') { spline_final_point.z.to_string() } else { format!("{}.0", spline_final_point.z) }).unwrap();

                                                writeln!(s, "                }};").unwrap();
                                            }
                                        }
                                    }

                                    writeln!(s, "                time_passed = {};", if_statement.time_passed).unwrap();
                                    writeln!(s, "                duration = {};", if_statement.duration).unwrap();
                                    writeln!(s, "                id = {};", if_statement.id).unwrap();
                                    writeln!(s, "                amount_of_nodes = {};", if_statement.nodes.len()).unwrap();
                                    writeln!(s, "                nodes = [").unwrap();
                                    for v in if_statement.nodes.as_slice() {
                                        writeln!(s, "                    {{").unwrap();
                                        // Members
                                        writeln!(s, "                        x = {};", if v.x.to_string().contains('.') { v.x.to_string() } else { format!("{}.0", v.x) }).unwrap();
                                        writeln!(s, "                        y = {};", if v.y.to_string().contains('.') { v.y.to_string() } else { format!("{}.0", v.y) }).unwrap();
                                        writeln!(s, "                        z = {};", if v.z.to_string().contains('.') { v.z.to_string() } else { format!("{}.0", v.z) }).unwrap();

                                        writeln!(s, "                    }},").unwrap();
                                    }
                                    writeln!(s, "                ];").unwrap();
                                    // final_node: Vector3d
                                    writeln!(s, "                final_node = {{").unwrap();
                                    // Members
                                    writeln!(s, "                    x = {};", if if_statement.final_node.x.to_string().contains('.') { if_statement.final_node.x.to_string() } else { format!("{}.0", if_statement.final_node.x) }).unwrap();
                                    writeln!(s, "                    y = {};", if if_statement.final_node.y.to_string().contains('.') { if_statement.final_node.y.to_string() } else { format!("{}.0", if_statement.final_node.y) }).unwrap();
                                    writeln!(s, "                    z = {};", if if_statement.final_node.z.to_string().contains('.') { if_statement.final_node.z.to_string() } else { format!("{}.0", if_statement.final_node.z) }).unwrap();

                                    writeln!(s, "                }};").unwrap();
                                }

                            }
                            crate::wrath::MovementBlock_UpdateFlag_Living::Position {
                                corpse_orientation,
                                orientation1,
                                position1,
                                transport_guid,
                            } => {
                                writeln!(s, "                transport_guid = {};", transport_guid.guid()).unwrap();
                                // position1: Vector3d
                                writeln!(s, "                position1 = {{").unwrap();
                                // Members
                                writeln!(s, "                    x = {};", if position1.x.to_string().contains('.') { position1.x.to_string() } else { format!("{}.0", position1.x) }).unwrap();
                                writeln!(s, "                    y = {};", if position1.y.to_string().contains('.') { position1.y.to_string() } else { format!("{}.0", position1.y) }).unwrap();
                                writeln!(s, "                    z = {};", if position1.z.to_string().contains('.') { position1.z.to_string() } else { format!("{}.0", position1.z) }).unwrap();

                                writeln!(s, "                }};").unwrap();
                                writeln!(s, "                orientation1 = {};", if orientation1.to_string().contains('.') { orientation1.to_string() } else { format!("{}.0", orientation1) }).unwrap();
                                writeln!(s, "                corpse_orientation = {};", if corpse_orientation.to_string().contains('.') { corpse_orientation.to_string() } else { format!("{}.0", corpse_orientation) }).unwrap();
                            }
                            crate::wrath::MovementBlock_UpdateFlag_Living::HasPosition {
                                orientation2,
                                position2,
                            } => {
                                // position2: Vector3d
                                writeln!(s, "                position2 = {{").unwrap();
                                // Members
                                writeln!(s, "                    x = {};", if position2.x.to_string().contains('.') { position2.x.to_string() } else { format!("{}.0", position2.x) }).unwrap();
                                writeln!(s, "                    y = {};", if position2.y.to_string().contains('.') { position2.y.to_string() } else { format!("{}.0", position2.y) }).unwrap();
                                writeln!(s, "                    z = {};", if position2.z.to_string().contains('.') { position2.z.to_string() } else { format!("{}.0", position2.z) }).unwrap();

                                writeln!(s, "                }};").unwrap();
                                writeln!(s, "                orientation2 = {};", if orientation2.to_string().contains('.') { orientation2.to_string() } else { format!("{}.0", orientation2) }).unwrap();
                            }
                        }
                    }

                    if let Some(if_statement) = &movement1.update_flag.get_high_guid() {
                        writeln!(s, "                unknown0 = {};", if_statement.unknown0).unwrap();
                    }

                    if let Some(if_statement) = &movement1.update_flag.get_low_guid() {
                        writeln!(s, "                unknown1 = {};", if_statement.unknown1).unwrap();
                    }

                    if let Some(if_statement) = &movement1.update_flag.get_has_attacking_target() {
                        writeln!(s, "                guid = {};", if_statement.guid.guid()).unwrap();
                    }

                    if let Some(if_statement) = &movement1.update_flag.get_transport() {
                        writeln!(s, "                transport_progress_in_ms = {};", if_statement.transport_progress_in_ms).unwrap();
                    }

                    if let Some(if_statement) = &movement1.update_flag.get_vehicle() {
                        writeln!(s, "                vehicle_id = {};", if_statement.vehicle_id).unwrap();
                        writeln!(s, "                vehicle_orientation = {};", if if_statement.vehicle_orientation.to_string().contains('.') { if_statement.vehicle_orientation.to_string() } else { format!("{}.0", if_statement.vehicle_orientation) }).unwrap();
                    }

                    if let Some(if_statement) = &movement1.update_flag.get_rotation() {
                        writeln!(s, "                packed_local_rotation = {};", if_statement.packed_local_rotation).unwrap();
                    }


                    writeln!(s, "            }};").unwrap();
                }
                crate::wrath::Object_UpdateType::CreateObject {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                } => {
                    writeln!(s, "            guid3 = {};", guid3.guid()).unwrap();
                    writeln!(s, "            object_type = {};", object_type.as_test_case_value()).unwrap();
                    // movement2: MovementBlock
                    writeln!(s, "            movement2 = {{").unwrap();
                    // Members
                    writeln!(s, "                update_flag = {};", UpdateFlag::new(movement2.update_flag.as_int()).as_test_case_value()).unwrap();
                    if let Some(if_statement) = &movement2.update_flag.get_living() {
                        match if_statement {
                            crate::wrath::MovementBlock_UpdateFlag_Living::Living {
                                backwards_flight_speed,
                                backwards_running_speed,
                                backwards_swimming_speed,
                                fall_time,
                                flags,
                                flight_speed,
                                orientation,
                                pitch_rate,
                                position,
                                running_speed,
                                swimming_speed,
                                timestamp,
                                turn_rate,
                                walking_speed,
                            } => {
                                writeln!(s, "                flags = {};", MovementFlags::new(flags.as_int()).as_test_case_value()).unwrap();
                                writeln!(s, "                timestamp = {};", timestamp).unwrap();
                                // position: Vector3d
                                writeln!(s, "                position = {{").unwrap();
                                // Members
                                writeln!(s, "                    x = {};", if position.x.to_string().contains('.') { position.x.to_string() } else { format!("{}.0", position.x) }).unwrap();
                                writeln!(s, "                    y = {};", if position.y.to_string().contains('.') { position.y.to_string() } else { format!("{}.0", position.y) }).unwrap();
                                writeln!(s, "                    z = {};", if position.z.to_string().contains('.') { position.z.to_string() } else { format!("{}.0", position.z) }).unwrap();

                                writeln!(s, "                }};").unwrap();
                                writeln!(s, "                orientation = {};", if orientation.to_string().contains('.') { orientation.to_string() } else { format!("{}.0", orientation) }).unwrap();
                                if let Some(if_statement) = &flags.get_on_transport_and_interpolated_movement() {
                                    match if_statement {
                                        crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                                            transport_info,
                                            transport_time,
                                        } => {
                                            // transport_info: TransportInfo
                                            writeln!(s, "                transport_info = {{").unwrap();
                                            // Members
                                            writeln!(s, "                    guid = {};", transport_info.guid.guid()).unwrap();
                                            // position: Vector3d
                                            writeln!(s, "                    position = {{").unwrap();
                                            // Members
                                            writeln!(s, "                        x = {};", if transport_info.position.x.to_string().contains('.') { transport_info.position.x.to_string() } else { format!("{}.0", transport_info.position.x) }).unwrap();
                                            writeln!(s, "                        y = {};", if transport_info.position.y.to_string().contains('.') { transport_info.position.y.to_string() } else { format!("{}.0", transport_info.position.y) }).unwrap();
                                            writeln!(s, "                        z = {};", if transport_info.position.z.to_string().contains('.') { transport_info.position.z.to_string() } else { format!("{}.0", transport_info.position.z) }).unwrap();

                                            writeln!(s, "                    }};").unwrap();
                                            writeln!(s, "                    orientation = {};", if transport_info.orientation.to_string().contains('.') { transport_info.orientation.to_string() } else { format!("{}.0", transport_info.orientation) }).unwrap();
                                            writeln!(s, "                    timestamp = {};", transport_info.timestamp).unwrap();
                                            writeln!(s, "                    seat = {};", transport_info.seat).unwrap();

                                            writeln!(s, "                }};").unwrap();
                                            writeln!(s, "                transport_time = {};", transport_time).unwrap();
                                        }
                                        crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                                            transport,
                                        } => {
                                            // transport: TransportInfo
                                            writeln!(s, "                transport = {{").unwrap();
                                            // Members
                                            writeln!(s, "                    guid = {};", transport.guid.guid()).unwrap();
                                            // position: Vector3d
                                            writeln!(s, "                    position = {{").unwrap();
                                            // Members
                                            writeln!(s, "                        x = {};", if transport.position.x.to_string().contains('.') { transport.position.x.to_string() } else { format!("{}.0", transport.position.x) }).unwrap();
                                            writeln!(s, "                        y = {};", if transport.position.y.to_string().contains('.') { transport.position.y.to_string() } else { format!("{}.0", transport.position.y) }).unwrap();
                                            writeln!(s, "                        z = {};", if transport.position.z.to_string().contains('.') { transport.position.z.to_string() } else { format!("{}.0", transport.position.z) }).unwrap();

                                            writeln!(s, "                    }};").unwrap();
                                            writeln!(s, "                    orientation = {};", if transport.orientation.to_string().contains('.') { transport.orientation.to_string() } else { format!("{}.0", transport.orientation) }).unwrap();
                                            writeln!(s, "                    timestamp = {};", transport.timestamp).unwrap();
                                            writeln!(s, "                    seat = {};", transport.seat).unwrap();

                                            writeln!(s, "                }};").unwrap();
                                        }
                                    }
                                }

                                if let Some(if_statement) = &flags.get_swimming() {
                                    match if_statement {
                                        crate::wrath::MovementBlock_MovementFlags_Swimming::Swimming {
                                            pitch1,
                                        } => {
                                            writeln!(s, "                pitch1 = {};", if pitch1.to_string().contains('.') { pitch1.to_string() } else { format!("{}.0", pitch1) }).unwrap();
                                        }
                                        crate::wrath::MovementBlock_MovementFlags_Swimming::Flying {
                                            pitch2,
                                        } => {
                                            writeln!(s, "                pitch2 = {};", if pitch2.to_string().contains('.') { pitch2.to_string() } else { format!("{}.0", pitch2) }).unwrap();
                                        }
                                        crate::wrath::MovementBlock_MovementFlags_Swimming::AlwaysAllowPitching {
                                            pitch3,
                                        } => {
                                            writeln!(s, "                pitch3 = {};", if pitch3.to_string().contains('.') { pitch3.to_string() } else { format!("{}.0", pitch3) }).unwrap();
                                        }
                                    }
                                }

                                writeln!(s, "                fall_time = {};", if fall_time.to_string().contains('.') { fall_time.to_string() } else { format!("{}.0", fall_time) }).unwrap();
                                if let Some(if_statement) = &flags.get_falling() {
                                    writeln!(s, "                z_speed = {};", if if_statement.z_speed.to_string().contains('.') { if_statement.z_speed.to_string() } else { format!("{}.0", if_statement.z_speed) }).unwrap();
                                    writeln!(s, "                cos_angle = {};", if if_statement.cos_angle.to_string().contains('.') { if_statement.cos_angle.to_string() } else { format!("{}.0", if_statement.cos_angle) }).unwrap();
                                    writeln!(s, "                sin_angle = {};", if if_statement.sin_angle.to_string().contains('.') { if_statement.sin_angle.to_string() } else { format!("{}.0", if_statement.sin_angle) }).unwrap();
                                    writeln!(s, "                xy_speed = {};", if if_statement.xy_speed.to_string().contains('.') { if_statement.xy_speed.to_string() } else { format!("{}.0", if_statement.xy_speed) }).unwrap();
                                }

                                if let Some(if_statement) = &flags.get_spline_elevation() {
                                    writeln!(s, "                spline_elevation = {};", if if_statement.spline_elevation.to_string().contains('.') { if_statement.spline_elevation.to_string() } else { format!("{}.0", if_statement.spline_elevation) }).unwrap();
                                }

                                writeln!(s, "                walking_speed = {};", if walking_speed.to_string().contains('.') { walking_speed.to_string() } else { format!("{}.0", walking_speed) }).unwrap();
                                writeln!(s, "                running_speed = {};", if running_speed.to_string().contains('.') { running_speed.to_string() } else { format!("{}.0", running_speed) }).unwrap();
                                writeln!(s, "                backwards_running_speed = {};", if backwards_running_speed.to_string().contains('.') { backwards_running_speed.to_string() } else { format!("{}.0", backwards_running_speed) }).unwrap();
                                writeln!(s, "                swimming_speed = {};", if swimming_speed.to_string().contains('.') { swimming_speed.to_string() } else { format!("{}.0", swimming_speed) }).unwrap();
                                writeln!(s, "                backwards_swimming_speed = {};", if backwards_swimming_speed.to_string().contains('.') { backwards_swimming_speed.to_string() } else { format!("{}.0", backwards_swimming_speed) }).unwrap();
                                writeln!(s, "                flight_speed = {};", if flight_speed.to_string().contains('.') { flight_speed.to_string() } else { format!("{}.0", flight_speed) }).unwrap();
                                writeln!(s, "                backwards_flight_speed = {};", if backwards_flight_speed.to_string().contains('.') { backwards_flight_speed.to_string() } else { format!("{}.0", backwards_flight_speed) }).unwrap();
                                writeln!(s, "                turn_rate = {};", if turn_rate.to_string().contains('.') { turn_rate.to_string() } else { format!("{}.0", turn_rate) }).unwrap();
                                writeln!(s, "                pitch_rate = {};", if pitch_rate.to_string().contains('.') { pitch_rate.to_string() } else { format!("{}.0", pitch_rate) }).unwrap();
                                if let Some(if_statement) = &flags.get_spline_enabled() {
                                    writeln!(s, "                spline_flags = {};", SplineFlag::new(if_statement.spline_flags.as_int()).as_test_case_value()).unwrap();
                                    if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                        match if_statement {
                                            crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                angle,
                                            } => {
                                                writeln!(s, "                angle = {};", if angle.to_string().contains('.') { angle.to_string() } else { format!("{}.0", angle) }).unwrap();
                                            }
                                            crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                target,
                                            } => {
                                                writeln!(s, "                target = {};", target).unwrap();
                                            }
                                            crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
                                                spline_final_point,
                                            } => {
                                                // spline_final_point: Vector3d
                                                writeln!(s, "                spline_final_point = {{").unwrap();
                                                // Members
                                                writeln!(s, "                    x = {};", if spline_final_point.x.to_string().contains('.') { spline_final_point.x.to_string() } else { format!("{}.0", spline_final_point.x) }).unwrap();
                                                writeln!(s, "                    y = {};", if spline_final_point.y.to_string().contains('.') { spline_final_point.y.to_string() } else { format!("{}.0", spline_final_point.y) }).unwrap();
                                                writeln!(s, "                    z = {};", if spline_final_point.z.to_string().contains('.') { spline_final_point.z.to_string() } else { format!("{}.0", spline_final_point.z) }).unwrap();

                                                writeln!(s, "                }};").unwrap();
                                            }
                                        }
                                    }

                                    writeln!(s, "                time_passed = {};", if_statement.time_passed).unwrap();
                                    writeln!(s, "                duration = {};", if_statement.duration).unwrap();
                                    writeln!(s, "                id = {};", if_statement.id).unwrap();
                                    writeln!(s, "                amount_of_nodes = {};", if_statement.nodes.len()).unwrap();
                                    writeln!(s, "                nodes = [").unwrap();
                                    for v in if_statement.nodes.as_slice() {
                                        writeln!(s, "                    {{").unwrap();
                                        // Members
                                        writeln!(s, "                        x = {};", if v.x.to_string().contains('.') { v.x.to_string() } else { format!("{}.0", v.x) }).unwrap();
                                        writeln!(s, "                        y = {};", if v.y.to_string().contains('.') { v.y.to_string() } else { format!("{}.0", v.y) }).unwrap();
                                        writeln!(s, "                        z = {};", if v.z.to_string().contains('.') { v.z.to_string() } else { format!("{}.0", v.z) }).unwrap();

                                        writeln!(s, "                    }},").unwrap();
                                    }
                                    writeln!(s, "                ];").unwrap();
                                    // final_node: Vector3d
                                    writeln!(s, "                final_node = {{").unwrap();
                                    // Members
                                    writeln!(s, "                    x = {};", if if_statement.final_node.x.to_string().contains('.') { if_statement.final_node.x.to_string() } else { format!("{}.0", if_statement.final_node.x) }).unwrap();
                                    writeln!(s, "                    y = {};", if if_statement.final_node.y.to_string().contains('.') { if_statement.final_node.y.to_string() } else { format!("{}.0", if_statement.final_node.y) }).unwrap();
                                    writeln!(s, "                    z = {};", if if_statement.final_node.z.to_string().contains('.') { if_statement.final_node.z.to_string() } else { format!("{}.0", if_statement.final_node.z) }).unwrap();

                                    writeln!(s, "                }};").unwrap();
                                }

                            }
                            crate::wrath::MovementBlock_UpdateFlag_Living::Position {
                                corpse_orientation,
                                orientation1,
                                position1,
                                transport_guid,
                            } => {
                                writeln!(s, "                transport_guid = {};", transport_guid.guid()).unwrap();
                                // position1: Vector3d
                                writeln!(s, "                position1 = {{").unwrap();
                                // Members
                                writeln!(s, "                    x = {};", if position1.x.to_string().contains('.') { position1.x.to_string() } else { format!("{}.0", position1.x) }).unwrap();
                                writeln!(s, "                    y = {};", if position1.y.to_string().contains('.') { position1.y.to_string() } else { format!("{}.0", position1.y) }).unwrap();
                                writeln!(s, "                    z = {};", if position1.z.to_string().contains('.') { position1.z.to_string() } else { format!("{}.0", position1.z) }).unwrap();

                                writeln!(s, "                }};").unwrap();
                                writeln!(s, "                orientation1 = {};", if orientation1.to_string().contains('.') { orientation1.to_string() } else { format!("{}.0", orientation1) }).unwrap();
                                writeln!(s, "                corpse_orientation = {};", if corpse_orientation.to_string().contains('.') { corpse_orientation.to_string() } else { format!("{}.0", corpse_orientation) }).unwrap();
                            }
                            crate::wrath::MovementBlock_UpdateFlag_Living::HasPosition {
                                orientation2,
                                position2,
                            } => {
                                // position2: Vector3d
                                writeln!(s, "                position2 = {{").unwrap();
                                // Members
                                writeln!(s, "                    x = {};", if position2.x.to_string().contains('.') { position2.x.to_string() } else { format!("{}.0", position2.x) }).unwrap();
                                writeln!(s, "                    y = {};", if position2.y.to_string().contains('.') { position2.y.to_string() } else { format!("{}.0", position2.y) }).unwrap();
                                writeln!(s, "                    z = {};", if position2.z.to_string().contains('.') { position2.z.to_string() } else { format!("{}.0", position2.z) }).unwrap();

                                writeln!(s, "                }};").unwrap();
                                writeln!(s, "                orientation2 = {};", if orientation2.to_string().contains('.') { orientation2.to_string() } else { format!("{}.0", orientation2) }).unwrap();
                            }
                        }
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_high_guid() {
                        writeln!(s, "                unknown0 = {};", if_statement.unknown0).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_low_guid() {
                        writeln!(s, "                unknown1 = {};", if_statement.unknown1).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_has_attacking_target() {
                        writeln!(s, "                guid = {};", if_statement.guid.guid()).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_transport() {
                        writeln!(s, "                transport_progress_in_ms = {};", if_statement.transport_progress_in_ms).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_vehicle() {
                        writeln!(s, "                vehicle_id = {};", if_statement.vehicle_id).unwrap();
                        writeln!(s, "                vehicle_orientation = {};", if if_statement.vehicle_orientation.to_string().contains('.') { if_statement.vehicle_orientation.to_string() } else { format!("{}.0", if_statement.vehicle_orientation) }).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_rotation() {
                        writeln!(s, "                packed_local_rotation = {};", if_statement.packed_local_rotation).unwrap();
                    }


                    writeln!(s, "            }};").unwrap();
                    panic!("unsupported type for test case printing: 'UpdateMask' for variable 'mask2'");
                }
                crate::wrath::Object_UpdateType::CreateObject2 {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                } => {
                    writeln!(s, "            guid3 = {};", guid3.guid()).unwrap();
                    writeln!(s, "            object_type = {};", object_type.as_test_case_value()).unwrap();
                    // movement2: MovementBlock
                    writeln!(s, "            movement2 = {{").unwrap();
                    // Members
                    writeln!(s, "                update_flag = {};", UpdateFlag::new(movement2.update_flag.as_int()).as_test_case_value()).unwrap();
                    if let Some(if_statement) = &movement2.update_flag.get_living() {
                        match if_statement {
                            crate::wrath::MovementBlock_UpdateFlag_Living::Living {
                                backwards_flight_speed,
                                backwards_running_speed,
                                backwards_swimming_speed,
                                fall_time,
                                flags,
                                flight_speed,
                                orientation,
                                pitch_rate,
                                position,
                                running_speed,
                                swimming_speed,
                                timestamp,
                                turn_rate,
                                walking_speed,
                            } => {
                                writeln!(s, "                flags = {};", MovementFlags::new(flags.as_int()).as_test_case_value()).unwrap();
                                writeln!(s, "                timestamp = {};", timestamp).unwrap();
                                // position: Vector3d
                                writeln!(s, "                position = {{").unwrap();
                                // Members
                                writeln!(s, "                    x = {};", if position.x.to_string().contains('.') { position.x.to_string() } else { format!("{}.0", position.x) }).unwrap();
                                writeln!(s, "                    y = {};", if position.y.to_string().contains('.') { position.y.to_string() } else { format!("{}.0", position.y) }).unwrap();
                                writeln!(s, "                    z = {};", if position.z.to_string().contains('.') { position.z.to_string() } else { format!("{}.0", position.z) }).unwrap();

                                writeln!(s, "                }};").unwrap();
                                writeln!(s, "                orientation = {};", if orientation.to_string().contains('.') { orientation.to_string() } else { format!("{}.0", orientation) }).unwrap();
                                if let Some(if_statement) = &flags.get_on_transport_and_interpolated_movement() {
                                    match if_statement {
                                        crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                                            transport_info,
                                            transport_time,
                                        } => {
                                            // transport_info: TransportInfo
                                            writeln!(s, "                transport_info = {{").unwrap();
                                            // Members
                                            writeln!(s, "                    guid = {};", transport_info.guid.guid()).unwrap();
                                            // position: Vector3d
                                            writeln!(s, "                    position = {{").unwrap();
                                            // Members
                                            writeln!(s, "                        x = {};", if transport_info.position.x.to_string().contains('.') { transport_info.position.x.to_string() } else { format!("{}.0", transport_info.position.x) }).unwrap();
                                            writeln!(s, "                        y = {};", if transport_info.position.y.to_string().contains('.') { transport_info.position.y.to_string() } else { format!("{}.0", transport_info.position.y) }).unwrap();
                                            writeln!(s, "                        z = {};", if transport_info.position.z.to_string().contains('.') { transport_info.position.z.to_string() } else { format!("{}.0", transport_info.position.z) }).unwrap();

                                            writeln!(s, "                    }};").unwrap();
                                            writeln!(s, "                    orientation = {};", if transport_info.orientation.to_string().contains('.') { transport_info.orientation.to_string() } else { format!("{}.0", transport_info.orientation) }).unwrap();
                                            writeln!(s, "                    timestamp = {};", transport_info.timestamp).unwrap();
                                            writeln!(s, "                    seat = {};", transport_info.seat).unwrap();

                                            writeln!(s, "                }};").unwrap();
                                            writeln!(s, "                transport_time = {};", transport_time).unwrap();
                                        }
                                        crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                                            transport,
                                        } => {
                                            // transport: TransportInfo
                                            writeln!(s, "                transport = {{").unwrap();
                                            // Members
                                            writeln!(s, "                    guid = {};", transport.guid.guid()).unwrap();
                                            // position: Vector3d
                                            writeln!(s, "                    position = {{").unwrap();
                                            // Members
                                            writeln!(s, "                        x = {};", if transport.position.x.to_string().contains('.') { transport.position.x.to_string() } else { format!("{}.0", transport.position.x) }).unwrap();
                                            writeln!(s, "                        y = {};", if transport.position.y.to_string().contains('.') { transport.position.y.to_string() } else { format!("{}.0", transport.position.y) }).unwrap();
                                            writeln!(s, "                        z = {};", if transport.position.z.to_string().contains('.') { transport.position.z.to_string() } else { format!("{}.0", transport.position.z) }).unwrap();

                                            writeln!(s, "                    }};").unwrap();
                                            writeln!(s, "                    orientation = {};", if transport.orientation.to_string().contains('.') { transport.orientation.to_string() } else { format!("{}.0", transport.orientation) }).unwrap();
                                            writeln!(s, "                    timestamp = {};", transport.timestamp).unwrap();
                                            writeln!(s, "                    seat = {};", transport.seat).unwrap();

                                            writeln!(s, "                }};").unwrap();
                                        }
                                    }
                                }

                                if let Some(if_statement) = &flags.get_swimming() {
                                    match if_statement {
                                        crate::wrath::MovementBlock_MovementFlags_Swimming::Swimming {
                                            pitch1,
                                        } => {
                                            writeln!(s, "                pitch1 = {};", if pitch1.to_string().contains('.') { pitch1.to_string() } else { format!("{}.0", pitch1) }).unwrap();
                                        }
                                        crate::wrath::MovementBlock_MovementFlags_Swimming::Flying {
                                            pitch2,
                                        } => {
                                            writeln!(s, "                pitch2 = {};", if pitch2.to_string().contains('.') { pitch2.to_string() } else { format!("{}.0", pitch2) }).unwrap();
                                        }
                                        crate::wrath::MovementBlock_MovementFlags_Swimming::AlwaysAllowPitching {
                                            pitch3,
                                        } => {
                                            writeln!(s, "                pitch3 = {};", if pitch3.to_string().contains('.') { pitch3.to_string() } else { format!("{}.0", pitch3) }).unwrap();
                                        }
                                    }
                                }

                                writeln!(s, "                fall_time = {};", if fall_time.to_string().contains('.') { fall_time.to_string() } else { format!("{}.0", fall_time) }).unwrap();
                                if let Some(if_statement) = &flags.get_falling() {
                                    writeln!(s, "                z_speed = {};", if if_statement.z_speed.to_string().contains('.') { if_statement.z_speed.to_string() } else { format!("{}.0", if_statement.z_speed) }).unwrap();
                                    writeln!(s, "                cos_angle = {};", if if_statement.cos_angle.to_string().contains('.') { if_statement.cos_angle.to_string() } else { format!("{}.0", if_statement.cos_angle) }).unwrap();
                                    writeln!(s, "                sin_angle = {};", if if_statement.sin_angle.to_string().contains('.') { if_statement.sin_angle.to_string() } else { format!("{}.0", if_statement.sin_angle) }).unwrap();
                                    writeln!(s, "                xy_speed = {};", if if_statement.xy_speed.to_string().contains('.') { if_statement.xy_speed.to_string() } else { format!("{}.0", if_statement.xy_speed) }).unwrap();
                                }

                                if let Some(if_statement) = &flags.get_spline_elevation() {
                                    writeln!(s, "                spline_elevation = {};", if if_statement.spline_elevation.to_string().contains('.') { if_statement.spline_elevation.to_string() } else { format!("{}.0", if_statement.spline_elevation) }).unwrap();
                                }

                                writeln!(s, "                walking_speed = {};", if walking_speed.to_string().contains('.') { walking_speed.to_string() } else { format!("{}.0", walking_speed) }).unwrap();
                                writeln!(s, "                running_speed = {};", if running_speed.to_string().contains('.') { running_speed.to_string() } else { format!("{}.0", running_speed) }).unwrap();
                                writeln!(s, "                backwards_running_speed = {};", if backwards_running_speed.to_string().contains('.') { backwards_running_speed.to_string() } else { format!("{}.0", backwards_running_speed) }).unwrap();
                                writeln!(s, "                swimming_speed = {};", if swimming_speed.to_string().contains('.') { swimming_speed.to_string() } else { format!("{}.0", swimming_speed) }).unwrap();
                                writeln!(s, "                backwards_swimming_speed = {};", if backwards_swimming_speed.to_string().contains('.') { backwards_swimming_speed.to_string() } else { format!("{}.0", backwards_swimming_speed) }).unwrap();
                                writeln!(s, "                flight_speed = {};", if flight_speed.to_string().contains('.') { flight_speed.to_string() } else { format!("{}.0", flight_speed) }).unwrap();
                                writeln!(s, "                backwards_flight_speed = {};", if backwards_flight_speed.to_string().contains('.') { backwards_flight_speed.to_string() } else { format!("{}.0", backwards_flight_speed) }).unwrap();
                                writeln!(s, "                turn_rate = {};", if turn_rate.to_string().contains('.') { turn_rate.to_string() } else { format!("{}.0", turn_rate) }).unwrap();
                                writeln!(s, "                pitch_rate = {};", if pitch_rate.to_string().contains('.') { pitch_rate.to_string() } else { format!("{}.0", pitch_rate) }).unwrap();
                                if let Some(if_statement) = &flags.get_spline_enabled() {
                                    writeln!(s, "                spline_flags = {};", SplineFlag::new(if_statement.spline_flags.as_int()).as_test_case_value()).unwrap();
                                    if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                        match if_statement {
                                            crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                angle,
                                            } => {
                                                writeln!(s, "                angle = {};", if angle.to_string().contains('.') { angle.to_string() } else { format!("{}.0", angle) }).unwrap();
                                            }
                                            crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                target,
                                            } => {
                                                writeln!(s, "                target = {};", target).unwrap();
                                            }
                                            crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
                                                spline_final_point,
                                            } => {
                                                // spline_final_point: Vector3d
                                                writeln!(s, "                spline_final_point = {{").unwrap();
                                                // Members
                                                writeln!(s, "                    x = {};", if spline_final_point.x.to_string().contains('.') { spline_final_point.x.to_string() } else { format!("{}.0", spline_final_point.x) }).unwrap();
                                                writeln!(s, "                    y = {};", if spline_final_point.y.to_string().contains('.') { spline_final_point.y.to_string() } else { format!("{}.0", spline_final_point.y) }).unwrap();
                                                writeln!(s, "                    z = {};", if spline_final_point.z.to_string().contains('.') { spline_final_point.z.to_string() } else { format!("{}.0", spline_final_point.z) }).unwrap();

                                                writeln!(s, "                }};").unwrap();
                                            }
                                        }
                                    }

                                    writeln!(s, "                time_passed = {};", if_statement.time_passed).unwrap();
                                    writeln!(s, "                duration = {};", if_statement.duration).unwrap();
                                    writeln!(s, "                id = {};", if_statement.id).unwrap();
                                    writeln!(s, "                amount_of_nodes = {};", if_statement.nodes.len()).unwrap();
                                    writeln!(s, "                nodes = [").unwrap();
                                    for v in if_statement.nodes.as_slice() {
                                        writeln!(s, "                    {{").unwrap();
                                        // Members
                                        writeln!(s, "                        x = {};", if v.x.to_string().contains('.') { v.x.to_string() } else { format!("{}.0", v.x) }).unwrap();
                                        writeln!(s, "                        y = {};", if v.y.to_string().contains('.') { v.y.to_string() } else { format!("{}.0", v.y) }).unwrap();
                                        writeln!(s, "                        z = {};", if v.z.to_string().contains('.') { v.z.to_string() } else { format!("{}.0", v.z) }).unwrap();

                                        writeln!(s, "                    }},").unwrap();
                                    }
                                    writeln!(s, "                ];").unwrap();
                                    // final_node: Vector3d
                                    writeln!(s, "                final_node = {{").unwrap();
                                    // Members
                                    writeln!(s, "                    x = {};", if if_statement.final_node.x.to_string().contains('.') { if_statement.final_node.x.to_string() } else { format!("{}.0", if_statement.final_node.x) }).unwrap();
                                    writeln!(s, "                    y = {};", if if_statement.final_node.y.to_string().contains('.') { if_statement.final_node.y.to_string() } else { format!("{}.0", if_statement.final_node.y) }).unwrap();
                                    writeln!(s, "                    z = {};", if if_statement.final_node.z.to_string().contains('.') { if_statement.final_node.z.to_string() } else { format!("{}.0", if_statement.final_node.z) }).unwrap();

                                    writeln!(s, "                }};").unwrap();
                                }

                            }
                            crate::wrath::MovementBlock_UpdateFlag_Living::Position {
                                corpse_orientation,
                                orientation1,
                                position1,
                                transport_guid,
                            } => {
                                writeln!(s, "                transport_guid = {};", transport_guid.guid()).unwrap();
                                // position1: Vector3d
                                writeln!(s, "                position1 = {{").unwrap();
                                // Members
                                writeln!(s, "                    x = {};", if position1.x.to_string().contains('.') { position1.x.to_string() } else { format!("{}.0", position1.x) }).unwrap();
                                writeln!(s, "                    y = {};", if position1.y.to_string().contains('.') { position1.y.to_string() } else { format!("{}.0", position1.y) }).unwrap();
                                writeln!(s, "                    z = {};", if position1.z.to_string().contains('.') { position1.z.to_string() } else { format!("{}.0", position1.z) }).unwrap();

                                writeln!(s, "                }};").unwrap();
                                writeln!(s, "                orientation1 = {};", if orientation1.to_string().contains('.') { orientation1.to_string() } else { format!("{}.0", orientation1) }).unwrap();
                                writeln!(s, "                corpse_orientation = {};", if corpse_orientation.to_string().contains('.') { corpse_orientation.to_string() } else { format!("{}.0", corpse_orientation) }).unwrap();
                            }
                            crate::wrath::MovementBlock_UpdateFlag_Living::HasPosition {
                                orientation2,
                                position2,
                            } => {
                                // position2: Vector3d
                                writeln!(s, "                position2 = {{").unwrap();
                                // Members
                                writeln!(s, "                    x = {};", if position2.x.to_string().contains('.') { position2.x.to_string() } else { format!("{}.0", position2.x) }).unwrap();
                                writeln!(s, "                    y = {};", if position2.y.to_string().contains('.') { position2.y.to_string() } else { format!("{}.0", position2.y) }).unwrap();
                                writeln!(s, "                    z = {};", if position2.z.to_string().contains('.') { position2.z.to_string() } else { format!("{}.0", position2.z) }).unwrap();

                                writeln!(s, "                }};").unwrap();
                                writeln!(s, "                orientation2 = {};", if orientation2.to_string().contains('.') { orientation2.to_string() } else { format!("{}.0", orientation2) }).unwrap();
                            }
                        }
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_high_guid() {
                        writeln!(s, "                unknown0 = {};", if_statement.unknown0).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_low_guid() {
                        writeln!(s, "                unknown1 = {};", if_statement.unknown1).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_has_attacking_target() {
                        writeln!(s, "                guid = {};", if_statement.guid.guid()).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_transport() {
                        writeln!(s, "                transport_progress_in_ms = {};", if_statement.transport_progress_in_ms).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_vehicle() {
                        writeln!(s, "                vehicle_id = {};", if_statement.vehicle_id).unwrap();
                        writeln!(s, "                vehicle_orientation = {};", if if_statement.vehicle_orientation.to_string().contains('.') { if_statement.vehicle_orientation.to_string() } else { format!("{}.0", if_statement.vehicle_orientation) }).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_rotation() {
                        writeln!(s, "                packed_local_rotation = {};", if_statement.packed_local_rotation).unwrap();
                    }


                    writeln!(s, "            }};").unwrap();
                    panic!("unsupported type for test case printing: 'UpdateMask' for variable 'mask2'");
                }
                crate::wrath::Object_UpdateType::OutOfRangeObjects {
                    guids,
                } => {
                    writeln!(s, "            count = {};", guids.len()).unwrap();
                    writeln!(s, "            guids = [").unwrap();
                    for v in guids.as_slice() {
                        write!(s, "{v:#08X}, ").unwrap();
                    }
                    writeln!(s, "            ];").unwrap();
                }
                crate::wrath::Object_UpdateType::NearObjects {
                    guids,
                } => {
                    writeln!(s, "            count = {};", guids.len()).unwrap();
                    writeln!(s, "            guids = [").unwrap();
                    for v in guids.as_slice() {
                        write!(s, "{v:#08X}, ").unwrap();
                    }
                    writeln!(s, "            ];").unwrap();
                }
            }


            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 169_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_objects", "    ");
        if !self.objects.is_empty() {
            writeln!(s, "    /* objects: Object[amount_of_objects] start */").unwrap();
            for (i, v) in self.objects.iter().enumerate() {
                writeln!(s, "    /* objects: Object[amount_of_objects] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "update_type", "        ");
                match &v.update_type {
                    crate::wrath::Object_UpdateType::Values {
                        guid1,
                        mask1,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid1), "guid1", "        ");
                        panic!("unsupported type UpdateMask for variable 'mask1'");
                    }
                    crate::wrath::Object_UpdateType::Movement {
                        guid2,
                        movement1,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid2), "guid2", "        ");
                        writeln!(s, "    /* movement1: MovementBlock start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 2, "update_flag", "            ");
                        if let Some(if_statement) = &movement1.update_flag.get_living() {
                            match if_statement {
                                crate::wrath::MovementBlock_UpdateFlag_Living::Living {
                                    backwards_flight_speed,
                                    backwards_running_speed,
                                    backwards_swimming_speed,
                                    fall_time,
                                    flags,
                                    flight_speed,
                                    orientation,
                                    pitch_rate,
                                    position,
                                    running_speed,
                                    swimming_speed,
                                    timestamp,
                                    turn_rate,
                                    walking_speed,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, 6, "flags", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "            ");
                                    writeln!(s, "    /* position: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "            ");
                                    if let Some(if_statement) = &flags.get_on_transport_and_interpolated_movement() {
                                        match if_statement {
                                            crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                                                transport_info,
                                                transport_time,
                                            } => {
                                                writeln!(s, "    /* transport_info: TransportInfo start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport_info.guid), "guid", "                ");
                                                writeln!(s, "    /* position: Vector3d start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                    ");
                                                writeln!(s, "    /* position: Vector3d end */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 1, "seat", "                ");
                                                writeln!(s, "    /* transport_info: TransportInfo end */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_time", "            ");
                                            }
                                            crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                                                transport,
                                            } => {
                                                writeln!(s, "    /* transport: TransportInfo start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport.guid), "guid", "                ");
                                                writeln!(s, "    /* position: Vector3d start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                    ");
                                                writeln!(s, "    /* position: Vector3d end */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 1, "seat", "                ");
                                                writeln!(s, "    /* transport: TransportInfo end */").unwrap();
                                            }
                                        }
                                    }

                                    if let Some(if_statement) = &flags.get_swimming() {
                                        match if_statement {
                                            crate::wrath::MovementBlock_MovementFlags_Swimming::Swimming {
                                                pitch1,
                                            } => {
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch1", "            ");
                                            }
                                            crate::wrath::MovementBlock_MovementFlags_Swimming::Flying {
                                                pitch2,
                                            } => {
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch2", "            ");
                                            }
                                            crate::wrath::MovementBlock_MovementFlags_Swimming::AlwaysAllowPitching {
                                                pitch3,
                                            } => {
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch3", "            ");
                                            }
                                        }
                                    }

                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "fall_time", "            ");
                                    if let Some(if_statement) = &flags.get_falling() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z_speed", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "cos_angle", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "sin_angle", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "xy_speed", "            ");
                                    }

                                    if let Some(if_statement) = &flags.get_spline_elevation() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_elevation", "            ");
                                    }

                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "walking_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "running_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "backwards_running_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "swimming_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "backwards_swimming_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "flight_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "backwards_flight_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "turn_rate", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch_rate", "            ");
                                    if let Some(if_statement) = &flags.get_spline_enabled() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                        if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                            match if_statement {
                                                crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                    angle,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "angle", "            ");
                                                }
                                                crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                    target,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "            ");
                                                }
                                                crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
                                                    spline_final_point,
                                                } => {
                                                    writeln!(s, "    /* spline_final_point: Vector3d start */").unwrap();
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                                    writeln!(s, "    /* spline_final_point: Vector3d end */").unwrap();
                                                }
                                            }
                                        }

                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_passed", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_nodes", "            ");
                                        if !if_statement.nodes.is_empty() {
                                            writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] start */").unwrap();
                                            for (i, v) in if_statement.nodes.iter().enumerate() {
                                                writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] {i} start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                                writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] {i} end */").unwrap();
                                            }
                                            writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] end */").unwrap();
                                        }
                                        writeln!(s, "    /* final_node: Vector3d start */").unwrap();
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                        writeln!(s, "    /* final_node: Vector3d end */").unwrap();
                                    }

                                }
                                crate::wrath::MovementBlock_UpdateFlag_Living::Position {
                                    corpse_orientation,
                                    orientation1,
                                    position1,
                                    transport_guid,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport_guid), "transport_guid", "            ");
                                    writeln!(s, "    /* position1: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position1: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation1", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "corpse_orientation", "            ");
                                }
                                crate::wrath::MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation2,
                                    position2,
                                } => {
                                    writeln!(s, "    /* position2: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position2: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation2", "            ");
                                }
                            }
                        }

                        if let Some(if_statement) = &movement1.update_flag.get_high_guid() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown0", "            ");
                        }

                        if let Some(if_statement) = &movement1.update_flag.get_low_guid() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "            ");
                        }

                        if let Some(if_statement) = &movement1.update_flag.get_has_attacking_target() {
                            crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.guid), "guid", "            ");
                        }

                        if let Some(if_statement) = &movement1.update_flag.get_transport() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_progress_in_ms", "            ");
                        }

                        if let Some(if_statement) = &movement1.update_flag.get_vehicle() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "vehicle_id", "            ");
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "vehicle_orientation", "            ");
                        }

                        if let Some(if_statement) = &movement1.update_flag.get_rotation() {
                            crate::util::write_bytes(&mut s, &mut bytes, 8, "packed_local_rotation", "            ");
                        }

                        writeln!(s, "    /* movement1: MovementBlock end */").unwrap();
                    }
                    crate::wrath::Object_UpdateType::CreateObject {
                        guid3,
                        mask2,
                        movement2,
                        object_type,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid3), "guid3", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "object_type", "        ");
                        writeln!(s, "    /* movement2: MovementBlock start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 2, "update_flag", "            ");
                        if let Some(if_statement) = &movement2.update_flag.get_living() {
                            match if_statement {
                                crate::wrath::MovementBlock_UpdateFlag_Living::Living {
                                    backwards_flight_speed,
                                    backwards_running_speed,
                                    backwards_swimming_speed,
                                    fall_time,
                                    flags,
                                    flight_speed,
                                    orientation,
                                    pitch_rate,
                                    position,
                                    running_speed,
                                    swimming_speed,
                                    timestamp,
                                    turn_rate,
                                    walking_speed,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, 6, "flags", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "            ");
                                    writeln!(s, "    /* position: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "            ");
                                    if let Some(if_statement) = &flags.get_on_transport_and_interpolated_movement() {
                                        match if_statement {
                                            crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                                                transport_info,
                                                transport_time,
                                            } => {
                                                writeln!(s, "    /* transport_info: TransportInfo start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport_info.guid), "guid", "                ");
                                                writeln!(s, "    /* position: Vector3d start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                    ");
                                                writeln!(s, "    /* position: Vector3d end */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 1, "seat", "                ");
                                                writeln!(s, "    /* transport_info: TransportInfo end */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_time", "            ");
                                            }
                                            crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                                                transport,
                                            } => {
                                                writeln!(s, "    /* transport: TransportInfo start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport.guid), "guid", "                ");
                                                writeln!(s, "    /* position: Vector3d start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                    ");
                                                writeln!(s, "    /* position: Vector3d end */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 1, "seat", "                ");
                                                writeln!(s, "    /* transport: TransportInfo end */").unwrap();
                                            }
                                        }
                                    }

                                    if let Some(if_statement) = &flags.get_swimming() {
                                        match if_statement {
                                            crate::wrath::MovementBlock_MovementFlags_Swimming::Swimming {
                                                pitch1,
                                            } => {
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch1", "            ");
                                            }
                                            crate::wrath::MovementBlock_MovementFlags_Swimming::Flying {
                                                pitch2,
                                            } => {
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch2", "            ");
                                            }
                                            crate::wrath::MovementBlock_MovementFlags_Swimming::AlwaysAllowPitching {
                                                pitch3,
                                            } => {
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch3", "            ");
                                            }
                                        }
                                    }

                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "fall_time", "            ");
                                    if let Some(if_statement) = &flags.get_falling() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z_speed", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "cos_angle", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "sin_angle", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "xy_speed", "            ");
                                    }

                                    if let Some(if_statement) = &flags.get_spline_elevation() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_elevation", "            ");
                                    }

                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "walking_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "running_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "backwards_running_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "swimming_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "backwards_swimming_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "flight_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "backwards_flight_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "turn_rate", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch_rate", "            ");
                                    if let Some(if_statement) = &flags.get_spline_enabled() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                        if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                            match if_statement {
                                                crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                    angle,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "angle", "            ");
                                                }
                                                crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                    target,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "            ");
                                                }
                                                crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
                                                    spline_final_point,
                                                } => {
                                                    writeln!(s, "    /* spline_final_point: Vector3d start */").unwrap();
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                                    writeln!(s, "    /* spline_final_point: Vector3d end */").unwrap();
                                                }
                                            }
                                        }

                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_passed", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_nodes", "            ");
                                        if !if_statement.nodes.is_empty() {
                                            writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] start */").unwrap();
                                            for (i, v) in if_statement.nodes.iter().enumerate() {
                                                writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] {i} start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                                writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] {i} end */").unwrap();
                                            }
                                            writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] end */").unwrap();
                                        }
                                        writeln!(s, "    /* final_node: Vector3d start */").unwrap();
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                        writeln!(s, "    /* final_node: Vector3d end */").unwrap();
                                    }

                                }
                                crate::wrath::MovementBlock_UpdateFlag_Living::Position {
                                    corpse_orientation,
                                    orientation1,
                                    position1,
                                    transport_guid,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport_guid), "transport_guid", "            ");
                                    writeln!(s, "    /* position1: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position1: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation1", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "corpse_orientation", "            ");
                                }
                                crate::wrath::MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation2,
                                    position2,
                                } => {
                                    writeln!(s, "    /* position2: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position2: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation2", "            ");
                                }
                            }
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_high_guid() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown0", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_low_guid() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_has_attacking_target() {
                            crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.guid), "guid", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_transport() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_progress_in_ms", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_vehicle() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "vehicle_id", "            ");
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "vehicle_orientation", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_rotation() {
                            crate::util::write_bytes(&mut s, &mut bytes, 8, "packed_local_rotation", "            ");
                        }

                        writeln!(s, "    /* movement2: MovementBlock end */").unwrap();
                        panic!("unsupported type UpdateMask for variable 'mask2'");
                    }
                    crate::wrath::Object_UpdateType::CreateObject2 {
                        guid3,
                        mask2,
                        movement2,
                        object_type,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid3), "guid3", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "object_type", "        ");
                        writeln!(s, "    /* movement2: MovementBlock start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 2, "update_flag", "            ");
                        if let Some(if_statement) = &movement2.update_flag.get_living() {
                            match if_statement {
                                crate::wrath::MovementBlock_UpdateFlag_Living::Living {
                                    backwards_flight_speed,
                                    backwards_running_speed,
                                    backwards_swimming_speed,
                                    fall_time,
                                    flags,
                                    flight_speed,
                                    orientation,
                                    pitch_rate,
                                    position,
                                    running_speed,
                                    swimming_speed,
                                    timestamp,
                                    turn_rate,
                                    walking_speed,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, 6, "flags", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "            ");
                                    writeln!(s, "    /* position: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "            ");
                                    if let Some(if_statement) = &flags.get_on_transport_and_interpolated_movement() {
                                        match if_statement {
                                            crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                                                transport_info,
                                                transport_time,
                                            } => {
                                                writeln!(s, "    /* transport_info: TransportInfo start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport_info.guid), "guid", "                ");
                                                writeln!(s, "    /* position: Vector3d start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                    ");
                                                writeln!(s, "    /* position: Vector3d end */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 1, "seat", "                ");
                                                writeln!(s, "    /* transport_info: TransportInfo end */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_time", "            ");
                                            }
                                            crate::wrath::MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                                                transport,
                                            } => {
                                                writeln!(s, "    /* transport: TransportInfo start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport.guid), "guid", "                ");
                                                writeln!(s, "    /* position: Vector3d start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                    ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                    ");
                                                writeln!(s, "    /* position: Vector3d end */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 1, "seat", "                ");
                                                writeln!(s, "    /* transport: TransportInfo end */").unwrap();
                                            }
                                        }
                                    }

                                    if let Some(if_statement) = &flags.get_swimming() {
                                        match if_statement {
                                            crate::wrath::MovementBlock_MovementFlags_Swimming::Swimming {
                                                pitch1,
                                            } => {
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch1", "            ");
                                            }
                                            crate::wrath::MovementBlock_MovementFlags_Swimming::Flying {
                                                pitch2,
                                            } => {
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch2", "            ");
                                            }
                                            crate::wrath::MovementBlock_MovementFlags_Swimming::AlwaysAllowPitching {
                                                pitch3,
                                            } => {
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch3", "            ");
                                            }
                                        }
                                    }

                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "fall_time", "            ");
                                    if let Some(if_statement) = &flags.get_falling() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z_speed", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "cos_angle", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "sin_angle", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "xy_speed", "            ");
                                    }

                                    if let Some(if_statement) = &flags.get_spline_elevation() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_elevation", "            ");
                                    }

                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "walking_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "running_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "backwards_running_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "swimming_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "backwards_swimming_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "flight_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "backwards_flight_speed", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "turn_rate", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch_rate", "            ");
                                    if let Some(if_statement) = &flags.get_spline_enabled() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                        if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                            match if_statement {
                                                crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                    angle,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "angle", "            ");
                                                }
                                                crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                    target,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "            ");
                                                }
                                                crate::wrath::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
                                                    spline_final_point,
                                                } => {
                                                    writeln!(s, "    /* spline_final_point: Vector3d start */").unwrap();
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                                    writeln!(s, "    /* spline_final_point: Vector3d end */").unwrap();
                                                }
                                            }
                                        }

                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_passed", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "            ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_nodes", "            ");
                                        if !if_statement.nodes.is_empty() {
                                            writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] start */").unwrap();
                                            for (i, v) in if_statement.nodes.iter().enumerate() {
                                                writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] {i} start */").unwrap();
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                                crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                                writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] {i} end */").unwrap();
                                            }
                                            writeln!(s, "    /* nodes: Vector3d[amount_of_nodes] end */").unwrap();
                                        }
                                        writeln!(s, "    /* final_node: Vector3d start */").unwrap();
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                        writeln!(s, "    /* final_node: Vector3d end */").unwrap();
                                    }

                                }
                                crate::wrath::MovementBlock_UpdateFlag_Living::Position {
                                    corpse_orientation,
                                    orientation1,
                                    position1,
                                    transport_guid,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport_guid), "transport_guid", "            ");
                                    writeln!(s, "    /* position1: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position1: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation1", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "corpse_orientation", "            ");
                                }
                                crate::wrath::MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation2,
                                    position2,
                                } => {
                                    writeln!(s, "    /* position2: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position2: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation2", "            ");
                                }
                            }
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_high_guid() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown0", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_low_guid() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_has_attacking_target() {
                            crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.guid), "guid", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_transport() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_progress_in_ms", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_vehicle() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "vehicle_id", "            ");
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "vehicle_orientation", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_rotation() {
                            crate::util::write_bytes(&mut s, &mut bytes, 8, "packed_local_rotation", "            ");
                        }

                        writeln!(s, "    /* movement2: MovementBlock end */").unwrap();
                        panic!("unsupported type UpdateMask for variable 'mask2'");
                    }
                    crate::wrath::Object_UpdateType::OutOfRangeObjects {
                        guids,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "count", "        ");
                        if !guids.is_empty() {
                            writeln!(s, "    /* guids: PackedGuid[count] start */").unwrap();
                            for (i, v) in guids.iter().enumerate() {
                                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(v), &format!("guids {i}"), "        ");
                            }
                            writeln!(s, "    /* guids: PackedGuid[count] end */").unwrap();
                        }
                    }
                    crate::wrath::Object_UpdateType::NearObjects {
                        guids,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "count", "        ");
                        if !guids.is_empty() {
                            writeln!(s, "    /* guids: PackedGuid[count] start */").unwrap();
                            for (i, v) in guids.iter().enumerate() {
                                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(v), &format!("guids {i}"), "        ");
                            }
                            writeln!(s, "    /* guids: PackedGuid[count] end */").unwrap();
                        }
                    }
                }

                writeln!(s, "    /* objects: Object[amount_of_objects] {i} end */").unwrap();
            }
            writeln!(s, "    /* objects: Object[amount_of_objects] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_objects: u32
        w.write_all(&(self.objects.len() as u32).to_le_bytes())?;

        // objects: Object[amount_of_objects]
        for i in self.objects.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(169, "SMSG_UPDATE_OBJECT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_OBJECT {}

impl SMSG_UPDATE_OBJECT {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_objects: u32
        + self.objects.iter().fold(0, |acc, x| acc + x.size()) // objects: Object[amount_of_objects]
    }
}

