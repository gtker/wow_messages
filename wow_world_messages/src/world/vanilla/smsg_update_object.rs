use std::io::{Read, Write};

use crate::vanilla::Object;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm:189`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm#L189):
/// ```text
/// smsg SMSG_UPDATE_OBJECT = 0x00A9 {
///     u32 amount_of_objects;
///     u8 has_transport;
///     Object[amount_of_objects] objects;
/// }
/// ```
pub struct SMSG_UPDATE_OBJECT {
    pub has_transport: u8,
    pub objects: Vec<Object>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_UPDATE_OBJECT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_OBJECT {{").unwrap();
        // Members
        writeln!(s, "    amount_of_objects = {};", self.objects.len()).unwrap();
        writeln!(s, "    has_transport = {};", self.has_transport).unwrap();
        write!(s, "    objects = [").unwrap();
        for v in self.objects.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        update_type = {};", crate::vanilla::UpdateType::try_from(v.update_type.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.update_type {
                crate::vanilla::Object_UpdateType::Values {
                    guid1,
                    mask1,
                } => {
                    writeln!(s, "        guid1 = {};", guid1.guid()).unwrap();
                    return None;
                }
                crate::vanilla::Object_UpdateType::Movement {
                    guid2,
                    movement1,
                } => {
                    writeln!(s, "        guid2 = {};", guid2.guid()).unwrap();
                    // movement1: MovementBlock
                    writeln!(s, "        movement1 = {{").unwrap();
                    // Members
                    writeln!(s, "            update_flag = {};", crate::vanilla::UpdateFlag::new(movement1.update_flag.as_int()).as_test_case_value()).unwrap();
                    if let Some(if_statement) = &movement1.update_flag.get_living() {
                        match if_statement {
                            crate::vanilla::MovementBlock_UpdateFlag_Living::Living {
                                backwards_running_speed,
                                backwards_swimming_speed,
                                fall_time,
                                flags,
                                living_orientation,
                                living_position,
                                running_speed,
                                swimming_speed,
                                timestamp,
                                turn_rate,
                                walking_speed,
                            } => {
                                writeln!(s, "            flags = {};", crate::vanilla::MovementFlags::new(flags.as_int()).as_test_case_value()).unwrap();
                                writeln!(s, "            timestamp = {};", timestamp).unwrap();
                                // living_position: Vector3d
                                writeln!(s, "            living_position = {{").unwrap();
                                // Members
                                writeln!(s, "    {}", if living_position.x.to_string().contains(".") { living_position.x.to_string() } else { format!("{}.0", living_position.x) }).unwrap();
                                writeln!(s, "    {}", if living_position.y.to_string().contains(".") { living_position.y.to_string() } else { format!("{}.0", living_position.y) }).unwrap();
                                writeln!(s, "    {}", if living_position.z.to_string().contains(".") { living_position.z.to_string() } else { format!("{}.0", living_position.z) }).unwrap();

                                writeln!(s, "    }};").unwrap();
                                writeln!(s, "    {}", if living_orientation.to_string().contains(".") { living_orientation.to_string() } else { format!("{}.0", living_orientation) }).unwrap();
                                if let Some(if_statement) = &flags.get_on_transport() {
                                    writeln!(s, "            transport_guid = {};", if_statement.transport_guid.guid()).unwrap();
                                    // transport_position: Vector3d
                                    writeln!(s, "            transport_position = {{").unwrap();
                                    // Members
                                    writeln!(s, "    {}", if if_statement.transport_position.x.to_string().contains(".") { if_statement.transport_position.x.to_string() } else { format!("{}.0", if_statement.transport_position.x) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.transport_position.y.to_string().contains(".") { if_statement.transport_position.y.to_string() } else { format!("{}.0", if_statement.transport_position.y) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.transport_position.z.to_string().contains(".") { if_statement.transport_position.z.to_string() } else { format!("{}.0", if_statement.transport_position.z) }).unwrap();

                                    writeln!(s, "    }};").unwrap();
                                    writeln!(s, "    {}", if if_statement.transport_orientation.to_string().contains(".") { if_statement.transport_orientation.to_string() } else { format!("{}.0", if_statement.transport_orientation) }).unwrap();
                                }

                                if let Some(if_statement) = &flags.get_swimming() {
                                    writeln!(s, "    {}", if if_statement.pitch.to_string().contains(".") { if_statement.pitch.to_string() } else { format!("{}.0", if_statement.pitch) }).unwrap();
                                }

                                writeln!(s, "    {}", if fall_time.to_string().contains(".") { fall_time.to_string() } else { format!("{}.0", fall_time) }).unwrap();
                                if let Some(if_statement) = &flags.get_jumping() {
                                    writeln!(s, "    {}", if if_statement.z_speed.to_string().contains(".") { if_statement.z_speed.to_string() } else { format!("{}.0", if_statement.z_speed) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.cos_angle.to_string().contains(".") { if_statement.cos_angle.to_string() } else { format!("{}.0", if_statement.cos_angle) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.sin_angle.to_string().contains(".") { if_statement.sin_angle.to_string() } else { format!("{}.0", if_statement.sin_angle) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.xy_speed.to_string().contains(".") { if_statement.xy_speed.to_string() } else { format!("{}.0", if_statement.xy_speed) }).unwrap();
                                }

                                if let Some(if_statement) = &flags.get_spline_elevation() {
                                    writeln!(s, "    {}", if if_statement.spline_elevation.to_string().contains(".") { if_statement.spline_elevation.to_string() } else { format!("{}.0", if_statement.spline_elevation) }).unwrap();
                                }

                                writeln!(s, "    {}", if walking_speed.to_string().contains(".") { walking_speed.to_string() } else { format!("{}.0", walking_speed) }).unwrap();
                                writeln!(s, "    {}", if running_speed.to_string().contains(".") { running_speed.to_string() } else { format!("{}.0", running_speed) }).unwrap();
                                writeln!(s, "    {}", if backwards_running_speed.to_string().contains(".") { backwards_running_speed.to_string() } else { format!("{}.0", backwards_running_speed) }).unwrap();
                                writeln!(s, "    {}", if swimming_speed.to_string().contains(".") { swimming_speed.to_string() } else { format!("{}.0", swimming_speed) }).unwrap();
                                writeln!(s, "    {}", if backwards_swimming_speed.to_string().contains(".") { backwards_swimming_speed.to_string() } else { format!("{}.0", backwards_swimming_speed) }).unwrap();
                                writeln!(s, "    {}", if turn_rate.to_string().contains(".") { turn_rate.to_string() } else { format!("{}.0", turn_rate) }).unwrap();
                                if let Some(if_statement) = &flags.get_spline_enabled() {
                                    writeln!(s, "            spline_flags = {};", crate::vanilla::SplineFlag::new(if_statement.spline_flags.as_int()).as_test_case_value()).unwrap();
                                    if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                        match if_statement {
                                            crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                angle,
                                            } => {
                                                writeln!(s, "    {}", if angle.to_string().contains(".") { angle.to_string() } else { format!("{}.0", angle) }).unwrap();
                                            }
                                            crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                target,
                                            } => {
                                                writeln!(s, "            target = {};", target).unwrap();
                                            }
                                            crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
                                                spline_final_point,
                                            } => {
                                                // spline_final_point: Vector3d
                                                writeln!(s, "            spline_final_point = {{").unwrap();
                                                // Members
                                                writeln!(s, "    {}", if spline_final_point.x.to_string().contains(".") { spline_final_point.x.to_string() } else { format!("{}.0", spline_final_point.x) }).unwrap();
                                                writeln!(s, "    {}", if spline_final_point.y.to_string().contains(".") { spline_final_point.y.to_string() } else { format!("{}.0", spline_final_point.y) }).unwrap();
                                                writeln!(s, "    {}", if spline_final_point.z.to_string().contains(".") { spline_final_point.z.to_string() } else { format!("{}.0", spline_final_point.z) }).unwrap();

                                                writeln!(s, "    }};").unwrap();
                                            }
                                        }
                                    }

                                    writeln!(s, "            time_passed = {};", if_statement.time_passed).unwrap();
                                    writeln!(s, "            duration = {};", if_statement.duration).unwrap();
                                    writeln!(s, "            id = {};", if_statement.id).unwrap();
                                    writeln!(s, "            amount_of_nodes = {};", if_statement.nodes.len()).unwrap();
                                    write!(s, "            nodes = [").unwrap();
                                    for v in if_statement.nodes.as_slice() {
                                        writeln!(s, "{{").unwrap();
                                        // Members
                                        writeln!(s, "    {}", if v.x.to_string().contains(".") { v.x.to_string() } else { format!("{}.0", v.x) }).unwrap();
                                        writeln!(s, "    {}", if v.y.to_string().contains(".") { v.y.to_string() } else { format!("{}.0", v.y) }).unwrap();
                                        writeln!(s, "    {}", if v.z.to_string().contains(".") { v.z.to_string() } else { format!("{}.0", v.z) }).unwrap();

                                        writeln!(s, "    }},").unwrap();
                                    }
                                    writeln!(s, "];").unwrap();
                                    // final_node: Vector3d
                                    writeln!(s, "            final_node = {{").unwrap();
                                    // Members
                                    writeln!(s, "    {}", if if_statement.final_node.x.to_string().contains(".") { if_statement.final_node.x.to_string() } else { format!("{}.0", if_statement.final_node.x) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.final_node.y.to_string().contains(".") { if_statement.final_node.y.to_string() } else { format!("{}.0", if_statement.final_node.y) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.final_node.z.to_string().contains(".") { if_statement.final_node.z.to_string() } else { format!("{}.0", if_statement.final_node.z) }).unwrap();

                                    writeln!(s, "    }};").unwrap();
                                }

                            }
                            crate::vanilla::MovementBlock_UpdateFlag_Living::HasPosition {
                                orientation,
                                position,
                            } => {
                                // position: Vector3d
                                writeln!(s, "            position = {{").unwrap();
                                // Members
                                writeln!(s, "    {}", if position.x.to_string().contains(".") { position.x.to_string() } else { format!("{}.0", position.x) }).unwrap();
                                writeln!(s, "    {}", if position.y.to_string().contains(".") { position.y.to_string() } else { format!("{}.0", position.y) }).unwrap();
                                writeln!(s, "    {}", if position.z.to_string().contains(".") { position.z.to_string() } else { format!("{}.0", position.z) }).unwrap();

                                writeln!(s, "    }};").unwrap();
                                writeln!(s, "    {}", if orientation.to_string().contains(".") { orientation.to_string() } else { format!("{}.0", orientation) }).unwrap();
                            }
                        }
                    }

                    if let Some(if_statement) = &movement1.update_flag.get_high_guid() {
                        writeln!(s, "            unknown0 = {};", if_statement.unknown0).unwrap();
                    }

                    if let Some(if_statement) = &movement1.update_flag.get_all() {
                        writeln!(s, "            unknown1 = {};", if_statement.unknown1).unwrap();
                    }

                    if let Some(if_statement) = &movement1.update_flag.get_melee_attacking() {
                        writeln!(s, "            guid = {};", if_statement.guid.guid()).unwrap();
                    }

                    if let Some(if_statement) = &movement1.update_flag.get_transport() {
                        writeln!(s, "            transport_progress_in_ms = {};", if_statement.transport_progress_in_ms).unwrap();
                    }


                    writeln!(s, "    }};").unwrap();
                }
                crate::vanilla::Object_UpdateType::CreateObject {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                } => {
                    writeln!(s, "        guid3 = {};", guid3.guid()).unwrap();
                    writeln!(s, "        object_type = {};", object_type.as_test_case_value()).unwrap();
                    // movement2: MovementBlock
                    writeln!(s, "        movement2 = {{").unwrap();
                    // Members
                    writeln!(s, "            update_flag = {};", crate::vanilla::UpdateFlag::new(movement2.update_flag.as_int()).as_test_case_value()).unwrap();
                    if let Some(if_statement) = &movement2.update_flag.get_living() {
                        match if_statement {
                            crate::vanilla::MovementBlock_UpdateFlag_Living::Living {
                                backwards_running_speed,
                                backwards_swimming_speed,
                                fall_time,
                                flags,
                                living_orientation,
                                living_position,
                                running_speed,
                                swimming_speed,
                                timestamp,
                                turn_rate,
                                walking_speed,
                            } => {
                                writeln!(s, "            flags = {};", crate::vanilla::MovementFlags::new(flags.as_int()).as_test_case_value()).unwrap();
                                writeln!(s, "            timestamp = {};", timestamp).unwrap();
                                // living_position: Vector3d
                                writeln!(s, "            living_position = {{").unwrap();
                                // Members
                                writeln!(s, "    {}", if living_position.x.to_string().contains(".") { living_position.x.to_string() } else { format!("{}.0", living_position.x) }).unwrap();
                                writeln!(s, "    {}", if living_position.y.to_string().contains(".") { living_position.y.to_string() } else { format!("{}.0", living_position.y) }).unwrap();
                                writeln!(s, "    {}", if living_position.z.to_string().contains(".") { living_position.z.to_string() } else { format!("{}.0", living_position.z) }).unwrap();

                                writeln!(s, "    }};").unwrap();
                                writeln!(s, "    {}", if living_orientation.to_string().contains(".") { living_orientation.to_string() } else { format!("{}.0", living_orientation) }).unwrap();
                                if let Some(if_statement) = &flags.get_on_transport() {
                                    writeln!(s, "            transport_guid = {};", if_statement.transport_guid.guid()).unwrap();
                                    // transport_position: Vector3d
                                    writeln!(s, "            transport_position = {{").unwrap();
                                    // Members
                                    writeln!(s, "    {}", if if_statement.transport_position.x.to_string().contains(".") { if_statement.transport_position.x.to_string() } else { format!("{}.0", if_statement.transport_position.x) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.transport_position.y.to_string().contains(".") { if_statement.transport_position.y.to_string() } else { format!("{}.0", if_statement.transport_position.y) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.transport_position.z.to_string().contains(".") { if_statement.transport_position.z.to_string() } else { format!("{}.0", if_statement.transport_position.z) }).unwrap();

                                    writeln!(s, "    }};").unwrap();
                                    writeln!(s, "    {}", if if_statement.transport_orientation.to_string().contains(".") { if_statement.transport_orientation.to_string() } else { format!("{}.0", if_statement.transport_orientation) }).unwrap();
                                }

                                if let Some(if_statement) = &flags.get_swimming() {
                                    writeln!(s, "    {}", if if_statement.pitch.to_string().contains(".") { if_statement.pitch.to_string() } else { format!("{}.0", if_statement.pitch) }).unwrap();
                                }

                                writeln!(s, "    {}", if fall_time.to_string().contains(".") { fall_time.to_string() } else { format!("{}.0", fall_time) }).unwrap();
                                if let Some(if_statement) = &flags.get_jumping() {
                                    writeln!(s, "    {}", if if_statement.z_speed.to_string().contains(".") { if_statement.z_speed.to_string() } else { format!("{}.0", if_statement.z_speed) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.cos_angle.to_string().contains(".") { if_statement.cos_angle.to_string() } else { format!("{}.0", if_statement.cos_angle) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.sin_angle.to_string().contains(".") { if_statement.sin_angle.to_string() } else { format!("{}.0", if_statement.sin_angle) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.xy_speed.to_string().contains(".") { if_statement.xy_speed.to_string() } else { format!("{}.0", if_statement.xy_speed) }).unwrap();
                                }

                                if let Some(if_statement) = &flags.get_spline_elevation() {
                                    writeln!(s, "    {}", if if_statement.spline_elevation.to_string().contains(".") { if_statement.spline_elevation.to_string() } else { format!("{}.0", if_statement.spline_elevation) }).unwrap();
                                }

                                writeln!(s, "    {}", if walking_speed.to_string().contains(".") { walking_speed.to_string() } else { format!("{}.0", walking_speed) }).unwrap();
                                writeln!(s, "    {}", if running_speed.to_string().contains(".") { running_speed.to_string() } else { format!("{}.0", running_speed) }).unwrap();
                                writeln!(s, "    {}", if backwards_running_speed.to_string().contains(".") { backwards_running_speed.to_string() } else { format!("{}.0", backwards_running_speed) }).unwrap();
                                writeln!(s, "    {}", if swimming_speed.to_string().contains(".") { swimming_speed.to_string() } else { format!("{}.0", swimming_speed) }).unwrap();
                                writeln!(s, "    {}", if backwards_swimming_speed.to_string().contains(".") { backwards_swimming_speed.to_string() } else { format!("{}.0", backwards_swimming_speed) }).unwrap();
                                writeln!(s, "    {}", if turn_rate.to_string().contains(".") { turn_rate.to_string() } else { format!("{}.0", turn_rate) }).unwrap();
                                if let Some(if_statement) = &flags.get_spline_enabled() {
                                    writeln!(s, "            spline_flags = {};", crate::vanilla::SplineFlag::new(if_statement.spline_flags.as_int()).as_test_case_value()).unwrap();
                                    if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                        match if_statement {
                                            crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                angle,
                                            } => {
                                                writeln!(s, "    {}", if angle.to_string().contains(".") { angle.to_string() } else { format!("{}.0", angle) }).unwrap();
                                            }
                                            crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                target,
                                            } => {
                                                writeln!(s, "            target = {};", target).unwrap();
                                            }
                                            crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
                                                spline_final_point,
                                            } => {
                                                // spline_final_point: Vector3d
                                                writeln!(s, "            spline_final_point = {{").unwrap();
                                                // Members
                                                writeln!(s, "    {}", if spline_final_point.x.to_string().contains(".") { spline_final_point.x.to_string() } else { format!("{}.0", spline_final_point.x) }).unwrap();
                                                writeln!(s, "    {}", if spline_final_point.y.to_string().contains(".") { spline_final_point.y.to_string() } else { format!("{}.0", spline_final_point.y) }).unwrap();
                                                writeln!(s, "    {}", if spline_final_point.z.to_string().contains(".") { spline_final_point.z.to_string() } else { format!("{}.0", spline_final_point.z) }).unwrap();

                                                writeln!(s, "    }};").unwrap();
                                            }
                                        }
                                    }

                                    writeln!(s, "            time_passed = {};", if_statement.time_passed).unwrap();
                                    writeln!(s, "            duration = {};", if_statement.duration).unwrap();
                                    writeln!(s, "            id = {};", if_statement.id).unwrap();
                                    writeln!(s, "            amount_of_nodes = {};", if_statement.nodes.len()).unwrap();
                                    write!(s, "            nodes = [").unwrap();
                                    for v in if_statement.nodes.as_slice() {
                                        writeln!(s, "{{").unwrap();
                                        // Members
                                        writeln!(s, "    {}", if v.x.to_string().contains(".") { v.x.to_string() } else { format!("{}.0", v.x) }).unwrap();
                                        writeln!(s, "    {}", if v.y.to_string().contains(".") { v.y.to_string() } else { format!("{}.0", v.y) }).unwrap();
                                        writeln!(s, "    {}", if v.z.to_string().contains(".") { v.z.to_string() } else { format!("{}.0", v.z) }).unwrap();

                                        writeln!(s, "    }},").unwrap();
                                    }
                                    writeln!(s, "];").unwrap();
                                    // final_node: Vector3d
                                    writeln!(s, "            final_node = {{").unwrap();
                                    // Members
                                    writeln!(s, "    {}", if if_statement.final_node.x.to_string().contains(".") { if_statement.final_node.x.to_string() } else { format!("{}.0", if_statement.final_node.x) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.final_node.y.to_string().contains(".") { if_statement.final_node.y.to_string() } else { format!("{}.0", if_statement.final_node.y) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.final_node.z.to_string().contains(".") { if_statement.final_node.z.to_string() } else { format!("{}.0", if_statement.final_node.z) }).unwrap();

                                    writeln!(s, "    }};").unwrap();
                                }

                            }
                            crate::vanilla::MovementBlock_UpdateFlag_Living::HasPosition {
                                orientation,
                                position,
                            } => {
                                // position: Vector3d
                                writeln!(s, "            position = {{").unwrap();
                                // Members
                                writeln!(s, "    {}", if position.x.to_string().contains(".") { position.x.to_string() } else { format!("{}.0", position.x) }).unwrap();
                                writeln!(s, "    {}", if position.y.to_string().contains(".") { position.y.to_string() } else { format!("{}.0", position.y) }).unwrap();
                                writeln!(s, "    {}", if position.z.to_string().contains(".") { position.z.to_string() } else { format!("{}.0", position.z) }).unwrap();

                                writeln!(s, "    }};").unwrap();
                                writeln!(s, "    {}", if orientation.to_string().contains(".") { orientation.to_string() } else { format!("{}.0", orientation) }).unwrap();
                            }
                        }
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_high_guid() {
                        writeln!(s, "            unknown0 = {};", if_statement.unknown0).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_all() {
                        writeln!(s, "            unknown1 = {};", if_statement.unknown1).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_melee_attacking() {
                        writeln!(s, "            guid = {};", if_statement.guid.guid()).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_transport() {
                        writeln!(s, "            transport_progress_in_ms = {};", if_statement.transport_progress_in_ms).unwrap();
                    }


                    writeln!(s, "    }};").unwrap();
                    return None;
                }
                crate::vanilla::Object_UpdateType::CreateObject2 {
                    guid3,
                    mask2,
                    movement2,
                    object_type,
                } => {
                    writeln!(s, "        guid3 = {};", guid3.guid()).unwrap();
                    writeln!(s, "        object_type = {};", object_type.as_test_case_value()).unwrap();
                    // movement2: MovementBlock
                    writeln!(s, "        movement2 = {{").unwrap();
                    // Members
                    writeln!(s, "            update_flag = {};", crate::vanilla::UpdateFlag::new(movement2.update_flag.as_int()).as_test_case_value()).unwrap();
                    if let Some(if_statement) = &movement2.update_flag.get_living() {
                        match if_statement {
                            crate::vanilla::MovementBlock_UpdateFlag_Living::Living {
                                backwards_running_speed,
                                backwards_swimming_speed,
                                fall_time,
                                flags,
                                living_orientation,
                                living_position,
                                running_speed,
                                swimming_speed,
                                timestamp,
                                turn_rate,
                                walking_speed,
                            } => {
                                writeln!(s, "            flags = {};", crate::vanilla::MovementFlags::new(flags.as_int()).as_test_case_value()).unwrap();
                                writeln!(s, "            timestamp = {};", timestamp).unwrap();
                                // living_position: Vector3d
                                writeln!(s, "            living_position = {{").unwrap();
                                // Members
                                writeln!(s, "    {}", if living_position.x.to_string().contains(".") { living_position.x.to_string() } else { format!("{}.0", living_position.x) }).unwrap();
                                writeln!(s, "    {}", if living_position.y.to_string().contains(".") { living_position.y.to_string() } else { format!("{}.0", living_position.y) }).unwrap();
                                writeln!(s, "    {}", if living_position.z.to_string().contains(".") { living_position.z.to_string() } else { format!("{}.0", living_position.z) }).unwrap();

                                writeln!(s, "    }};").unwrap();
                                writeln!(s, "    {}", if living_orientation.to_string().contains(".") { living_orientation.to_string() } else { format!("{}.0", living_orientation) }).unwrap();
                                if let Some(if_statement) = &flags.get_on_transport() {
                                    writeln!(s, "            transport_guid = {};", if_statement.transport_guid.guid()).unwrap();
                                    // transport_position: Vector3d
                                    writeln!(s, "            transport_position = {{").unwrap();
                                    // Members
                                    writeln!(s, "    {}", if if_statement.transport_position.x.to_string().contains(".") { if_statement.transport_position.x.to_string() } else { format!("{}.0", if_statement.transport_position.x) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.transport_position.y.to_string().contains(".") { if_statement.transport_position.y.to_string() } else { format!("{}.0", if_statement.transport_position.y) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.transport_position.z.to_string().contains(".") { if_statement.transport_position.z.to_string() } else { format!("{}.0", if_statement.transport_position.z) }).unwrap();

                                    writeln!(s, "    }};").unwrap();
                                    writeln!(s, "    {}", if if_statement.transport_orientation.to_string().contains(".") { if_statement.transport_orientation.to_string() } else { format!("{}.0", if_statement.transport_orientation) }).unwrap();
                                }

                                if let Some(if_statement) = &flags.get_swimming() {
                                    writeln!(s, "    {}", if if_statement.pitch.to_string().contains(".") { if_statement.pitch.to_string() } else { format!("{}.0", if_statement.pitch) }).unwrap();
                                }

                                writeln!(s, "    {}", if fall_time.to_string().contains(".") { fall_time.to_string() } else { format!("{}.0", fall_time) }).unwrap();
                                if let Some(if_statement) = &flags.get_jumping() {
                                    writeln!(s, "    {}", if if_statement.z_speed.to_string().contains(".") { if_statement.z_speed.to_string() } else { format!("{}.0", if_statement.z_speed) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.cos_angle.to_string().contains(".") { if_statement.cos_angle.to_string() } else { format!("{}.0", if_statement.cos_angle) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.sin_angle.to_string().contains(".") { if_statement.sin_angle.to_string() } else { format!("{}.0", if_statement.sin_angle) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.xy_speed.to_string().contains(".") { if_statement.xy_speed.to_string() } else { format!("{}.0", if_statement.xy_speed) }).unwrap();
                                }

                                if let Some(if_statement) = &flags.get_spline_elevation() {
                                    writeln!(s, "    {}", if if_statement.spline_elevation.to_string().contains(".") { if_statement.spline_elevation.to_string() } else { format!("{}.0", if_statement.spline_elevation) }).unwrap();
                                }

                                writeln!(s, "    {}", if walking_speed.to_string().contains(".") { walking_speed.to_string() } else { format!("{}.0", walking_speed) }).unwrap();
                                writeln!(s, "    {}", if running_speed.to_string().contains(".") { running_speed.to_string() } else { format!("{}.0", running_speed) }).unwrap();
                                writeln!(s, "    {}", if backwards_running_speed.to_string().contains(".") { backwards_running_speed.to_string() } else { format!("{}.0", backwards_running_speed) }).unwrap();
                                writeln!(s, "    {}", if swimming_speed.to_string().contains(".") { swimming_speed.to_string() } else { format!("{}.0", swimming_speed) }).unwrap();
                                writeln!(s, "    {}", if backwards_swimming_speed.to_string().contains(".") { backwards_swimming_speed.to_string() } else { format!("{}.0", backwards_swimming_speed) }).unwrap();
                                writeln!(s, "    {}", if turn_rate.to_string().contains(".") { turn_rate.to_string() } else { format!("{}.0", turn_rate) }).unwrap();
                                if let Some(if_statement) = &flags.get_spline_enabled() {
                                    writeln!(s, "            spline_flags = {};", crate::vanilla::SplineFlag::new(if_statement.spline_flags.as_int()).as_test_case_value()).unwrap();
                                    if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                        match if_statement {
                                            crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                angle,
                                            } => {
                                                writeln!(s, "    {}", if angle.to_string().contains(".") { angle.to_string() } else { format!("{}.0", angle) }).unwrap();
                                            }
                                            crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                target,
                                            } => {
                                                writeln!(s, "            target = {};", target).unwrap();
                                            }
                                            crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
                                                spline_final_point,
                                            } => {
                                                // spline_final_point: Vector3d
                                                writeln!(s, "            spline_final_point = {{").unwrap();
                                                // Members
                                                writeln!(s, "    {}", if spline_final_point.x.to_string().contains(".") { spline_final_point.x.to_string() } else { format!("{}.0", spline_final_point.x) }).unwrap();
                                                writeln!(s, "    {}", if spline_final_point.y.to_string().contains(".") { spline_final_point.y.to_string() } else { format!("{}.0", spline_final_point.y) }).unwrap();
                                                writeln!(s, "    {}", if spline_final_point.z.to_string().contains(".") { spline_final_point.z.to_string() } else { format!("{}.0", spline_final_point.z) }).unwrap();

                                                writeln!(s, "    }};").unwrap();
                                            }
                                        }
                                    }

                                    writeln!(s, "            time_passed = {};", if_statement.time_passed).unwrap();
                                    writeln!(s, "            duration = {};", if_statement.duration).unwrap();
                                    writeln!(s, "            id = {};", if_statement.id).unwrap();
                                    writeln!(s, "            amount_of_nodes = {};", if_statement.nodes.len()).unwrap();
                                    write!(s, "            nodes = [").unwrap();
                                    for v in if_statement.nodes.as_slice() {
                                        writeln!(s, "{{").unwrap();
                                        // Members
                                        writeln!(s, "    {}", if v.x.to_string().contains(".") { v.x.to_string() } else { format!("{}.0", v.x) }).unwrap();
                                        writeln!(s, "    {}", if v.y.to_string().contains(".") { v.y.to_string() } else { format!("{}.0", v.y) }).unwrap();
                                        writeln!(s, "    {}", if v.z.to_string().contains(".") { v.z.to_string() } else { format!("{}.0", v.z) }).unwrap();

                                        writeln!(s, "    }},").unwrap();
                                    }
                                    writeln!(s, "];").unwrap();
                                    // final_node: Vector3d
                                    writeln!(s, "            final_node = {{").unwrap();
                                    // Members
                                    writeln!(s, "    {}", if if_statement.final_node.x.to_string().contains(".") { if_statement.final_node.x.to_string() } else { format!("{}.0", if_statement.final_node.x) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.final_node.y.to_string().contains(".") { if_statement.final_node.y.to_string() } else { format!("{}.0", if_statement.final_node.y) }).unwrap();
                                    writeln!(s, "    {}", if if_statement.final_node.z.to_string().contains(".") { if_statement.final_node.z.to_string() } else { format!("{}.0", if_statement.final_node.z) }).unwrap();

                                    writeln!(s, "    }};").unwrap();
                                }

                            }
                            crate::vanilla::MovementBlock_UpdateFlag_Living::HasPosition {
                                orientation,
                                position,
                            } => {
                                // position: Vector3d
                                writeln!(s, "            position = {{").unwrap();
                                // Members
                                writeln!(s, "    {}", if position.x.to_string().contains(".") { position.x.to_string() } else { format!("{}.0", position.x) }).unwrap();
                                writeln!(s, "    {}", if position.y.to_string().contains(".") { position.y.to_string() } else { format!("{}.0", position.y) }).unwrap();
                                writeln!(s, "    {}", if position.z.to_string().contains(".") { position.z.to_string() } else { format!("{}.0", position.z) }).unwrap();

                                writeln!(s, "    }};").unwrap();
                                writeln!(s, "    {}", if orientation.to_string().contains(".") { orientation.to_string() } else { format!("{}.0", orientation) }).unwrap();
                            }
                        }
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_high_guid() {
                        writeln!(s, "            unknown0 = {};", if_statement.unknown0).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_all() {
                        writeln!(s, "            unknown1 = {};", if_statement.unknown1).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_melee_attacking() {
                        writeln!(s, "            guid = {};", if_statement.guid.guid()).unwrap();
                    }

                    if let Some(if_statement) = &movement2.update_flag.get_transport() {
                        writeln!(s, "            transport_progress_in_ms = {};", if_statement.transport_progress_in_ms).unwrap();
                    }


                    writeln!(s, "    }};").unwrap();
                    return None;
                }
                crate::vanilla::Object_UpdateType::OutOfRangeObjects {
                    guids,
                } => {
                    writeln!(s, "        count = {};", guids.len()).unwrap();
                    write!(s, "        guids = [").unwrap();
                    for v in guids.as_slice() {
                        write!(s, "{v:#08X}, ").unwrap();
                    }
                    writeln!(s, "];").unwrap();
                }
                crate::vanilla::Object_UpdateType::NearObjects {
                    guids,
                } => {
                    writeln!(s, "        count = {};", guids.len()).unwrap();
                    write!(s, "        guids = [").unwrap();
                    for v in guids.as_slice() {
                        write!(s, "{v:#08X}, ").unwrap();
                    }
                    writeln!(s, "];").unwrap();
                }
            }


            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 169_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_objects", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "has_transport", "    ");
        if !self.objects.is_empty() {
            writeln!(s, "    /* objects: Object[amount_of_objects] start */").unwrap();
            for (i, v) in self.objects.iter().enumerate() {
                writeln!(s, "    /* objects: Object[amount_of_objects] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "update_type", "        ");
                match &v.update_type {
                    crate::vanilla::Object_UpdateType::Values {
                        guid1,
                        mask1,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid1), "guid1", "        ");
                        panic!("unsupported type UpdateMask for variable 'mask1'");
                    }
                    crate::vanilla::Object_UpdateType::Movement {
                        guid2,
                        movement1,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid2), "guid2", "        ");
                        writeln!(s, "    /* movement1: MovementBlock start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "update_flag", "            ");
                        if let Some(if_statement) = &movement1.update_flag.get_living() {
                            match if_statement {
                                crate::vanilla::MovementBlock_UpdateFlag_Living::Living {
                                    backwards_running_speed,
                                    backwards_swimming_speed,
                                    fall_time,
                                    flags,
                                    living_orientation,
                                    living_position,
                                    running_speed,
                                    swimming_speed,
                                    timestamp,
                                    turn_rate,
                                    walking_speed,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "            ");
                                    writeln!(s, "    /* living_position: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* living_position: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "living_orientation", "            ");
                                    if let Some(if_statement) = &flags.get_on_transport() {
                                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.transport_guid), "transport_guid", "            ");
                                        writeln!(s, "    /* transport_position: Vector3d start */").unwrap();
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                        writeln!(s, "    /* transport_position: Vector3d end */").unwrap();
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_orientation", "            ");
                                    }

                                    if let Some(if_statement) = &flags.get_swimming() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch", "            ");
                                    }

                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "fall_time", "            ");
                                    if let Some(if_statement) = &flags.get_jumping() {
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
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "turn_rate", "            ");
                                    if let Some(if_statement) = &flags.get_spline_enabled() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                        if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                            match if_statement {
                                                crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                    angle,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "angle", "            ");
                                                }
                                                crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                    target,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "            ");
                                                }
                                                crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
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
                                crate::vanilla::MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation,
                                    position,
                                } => {
                                    writeln!(s, "    /* position: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "            ");
                                }
                            }
                        }

                        if let Some(if_statement) = &movement1.update_flag.get_high_guid() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown0", "            ");
                        }

                        if let Some(if_statement) = &movement1.update_flag.get_all() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "            ");
                        }

                        if let Some(if_statement) = &movement1.update_flag.get_melee_attacking() {
                            crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.guid), "guid", "            ");
                        }

                        if let Some(if_statement) = &movement1.update_flag.get_transport() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_progress_in_ms", "            ");
                        }

                        writeln!(s, "    /* movement1: MovementBlock end */").unwrap();
                    }
                    crate::vanilla::Object_UpdateType::CreateObject {
                        guid3,
                        mask2,
                        movement2,
                        object_type,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid3), "guid3", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "object_type", "        ");
                        writeln!(s, "    /* movement2: MovementBlock start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "update_flag", "            ");
                        if let Some(if_statement) = &movement2.update_flag.get_living() {
                            match if_statement {
                                crate::vanilla::MovementBlock_UpdateFlag_Living::Living {
                                    backwards_running_speed,
                                    backwards_swimming_speed,
                                    fall_time,
                                    flags,
                                    living_orientation,
                                    living_position,
                                    running_speed,
                                    swimming_speed,
                                    timestamp,
                                    turn_rate,
                                    walking_speed,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "            ");
                                    writeln!(s, "    /* living_position: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* living_position: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "living_orientation", "            ");
                                    if let Some(if_statement) = &flags.get_on_transport() {
                                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.transport_guid), "transport_guid", "            ");
                                        writeln!(s, "    /* transport_position: Vector3d start */").unwrap();
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                        writeln!(s, "    /* transport_position: Vector3d end */").unwrap();
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_orientation", "            ");
                                    }

                                    if let Some(if_statement) = &flags.get_swimming() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch", "            ");
                                    }

                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "fall_time", "            ");
                                    if let Some(if_statement) = &flags.get_jumping() {
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
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "turn_rate", "            ");
                                    if let Some(if_statement) = &flags.get_spline_enabled() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                        if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                            match if_statement {
                                                crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                    angle,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "angle", "            ");
                                                }
                                                crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                    target,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "            ");
                                                }
                                                crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
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
                                crate::vanilla::MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation,
                                    position,
                                } => {
                                    writeln!(s, "    /* position: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "            ");
                                }
                            }
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_high_guid() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown0", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_all() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_melee_attacking() {
                            crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.guid), "guid", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_transport() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_progress_in_ms", "            ");
                        }

                        writeln!(s, "    /* movement2: MovementBlock end */").unwrap();
                        panic!("unsupported type UpdateMask for variable 'mask2'");
                    }
                    crate::vanilla::Object_UpdateType::CreateObject2 {
                        guid3,
                        mask2,
                        movement2,
                        object_type,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid3), "guid3", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "object_type", "        ");
                        writeln!(s, "    /* movement2: MovementBlock start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "update_flag", "            ");
                        if let Some(if_statement) = &movement2.update_flag.get_living() {
                            match if_statement {
                                crate::vanilla::MovementBlock_UpdateFlag_Living::Living {
                                    backwards_running_speed,
                                    backwards_swimming_speed,
                                    fall_time,
                                    flags,
                                    living_orientation,
                                    living_position,
                                    running_speed,
                                    swimming_speed,
                                    timestamp,
                                    turn_rate,
                                    walking_speed,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "            ");
                                    writeln!(s, "    /* living_position: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* living_position: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "living_orientation", "            ");
                                    if let Some(if_statement) = &flags.get_on_transport() {
                                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.transport_guid), "transport_guid", "            ");
                                        writeln!(s, "    /* transport_position: Vector3d start */").unwrap();
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                        writeln!(s, "    /* transport_position: Vector3d end */").unwrap();
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_orientation", "            ");
                                    }

                                    if let Some(if_statement) = &flags.get_swimming() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch", "            ");
                                    }

                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "fall_time", "            ");
                                    if let Some(if_statement) = &flags.get_jumping() {
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
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "turn_rate", "            ");
                                    if let Some(if_statement) = &flags.get_spline_enabled() {
                                        crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_flags", "            ");
                                        if let Some(if_statement) = &if_statement.spline_flags.get_final_angle() {
                                            match if_statement {
                                                crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                                    angle,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "angle", "            ");
                                                }
                                                crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                                    target,
                                                } => {
                                                    crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "            ");
                                                }
                                                crate::vanilla::MovementBlock_SplineFlag_FinalAngle::FinalPoint {
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
                                crate::vanilla::MovementBlock_UpdateFlag_Living::HasPosition {
                                    orientation,
                                    position,
                                } => {
                                    writeln!(s, "    /* position: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "            ");
                                }
                            }
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_high_guid() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown0", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_all() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_melee_attacking() {
                            crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.guid), "guid", "            ");
                        }

                        if let Some(if_statement) = &movement2.update_flag.get_transport() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_progress_in_ms", "            ");
                        }

                        writeln!(s, "    /* movement2: MovementBlock end */").unwrap();
                        panic!("unsupported type UpdateMask for variable 'mask2'");
                    }
                    crate::vanilla::Object_UpdateType::OutOfRangeObjects {
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
                    crate::vanilla::Object_UpdateType::NearObjects {
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
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_UPDATE_OBJECT {}
impl crate::Message for SMSG_UPDATE_OBJECT {
    const OPCODE: u32 = 0x00a9;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_UPDATE_OBJECT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_objects: u32
        w.write_all(&(self.objects.len() as u32).to_le_bytes())?;

        // has_transport: u8
        w.write_all(&self.has_transport.to_le_bytes())?;

        // objects: Object[amount_of_objects]
        for i in self.objects.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(5..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00A9, size: body_size });
        }

        // amount_of_objects: u32
        let amount_of_objects = crate::util::read_u32_le(&mut r)?;

        // has_transport: u8
        let has_transport = crate::util::read_u8_le(&mut r)?;

        // objects: Object[amount_of_objects]
        let objects = {
            let mut objects = Vec::with_capacity(amount_of_objects as usize);
            for _ in 0..amount_of_objects {
                objects.push(Object::read(&mut r)?);
            }
            objects
        };

        Ok(Self {
            has_transport,
            objects,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_UPDATE_OBJECT {}

impl SMSG_UPDATE_OBJECT {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_objects: u32
        + 1 // has_transport: u8
        + self.objects.iter().fold(0, |acc, x| acc + x.size()) // objects: Object[amount_of_objects]
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_UPDATE_OBJECT;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_UPDATE_OBJECT, expected: &SMSG_UPDATE_OBJECT) {
        assert_eq!(t.has_transport, expected.has_transport);
        assert_eq!(t.objects, expected.objects);
    }

    const RAW0: [u8; 99] = [ 0x00, 0x61, 0xA9, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
         0x03, 0x01, 0x04, 0x04, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0xCD, 0xD7, 0x0B, 0xC6, 0x35, 0x7E, 0x04, 0xC3, 0xF9, 0x0F, 0xA7,
         0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
         0x3F, 0x00, 0x00, 0xE0, 0x40, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0xDB, 0x0F, 0x49, 0x40, 0x01, 0x00, 0x00,
         0x00, 0x02, 0x07, 0x00, 0x40, 0x00, 0x10, 0x00, 0x00, 0x00, 0x04, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x64, 0x00,
         0x00, 0x00, 0x01, 0x01, 0x01, 0x01, ];

    pub(crate) fn expected0() -> SMSG_UPDATE_OBJECT {
        SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CreateObject2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::builder()
                            .set_object_guid(Guid::new(4))
                            .set_unit_health(100)
                            .set_unit_bytes_0(1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap())
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_living(MovementBlock_UpdateFlag_Living::Living {
                                    backwards_running_speed: 4.5_f32,
                                    backwards_swimming_speed: 0_f32,
                                    fall_time: 0_f32,
                                    flags: MovementBlock_MovementFlags::empty()
                                        ,
                                    living_orientation: 0_f32,
                                    living_position: Vector3d {
                                        x: -8949.95_f32,
                                        y: -132.493_f32,
                                        z: 83.5312_f32,
                                    },
                                    running_speed: 7_f32,
                                    swimming_speed: 0_f32,
                                    timestamp: 0x0,
                                    turn_rate: 3.1415927_f32,
                                    walking_speed: 1_f32,
                                })
                                .set_all(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_self()
                                ,
                        },
                        object_type: ObjectType::Player,
                    },
                },
            ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 195.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_update_object0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 195.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_update_object0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 195.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_update_object0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 135] = [ 0x00, 0x85, 0xA9, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
         0x03, 0x01, 0x04, 0x04, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0xCD, 0xD7, 0x0B, 0xC6, 0x35, 0x7E, 0x04, 0xC3, 0xF9, 0x0F, 0xA7,
         0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
         0x3F, 0x00, 0x00, 0xE0, 0x40, 0x00, 0x00, 0x90, 0x40, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0xDB, 0x0F, 0x49, 0x40, 0x01, 0x00, 0x00,
         0x00, 0x05, 0x17, 0x00, 0x40, 0x10, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x04, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x80, 0x3F, 0x64, 0x00, 0x00, 0x00, 0x64, 0x00, 0x00, 0x00, 0x01, 0x00,
         0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x32, 0x00,
         0x00, 0x00, 0x32, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected1() -> SMSG_UPDATE_OBJECT {
        SMSG_UPDATE_OBJECT {
            has_transport: 0x0,
            objects: vec![
                Object {
                    update_type: Object_UpdateType::CreateObject2 {
                        guid3: Guid::new(0x4),
                        mask2: UpdateMask::Player(UpdatePlayer::builder()
                            .set_object_guid(Guid::new(4))
                            .set_object_scale_x(1.0)
                            .set_unit_health(100)
                            .set_unit_maxhealth(100)
                            .set_unit_level(1)
                            .set_unit_factiontemplate(1)
                            .set_unit_bytes_0(1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap(), 1.try_into().unwrap())
                            .set_unit_displayid(50)
                            .set_unit_nativedisplayid(50)
                            .finalize()
                        ),
                        movement2: MovementBlock {
                            update_flag: MovementBlock_UpdateFlag::empty()
                                .set_living(MovementBlock_UpdateFlag_Living::Living {
                                    backwards_running_speed: 4.5_f32,
                                    backwards_swimming_speed: 0_f32,
                                    fall_time: 0_f32,
                                    flags: MovementBlock_MovementFlags::empty()
                                        ,
                                    living_orientation: 0_f32,
                                    living_position: Vector3d {
                                        x: -8949.95_f32,
                                        y: -132.493_f32,
                                        z: 83.5312_f32,
                                    },
                                    running_speed: 7_f32,
                                    swimming_speed: 0_f32,
                                    timestamp: 0x0,
                                    turn_rate: 3.1415927_f32,
                                    walking_speed: 1_f32,
                                })
                                .set_all(MovementBlock_UpdateFlag_All {
                                    unknown1: 0x1,
                                })
                                .set_self()
                                ,
                        },
                        object_type: ObjectType::Player,
                    },
                },
            ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 262.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_update_object1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 262.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_update_object1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm` line 262.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_update_object1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_UPDATE_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_UPDATE_OBJECT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

