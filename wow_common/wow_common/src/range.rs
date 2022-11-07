use wow_world_base::tbc::Vector2d;
use wow_world_base::vanilla::Vector3d;

pub fn trace_point_2d(from: Vector2d, angle: f32, distance: f32) -> (f32, f32) {
    (
        from.x + (distance * angle.cos()),
        from.y + (distance * angle.sin()),
    )
}

pub fn is_within_distance(from: Vector3d, to: Vector3d, distance: f32) -> bool {
    distance_between(from, to) < distance
}

pub fn distance_between(from: Vector3d, to: Vector3d) -> f32 {
    let delta_x = from.x - to.x;
    let delta_y = from.y - to.y;
    let delta_z = from.z - to.z;

    ((delta_x * delta_x) + (delta_y * delta_y) + (delta_z * delta_z)).sqrt()
}

pub fn distance_2d(from: Vector2d, to: Vector2d) -> f32 {
    let delta_x = from.x - to.x;
    let delta_y = from.y - to.y;

    ((delta_x * delta_x) + (delta_y * delta_y)).sqrt()
}

pub const SAY: f32 = 25.0;
pub const EMOTE: f32 = 25.0;
pub const YELL: f32 = 300.0;

/// Maximum range that stealth can be detected.
/// Outside of this range stealth will never be detected.
/// This is valid for both players and creatures.
pub const STEALTH_DETECTION: f32 = 30.0;

pub const TRADE: f32 = 11.11;

pub const INTERACTION: f32 = 5.0;

pub const MELEE_ATTACK: f32 = 5.0;
