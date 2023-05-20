use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    MovementFlags, SplineFlag, TransportInfo, UpdateFlag, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm:87`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm#L87):
/// ```text
/// struct MovementBlock {
///     UpdateFlag update_flag;
///     if (update_flag & LIVING) {
///         MovementFlags flags;
///         u32 timestamp;
///         Vector3d position;
///         f32 orientation;
///         if (flags & ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT) {
///             TransportInfo transport_info;
///             u32 transport_time;
///         }
///         else if (flags & ON_TRANSPORT) {
///             TransportInfo transport;
///         }
///         if (flags & SWIMMING) {
///             f32 pitch1;
///         }
///         else if (flags & FLYING) {
///             f32 pitch2;
///         }
///         else if (flags & ALWAYS_ALLOW_PITCHING) {
///             f32 pitch3;
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
        w.write_all(&(self.update_flag.as_int().to_le_bytes()))?;

        if let Some(if_statement) = &self.update_flag.living {
            match if_statement {
                MovementBlock_UpdateFlag_Living::Living {
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
                    // flags: MovementFlags
                    w.write_all(&(flags.as_int() as u32).to_le_bytes())?;
                    w.write_all(&((flags.as_int() >> 32) as u16).to_le_bytes())?;

                    // timestamp: u32
                    w.write_all(&timestamp.to_le_bytes())?;

                    // position: Vector3d
                    position.write_into_vec(&mut w)?;

                    // orientation: f32
                    w.write_all(&orientation.to_le_bytes())?;

                    if let Some(if_statement) = &flags.on_transport_and_interpolated_movement {
                        match if_statement {
                            MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                                transport_info,
                                transport_time,
                            } => {
                                // transport_info: TransportInfo
                                transport_info.write_into_vec(&mut w)?;

                                // transport_time: u32
                                w.write_all(&transport_time.to_le_bytes())?;

                            }
                            MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                                transport,
                            } => {
                                // transport: TransportInfo
                                transport.write_into_vec(&mut w)?;

                            }
                        }
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
                            MovementBlock_MovementFlags_Swimming::AlwaysAllowPitching {
                                pitch3,
                            } => {
                                // pitch3: f32
                                w.write_all(&pitch3.to_le_bytes())?;

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
                        w.write_all(&(if_statement.spline_flags.as_int().to_le_bytes()))?;

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
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // update_flag: UpdateFlag
        let update_flag = UpdateFlag::new(crate::util::read_u16_le(&mut r)?);

        let update_flag_living = if update_flag.is_living() {
            // flags: MovementFlags
            let flags: MovementFlags = {
                let a = crate::util::read_u32_le(&mut r)?;
                let b = crate::util::read_u16_le(&mut r)?;
                MovementFlags::new((a as u64) | ((b as u64) << 32))
            };

            // timestamp: u32
            let timestamp = crate::util::read_u32_le(&mut r)?;

            // position: Vector3d
            let position = Vector3d::read(&mut r)?;

            // orientation: f32
            let orientation = crate::util::read_f32_le(&mut r)?;

            let flags_on_transport_and_interpolated_movement = if flags.is_on_transport_and_interpolated_movement() {
                // transport_info: TransportInfo
                let transport_info = TransportInfo::read(&mut r)?;

                // transport_time: u32
                let transport_time = crate::util::read_u32_le(&mut r)?;

                Some(MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                    transport_info,
                    transport_time,
                })
            }
            else if flags.is_on_transport() {
                // transport: TransportInfo
                let transport = TransportInfo::read(&mut r)?;

                Some(MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                    transport,
                })
            }
            else {
                None
            };

            let flags_swimming = if flags.is_swimming() {
                // pitch1: f32
                let pitch1 = crate::util::read_f32_le(&mut r)?;

                Some(MovementBlock_MovementFlags_Swimming::Swimming {
                    pitch1,
                })
            }
            else if flags.is_flying() {
                // pitch2: f32
                let pitch2 = crate::util::read_f32_le(&mut r)?;

                Some(MovementBlock_MovementFlags_Swimming::Flying {
                    pitch2,
                })
            }
            else if flags.is_always_allow_pitching() {
                // pitch3: f32
                let pitch3 = crate::util::read_f32_le(&mut r)?;

                Some(MovementBlock_MovementFlags_Swimming::AlwaysAllowPitching {
                    pitch3,
                })
            }
            else {
                None
            };

            // fall_time: f32
            let fall_time = crate::util::read_f32_le(&mut r)?;

            let flags_falling = if flags.is_falling() {
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

            let flags_spline_elevation = if flags.is_spline_elevation() {
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

            let flags_spline_enabled = if flags.is_spline_enabled() {
                // spline_flags: SplineFlag
                let spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

                let spline_flags_final_angle = if spline_flags.is_final_angle() {
                    // angle: f32
                    let angle = crate::util::read_f32_le(&mut r)?;

                    Some(MovementBlock_SplineFlag_FinalAngle::FinalAngle {
                        angle,
                    })
                }
                else if spline_flags.is_final_target() {
                    // target: u64
                    let target = crate::util::read_u64_le(&mut r)?;

                    Some(MovementBlock_SplineFlag_FinalAngle::FinalTarget {
                        target,
                    })
                }
                else if spline_flags.is_final_point() {
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
                    final_angle: spline_flags_final_angle,
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
                falling: flags_falling,
                swimming: flags_swimming,
                spline_elevation: flags_spline_elevation,
                spline_enabled: flags_spline_enabled,
                on_transport_and_interpolated_movement: flags_on_transport_and_interpolated_movement,
            };

            Some(MovementBlock_UpdateFlag_Living::Living {
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
            })
        }
        else if update_flag.is_position() {
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
        else if update_flag.is_has_position() {
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

        let update_flag_high_guid = if update_flag.is_high_guid() {
            // unknown0: u32
            let unknown0 = crate::util::read_u32_le(&mut r)?;

            Some(MovementBlock_UpdateFlag_HighGuid {
                unknown0,
            })
        }
        else {
            None
        };

        let update_flag_low_guid = if update_flag.is_low_guid() {
            // unknown1: u32
            let unknown1 = crate::util::read_u32_le(&mut r)?;

            Some(MovementBlock_UpdateFlag_LowGuid {
                unknown1,
            })
        }
        else {
            None
        };

        let update_flag_has_attacking_target = if update_flag.is_has_attacking_target() {
            // guid: PackedGuid
            let guid = Guid::read_packed(&mut r)?;

            Some(MovementBlock_UpdateFlag_HasAttackingTarget {
                guid,
            })
        }
        else {
            None
        };

        let update_flag_transport = if update_flag.is_transport() {
            // transport_progress_in_ms: u32
            let transport_progress_in_ms = crate::util::read_u32_le(&mut r)?;

            Some(MovementBlock_UpdateFlag_Transport {
                transport_progress_in_ms,
            })
        }
        else {
            None
        };

        let update_flag_vehicle = if update_flag.is_vehicle() {
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

        let update_flag_rotation = if update_flag.is_rotation() {
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
            transport: update_flag_transport,
            has_attacking_target: update_flag_has_attacking_target,
            low_guid: update_flag_low_guid,
            high_guid: update_flag_high_guid,
            living: update_flag_living,
            vehicle: update_flag_vehicle,
            rotation: update_flag_rotation,
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
    AlwaysAllowPitching {
        pitch3: f32,
    },
}

impl MovementBlock_MovementFlags_Swimming {
    pub(crate) const fn as_int(&self) -> u64 {
        match self {
            Self::Swimming { .. } => 2097152,
            Self::Flying { .. } => 33554432,
            Self::AlwaysAllowPitching { .. } => 137438953472,
        }
    }

}

impl MovementBlock_MovementFlags_Swimming {
    pub(crate) const fn size(&self) -> usize {
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
            Self::AlwaysAllowPitching {
                pitch3,
            } => {
                // Not an actual enum sent over the wire
                4 // pitch3: f32
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
    pub(crate) const fn size(&self) -> usize {
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

    pub const fn new_done() -> Self {
        Self {
            inner: SplineFlag::DONE,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_done(mut self) -> Self {
        self.inner |= SplineFlag::DONE;
        self
    }

    pub const fn get_done(&self) -> bool {
        (self.inner & SplineFlag::DONE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_done(mut self) -> Self {
        self.inner &= SplineFlag::DONE.reverse_bits();
        self
    }

    pub const fn new_falling() -> Self {
        Self {
            inner: SplineFlag::FALLING,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_falling(mut self) -> Self {
        self.inner |= SplineFlag::FALLING;
        self
    }

    pub const fn get_falling(&self) -> bool {
        (self.inner & SplineFlag::FALLING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_falling(mut self) -> Self {
        self.inner &= SplineFlag::FALLING.reverse_bits();
        self
    }

    pub const fn new_unknown3() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN3,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown3(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN3;
        self
    }

    pub const fn get_unknown3(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN3) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown3(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN3.reverse_bits();
        self
    }

    pub const fn new_unknown4() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN4,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown4(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN4;
        self
    }

    pub const fn get_unknown4(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN4) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown4(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN4.reverse_bits();
        self
    }

    pub const fn new_unknown5() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN5,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown5(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN5;
        self
    }

    pub const fn get_unknown5(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN5) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown5(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN5.reverse_bits();
        self
    }

    pub const fn new_unknown6() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN6,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown6(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN6;
        self
    }

    pub const fn get_unknown6(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN6) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown6(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN6.reverse_bits();
        self
    }

    pub const fn new_unknown7() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN7,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown7(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN7;
        self
    }

    pub const fn get_unknown7(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN7) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown7(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN7.reverse_bits();
        self
    }

    pub const fn new_unknown8() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN8,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown8(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN8;
        self
    }

    pub const fn get_unknown8(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN8) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown8(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN8.reverse_bits();
        self
    }

    pub const fn new_runmode() -> Self {
        Self {
            inner: SplineFlag::RUNMODE,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_runmode(mut self) -> Self {
        self.inner |= SplineFlag::RUNMODE;
        self
    }

    pub const fn get_runmode(&self) -> bool {
        (self.inner & SplineFlag::RUNMODE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_runmode(mut self) -> Self {
        self.inner &= SplineFlag::RUNMODE.reverse_bits();
        self
    }

    pub const fn new_flying() -> Self {
        Self {
            inner: SplineFlag::FLYING,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_flying(mut self) -> Self {
        self.inner |= SplineFlag::FLYING;
        self
    }

    pub const fn get_flying(&self) -> bool {
        (self.inner & SplineFlag::FLYING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_flying(mut self) -> Self {
        self.inner &= SplineFlag::FLYING.reverse_bits();
        self
    }

    pub const fn new_no_spline() -> Self {
        Self {
            inner: SplineFlag::NO_SPLINE,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_no_spline(mut self) -> Self {
        self.inner |= SplineFlag::NO_SPLINE;
        self
    }

    pub const fn get_no_spline(&self) -> bool {
        (self.inner & SplineFlag::NO_SPLINE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_no_spline(mut self) -> Self {
        self.inner &= SplineFlag::NO_SPLINE.reverse_bits();
        self
    }

    pub const fn new_parabolic() -> Self {
        Self {
            inner: SplineFlag::PARABOLIC,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_parabolic(mut self) -> Self {
        self.inner |= SplineFlag::PARABOLIC;
        self
    }

    pub const fn get_parabolic(&self) -> bool {
        (self.inner & SplineFlag::PARABOLIC) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_parabolic(mut self) -> Self {
        self.inner &= SplineFlag::PARABOLIC.reverse_bits();
        self
    }

    pub const fn new_unknown13() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN13,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown13(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN13;
        self
    }

    pub const fn get_unknown13(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN13) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown13(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN13.reverse_bits();
        self
    }

    pub const fn new_unknown14() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN14,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown14(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN14;
        self
    }

    pub const fn get_unknown14(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN14) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown14(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN14.reverse_bits();
        self
    }

    pub const fn new_unknown15() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN15,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown15(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN15;
        self
    }

    pub const fn get_unknown15(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN15) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown15(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN15.reverse_bits();
        self
    }

    pub const fn new_unknown16() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN16,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown16(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN16;
        self
    }

    pub const fn get_unknown16(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN16) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown16(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN16.reverse_bits();
        self
    }

    pub const fn new_final_angle(final_angle: MovementBlock_SplineFlag_FinalAngle) -> Self {
        Self {
            inner: final_angle.as_int(),
            final_angle: Some(final_angle),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_final_angle(mut self, final_angle: MovementBlock_SplineFlag_FinalAngle) -> Self {
        self.inner |= final_angle.as_int();
        self.final_angle = Some(final_angle);
        self
    }

    pub const fn get_final_angle(&self) -> Option<&MovementBlock_SplineFlag_FinalAngle> {
        self.final_angle.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_final_angle(mut self) -> Self {
        self.inner &= SplineFlag::FINAL_ANGLE.reverse_bits();
        self.final_angle = None;
        self
    }

    pub const fn new_unknown19() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN19,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown19(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN19;
        self
    }

    pub const fn get_unknown19(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN19) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown19(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN19.reverse_bits();
        self
    }

    pub const fn new_cyclic() -> Self {
        Self {
            inner: SplineFlag::CYCLIC,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_cyclic(mut self) -> Self {
        self.inner |= SplineFlag::CYCLIC;
        self
    }

    pub const fn get_cyclic(&self) -> bool {
        (self.inner & SplineFlag::CYCLIC) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_cyclic(mut self) -> Self {
        self.inner &= SplineFlag::CYCLIC.reverse_bits();
        self
    }

    pub const fn new_enter_cycle() -> Self {
        Self {
            inner: SplineFlag::ENTER_CYCLE,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_enter_cycle(mut self) -> Self {
        self.inner |= SplineFlag::ENTER_CYCLE;
        self
    }

    pub const fn get_enter_cycle(&self) -> bool {
        (self.inner & SplineFlag::ENTER_CYCLE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_enter_cycle(mut self) -> Self {
        self.inner &= SplineFlag::ENTER_CYCLE.reverse_bits();
        self
    }

    pub const fn new_frozen() -> Self {
        Self {
            inner: SplineFlag::FROZEN,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_frozen(mut self) -> Self {
        self.inner |= SplineFlag::FROZEN;
        self
    }

    pub const fn get_frozen(&self) -> bool {
        (self.inner & SplineFlag::FROZEN) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_frozen(mut self) -> Self {
        self.inner &= SplineFlag::FROZEN.reverse_bits();
        self
    }

    pub const fn new_unknown23() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN23,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown23(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN23;
        self
    }

    pub const fn get_unknown23(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN23) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown23(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN23.reverse_bits();
        self
    }

    pub const fn new_unknown24() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN24,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown24(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN24;
        self
    }

    pub const fn get_unknown24(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN24) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown24(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN24.reverse_bits();
        self
    }

    pub const fn new_unknown25() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN25,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown25(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN25;
        self
    }

    pub const fn get_unknown25(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN25) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown25(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN25.reverse_bits();
        self
    }

    pub const fn new_unknown26() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN26,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown26(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN26;
        self
    }

    pub const fn get_unknown26(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN26) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown26(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN26.reverse_bits();
        self
    }

    pub const fn new_unknown27() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN27,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown27(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN27;
        self
    }

    pub const fn get_unknown27(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN27) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown27(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN27.reverse_bits();
        self
    }

    pub const fn new_unknown28() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN28,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown28(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN28;
        self
    }

    pub const fn get_unknown28(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN28) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown28(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN28.reverse_bits();
        self
    }

    pub const fn new_unknown29() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN29,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown29(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN29;
        self
    }

    pub const fn get_unknown29(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN29) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown29(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN29.reverse_bits();
        self
    }

    pub const fn new_unknown30() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN30,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown30(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN30;
        self
    }

    pub const fn get_unknown30(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN30) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown30(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN30.reverse_bits();
        self
    }

    pub const fn new_unknown31() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN31,
            final_angle: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown31(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN31;
        self
    }

    pub const fn get_unknown31(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN31) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown31(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN31.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl MovementBlock_SplineFlag {
    pub(crate) const fn size(&self) -> usize {
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement {
    OnTransportAndInterpolatedMovement {
        transport_info: TransportInfo,
        transport_time: u32,
    },
    OnTransport {
        transport: TransportInfo,
    },
}

impl MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement {
    pub(crate) const fn as_int(&self) -> u64 {
        match self {
            Self::OnTransportAndInterpolatedMovement { .. } => 4398046511616,
            Self::OnTransport { .. } => 512,
        }
    }

}

impl MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::OnTransportAndInterpolatedMovement {
                transport_info,
                transport_time,
            } => {
                // Not an actual enum sent over the wire
                transport_info.size() // transport_info: TransportInfo
                + 4 // transport_time: u32
            }
            Self::OnTransport {
                transport,
            } => {
                // Not an actual enum sent over the wire
                transport.size() // transport: TransportInfo
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MovementBlock_MovementFlags {
    inner: u64,
    falling: Option<MovementBlock_MovementFlags_Falling>,
    swimming: Option<MovementBlock_MovementFlags_Swimming>,
    spline_elevation: Option<MovementBlock_MovementFlags_SplineElevation>,
    spline_enabled: Option<MovementBlock_MovementFlags_SplineEnabled>,
    on_transport_and_interpolated_movement: Option<MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement>,
}

impl MovementBlock_MovementFlags {
    pub const fn new(inner: u64, falling: Option<MovementBlock_MovementFlags_Falling>,swimming: Option<MovementBlock_MovementFlags_Swimming>,spline_elevation: Option<MovementBlock_MovementFlags_SplineElevation>,spline_enabled: Option<MovementBlock_MovementFlags_SplineEnabled>,on_transport_and_interpolated_movement: Option<MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement>,) -> Self {
        Self {
            inner,
            falling, 
            swimming, 
            spline_elevation, 
            spline_enabled, 
            on_transport_and_interpolated_movement, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.falling.is_none()
        && self.swimming.is_none()
        && self.spline_elevation.is_none()
        && self.spline_enabled.is_none()
        && self.on_transport_and_interpolated_movement.is_none()
    }

    pub const fn new_forward() -> Self {
        Self {
            inner: MovementFlags::FORWARD,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_forward(mut self) -> Self {
        self.inner |= MovementFlags::FORWARD;
        self
    }

    pub const fn get_forward(&self) -> bool {
        (self.inner & MovementFlags::FORWARD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_forward(mut self) -> Self {
        self.inner &= MovementFlags::FORWARD.reverse_bits();
        self
    }

    pub const fn new_backward() -> Self {
        Self {
            inner: MovementFlags::BACKWARD,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_backward(mut self) -> Self {
        self.inner |= MovementFlags::BACKWARD;
        self
    }

    pub const fn get_backward(&self) -> bool {
        (self.inner & MovementFlags::BACKWARD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_backward(mut self) -> Self {
        self.inner &= MovementFlags::BACKWARD.reverse_bits();
        self
    }

    pub const fn new_strafe_left() -> Self {
        Self {
            inner: MovementFlags::STRAFE_LEFT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_strafe_left(mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_LEFT;
        self
    }

    pub const fn get_strafe_left(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_LEFT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_strafe_left(mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_LEFT.reverse_bits();
        self
    }

    pub const fn new_strafe_right() -> Self {
        Self {
            inner: MovementFlags::STRAFE_RIGHT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_strafe_right(mut self) -> Self {
        self.inner |= MovementFlags::STRAFE_RIGHT;
        self
    }

    pub const fn get_strafe_right(&self) -> bool {
        (self.inner & MovementFlags::STRAFE_RIGHT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_strafe_right(mut self) -> Self {
        self.inner &= MovementFlags::STRAFE_RIGHT.reverse_bits();
        self
    }

    pub const fn new_left() -> Self {
        Self {
            inner: MovementFlags::LEFT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_left(mut self) -> Self {
        self.inner |= MovementFlags::LEFT;
        self
    }

    pub const fn get_left(&self) -> bool {
        (self.inner & MovementFlags::LEFT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_left(mut self) -> Self {
        self.inner &= MovementFlags::LEFT.reverse_bits();
        self
    }

    pub const fn new_right() -> Self {
        Self {
            inner: MovementFlags::RIGHT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_right(mut self) -> Self {
        self.inner |= MovementFlags::RIGHT;
        self
    }

    pub const fn get_right(&self) -> bool {
        (self.inner & MovementFlags::RIGHT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_right(mut self) -> Self {
        self.inner &= MovementFlags::RIGHT.reverse_bits();
        self
    }

    pub const fn new_pitch_up() -> Self {
        Self {
            inner: MovementFlags::PITCH_UP,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pitch_up(mut self) -> Self {
        self.inner |= MovementFlags::PITCH_UP;
        self
    }

    pub const fn get_pitch_up(&self) -> bool {
        (self.inner & MovementFlags::PITCH_UP) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pitch_up(mut self) -> Self {
        self.inner &= MovementFlags::PITCH_UP.reverse_bits();
        self
    }

    pub const fn new_pitch_down() -> Self {
        Self {
            inner: MovementFlags::PITCH_DOWN,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pitch_down(mut self) -> Self {
        self.inner |= MovementFlags::PITCH_DOWN;
        self
    }

    pub const fn get_pitch_down(&self) -> bool {
        (self.inner & MovementFlags::PITCH_DOWN) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pitch_down(mut self) -> Self {
        self.inner &= MovementFlags::PITCH_DOWN.reverse_bits();
        self
    }

    pub const fn new_walking() -> Self {
        Self {
            inner: MovementFlags::WALKING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_walking(mut self) -> Self {
        self.inner |= MovementFlags::WALKING;
        self
    }

    pub const fn get_walking(&self) -> bool {
        (self.inner & MovementFlags::WALKING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_walking(mut self) -> Self {
        self.inner &= MovementFlags::WALKING.reverse_bits();
        self
    }

    pub const fn new_disable_gravity() -> Self {
        Self {
            inner: MovementFlags::DISABLE_GRAVITY,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_disable_gravity(mut self) -> Self {
        self.inner |= MovementFlags::DISABLE_GRAVITY;
        self
    }

    pub const fn get_disable_gravity(&self) -> bool {
        (self.inner & MovementFlags::DISABLE_GRAVITY) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_disable_gravity(mut self) -> Self {
        self.inner &= MovementFlags::DISABLE_GRAVITY.reverse_bits();
        self
    }

    pub const fn new_root() -> Self {
        Self {
            inner: MovementFlags::ROOT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_root(mut self) -> Self {
        self.inner |= MovementFlags::ROOT;
        self
    }

    pub const fn get_root(&self) -> bool {
        (self.inner & MovementFlags::ROOT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_root(mut self) -> Self {
        self.inner &= MovementFlags::ROOT.reverse_bits();
        self
    }

    pub const fn new_falling(falling: MovementBlock_MovementFlags_Falling) -> Self {
        Self {
            inner: MovementFlags::FALLING,
            falling: Some(falling),
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_falling(mut self, falling: MovementBlock_MovementFlags_Falling) -> Self {
        self.inner |= MovementFlags::FALLING;
        self.falling = Some(falling);
        self
    }

    pub const fn get_falling(&self) -> Option<&MovementBlock_MovementFlags_Falling> {
        self.falling.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_falling(mut self) -> Self {
        self.inner &= MovementFlags::FALLING.reverse_bits();
        self.falling = None;
        self
    }

    pub const fn new_falling_far() -> Self {
        Self {
            inner: MovementFlags::FALLING_FAR,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_falling_far(mut self) -> Self {
        self.inner |= MovementFlags::FALLING_FAR;
        self
    }

    pub const fn get_falling_far(&self) -> bool {
        (self.inner & MovementFlags::FALLING_FAR) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_falling_far(mut self) -> Self {
        self.inner &= MovementFlags::FALLING_FAR.reverse_bits();
        self
    }

    pub const fn new_pending_stop() -> Self {
        Self {
            inner: MovementFlags::PENDING_STOP,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pending_stop(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STOP;
        self
    }

    pub const fn get_pending_stop(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STOP) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pending_stop(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STOP.reverse_bits();
        self
    }

    pub const fn new_pending_strafe_stop() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_STOP,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pending_strafe_stop(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_STOP;
        self
    }

    pub const fn get_pending_strafe_stop(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_STOP) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pending_strafe_stop(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_STOP.reverse_bits();
        self
    }

    pub const fn new_pending_forward() -> Self {
        Self {
            inner: MovementFlags::PENDING_FORWARD,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pending_forward(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_FORWARD;
        self
    }

    pub const fn get_pending_forward(&self) -> bool {
        (self.inner & MovementFlags::PENDING_FORWARD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pending_forward(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_FORWARD.reverse_bits();
        self
    }

    pub const fn new_pending_backward() -> Self {
        Self {
            inner: MovementFlags::PENDING_BACKWARD,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pending_backward(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_BACKWARD;
        self
    }

    pub const fn get_pending_backward(&self) -> bool {
        (self.inner & MovementFlags::PENDING_BACKWARD) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pending_backward(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_BACKWARD.reverse_bits();
        self
    }

    pub const fn new_pending_strafe_left() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_LEFT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pending_strafe_left(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_LEFT;
        self
    }

    pub const fn get_pending_strafe_left(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_LEFT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pending_strafe_left(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_LEFT.reverse_bits();
        self
    }

    pub const fn new_pending_strafe_right() -> Self {
        Self {
            inner: MovementFlags::PENDING_STRAFE_RIGHT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pending_strafe_right(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_STRAFE_RIGHT;
        self
    }

    pub const fn get_pending_strafe_right(&self) -> bool {
        (self.inner & MovementFlags::PENDING_STRAFE_RIGHT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pending_strafe_right(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_STRAFE_RIGHT.reverse_bits();
        self
    }

    pub const fn new_pending_root() -> Self {
        Self {
            inner: MovementFlags::PENDING_ROOT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pending_root(mut self) -> Self {
        self.inner |= MovementFlags::PENDING_ROOT;
        self
    }

    pub const fn get_pending_root(&self) -> bool {
        (self.inner & MovementFlags::PENDING_ROOT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pending_root(mut self) -> Self {
        self.inner &= MovementFlags::PENDING_ROOT.reverse_bits();
        self
    }

    pub const fn new_swimming(swimming: MovementBlock_MovementFlags_Swimming) -> Self {
        Self {
            inner: swimming.as_int(),
            falling: None,
            swimming: Some(swimming),
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_swimming(mut self, swimming: MovementBlock_MovementFlags_Swimming) -> Self {
        self.inner |= swimming.as_int();
        self.swimming = Some(swimming);
        self
    }

    pub const fn get_swimming(&self) -> Option<&MovementBlock_MovementFlags_Swimming> {
        self.swimming.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_swimming(mut self) -> Self {
        self.inner &= MovementFlags::SWIMMING.reverse_bits();
        self.swimming = None;
        self
    }

    pub const fn new_ascending() -> Self {
        Self {
            inner: MovementFlags::ASCENDING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_ascending(mut self) -> Self {
        self.inner |= MovementFlags::ASCENDING;
        self
    }

    pub const fn get_ascending(&self) -> bool {
        (self.inner & MovementFlags::ASCENDING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_ascending(mut self) -> Self {
        self.inner &= MovementFlags::ASCENDING.reverse_bits();
        self
    }

    pub const fn new_descending() -> Self {
        Self {
            inner: MovementFlags::DESCENDING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_descending(mut self) -> Self {
        self.inner |= MovementFlags::DESCENDING;
        self
    }

    pub const fn get_descending(&self) -> bool {
        (self.inner & MovementFlags::DESCENDING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_descending(mut self) -> Self {
        self.inner &= MovementFlags::DESCENDING.reverse_bits();
        self
    }

    pub const fn new_can_fly() -> Self {
        Self {
            inner: MovementFlags::CAN_FLY,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_can_fly(mut self) -> Self {
        self.inner |= MovementFlags::CAN_FLY;
        self
    }

    pub const fn get_can_fly(&self) -> bool {
        (self.inner & MovementFlags::CAN_FLY) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_can_fly(mut self) -> Self {
        self.inner &= MovementFlags::CAN_FLY.reverse_bits();
        self
    }

    pub const fn new_spline_elevation(spline_elevation: MovementBlock_MovementFlags_SplineElevation) -> Self {
        Self {
            inner: MovementFlags::SPLINE_ELEVATION,
            falling: None,
            swimming: None,
            spline_elevation: Some(spline_elevation),
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_spline_elevation(mut self, spline_elevation: MovementBlock_MovementFlags_SplineElevation) -> Self {
        self.inner |= MovementFlags::SPLINE_ELEVATION;
        self.spline_elevation = Some(spline_elevation);
        self
    }

    pub const fn get_spline_elevation(&self) -> Option<&MovementBlock_MovementFlags_SplineElevation> {
        self.spline_elevation.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_spline_elevation(mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ELEVATION.reverse_bits();
        self.spline_elevation = None;
        self
    }

    pub const fn new_spline_enabled(spline_enabled: MovementBlock_MovementFlags_SplineEnabled) -> Self {
        Self {
            inner: MovementFlags::SPLINE_ENABLED,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: Some(spline_enabled),
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_spline_enabled(mut self, spline_enabled: MovementBlock_MovementFlags_SplineEnabled) -> Self {
        self.inner |= MovementFlags::SPLINE_ENABLED;
        self.spline_enabled = Some(spline_enabled);
        self
    }

    pub const fn get_spline_enabled(&self) -> Option<&MovementBlock_MovementFlags_SplineEnabled> {
        self.spline_enabled.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_spline_enabled(mut self) -> Self {
        self.inner &= MovementFlags::SPLINE_ENABLED.reverse_bits();
        self.spline_enabled = None;
        self
    }

    pub const fn new_waterwalking() -> Self {
        Self {
            inner: MovementFlags::WATERWALKING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_waterwalking(mut self) -> Self {
        self.inner |= MovementFlags::WATERWALKING;
        self
    }

    pub const fn get_waterwalking(&self) -> bool {
        (self.inner & MovementFlags::WATERWALKING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_waterwalking(mut self) -> Self {
        self.inner &= MovementFlags::WATERWALKING.reverse_bits();
        self
    }

    pub const fn new_falling_slow() -> Self {
        Self {
            inner: MovementFlags::FALLING_SLOW,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_falling_slow(mut self) -> Self {
        self.inner |= MovementFlags::FALLING_SLOW;
        self
    }

    pub const fn get_falling_slow(&self) -> bool {
        (self.inner & MovementFlags::FALLING_SLOW) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_falling_slow(mut self) -> Self {
        self.inner &= MovementFlags::FALLING_SLOW.reverse_bits();
        self
    }

    pub const fn new_hover() -> Self {
        Self {
            inner: MovementFlags::HOVER,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_hover(mut self) -> Self {
        self.inner |= MovementFlags::HOVER;
        self
    }

    pub const fn get_hover(&self) -> bool {
        (self.inner & MovementFlags::HOVER) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_hover(mut self) -> Self {
        self.inner &= MovementFlags::HOVER.reverse_bits();
        self
    }

    pub const fn new_no_strafe() -> Self {
        Self {
            inner: MovementFlags::NO_STRAFE,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_no_strafe(mut self) -> Self {
        self.inner |= MovementFlags::NO_STRAFE;
        self
    }

    pub const fn get_no_strafe(&self) -> bool {
        (self.inner & MovementFlags::NO_STRAFE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_no_strafe(mut self) -> Self {
        self.inner &= MovementFlags::NO_STRAFE.reverse_bits();
        self
    }

    pub const fn new_no_jumping() -> Self {
        Self {
            inner: MovementFlags::NO_JUMPING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_no_jumping(mut self) -> Self {
        self.inner |= MovementFlags::NO_JUMPING;
        self
    }

    pub const fn get_no_jumping(&self) -> bool {
        (self.inner & MovementFlags::NO_JUMPING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_no_jumping(mut self) -> Self {
        self.inner &= MovementFlags::NO_JUMPING.reverse_bits();
        self
    }

    pub const fn new_unk3() -> Self {
        Self {
            inner: MovementFlags::UNK3,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk3(mut self) -> Self {
        self.inner |= MovementFlags::UNK3;
        self
    }

    pub const fn get_unk3(&self) -> bool {
        (self.inner & MovementFlags::UNK3) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk3(mut self) -> Self {
        self.inner &= MovementFlags::UNK3.reverse_bits();
        self
    }

    pub const fn new_full_speed_turning() -> Self {
        Self {
            inner: MovementFlags::FULL_SPEED_TURNING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_full_speed_turning(mut self) -> Self {
        self.inner |= MovementFlags::FULL_SPEED_TURNING;
        self
    }

    pub const fn get_full_speed_turning(&self) -> bool {
        (self.inner & MovementFlags::FULL_SPEED_TURNING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_full_speed_turning(mut self) -> Self {
        self.inner &= MovementFlags::FULL_SPEED_TURNING.reverse_bits();
        self
    }

    pub const fn new_full_speed_pitching() -> Self {
        Self {
            inner: MovementFlags::FULL_SPEED_PITCHING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_full_speed_pitching(mut self) -> Self {
        self.inner |= MovementFlags::FULL_SPEED_PITCHING;
        self
    }

    pub const fn get_full_speed_pitching(&self) -> bool {
        (self.inner & MovementFlags::FULL_SPEED_PITCHING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_full_speed_pitching(mut self) -> Self {
        self.inner &= MovementFlags::FULL_SPEED_PITCHING.reverse_bits();
        self
    }

    pub const fn new_unk7() -> Self {
        Self {
            inner: MovementFlags::UNK7,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk7(mut self) -> Self {
        self.inner |= MovementFlags::UNK7;
        self
    }

    pub const fn get_unk7(&self) -> bool {
        (self.inner & MovementFlags::UNK7) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk7(mut self) -> Self {
        self.inner &= MovementFlags::UNK7.reverse_bits();
        self
    }

    pub const fn new_unk8() -> Self {
        Self {
            inner: MovementFlags::UNK8,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk8(mut self) -> Self {
        self.inner |= MovementFlags::UNK8;
        self
    }

    pub const fn get_unk8(&self) -> bool {
        (self.inner & MovementFlags::UNK8) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk8(mut self) -> Self {
        self.inner &= MovementFlags::UNK8.reverse_bits();
        self
    }

    pub const fn new_unk9() -> Self {
        Self {
            inner: MovementFlags::UNK9,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk9(mut self) -> Self {
        self.inner |= MovementFlags::UNK9;
        self
    }

    pub const fn get_unk9(&self) -> bool {
        (self.inner & MovementFlags::UNK9) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk9(mut self) -> Self {
        self.inner &= MovementFlags::UNK9.reverse_bits();
        self
    }

    pub const fn new_unk10() -> Self {
        Self {
            inner: MovementFlags::UNK10,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk10(mut self) -> Self {
        self.inner |= MovementFlags::UNK10;
        self
    }

    pub const fn get_unk10(&self) -> bool {
        (self.inner & MovementFlags::UNK10) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk10(mut self) -> Self {
        self.inner &= MovementFlags::UNK10.reverse_bits();
        self
    }

    pub const fn new_interpolated_movement() -> Self {
        Self {
            inner: MovementFlags::INTERPOLATED_MOVEMENT,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_interpolated_movement(mut self) -> Self {
        self.inner |= MovementFlags::INTERPOLATED_MOVEMENT;
        self
    }

    pub const fn get_interpolated_movement(&self) -> bool {
        (self.inner & MovementFlags::INTERPOLATED_MOVEMENT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_interpolated_movement(mut self) -> Self {
        self.inner &= MovementFlags::INTERPOLATED_MOVEMENT.reverse_bits();
        self
    }

    pub const fn new_interpolated_turning() -> Self {
        Self {
            inner: MovementFlags::INTERPOLATED_TURNING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_interpolated_turning(mut self) -> Self {
        self.inner |= MovementFlags::INTERPOLATED_TURNING;
        self
    }

    pub const fn get_interpolated_turning(&self) -> bool {
        (self.inner & MovementFlags::INTERPOLATED_TURNING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_interpolated_turning(mut self) -> Self {
        self.inner &= MovementFlags::INTERPOLATED_TURNING.reverse_bits();
        self
    }

    pub const fn new_interpolated_pitching() -> Self {
        Self {
            inner: MovementFlags::INTERPOLATED_PITCHING,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_interpolated_pitching(mut self) -> Self {
        self.inner |= MovementFlags::INTERPOLATED_PITCHING;
        self
    }

    pub const fn get_interpolated_pitching(&self) -> bool {
        (self.inner & MovementFlags::INTERPOLATED_PITCHING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_interpolated_pitching(mut self) -> Self {
        self.inner &= MovementFlags::INTERPOLATED_PITCHING.reverse_bits();
        self
    }

    pub const fn new_unk14() -> Self {
        Self {
            inner: MovementFlags::UNK14,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk14(mut self) -> Self {
        self.inner |= MovementFlags::UNK14;
        self
    }

    pub const fn get_unk14(&self) -> bool {
        (self.inner & MovementFlags::UNK14) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk14(mut self) -> Self {
        self.inner &= MovementFlags::UNK14.reverse_bits();
        self
    }

    pub const fn new_unk15() -> Self {
        Self {
            inner: MovementFlags::UNK15,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk15(mut self) -> Self {
        self.inner |= MovementFlags::UNK15;
        self
    }

    pub const fn get_unk15(&self) -> bool {
        (self.inner & MovementFlags::UNK15) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk15(mut self) -> Self {
        self.inner &= MovementFlags::UNK15.reverse_bits();
        self
    }

    pub const fn new_unk16() -> Self {
        Self {
            inner: MovementFlags::UNK16,
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk16(mut self) -> Self {
        self.inner |= MovementFlags::UNK16;
        self
    }

    pub const fn get_unk16(&self) -> bool {
        (self.inner & MovementFlags::UNK16) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk16(mut self) -> Self {
        self.inner &= MovementFlags::UNK16.reverse_bits();
        self
    }

    pub const fn new_on_transport_and_interpolated_movement(on_transport_and_interpolated_movement: MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement) -> Self {
        Self {
            inner: on_transport_and_interpolated_movement.as_int(),
            falling: None,
            swimming: None,
            spline_elevation: None,
            spline_enabled: None,
            on_transport_and_interpolated_movement: Some(on_transport_and_interpolated_movement),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_on_transport_and_interpolated_movement(mut self, on_transport_and_interpolated_movement: MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement) -> Self {
        self.inner |= on_transport_and_interpolated_movement.as_int();
        self.on_transport_and_interpolated_movement = Some(on_transport_and_interpolated_movement);
        self
    }

    pub const fn get_on_transport_and_interpolated_movement(&self) -> Option<&MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement> {
        self.on_transport_and_interpolated_movement.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_on_transport_and_interpolated_movement(mut self) -> Self {
        self.inner &= MovementFlags::ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT.reverse_bits();
        self.on_transport_and_interpolated_movement = None;
        self
    }

    pub(crate) const fn as_int(&self) -> u64 {
        self.inner
    }

}
impl MovementBlock_MovementFlags {
    pub(crate) fn size(&self) -> usize {
        6 // inner
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
        + {
            if let Some(s) = &self.on_transport_and_interpolated_movement {
                s.size()
            } else {
                0
            }
        }
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
    pub(crate) const fn size(&self) -> usize {
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
    pub(crate) const fn size(&self) -> usize {
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
        fall_time: f32,
        flags: MovementBlock_MovementFlags,
        flight_speed: f32,
        orientation: f32,
        pitch_rate: f32,
        position: Vector3d,
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
                // Not an actual enum sent over the wire
                4 // backwards_flight_speed: f32
                + 4 // backwards_running_speed: f32
                + 4 // backwards_swimming_speed: f32
                + 4 // fall_time: f32
                + flags.size() // flags: MovementBlock_MovementFlags
                + 4 // flight_speed: f32
                + 4 // orientation: f32
                + 4 // pitch_rate: f32
                + 12 // position: Vector3d
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

    pub const fn new_self() -> Self {
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_self(mut self) -> Self {
        self.inner |= UpdateFlag::SELF;
        self
    }

    pub const fn get_self(&self) -> bool {
        (self.inner & UpdateFlag::SELF) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_self(mut self) -> Self {
        self.inner &= UpdateFlag::SELF.reverse_bits();
        self
    }

    pub const fn new_transport(transport: MovementBlock_UpdateFlag_Transport) -> Self {
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_transport(mut self, transport: MovementBlock_UpdateFlag_Transport) -> Self {
        self.inner |= UpdateFlag::TRANSPORT;
        self.transport = Some(transport);
        self
    }

    pub const fn get_transport(&self) -> Option<&MovementBlock_UpdateFlag_Transport> {
        self.transport.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_transport(mut self) -> Self {
        self.inner &= UpdateFlag::TRANSPORT.reverse_bits();
        self.transport = None;
        self
    }

    pub const fn new_has_attacking_target(has_attacking_target: MovementBlock_UpdateFlag_HasAttackingTarget) -> Self {
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_has_attacking_target(mut self, has_attacking_target: MovementBlock_UpdateFlag_HasAttackingTarget) -> Self {
        self.inner |= UpdateFlag::HAS_ATTACKING_TARGET;
        self.has_attacking_target = Some(has_attacking_target);
        self
    }

    pub const fn get_has_attacking_target(&self) -> Option<&MovementBlock_UpdateFlag_HasAttackingTarget> {
        self.has_attacking_target.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_has_attacking_target(mut self) -> Self {
        self.inner &= UpdateFlag::HAS_ATTACKING_TARGET.reverse_bits();
        self.has_attacking_target = None;
        self
    }

    pub const fn new_low_guid(low_guid: MovementBlock_UpdateFlag_LowGuid) -> Self {
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_low_guid(mut self, low_guid: MovementBlock_UpdateFlag_LowGuid) -> Self {
        self.inner |= UpdateFlag::LOW_GUID;
        self.low_guid = Some(low_guid);
        self
    }

    pub const fn get_low_guid(&self) -> Option<&MovementBlock_UpdateFlag_LowGuid> {
        self.low_guid.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_low_guid(mut self) -> Self {
        self.inner &= UpdateFlag::LOW_GUID.reverse_bits();
        self.low_guid = None;
        self
    }

    pub const fn new_high_guid(high_guid: MovementBlock_UpdateFlag_HighGuid) -> Self {
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_high_guid(mut self, high_guid: MovementBlock_UpdateFlag_HighGuid) -> Self {
        self.inner |= UpdateFlag::HIGH_GUID;
        self.high_guid = Some(high_guid);
        self
    }

    pub const fn get_high_guid(&self) -> Option<&MovementBlock_UpdateFlag_HighGuid> {
        self.high_guid.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_high_guid(mut self) -> Self {
        self.inner &= UpdateFlag::HIGH_GUID.reverse_bits();
        self.high_guid = None;
        self
    }

    pub const fn new_living(living: MovementBlock_UpdateFlag_Living) -> Self {
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_living(mut self, living: MovementBlock_UpdateFlag_Living) -> Self {
        self.inner |= living.as_int();
        self.living = Some(living);
        self
    }

    pub const fn get_living(&self) -> Option<&MovementBlock_UpdateFlag_Living> {
        self.living.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_living(mut self) -> Self {
        self.inner &= UpdateFlag::LIVING.reverse_bits();
        self.living = None;
        self
    }

    pub const fn new_vehicle(vehicle: MovementBlock_UpdateFlag_Vehicle) -> Self {
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_vehicle(mut self, vehicle: MovementBlock_UpdateFlag_Vehicle) -> Self {
        self.inner |= UpdateFlag::VEHICLE;
        self.vehicle = Some(vehicle);
        self
    }

    pub const fn get_vehicle(&self) -> Option<&MovementBlock_UpdateFlag_Vehicle> {
        self.vehicle.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_vehicle(mut self) -> Self {
        self.inner &= UpdateFlag::VEHICLE.reverse_bits();
        self.vehicle = None;
        self
    }

    pub const fn new_rotation(rotation: MovementBlock_UpdateFlag_Rotation) -> Self {
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_rotation(mut self, rotation: MovementBlock_UpdateFlag_Rotation) -> Self {
        self.inner |= UpdateFlag::ROTATION;
        self.rotation = Some(rotation);
        self
    }

    pub const fn get_rotation(&self) -> Option<&MovementBlock_UpdateFlag_Rotation> {
        self.rotation.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_rotation(mut self) -> Self {
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
    pub(crate) const fn size(&self) -> usize {
        4 // transport_progress_in_ms: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MovementBlock_UpdateFlag_HasAttackingTarget {
    pub guid: Guid,
}

impl MovementBlock_UpdateFlag_HasAttackingTarget {
    pub(crate) const fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MovementBlock_UpdateFlag_LowGuid {
    pub unknown1: u32,
}

impl MovementBlock_UpdateFlag_LowGuid {
    pub(crate) const fn size(&self) -> usize {
        4 // unknown1: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MovementBlock_UpdateFlag_HighGuid {
    pub unknown0: u32,
}

impl MovementBlock_UpdateFlag_HighGuid {
    pub(crate) const fn size(&self) -> usize {
        4 // unknown0: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MovementBlock_UpdateFlag_Vehicle {
    pub vehicle_id: u32,
    pub vehicle_orientation: f32,
}

impl MovementBlock_UpdateFlag_Vehicle {
    pub(crate) const fn size(&self) -> usize {
        4 // vehicle_id: u32
        + 4 // vehicle_orientation: f32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MovementBlock_UpdateFlag_Rotation {
    pub packed_local_rotation: u64,
}

impl MovementBlock_UpdateFlag_Rotation {
    pub(crate) const fn size(&self) -> usize {
        8 // packed_local_rotation: u64
    }
}

