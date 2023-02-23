use crate::wrath::{
    MovementBlock_MovementFlags, MovementBlock_MovementFlags_Falling,
    MovementBlock_MovementFlags_OnTransport, MovementBlock_MovementFlags_SplineElevation,
    MovementBlock_MovementFlags_SplineEnabled, MovementBlock_MovementFlags_Swimming,
    MovementBlock_UpdateFlag_Living, MovementInfo, MovementInfo_MovementFlags_Swimming,
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
                .get_ON_TRANSPORT()
                .map(|t| MovementBlock_MovementFlags_OnTransport {
                    transport: t.transport,
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

        let swimming = if let Some(t) = self.flags.get_SWIMMING() {
            let pitch = match t {
                MovementInfo_MovementFlags_Swimming::Swimming { pitch1: pitch }
                | MovementInfo_MovementFlags_Swimming::Flying { pitch2: pitch } => *pitch,
            };
            Some(MovementBlock_MovementFlags_Swimming { pitch })
        } else {
            None
        };

        let spline_elevation = self.flags.get_SPLINE_ELEVATION().map(|t| {
            MovementBlock_MovementFlags_SplineElevation {
                spline_elevation: t.spline_elevation,
            }
        });

        MovementBlock_UpdateFlag_Living::Living {
            living_orientation: self.orientation,
            living_position: self.position,
            timestamp: self.timestamp,
            extra_flags: self.extra_flags,
            fall_time: self.fall_time,
            flags: MovementBlock_MovementFlags::new(
                self.flags.as_int(),
                on_transport,
                falling,
                swimming,
                spline_elevation,
                spline_enabled,
            ),

            backwards_flight_speed,
            backwards_running_speed,
            backwards_swimming_speed,
            flight_speed,
            pitch_rate,
            running_speed,
            swimming_speed,
            turn_rate,
            walking_speed,
        }
    }
}
