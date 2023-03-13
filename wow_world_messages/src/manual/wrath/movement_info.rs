use crate::wrath::{
    MovementBlock_MovementFlags, MovementBlock_MovementFlags_Falling,
    MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement,
    MovementBlock_MovementFlags_SplineElevation, MovementBlock_MovementFlags_SplineEnabled,
    MovementBlock_MovementFlags_Swimming, MovementBlock_UpdateFlag_Living, MovementInfo,
    MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement,
    MovementInfo_MovementFlags_Swimming,
};

impl MovementInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn into_movement_block_update_flag_living(
        &self,
        backwards_flight_speed: f32,
        backwards_running_speed: f32,
        backwards_swimming_speed: f32,
        flight_speed: f32,
        pitch_rate: f32,
        running_speed: f32,
        swimming_speed: f32,
        turn_rate: f32,
        walking_speed: f32,
        spline_enabled: Option<MovementBlock_MovementFlags_SplineEnabled>,
    ) -> MovementBlock_UpdateFlag_Living {
        let on_transport =
            self.flags
                .get_ON_TRANSPORT_AND_INTERPOLATED_MOVEMENT()
                .map(|t| match t {
                    MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement { transport_info, transport_time } => {
                         MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                             transport_info: *transport_info,
                             transport_time: *transport_time,
                         }
                    }
                    MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport { transport } => {
                        MovementBlock_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                            transport: *transport,
                        }
                    }
                });

        let falling = self
            .flags
            .get_FALLING()
            .map(|t| MovementBlock_MovementFlags_Falling {
                cos_angle: t.cos_angle,
                sin_angle: t.sin_angle,
                xy_speed: t.xy_speed,
                z_speed: t.z_speed,
            });

        let swimming = self.flags.get_SWIMMING().map(|t| match t {
            MovementInfo_MovementFlags_Swimming::Swimming { pitch1 } => {
                MovementBlock_MovementFlags_Swimming::Swimming { pitch1: *pitch1 }
            }
            MovementInfo_MovementFlags_Swimming::Flying { pitch2 } => {
                MovementBlock_MovementFlags_Swimming::Flying { pitch2: *pitch2 }
            }
            MovementInfo_MovementFlags_Swimming::AlwaysAllowPitching { pitch3 } => {
                MovementBlock_MovementFlags_Swimming::AlwaysAllowPitching { pitch3: *pitch3 }
            }
        });

        let spline_elevation = self.flags.get_SPLINE_ELEVATION().map(|t| {
            MovementBlock_MovementFlags_SplineElevation {
                spline_elevation: t.spline_elevation,
            }
        });

        MovementBlock_UpdateFlag_Living::Living {
            timestamp: self.timestamp,
            fall_time: self.fall_time,
            flags: MovementBlock_MovementFlags::new(
                self.flags.as_int(),
                falling,
                swimming,
                spline_elevation,
                spline_enabled,
                on_transport,
            ),

            backwards_flight_speed,
            backwards_running_speed,
            backwards_swimming_speed,
            flight_speed,
            orientation: self.orientation,
            pitch_rate,
            position: self.position,
            running_speed,
            swimming_speed,
            turn_rate,
            walking_speed,
        }
    }
}
