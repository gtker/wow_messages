use crate::vanilla::{
    MovementBlock_MovementFlags, MovementBlock_MovementFlags_Jumping,
    MovementBlock_MovementFlags_OnTransport, MovementBlock_MovementFlags_SplineElevation,
    MovementBlock_MovementFlags_SplineEnabled, MovementBlock_MovementFlags_Swimming,
    MovementBlock_UpdateFlag_Living, MovementInfo,
};

impl MovementInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn to_movement_block_update_flag_living(
        &self,
        backwards_running_speed: f32,
        backwards_swimming_speed: f32,
        running_speed: f32,
        swimming_speed: f32,
        turn_rate: f32,
        walking_speed: f32,
        spline_enabled: Option<MovementBlock_MovementFlags_SplineEnabled>,
    ) -> MovementBlock_UpdateFlag_Living {
        let on_transport =
            self.flags
                .get_on_transport()
                .map(|t| MovementBlock_MovementFlags_OnTransport {
                    transport_guid: t.transport.guid,
                    transport_orientation: t.transport.orientation,
                    transport_position: t.transport.position,
                });

        let jumping = self
            .flags
            .get_jumping()
            .map(|t| MovementBlock_MovementFlags_Jumping {
                cos_angle: t.cos_angle,
                sin_angle: t.sin_angle,
                xy_speed: t.xy_speed,
                z_speed: t.z_speed,
            });

        let swimming = self
            .flags
            .get_swimming()
            .map(|t| MovementBlock_MovementFlags_Swimming { pitch: t.pitch });

        let spline_elevation = self.flags.get_spline_elevation().map(|t| {
            MovementBlock_MovementFlags_SplineElevation {
                spline_elevation: t.spline_elevation,
            }
        });

        MovementBlock_UpdateFlag_Living::Living {
            living_orientation: self.orientation,
            living_position: self.position,
            timestamp: self.timestamp,
            fall_time: self.fall_time,
            flags: MovementBlock_MovementFlags::new(
                self.flags.as_int(),
                on_transport,
                jumping,
                swimming,
                spline_enabled,
                spline_elevation,
            ),

            backwards_running_speed,
            backwards_swimming_speed,
            running_speed,
            swimming_speed,
            turn_rate,
            walking_speed,
        }
    }
}
