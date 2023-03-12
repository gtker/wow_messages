use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    ExtraMovementFlags, MovementFlags, SplineFlag, TransportInfo, UpdateFlag, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm:87`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm#L87):
/// ```text
/// struct MovementBlock {
///     UpdateFlag update_flag;
///     if (update_flag & LIVING) {
///         MovementFlags flags;
///         ExtraMovementFlags extra_flags;
///         u32 timestamp;
///         Vector3d living_position;
///         f32 living_orientation;
///         if (flags & ON_TRANSPORT) {
///             TransportInfo transport;
///         }
///         if (flags & SWIMMING) {
///             f32 pitch1;
///         }
///         else if (flags & FLYING) {
///             f32 pitch2;
///         }
///         f32 fall_time;
///         if (flags & FALLING) {
///             f32 z_speed;
///             f32 cos_angle;
///             f32 sin_angle;
///             f32 xy_speed;
///         }
///         if (flags & SPLINE_ELEVATION) {
///             f32 spline_elevation;
///         }
///         f32 walking_speed;
///         f32 running_speed;
///         f32 backwards_running_speed;
///         f32 swimming_speed;
///         f32 backwards_swimming_speed;
///         f32 flight_speed;
///         f32 backwards_flight_speed;
///         f32 turn_rate;
///         f32 pitch_rate;
///         if (flags & SPLINE_ENABLED) {
///             SplineFlag spline_flags;
///             if (spline_flags & FINAL_ANGLE) {
///                 f32 angle;
///             }
///             else if (spline_flags & FINAL_TARGET) {
///                 u64 target;
///             }
///             else if (spline_flags & FINAL_POINT) {
///                 Vector3d spline_final_point;
///             }
///             u32 time_passed;
///             u32 duration;
///             u32 id;
///             u32 amount_of_nodes;
///             Vector3d[amount_of_nodes] nodes;
///             Vector3d final_node;
///         }
///     }
///     else if (update_flag & POSITION) {
///         PackedGuid transport_guid;
///         Vector3d position1;
///         f32 orientation1;
///         f32 corpse_orientation;
///     }
///     else if (update_flag & HAS_POSITION) {
///         Vector3d position2;
///         f32 orientation2;
///     }
///     if (update_flag & HIGH_GUID) {
///         u32 unknown0;
///     }
///     if (update_flag & LOW_GUID) {
///         u32 unknown1;
///     }
///     if (update_flag & HAS_ATTACKING_TARGET) {
///         PackedGuid guid;
///     }
///     if (update_flag & TRANSPORT) {
///         u32 transport_progress_in_ms;
///     }
///     if (update_flag & VEHICLE) {
///         u32 vehicle_id;
///         f32 vehicle_orientation;
///     }
///     if (update_flag & ROTATION) {
///         u64 packed_local_rotation;
///     }
/// }
/// ```
pub struct MovementBlock {
    pub update_flag: MovementBlock_UpdateFlag,
}

impl MovementBlock {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // update_flag: UpdateFlag
        w.write_all(&u16::from(self.update_flag.as_int()).to_le_bytes())?;

        if let Some(if_statement) = &self.update_flag.living {
            match if_statement {
                MovementBlock_UpdateFlag_Living::Living {
                    backwards_flight_speed,
                    backwards_running_speed,
                    backwards_swimming_speed,
                    extra_flags,
                    fall_time,
                    flags,
                    flight_speed,
                    living_orientation,
                    living_position,
                    pitch_rate,
                    running_speed,
                    swimming_speed,
                    timestamp,
                    turn_rate,
                    walking_speed,
                } => {
                    // flags: MovementFlags
                    w.write_all(&u32::from(flags.as_int()).to_le_bytes())?;

                    // extra_flags: ExtraMovementFlags
                    w.write_all(&u16::from(extra_flags.as_int()).to_le_bytes())?;

                    // timestamp: u32
                    w.write_all(&timestamp.to_le_bytes())?;

                    // living_position: Vector3d
                    living_position.write_into_vec(&mut w)?;

                    // living_orientation: f32
                    w.write_all(&living_orientation.to_le_bytes())?;

                    if let Some(if_statement) = &flags.on_transport {
                        // transport: TransportInfo
                        if_statement.transport.write_into_vec(&mut w)?;

                    }

                    if let Some(if_statement) = &flags.swimming {
                        match if_statement {
                            MovementBlock_MovementFlags_Swimming::Swimming {
                                pitch1,
                            } => {
                                // pitch1: f32
                                w.write_all(&pitch1.to_le_bytes())?;

                            }
                            MovementBlock_MovementFlags_Swimming::Flying {
                                pitch2,
                            } => {
                                // pitch2: f32
                                w.write_all(&pitch2.to_le_bytes())?;

                            }
                        }
                    }

                    // fall_time: f32
                    w.write_all(&fall_time.to_le_bytes())?;

                    if let Some(if_statement) = &flags.falling {
                        // z_speed: f32
                        w.write_all(&if_statement.z_speed.to_le_bytes())?;

                        // cos_angle: f32
                        w.write_all(&if_statement.cos_angle.to_le_bytes())?;

                        // sin_angle: f32
                        w.write_all(&if_statement.sin_angle.to_le_bytes())?;

                        // xy_speed: f32
                        w.write_all(&if_statement.xy_speed.to_le_bytes())?;

                    }

                    if let Some(if_statement) = &flags.spline_elevation {
                        // spline_elevation: f32
                        w.write_all(&if_statement.spline_elevation.to_le_bytes())?;

                    }

                    // walking_speed: f32
                    w.write_all(&walking_speed.to_le_bytes())?;

                    // running_speed: f32
                    w.write_all(&running_speed.to_le_bytes())?;

                    // backwards_running_speed: f32
                    w.write_all(&backwards_running_speed.to_le_bytes())?;

                    // swimming_speed: f32
                    w.write_all(&swimming_speed.to_le_bytes())?;

                    // backwards_swimming_speed: f32
                    w.write_all(&backwards_swimming_speed.to_le_bytes())?;

                    // flight_speed: f32
                    w.write_all(&flight_speed.to_le_bytes())?;

                    // backwards_flight_speed: f32
                    w.write_all(&backwards_flight_speed.to_le_bytes())?;

                    // turn_rate: f32
                    w.write_all(&turn_rate.to_le_bytes())?;

                    // pitch_rate: f32
                    w.write_all(&pitch_rate.to_le_bytes())?;

                    if let Some(if_statement) = &flags.spline_enabled {
                        // spline_flags: SplineFlag
                        w.write_all(&u32::from(if_statement.spline_flags.as_int()).to_le_bytes())?;

                        if let Some(if_statement) = &if_statement.spline_flags.final_angle {
                            match if_statement {
                                MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                                    angle,
                                } => {
                                    // angle: f32
                                    w.write_all(&angle.to_le_bytes())?;

                                }
                                MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                                    target,
                                } => {
                                    // target: u64
                                    w.write_all(&target.to_le_bytes())?;

                                }
                                MovementBlock_SplineFlag_FinalAngle::FinalPoint {
                                    spline_final_point,
                                } => {
                                    // spline_final_point: Vector3d
                                    spline_final_point.write_into_vec(&mut w)?;

                                }
                            }
                        }

                        // time_passed: u32
                        w.write_all(&if_statement.time_passed.to_le_bytes())?;

                        // duration: u32
                        w.write_all(&if_statement.duration.to_le_bytes())?;

                        // id: u32
                        w.write_all(&if_statement.id.to_le_bytes())?;

                        // amount_of_nodes: u32
                        w.write_all(&(if_statement.nodes.len() as u32).to_le_bytes())?;

                        // nodes: Vector3d[amount_of_nodes]
                        for i in if_statement.nodes.iter() {
                            i.write_into_vec(&mut w)?;
                        }

                        // final_node: Vector3d
                        if_statement.final_node.write_into_vec(&mut w)?;

                    }

                }
                MovementBlock_UpdateFlag_Living::Position {
                    corpse_orientation,
                    orientation1,
                    position1,
                    transport_guid,
                } => {
                    // transport_guid: PackedGuid
                    transport_guid.write_packed_guid_into_vec(&mut w)?;

                    // position1: Vector3d
                    position1.write_into_vec(&mut w)?;

                    // orientation1: f32
                    w.write_all(&orientation1.to_le_bytes())?;

                    // corpse_orientation: f32
                    w.write_all(&corpse_orientation.to_le_bytes())?;

                }
                MovementBlock_UpdateFlag_Living::HasPosition {
                    orientation2,
                    position2,
                } => {
                    // position2: Vector3d
                    position2.write_into_vec(&mut w)?;

                    // orientation2: f32
                    w.write_all(&orientation2.to_le_bytes())?;

                }
            }
        }

        if let Some(if_statement) = &self.update_flag.high_guid {
            // unknown0: u32
            w.write_all(&if_statement.unknown0.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.update_flag.low_guid {
            // unknown1: u32
            w.write_all(&if_statement.unknown1.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.update_flag.has_attacking_target {
            // guid: PackedGuid
            if_statement.guid.write_packed_guid_into_vec(&mut w)?;

        }

        if let Some(if_statement) = &self.update_flag.transport {
            // transport_progress_in_ms: u32
            w.write_all(&if_statement.transport_progress_in_ms.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.update_flag.vehicle {
            // vehicle_id: u32
            w.write_all(&if_statement.vehicle_id.to_le_bytes())?;

            // vehicle_orientation: f32
            w.write_all(&if_statement.vehicle_orientation.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.update_flag.rotation {
            // packed_local_rotation: u64
            w.write_all(&if_statement.packed_local_rotation.to_le_bytes())?;

        }

        Ok(())
    }
}

impl MovementBlock {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // update_flag: UpdateFlag
        let update_flag = UpdateFlag::new(crate::util::read_u16_le(&mut r)?);

        let update_flag_LIVING = if update_flag.is_LIVING() {
            // flags: MovementFlags
            let flags = MovementFlags::new(crate::util::read_u32_le(&mut r)?);

            // extra_flags: ExtraMovementFlags
            let extra_flags = ExtraMovementFlags::new(crate::util::read_u16_le(&mut r)?);

            // timestamp: u32
            let timestamp = crate::util::read_u32_le(&mut r)?;

            // living_position: Vector3d
            let living_position = Vector3d::read(&mut r)?;

            // living_orientation: f32
            let living_orientation = crate::util::read_f32_le(&mut r)?;

            let flags_ON_TRANSPORT = if flags.is_ON_TRANSPORT() {
                // transport: TransportInfo
                let transport = TransportInfo::read(&mut r)?;

                Some(MovementBlock_MovementFlags_OnTransport {
                    transport,
                })
            }
            else {
                None
            };

            let flags_SWIMMING = if flags.is_SWIMMING() {
                // pitch1: f32
                let pitch1 = crate::util::read_f32_le(&mut r)?;

                Some(MovementBlock_MovementFlags_Swimming::Swimming {
                    pitch1,
                })
            }
            else if flags.is_FLYING() {
                // pitch2: f32
                let pitch2 = crate::util::read_f32_le(&mut r)?;

                Some(MovementBlock_MovementFlags_Swimming::Flying {
                    pitch2,
                })
            }
            else {
                None
            };

            // fall_time: f32
            let fall_time = crate::util::read_f32_le(&mut r)?;

            let flags_FALLING = if flags.is_FALLING() {
                // z_speed: f32
                let z_speed = crate::util::read_f32_le(&mut r)?;

                // cos_angle: f32
                let cos_angle = crate::util::read_f32_le(&mut r)?;

                // sin_angle: f32
                let sin_angle = crate::util::read_f32_le(&mut r)?;

                // xy_speed: f32
                let xy_speed = crate::util::read_f32_le(&mut r)?;

                Some(MovementBlock_MovementFlags_Falling {
                    cos_angle,
                    sin_angle,
                    xy_speed,
                    z_speed,
                })
            }
            else {
                None
            };

            let flags_SPLINE_ELEVATION = if flags.is_SPLINE_ELEVATION() {
                // spline_elevation: f32
                let spline_elevation = crate::util::read_f32_le(&mut r)?;

                Some(MovementBlock_MovementFlags_SplineElevation {
                    spline_elevation,
                })
            }
            else {
                None
            };

            // walking_speed: f32
            let walking_speed = crate::util::read_f32_le(&mut r)?;

            // running_speed: f32
            let running_speed = crate::util::read_f32_le(&mut r)?;

            // backwards_running_speed: f32
            let backwards_running_speed = crate::util::read_f32_le(&mut r)?;

            // swimming_speed: f32
            let swimming_speed = crate::util::read_f32_le(&mut r)?;

            // backwards_swimming_speed: f32
            let backwards_swimming_speed = crate::util::read_f32_le(&mut r)?;

            // flight_speed: f32
            let flight_speed = crate::util::read_f32_le(&mut r)?;

            // backwards_flight_speed: f32
            let backwards_flight_speed = crate::util::read_f32_le(&mut r)?;

            // turn_rate: f32
            let turn_rate = crate::util::read_f32_le(&mut r)?;

            // pitch_rate: f32
            let pitch_rate = crate::util::read_f32_le(&mut r)?;

            let flags_SPLINE_ENABLED = if flags.is_SPLINE_ENABLED() {
                // spline_flags: SplineFlag
                let spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

                let spline_flags_FINAL_ANGLE = if spline_flags.is_FINAL_ANGLE() {
                    // angle: f32
                    let angle = crate::util::read_f32_le(&mut r)?;

                    Some(MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                        angle,
                    })
                }
                else if spline_flags.is_FINAL_TARGET() {
                    // target: u64
                    let target = crate::util::read_u64_le(&mut r)?;

                    Some(MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                        target,
                    })
                }
                else if spline_flags.is_FINAL_POINT() {
                    // spline_final_point: Vector3d
                    let spline_final_point = Vector3d::read(&mut r)?;

                    Some(MovementBlock_SplineFlag_FinalAngle::FinalPoint {
                        spline_final_point,
                    })
                }
                else {
                    None
                };

                // time_passed: u32
                let time_passed = crate::util::read_u32_le(&mut r)?;

                // duration: u32
                let duration = crate::util::read_u32_le(&mut r)?;

                // id: u32
                let id = crate::util::read_u32_le(&mut r)?;

                // amount_of_nodes: u32
                let amount_of_nodes = crate::util::read_u32_le(&mut r)?;

                // nodes: Vector3d[amount_of_nodes]
                let nodes = {
                    let mut nodes = Vec::with_capacity(amount_of_nodes as usize);
                    for i in 0..amount_of_nodes {
                        nodes.push(Vector3d::read(&mut r)?);
                    }
                    nodes
                };

                // final_node: Vector3d
                let final_node = Vector3d::read(&mut r)?;

                let spline_flags = MovementBlock_SplineFlag {
                    inner: spline_flags.as_int(),
                    final_angle: spline_flags_FINAL_ANGLE,
                };

                Some(MovementBlock_MovementFlags_SplineEnabled {
                    duration,
                    final_node,
                    id,
                    nodes,
                    spline_flags,
                    time_passed,
                })
            }
            else {
                None
            };

            let flags = MovementBlock_MovementFlags {
                inner: flags.as_int(),
                on_transport: flags_ON_TRANSPORT,
                falling: flags_FALLING,
                swimming: flags_SWIMMING,
                spline_elevation: flags_SPLINE_ELEVATION,
                spline_enabled: flags_SPLINE_ENABLED,
            };

            Some(MovementBlock_UpdateFlag_Living::Living {
                backwards_flight_speed,
                backwards_running_speed,
                backwards_swimming_speed,
                extra_flags,
                fall_time,
                flags,
                flight_speed,
                living_orientation,
                living_position,
                pitch_rate,
                running_speed,
                swimming_speed,
                timestamp,
                turn_rate,
                walking_speed,
            })
        }
        else if update_flag.is_POSITION() {
            // transport_guid: PackedGuid
            let transport_guid = Guid::read_packed(&mut r)?;

            // position1: Vector3d
            let position1 = Vector3d::read(&mut r)?;

            // orientation1: f32
            let orientation1 = crate::util::read_f32_le(&mut r)?;

            // corpse_orientation: f32
            let corpse_orientation = crate::util::read_f32_le(&mut r)?;

            Some(MovementBlock_UpdateFlag_Living::Position {
                corpse_orientation,
                orientation1,
                position1,
                transport_guid,
            })
        }
        else if update_flag.is_HAS_POSITION() {
            // position2: Vector3d
            let position2 = Vector3d::read(&mut r)?;

            // orientation2: f32
            let orientation2 = crate::util::read_f32_le(&mut r)?;

            Some(MovementBlock_UpdateFlag_Living::HasPosition {
                orientation2,
                position2,
            })
        }
        else {
            None
        };

        let update_flag_HIGH_GUID = if update_flag.is_HIGH_GUID() {
            // unknown0: u32
            let unknown0 = crate::util::read_u32_le(&mut r)?;

            Some(MovementBlock_UpdateFlag_HighGuid {
                unknown0,
            })
        }
        else {
            None
        };

        let update_flag_LOW_GUID = if update_flag.is_LOW_GUID() {
            // unknown1: u32
            let unknown1 = crate::util::read_u32_le(&mut r)?;

            Some(MovementBlock_UpdateFlag_LowGuid {
                unknown1,
            })
        }
        else {
            None
        };

        let update_flag_HAS_ATTACKING_TARGET = if update_flag.is_HAS_ATTACKING_TARGET() {
            // guid: PackedGuid
            let guid = Guid::read_packed(&mut r)?;

            Some(MovementBlock_UpdateFlag_HasAttackingTarget {
                guid,
            })
        }
        else {
            None
        };

        let update_flag_TRANSPORT = if update_flag.is_TRANSPORT() {
            // transport_progress_in_ms: u32
            let transport_progress_in_ms = crate::util::read_u32_le(&mut r)?;

            Some(MovementBlock_UpdateFlag_Transport {
                transport_progress_in_ms,
            })
        }
        else {
            None
        };

        let update_flag_VEHICLE = if update_flag.is_VEHICLE() {
            // vehicle_id: u32
            let vehicle_id = crate::util::read_u32_le(&mut r)?;

            // vehicle_orientation: f32
            let vehicle_orientation = crate::util::read_f32_le(&mut r)?;

            Some(MovementBlock_UpdateFlag_Vehicle {
                vehicle_id,
                vehicle_orientation,
            })
        }
        else {
            None
        };

        let update_flag_ROTATION = if update_flag.is_ROTATION() {
            // packed_local_rotation: u64
            let packed_local_rotation = crate::util::read_u64_le(&mut r)?;

            Some(MovementBlock_UpdateFlag_Rotation {
                packed_local_rotation,
            })
        }
        else {
            None
        };

        let update_flag = MovementBlock_UpdateFlag {
            inner: update_flag.as_int(),
            transport: update_flag_TRANSPORT,
            has_attacking_target: update_flag_HAS_ATTACKING_TARGET,
            low_guid: update_flag_LOW_GUID,
            high_guid: update_flag_HIGH_GUID,
            living: update_flag_LIVING,
            vehicle: update_flag_VEHICLE,
            rotation: update_flag_ROTATION,
        };

        Ok(Self {
            update_flag,
        })
    }

}

impl MovementBlock {
    pub(crate) fn size(&self) -> usize {
        self.update_flag.size() // update_flag: MovementBlock_UpdateFlag
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MovementBlock_MovementFlags_Swimming {
    Swimming {
        pitch1: f32,
    },
    Flying {
        pitch2: f32,
    },
}

impl MovementBlock_MovementFlags_Swimming {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Swimming { .. } => 2097152,
            Self::Flying { .. } => 33554432,
        }
    }

}

impl MovementBlock_MovementFlags_Swimming {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Swimming {
                pitch1,
            } => {
                // Not an actual enum sent over the wire
                4 // pitch1: f32
            }
            Self::Flying {
                pitch2,
            } => {
                // Not an actual enum sent over the wire
                4 // pitch2: f32
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MovementBlock_SplineFlag_FinalAngle {
    FinalAngle {
        angle: f32,
    },
    FinalTarget {
        target: u64,
    },
    FinalPoint {
        spline_final_point: Vector3d,
    },
}

impl MovementBlock_SplineFlag_FinalAngle {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::FinalAngle { .. } => 262144,
            Self::FinalTarget { .. } => 131072,
            Self::FinalPoint { .. } => 65536,
        }
    }

}

impl MovementBlock_SplineFlag_FinalAngle {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::FinalAngle {
                angle,
            } => {
                // Not an actual enum sent over the wire
                4 // angle: f32
            }
            Self::FinalTarget {
                target,
            } => {
                // Not an actual enum sent over the wire
                8 // target: u64
            }
            Self::FinalPoint {
                spline_final_point,
            } => {
                // Not an actual enum sent over the wire
                12 // spline_final_point: Vector3d
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MovementBlock_SplineFlag {
    inner: u32,
    final_angle: Option<MovementBlock_SplineFlag_FinalAngle>,
}

impl MovementBlock_SplineFlag {
    pub const fn new(inner: u32, final_angle: Option<MovementBlock_SplineFlag_FinalAngle>,) -> Self {
        Self {
            inner,
            final_angle, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            final_angle: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.final_angle.is_none()
    }

    pub const fn new_DONE() -> Self {
        Self {
            inner: SplineFlag::DONE,
            final_angle: None,
        }
    }

    pub fn set_DONE(mut self) -> Self {
        self.inner |= SplineFlag::DONE;
        self
    }

    pub const fn get_DONE(&self) -> bool {
        (self.inner & SplineFlag::DONE) != 0
    }

    pub fn clear_DONE(mut self) -> Self {
        self.inner &= SplineFlag::DONE.reverse_bits();
        self
    }

    pub const fn new_FALLING() -> Self {
        Self {
            inner: SplineFlag::FALLING,
            final_angle: None,
        }
    }

    pub fn set_FALLING(mut self) -> Self {
        self.inner |= SplineFlag::FALLING;
        self
    }

    pub const fn get_FALLING(&self) -> bool {
        (self.inner & SplineFlag::FALLING) != 0
    }

    pub fn clear_FALLING(mut self) -> Self {
        self.inner &= SplineFlag::FALLING.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN3() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN3,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN3(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN3;
        self
    }

    pub const fn get_UNKNOWN3(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN3) != 0
    }

    pub fn clear_UNKNOWN3(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN3.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN4() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN4,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN4(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN4;
        self
    }

    pub const fn get_UNKNOWN4(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN4) != 0
    }

    pub fn clear_UNKNOWN4(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN4.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN5() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN5,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN5(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN5;
        self
    }

    pub const fn get_UNKNOWN5(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN5) != 0
    }

    pub fn clear_UNKNOWN5(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN5.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN6() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN6,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN6(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN6;
        self
    }

    pub const fn get_UNKNOWN6(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN6) != 0
    }

    pub fn clear_UNKNOWN6(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN6.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN7() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN7,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN7(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN7;
        self
    }

    pub const fn get_UNKNOWN7(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN7) != 0
    }

    pub fn clear_UNKNOWN7(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN7.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN8() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN8,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN8(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN8;
        self
    }

    pub const fn get_UNKNOWN8(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN8) != 0
    }

    pub fn clear_UNKNOWN8(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN8.reverse_bits();
        self
    }

    pub const fn new_RUNMODE() -> Self {
        Self {
            inner: SplineFlag::RUNMODE,
            final_angle: None,
        }
    }

    pub fn set_RUNMODE(mut self) -> Self {
        self.inner |= SplineFlag::RUNMODE;
        self
    }

    pub const fn get_RUNMODE(&self) -> bool {
        (self.inner & SplineFlag::RUNMODE) != 0
    }

    pub fn clear_RUNMODE(mut self) -> Self {
        self.inner &= SplineFlag::RUNMODE.reverse_bits();
        self
    }

    pub const fn new_FLYING() -> Self {
        Self {
            inner: SplineFlag::FLYING,
            final_angle: None,
        }
    }

    pub fn set_FLYING(mut self) -> Self {
        self.inner |= SplineFlag::FLYING;
        self
    }

    pub const fn get_FLYING(&self) -> bool {
        (self.inner & SplineFlag::FLYING) != 0
    }

    pub fn clear_FLYING(mut self) -> Self {
        self.inner &= SplineFlag::FLYING.reverse_bits();
        self
    }

    pub const fn new_NO_SPLINE() -> Self {
        Self {
            inner: SplineFlag::NO_SPLINE,
            final_angle: None,
        }
    }

    pub fn set_NO_SPLINE(mut self) -> Self {
        self.inner |= SplineFlag::NO_SPLINE;
        self
    }

    pub const fn get_NO_SPLINE(&self) -> bool {
        (self.inner & SplineFlag::NO_SPLINE) != 0
    }

    pub fn clear_NO_SPLINE(mut self) -> Self {
        self.inner &= SplineFlag::NO_SPLINE.reverse_bits();
        self
    }

    pub const fn new_PARABOLIC() -> Self {
        Self {
            inner: SplineFlag::PARABOLIC,
            final_angle: None,
        }
    }

    pub fn set_PARABOLIC(mut self) -> Self {
        self.inner |= SplineFlag::PARABOLIC;
        self
    }

    pub const fn get_PARABOLIC(&self) -> bool {
        (self.inner & SplineFlag::PARABOLIC) != 0
    }

    pub fn clear_PARABOLIC(mut self) -> Self {
        self.inner &= SplineFlag::PARABOLIC.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN13() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN13,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN13(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN13;
        self
    }

    pub const fn get_UNKNOWN13(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN13) != 0
    }

    pub fn clear_UNKNOWN13(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN13.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN14() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN14,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN14(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN14;
        self
    }

    pub const fn get_UNKNOWN14(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN14) != 0
    }

    pub fn clear_UNKNOWN14(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN14.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN15() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN15,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN15(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN15;
        self
    }

    pub const fn get_UNKNOWN15(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN15) != 0
    }

    pub fn clear_UNKNOWN15(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN15.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN16() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN16,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN16(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN16;
        self
    }

    pub const fn get_UNKNOWN16(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN16) != 0
    }

    pub fn clear_UNKNOWN16(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN16.reverse_bits();
        self
    }

    pub const fn new_FINAL_ANGLE(final_angle: MovementBlock_SplineFlag_FinalAngle) -> Self {
        Self {
            inner: final_angle.as_int(),
            final_angle: Some(final_angle),
        }
    }

    pub fn set_FINAL_ANGLE(mut self, final_angle: MovementBlock_SplineFlag_FinalAngle) -> Self {
        self.inner |= final_angle.as_int();
        self.final_angle = Some(final_angle);
        self
    }

    pub const fn get_FINAL_ANGLE(&self) -> Option<&MovementBlock_SplineFlag_FinalAngle> {
        self.final_angle.as_ref()
    }

    pub fn clear_FINAL_ANGLE(mut self) -> Self {
        self.inner &= SplineFlag::FINAL_ANGLE.reverse_bits();
        self.final_angle = None;
        self
    }

    pub const fn new_UNKNOWN19() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN19,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN19(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN19;
        self
    }

    pub const fn get_UNKNOWN19(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN19) != 0
    }

    pub fn clear_UNKNOWN19(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN19.reverse_bits();
        self
    }

    pub const fn new_CYCLIC() -> Self {
        Self {
            inner: SplineFlag::CYCLIC,
            final_angle: None,
        }
    }

    pub fn set_CYCLIC(mut self) -> Self {
        self.inner |= SplineFlag::CYCLIC;
        self
    }

    pub const fn get_CYCLIC(&self) -> bool {
        (self.inner & SplineFlag::CYCLIC) != 0
    }

    pub fn clear_CYCLIC(mut self) -> Self {
        self.inner &= SplineFlag::CYCLIC.reverse_bits();
        self
    }

    pub const fn new_ENTER_CYCLE() -> Self {
        Self {
            inner: SplineFlag::ENTER_CYCLE,
            final_angle: None,
        }
    }

    pub fn set_ENTER_CYCLE(mut self) -> Self {
        self.inner |= SplineFlag::ENTER_CYCLE;
        self
    }

    pub const fn get_ENTER_CYCLE(&self) -> bool {
        (self.inner & SplineFlag::ENTER_CYCLE) != 0
    }

    pub fn clear_ENTER_CYCLE(mut self) -> Self {
        self.inner &= SplineFlag::ENTER_CYCLE.reverse_bits();
        self
    }

    pub const fn new_FROZEN() -> Self {
        Self {
            inner: SplineFlag::FROZEN,
            final_angle: None,
        }
    }

    pub fn set_FROZEN(mut self) -> Self {
        self.inner |= SplineFlag::FROZEN;
        self
    }

    pub const fn get_FROZEN(&self) -> bool {
        (self.inner & SplineFlag::FROZEN) != 0
    }

    pub fn clear_FROZEN(mut self) -> Self {
        self.inner &= SplineFlag::FROZEN.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN23() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN23,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN23(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN23;
        self
    }

    pub const fn get_UNKNOWN23(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN23) != 0
    }

    pub fn clear_UNKNOWN23(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN23.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN24() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN24,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN24(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN24;
        self
    }

    pub const fn get_UNKNOWN24(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN24) != 0
    }

    pub fn clear_UNKNOWN24(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN24.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN25() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN25,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN25(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN25;
        self
    }

    pub const fn get_UNKNOWN25(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN25) != 0
    }

    pub fn clear_UNKNOWN25(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN25.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN26() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN26,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN26(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN26;
        self
    }

    pub const fn get_UNKNOWN26(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN26) != 0
    }

    pub fn clear_UNKNOWN26(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN26.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN27() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN27,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN27(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN27;
        self
    }

    pub const fn get_UNKNOWN27(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN27) != 0
    }

    pub fn clear_UNKNOWN27(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN27.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN28() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN28,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN28(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN28;
        self
    }

    pub const fn get_UNKNOWN28(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN28) != 0
    }

    pub fn clear_UNKNOWN28(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN28.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN29() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN29,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN29(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN29;
        self
    }

    pub const fn get_UNKNOWN29(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN29) != 0
    }

    pub fn clear_UNKNOWN29(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN29.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN30() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN30,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN30(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN30;
        self
    }

    pub const fn get_UNKNOWN30(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN30) != 0
    }

    pub fn clear_UNKNOWN30(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN30.reverse_bits();
        self
    }

    pub const fn new_UNKNOWN31() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN31,
            final_angle: None,
        }
    }

    pub fn set_UNKNOWN31(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN31;
        self
    }

    pub const fn get_UNKNOWN31(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN31) != 0
    }

    pub fn clear_UNKNOWN31(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN31.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl MovementBlock_SplineFlag {
    pub(crate) fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.final_angle {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MovementBlock_MovementFlags {
    inner: u32,
    on_transport: Option<MovementBlock_MovementFlags_OnTransport>,
    falling: Option<MovementBlock_MovementFlags_Falling>,
    swimming: Option<MovementBlock_MovementFlags_Swimming>,
    spline_elevation: Option<MovementBlock_MovementFlags_SplineElevation>,
    spline_enabled: Option<MovementBlock_MovementFlags_SplineEnabled>,
}

impl MovementBlock_MovementFlags {
    pub const fn new(inner: u32, on_transport: Option<MovementBlock_MovementFlags_OnTransport>,falling: Option<MovementBlock_MovementFlags_Falling>,swimming: Option<MovementBlock_MovementFlags_Swimming>,spline_elevation: Option<MovementBlock_MovementFlags_SplineElevation>,spline_enabled: Option<MovementBlock_MovementFlags_SplineEnabled>,) -> Self {
        Self {
            inner,
            on_transport, 
            falling, 
            swimming, 
            spline_elevation, 
            spline_enabled, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.on_transport.is_none()
        && self.falling.is_none()
        && self.swimming.is_none()
        && self.spline_elevation.is_none()
        && self.spline_enabled.is_none()
    }

    pub const fn new_FORWARD() -> Self {
        Self {
            inner: MovementFlags::FORWARD,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_FORWARD(mut self) -> Self {
        self.inner |= MovementFlags::FORWARD;
        self
    }

    pub const fn get_FORWARD(&self) -> bool {
        (self.inner & MovementFlags::FORWARD) != 0
    }

    pub fn clear_FORWARD(mut self) -> Self {
        self.inner &= MovementFlags::FORWARD.reverse_bits();
        self
    }

    pub const fn new_BACKWARD() -> Self {
        Self {
            inner: MovementFlags::BACKWARD,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_BACKWARD(mut self) -> Self {
        self.inner |= MovementFlags::BACKWARD;
        self
    }

    pub const fn get_BACKWARD(&self) -> bool {
        (self.inner & MovementFlags::BACKWARD) != 0
    }

    pub fn clear_BACKWARD(mut self) -> Self {
        self.inner &= MovementFlags::BACKWARD.reverse_bits();
        self
    }

    pub const fn new_STRAFE_LEFT() -> Self {
        Self {
            inner: MovementFlags::STRAFE_LEFT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_STRAFE_LEFT(mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_LEFT;
        self
    }

    pub const fn get_STRAFE_LEFT(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_LEFT) != 0
    }

    pub fn clear_STRAFE_LEFT(mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_LEFT.reverse_bits();
        self
    }

    pub const fn new_STRAFE_RIGHT() -> Self {
        Self {
            inner: MovementFlags::STRAFE_RIGHT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_STRAFE_RIGHT(mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_RIGHT;
        self
    }

    pub const fn get_STRAFE_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_RIGHT) != 0
    }

    pub fn clear_STRAFE_RIGHT(mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_RIGHT.reverse_bits();
        self
    }

    pub const fn new_LEFT() -> Self {
        Self {
            inner: MovementFlags::LEFT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_LEFT(mut self) -> Self {
        self.inner |= MovementFlags::LEFT;
        self
    }

    pub const fn get_LEFT(&self) -> bool {
        (self.inner & MovementFlags::LEFT) != 0
    }

    pub fn clear_LEFT(mut self) -> Self {
        self.inner &= MovementFlags::LEFT.reverse_bits();
        self
    }

    pub const fn new_RIGHT() -> Self {
        Self {
            inner: MovementFlags::RIGHT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_RIGHT(mut self) -> Self {
        self.inner |= MovementFlags::RIGHT;
        self
    }

    pub const fn get_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::RIGHT) != 0
    }

    pub fn clear_RIGHT(mut self) -> Self {
        self.inner &= MovementFlags::RIGHT.reverse_bits();
        self
    }

    pub const fn new_PITCH_UP() -> Self {
        Self {
            inner: MovementFlags::PITCH_UP,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_PITCH_UP(mut self) -> Self {
        self.inner |= MovementFlags::PITCH_UP;
        self
    }

    pub const fn get_PITCH_UP(&self) -> bool {
        (self.inner & MovementFlags::PITCH_UP) != 0
    }

    pub fn clear_PITCH_UP(mut self) -> Self {
        self.inner &= MovementFlags::PITCH_UP.reverse_bits();
        self
    }

    pub const fn new_PITCH_DOWN() -> Self {
        Self {
            inner: MovementFlags::PITCH_DOWN,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_PITCH_DOWN(mut self) -> Self {
        self.inner |= MovementFlags::PITCH_DOWN;
        self
    }

    pub const fn get_PITCH_DOWN(&self) -> bool {
        (self.inner & MovementFlags::PITCH_DOWN) != 0
    }

    pub fn clear_PITCH_DOWN(mut self) -> Self {
        self.inner &= MovementFlags::PITCH_DOWN.reverse_bits();
        self
    }

    pub const fn new_WALKING() -> Self {
        Self {
            inner: MovementFlags::WALKING,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_WALKING(mut self) -> Self {
        self.inner |= MovementFlags::WALKING;
        self
    }

    pub const fn get_WALKING(&self) -> bool {
        (self.inner & MovementFlags::WALKING) != 0
    }

    pub fn clear_WALKING(mut self) -> Self {
        self.inner &= MovementFlags::WALKING.reverse_bits();
        self
    }

    pub const fn new_ON_TRANSPORT(on_transport: MovementBlock_MovementFlags_OnTransport) -> Self {
        Self {
            inner: MovementFlags::ON_TRANSPORT,
            on_transport: Some(on_transport),
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_ON_TRANSPORT(mut self, on_transport: MovementBlock_MovementFlags_OnTransport) -> Self {
        self.inner |= MovementFlags::ON_TRANSPORT;
        self.on_transport = Some(on_transport);
        self
    }

    pub const fn get_ON_TRANSPORT(&self) -> Option<&MovementBlock_MovementFlags_OnTransport> {
        self.on_transport.as_ref()
    }

    pub fn clear_ON_TRANSPORT(mut self) -> Self {
        self.inner &= MovementFlags::ON_TRANSPORT.reverse_bits();
        self.on_transport = None;
        self
    }

    pub const fn new_DISABLE_GRAVITY() -> Self {
        Self {
            inner: MovementFlags::DISABLE_GRAVITY,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_DISABLE_GRAVITY(mut self) -> Self {
        self.inner |= MovementFlags::DISABLE_GRAVITY;
        self
    }

    pub const fn get_DISABLE_GRAVITY(&self) -> bool {
        (self.inner & MovementFlags::DISABLE_GRAVITY) != 0
    }

    pub fn clear_DISABLE_GRAVITY(mut self) -> Self {
        self.inner &= MovementFlags::DISABLE_GRAVITY.reverse_bits();
        self
    }

    pub const fn new_ROOT() -> Self {
        Self {
            inner: MovementFlags::ROOT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_ROOT(mut self) -> Self {
        self.inner |= MovementFlags::ROOT;
        self
    }

    pub const fn get_ROOT(&self) -> bool {
        (self.inner & MovementFlags::ROOT) != 0
    }

    pub fn clear_ROOT(mut self) -> Self {
        self.inner &= MovementFlags::ROOT.reverse_bits();
        self
    }

    pub const fn new_FALLING(falling: MovementBlock_MovementFlags_Falling) -> Self {
        Self {
            inner: MovementFlags::FALLING,
            on_transport: None,
            falling: Some(falling),
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_FALLING(mut self, falling: MovementBlock_MovementFlags_Falling) -> Self {
        self.inner |= MovementFlags::FALLING;
        self.falling = Some(falling);
        self
    }

    pub const fn get_FALLING(&self) -> Option<&MovementBlock_MovementFlags_Falling> {
        self.falling.as_ref()
    }

    pub fn clear_FALLING(mut self) -> Self {
        self.inner &= MovementFlags::FALLING.reverse_bits();
        self.falling = None;
        self
    }

    pub const fn new_FALLING_FAR() -> Self {
        Self {
            inner: MovementFlags::FALLING_FAR,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_FALLING_FAR(mut self) -> Self {
        self.inner |= MovementFlags::FALLING_FAR;
        self
    }

    pub const fn get_FALLING_FAR(&self) -> bool {
        (self.inner & MovementFlags::FALLING_FAR) != 0
    }

    pub fn clear_FALLING_FAR(mut self) -> Self {
        self.inner &= MovementFlags::FALLING_FAR.reverse_bits();
        self
    }

    pub const fn new_PENDING_STOP() -> Self {
        Self {
            inner: MovementFlags::PENDING_STOP,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_PENDING_STOP(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STOP;
        self
    }

    pub const fn get_PENDING_STOP(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STOP) != 0
    }

    pub fn clear_PENDING_STOP(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STOP.reverse_bits();
        self
    }

    pub const fn new_PENDING_STRAFE_STOP() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_STOP,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_PENDING_STRAFE_STOP(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_STOP;
        self
    }

    pub const fn get_PENDING_STRAFE_STOP(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_STOP) != 0
    }

    pub fn clear_PENDING_STRAFE_STOP(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_STOP.reverse_bits();
        self
    }

    pub const fn new_PENDING_FORWARD() -> Self {
        Self {
            inner: MovementFlags::PENDING_FORWARD,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_PENDING_FORWARD(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_FORWARD;
        self
    }

    pub const fn get_PENDING_FORWARD(&self) -> bool {
        (self.inner & MovementFlags::PENDING_FORWARD) != 0
    }

    pub fn clear_PENDING_FORWARD(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_FORWARD.reverse_bits();
        self
    }

    pub const fn new_PENDING_BACKWARD() -> Self {
        Self {
            inner: MovementFlags::PENDING_BACKWARD,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_PENDING_BACKWARD(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_BACKWARD;
        self
    }

    pub const fn get_PENDING_BACKWARD(&self) -> bool {
        (self.inner & MovementFlags::PENDING_BACKWARD) != 0
    }

    pub fn clear_PENDING_BACKWARD(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_BACKWARD.reverse_bits();
        self
    }

    pub const fn new_PENDING_STRAFE_LEFT() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_LEFT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_PENDING_STRAFE_LEFT(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_LEFT;
        self
    }

    pub const fn get_PENDING_STRAFE_LEFT(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_LEFT) != 0
    }

    pub fn clear_PENDING_STRAFE_LEFT(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_LEFT.reverse_bits();
        self
    }

    pub const fn new_PENDING_STRAFE_RIGHT() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_RIGHT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_PENDING_STRAFE_RIGHT(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_RIGHT;
        self
    }

    pub const fn get_PENDING_STRAFE_RIGHT(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_RIGHT) != 0
    }

    pub fn clear_PENDING_STRAFE_RIGHT(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_RIGHT.reverse_bits();
        self
    }

    pub const fn new_PENDING_ROOT() -> Self {
        Self {
            inner: MovementFlags::PENDING_ROOT,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_PENDING_ROOT(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_ROOT;
        self
    }

    pub const fn get_PENDING_ROOT(&self) -> bool {
        (self.inner & MovementFlags::PENDING_ROOT) != 0
    }

    pub fn clear_PENDING_ROOT(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_ROOT.reverse_bits();
        self
    }

    pub const fn new_SWIMMING(swimming: MovementBlock_MovementFlags_Swimming) -> Self {
        Self {
            inner: swimming.as_int(),
            on_transport: None,
            falling: None,
            swimming: Some(swimming),
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_SWIMMING(mut self, swimming: MovementBlock_MovementFlags_Swimming) -> Self {
        self.inner |= swimming.as_int();
        self.swimming = Some(swimming);
        self
    }

    pub const fn get_SWIMMING(&self) -> Option<&MovementBlock_MovementFlags_Swimming> {
        self.swimming.as_ref()
    }

    pub fn clear_SWIMMING(mut self) -> Self {
        self.inner &= MovementFlags::SWIMMING.reverse_bits();
        self.swimming = None;
        self
    }

    pub const fn new_ASCENDING() -> Self {
        Self {
            inner: MovementFlags::ASCENDING,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_ASCENDING(mut self) -> Self {
        self.inner |= MovementFlags::ASCENDING;
        self
    }

    pub const fn get_ASCENDING(&self) -> bool {
        (self.inner & MovementFlags::ASCENDING) != 0
    }

    pub fn clear_ASCENDING(mut self) -> Self {
        self.inner &= MovementFlags::ASCENDING.reverse_bits();
        self
    }

    pub const fn new_DESCENDING() -> Self {
        Self {
            inner: MovementFlags::DESCENDING,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_DESCENDING(mut self) -> Self {
        self.inner |= MovementFlags::DESCENDING;
        self
    }

    pub const fn get_DESCENDING(&self) -> bool {
        (self.inner & MovementFlags::DESCENDING) != 0
    }

    pub fn clear_DESCENDING(mut self) -> Self {
        self.inner &= MovementFlags::DESCENDING.reverse_bits();
        self
    }

    pub const fn new_CAN_FLY() -> Self {
        Self {
            inner: MovementFlags::CAN_FLY,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_CAN_FLY(mut self) -> Self {
        self.inner |= MovementFlags::CAN_FLY;
        self
    }

    pub const fn get_CAN_FLY(&self) -> bool {
        (self.inner & MovementFlags::CAN_FLY) != 0
    }

    pub fn clear_CAN_FLY(mut self) -> Self {
        self.inner &= MovementFlags::CAN_FLY.reverse_bits();
        self
    }

    pub const fn new_SPLINE_ELEVATION(spline_elevation: MovementBlock_MovementFlags_SplineElevation) -> Self {
        Self {
            inner: MovementFlags::SPLINE_ELEVATION,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: Some(spline_elevation),
            spline_enabled: None,
        }
    }

    pub fn set_SPLINE_ELEVATION(mut self, spline_elevation: MovementBlock_MovementFlags_SplineElevation) -> Self {
        self.inner |= MovementFlags::SPLINE_ELEVATION;
        self.spline_elevation = Some(spline_elevation);
        self
    }

    pub const fn get_SPLINE_ELEVATION(&self) -> Option<&MovementBlock_MovementFlags_SplineElevation> {
        self.spline_elevation.as_ref()
    }

    pub fn clear_SPLINE_ELEVATION(mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ELEVATION.reverse_bits();
        self.spline_elevation = None;
        self
    }

    pub const fn new_SPLINE_ENABLED(spline_enabled: MovementBlock_MovementFlags_SplineEnabled) -> Self {
        Self {
            inner: MovementFlags::SPLINE_ENABLED,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: Some(spline_enabled),
        }
    }

    pub fn set_SPLINE_ENABLED(mut self, spline_enabled: MovementBlock_MovementFlags_SplineEnabled) -> Self {
        self.inner |= MovementFlags::SPLINE_ENABLED;
        self.spline_enabled = Some(spline_enabled);
        self
    }

    pub const fn get_SPLINE_ENABLED(&self) -> Option<&MovementBlock_MovementFlags_SplineEnabled> {
        self.spline_enabled.as_ref()
    }

    pub fn clear_SPLINE_ENABLED(mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ENABLED.reverse_bits();
        self.spline_enabled = None;
        self
    }

    pub const fn new_WATERWALKING() -> Self {
        Self {
            inner: MovementFlags::WATERWALKING,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_WATERWALKING(mut self) -> Self {
        self.inner |= MovementFlags::WATERWALKING;
        self
    }

    pub const fn get_WATERWALKING(&self) -> bool {
        (self.inner & MovementFlags::WATERWALKING) != 0
    }

    pub fn clear_WATERWALKING(mut self) -> Self {
        self.inner &= MovementFlags::WATERWALKING.reverse_bits();
        self
    }

    pub const fn new_FALLING_SLOW() -> Self {
        Self {
            inner: MovementFlags::FALLING_SLOW,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_FALLING_SLOW(mut self) -> Self {
        self.inner |= MovementFlags::FALLING_SLOW;
        self
    }

    pub const fn get_FALLING_SLOW(&self) -> bool {
        (self.inner & MovementFlags::FALLING_SLOW) != 0
    }

    pub fn clear_FALLING_SLOW(mut self) -> Self {
        self.inner &= MovementFlags::FALLING_SLOW.reverse_bits();
        self
    }

    pub const fn new_HOVER() -> Self {
        Self {
            inner: MovementFlags::HOVER,
            on_transport: None,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
        }
    }

    pub fn set_HOVER(mut self) -> Self {
        self.inner |= MovementFlags::HOVER;
        self
    }

    pub const fn get_HOVER(&self) -> bool {
        (self.inner & MovementFlags::HOVER) != 0
    }

    pub fn clear_HOVER(mut self) -> Self {
        self.inner &= MovementFlags::HOVER.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl MovementBlock_MovementFlags {
    pub(crate) fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.on_transport {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.falling {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.swimming {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.spline_elevation {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.spline_enabled {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MovementBlock_MovementFlags_OnTransport {
    pub transport: TransportInfo,
}

impl MovementBlock_MovementFlags_OnTransport {
    pub(crate) fn size(&self) -> usize {
        self.transport.size() // transport: TransportInfo
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MovementBlock_MovementFlags_Falling {
    pub cos_angle: f32,
    pub sin_angle: f32,
    pub xy_speed: f32,
    pub z_speed: f32,
}

impl MovementBlock_MovementFlags_Falling {
    pub(crate) fn size(&self) -> usize {
        4 // cos_angle: f32
        + 4 // sin_angle: f32
        + 4 // xy_speed: f32
        + 4 // z_speed: f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MovementBlock_MovementFlags_SplineElevation {
    pub spline_elevation: f32,
}

impl MovementBlock_MovementFlags_SplineElevation {
    pub(crate) fn size(&self) -> usize {
        4 // spline_elevation: f32
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MovementBlock_MovementFlags_SplineEnabled {
    pub duration: u32,
    pub final_node: Vector3d,
    pub id: u32,
    pub nodes: Vec<Vector3d>,
    pub spline_flags: MovementBlock_SplineFlag,
    pub time_passed: u32,
}

impl MovementBlock_MovementFlags_SplineEnabled {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_nodes: u32
        + 4 // duration: u32
        + 12 // final_node: Vector3d
        + 4 // id: u32
        + self.nodes.len() * 12 // nodes: Vector3d[amount_of_nodes]
        + self.spline_flags.size() // spline_flags: MovementBlock_SplineFlag
        + 4 // time_passed: u32
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MovementBlock_UpdateFlag_Living {
    Living {
        backwards_flight_speed: f32,
        backwards_running_speed: f32,
        backwards_swimming_speed: f32,
        extra_flags: ExtraMovementFlags,
        fall_time: f32,
        flags: MovementBlock_MovementFlags,
        flight_speed: f32,
        living_orientation: f32,
        living_position: Vector3d,
        pitch_rate: f32,
        running_speed: f32,
        swimming_speed: f32,
        timestamp: u32,
        turn_rate: f32,
        walking_speed: f32,
    },
    Position {
        corpse_orientation: f32,
        orientation1: f32,
        position1: Vector3d,
        transport_guid: Guid,
    },
    HasPosition {
        orientation2: f32,
        position2: Vector3d,
    },
}

impl MovementBlock_UpdateFlag_Living {
    pub(crate) const fn as_int(&self) -> u16 {
        match self {
            Self::Living { .. } => 32,
            Self::Position { .. } => 256,
            Self::HasPosition { .. } => 64,
        }
    }

}

impl MovementBlock_UpdateFlag_Living {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Living {
                backwards_flight_speed,
                backwards_running_speed,
                backwards_swimming_speed,
                extra_flags,
                fall_time,
                flags,
                flight_speed,
                living_orientation,
                living_position,
                pitch_rate,
                running_speed,
                swimming_speed,
                timestamp,
                turn_rate,
                walking_speed,
            } => {
                // Not an actual enum sent over the wire
                4 // backwards_flight_speed: f32
                + 4 // backwards_running_speed: f32
                + 4 // backwards_swimming_speed: f32
                + 2 // extra_flags: ExtraMovementFlags
                + 4 // fall_time: f32
                + flags.size() // flags: MovementBlock_MovementFlags
                + 4 // flight_speed: f32
                + 4 // living_orientation: f32
                + 12 // living_position: Vector3d
                + 4 // pitch_rate: f32
                + 4 // running_speed: f32
                + 4 // swimming_speed: f32
                + 4 // timestamp: u32
                + 4 // turn_rate: f32
                + 4 // walking_speed: f32
            }
            Self::Position {
                corpse_orientation,
                orientation1,
                position1,
                transport_guid,
            } => {
                // Not an actual enum sent over the wire
                4 // corpse_orientation: f32
                + 4 // orientation1: f32
                + 12 // position1: Vector3d
                + transport_guid.size() // transport_guid: PackedGuid
            }
            Self::HasPosition {
                orientation2,
                position2,
            } => {
                // Not an actual enum sent over the wire
                4 // orientation2: f32
                + 12 // position2: Vector3d
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MovementBlock_UpdateFlag {
    inner: u16,
    transport: Option<MovementBlock_UpdateFlag_Transport>,
    has_attacking_target: Option<MovementBlock_UpdateFlag_HasAttackingTarget>,
    low_guid: Option<MovementBlock_UpdateFlag_LowGuid>,
    high_guid: Option<MovementBlock_UpdateFlag_HighGuid>,
    living: Option<MovementBlock_UpdateFlag_Living>,
    vehicle: Option<MovementBlock_UpdateFlag_Vehicle>,
    rotation: Option<MovementBlock_UpdateFlag_Rotation>,
}

impl MovementBlock_UpdateFlag {
    pub const fn new(inner: u16, transport: Option<MovementBlock_UpdateFlag_Transport>,has_attacking_target: Option<MovementBlock_UpdateFlag_HasAttackingTarget>,low_guid: Option<MovementBlock_UpdateFlag_LowGuid>,high_guid: Option<MovementBlock_UpdateFlag_HighGuid>,living: Option<MovementBlock_UpdateFlag_Living>,vehicle: Option<MovementBlock_UpdateFlag_Vehicle>,rotation: Option<MovementBlock_UpdateFlag_Rotation>,) -> Self {
        Self {
            inner,
            transport, 
            has_attacking_target, 
            low_guid, 
            high_guid, 
            living, 
            vehicle, 
            rotation, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            transport: None,
            has_attacking_target: None,
            low_guid: None,
            high_guid: None,
            living: None,
            vehicle: None,
            rotation: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.transport.is_none()
        && self.has_attacking_target.is_none()
        && self.low_guid.is_none()
        && self.high_guid.is_none()
        && self.living.is_none()
        && self.vehicle.is_none()
        && self.rotation.is_none()
    }

    pub const fn new_SELF() -> Self {
        Self {
            inner: UpdateFlag::SELF,
            transport: None,
            has_attacking_target: None,
            low_guid: None,
            high_guid: None,
            living: None,
            vehicle: None,
            rotation: None,
        }
    }

    pub fn set_SELF(mut self) -> Self {
        self.inner |= UpdateFlag::SELF;
        self
    }

    pub const fn get_SELF(&self) -> bool {
        (self.inner & UpdateFlag::SELF) != 0
    }

    pub fn clear_SELF(mut self) -> Self {
        self.inner &= UpdateFlag::SELF.reverse_bits();
        self
    }

    pub const fn new_TRANSPORT(transport: MovementBlock_UpdateFlag_Transport) -> Self {
        Self {
            inner: UpdateFlag::TRANSPORT,
            transport: Some(transport),
            has_attacking_target: None,
            low_guid: None,
            high_guid: None,
            living: None,
            vehicle: None,
            rotation: None,
        }
    }

    pub fn set_TRANSPORT(mut self, transport: MovementBlock_UpdateFlag_Transport) -> Self {
        self.inner |= UpdateFlag::TRANSPORT;
        self.transport = Some(transport);
        self
    }

    pub const fn get_TRANSPORT(&self) -> Option<&MovementBlock_UpdateFlag_Transport> {
        self.transport.as_ref()
    }

    pub fn clear_TRANSPORT(mut self) -> Self {
        self.inner &= UpdateFlag::TRANSPORT.reverse_bits();
        self.transport = None;
        self
    }

    pub const fn new_HAS_ATTACKING_TARGET(has_attacking_target: MovementBlock_UpdateFlag_HasAttackingTarget) -> Self {
        Self {
            inner: UpdateFlag::HAS_ATTACKING_TARGET,
            transport: None,
            has_attacking_target: Some(has_attacking_target),
            low_guid: None,
            high_guid: None,
            living: None,
            vehicle: None,
            rotation: None,
        }
    }

    pub fn set_HAS_ATTACKING_TARGET(mut self, has_attacking_target: MovementBlock_UpdateFlag_HasAttackingTarget) -> Self {
        self.inner |= UpdateFlag::HAS_ATTACKING_TARGET;
        self.has_attacking_target = Some(has_attacking_target);
        self
    }

    pub const fn get_HAS_ATTACKING_TARGET(&self) -> Option<&MovementBlock_UpdateFlag_HasAttackingTarget> {
        self.has_attacking_target.as_ref()
    }

    pub fn clear_HAS_ATTACKING_TARGET(mut self) -> Self {
        self.inner &= UpdateFlag::HAS_ATTACKING_TARGET.reverse_bits();
        self.has_attacking_target = None;
        self
    }

    pub const fn new_LOW_GUID(low_guid: MovementBlock_UpdateFlag_LowGuid) -> Self {
        Self {
            inner: UpdateFlag::LOW_GUID,
            transport: None,
            has_attacking_target: None,
            low_guid: Some(low_guid),
            high_guid: None,
            living: None,
            vehicle: None,
            rotation: None,
        }
    }

    pub fn set_LOW_GUID(mut self, low_guid: MovementBlock_UpdateFlag_LowGuid) -> Self {
        self.inner |= UpdateFlag::LOW_GUID;
        self.low_guid = Some(low_guid);
        self
    }

    pub const fn get_LOW_GUID(&self) -> Option<&MovementBlock_UpdateFlag_LowGuid> {
        self.low_guid.as_ref()
    }

    pub fn clear_LOW_GUID(mut self) -> Self {
        self.inner &= UpdateFlag::LOW_GUID.reverse_bits();
        self.low_guid = None;
        self
    }

    pub const fn new_HIGH_GUID(high_guid: MovementBlock_UpdateFlag_HighGuid) -> Self {
        Self {
            inner: UpdateFlag::HIGH_GUID,
            transport: None,
            has_attacking_target: None,
            low_guid: None,
            high_guid: Some(high_guid),
            living: None,
            vehicle: None,
            rotation: None,
        }
    }

    pub fn set_HIGH_GUID(mut self, high_guid: MovementBlock_UpdateFlag_HighGuid) -> Self {
        self.inner |= UpdateFlag::HIGH_GUID;
        self.high_guid = Some(high_guid);
        self
    }

    pub const fn get_HIGH_GUID(&self) -> Option<&MovementBlock_UpdateFlag_HighGuid> {
        self.high_guid.as_ref()
    }

    pub fn clear_HIGH_GUID(mut self) -> Self {
        self.inner &= UpdateFlag::HIGH_GUID.reverse_bits();
        self.high_guid = None;
        self
    }

    pub const fn new_LIVING(living: MovementBlock_UpdateFlag_Living) -> Self {
        Self {
            inner: living.as_int(),
            transport: None,
            has_attacking_target: None,
            low_guid: None,
            high_guid: None,
            living: Some(living),
            vehicle: None,
            rotation: None,
        }
    }

    pub fn set_LIVING(mut self, living: MovementBlock_UpdateFlag_Living) -> Self {
        self.inner |= living.as_int();
        self.living = Some(living);
        self
    }

    pub const fn get_LIVING(&self) -> Option<&MovementBlock_UpdateFlag_Living> {
        self.living.as_ref()
    }

    pub fn clear_LIVING(mut self) -> Self {
        self.inner &= UpdateFlag::LIVING.reverse_bits();
        self.living = None;
        self
    }

    pub const fn new_VEHICLE(vehicle: MovementBlock_UpdateFlag_Vehicle) -> Self {
        Self {
            inner: UpdateFlag::VEHICLE,
            transport: None,
            has_attacking_target: None,
            low_guid: None,
            high_guid: None,
            living: None,
            vehicle: Some(vehicle),
            rotation: None,
        }
    }

    pub fn set_VEHICLE(mut self, vehicle: MovementBlock_UpdateFlag_Vehicle) -> Self {
        self.inner |= UpdateFlag::VEHICLE;
        self.vehicle = Some(vehicle);
        self
    }

    pub const fn get_VEHICLE(&self) -> Option<&MovementBlock_UpdateFlag_Vehicle> {
        self.vehicle.as_ref()
    }

    pub fn clear_VEHICLE(mut self) -> Self {
        self.inner &= UpdateFlag::VEHICLE.reverse_bits();
        self.vehicle = None;
        self
    }

    pub const fn new_ROTATION(rotation: MovementBlock_UpdateFlag_Rotation) -> Self {
        Self {
            inner: UpdateFlag::ROTATION,
            transport: None,
            has_attacking_target: None,
            low_guid: None,
            high_guid: None,
            living: None,
            vehicle: None,
            rotation: Some(rotation),
        }
    }

    pub fn set_ROTATION(mut self, rotation: MovementBlock_UpdateFlag_Rotation) -> Self {
        self.inner |= UpdateFlag::ROTATION;
        self.rotation = Some(rotation);
        self
    }

    pub const fn get_ROTATION(&self) -> Option<&MovementBlock_UpdateFlag_Rotation> {
        self.rotation.as_ref()
    }

    pub fn clear_ROTATION(mut self) -> Self {
        self.inner &= UpdateFlag::ROTATION.reverse_bits();
        self.rotation = None;
        self
    }

    pub(crate) const fn as_int(&self) -> u16 {
        self.inner
    }

}
impl MovementBlock_UpdateFlag {
    pub(crate) fn size(&self) -> usize {
        2 // inner
        + {
            if let Some(s) = &self.transport {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.has_attacking_target {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.low_guid {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.high_guid {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.living {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.vehicle {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.rotation {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MovementBlock_UpdateFlag_Transport {
    pub transport_progress_in_ms: u32,
}

impl MovementBlock_UpdateFlag_Transport {
    pub(crate) fn size(&self) -> usize {
        4 // transport_progress_in_ms: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MovementBlock_UpdateFlag_HasAttackingTarget {
    pub guid: Guid,
}

impl MovementBlock_UpdateFlag_HasAttackingTarget {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MovementBlock_UpdateFlag_LowGuid {
    pub unknown1: u32,
}

impl MovementBlock_UpdateFlag_LowGuid {
    pub(crate) fn size(&self) -> usize {
        4 // unknown1: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MovementBlock_UpdateFlag_HighGuid {
    pub unknown0: u32,
}

impl MovementBlock_UpdateFlag_HighGuid {
    pub(crate) fn size(&self) -> usize {
        4 // unknown0: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MovementBlock_UpdateFlag_Vehicle {
    pub vehicle_id: u32,
    pub vehicle_orientation: f32,
}

impl MovementBlock_UpdateFlag_Vehicle {
    pub(crate) fn size(&self) -> usize {
        4 // vehicle_id: u32
        + 4 // vehicle_orientation: f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MovementBlock_UpdateFlag_Rotation {
    pub packed_local_rotation: u64,
}

impl MovementBlock_UpdateFlag_Rotation {
    pub(crate) fn size(&self) -> usize {
        8 // packed_local_rotation: u64
    }
}

