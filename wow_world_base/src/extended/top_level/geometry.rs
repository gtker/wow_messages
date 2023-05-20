use crate::shared::vector2d_vanilla_tbc_wrath::Vector2d;
use crate::shared::vector3d_vanilla_tbc_wrath::Vector3d;
use std::f32::consts::PI;

/// Extends a point by an `angle` in degrees and a `distance`.
pub fn trace_point_2d(from: Vector2d, angle: f32, distance: f32) -> (f32, f32) {
    (
        from.x + (distance * angle.cos()),
        from.y + (distance * angle.sin()),
    )
}

/// Checks if two three-dimensional points are within a `distance`.
pub fn is_within_distance(from: Vector3d, to: Vector3d, distance: f32) -> bool {
    distance_between(from, to) < distance
}

/// Returns the distance between two three-dimensional points.
pub fn distance_between(from: Vector3d, to: Vector3d) -> f32 {
    let delta_x = from.x - to.x;
    let delta_y = from.y - to.y;
    let delta_z = from.z - to.z;

    ((delta_x * delta_x) + (delta_y * delta_y) + (delta_z * delta_z)).sqrt()
}

/// Returns the distance between two two-dimensional points.
pub fn distance_2d(from: Vector2d, to: Vector2d) -> f32 {
    let delta_x = from.x - to.x;
    let delta_y = from.y - to.y;

    ((delta_x * delta_x) + (delta_y * delta_y)).sqrt()
}

/// Checks whether a player is within a square trigger box.
///
/// Should not be used directly, but instead the `contains` method of the different `AreaTrigger`s.
pub fn is_within_square(
    player: Vector3d,
    square: Vector3d,
    length: f32,
    width: f32,
    height: f32,
    yaw: f32,
) -> bool {
    /*
       // 2PI = 360, keep in mind that ingame orientation is counter-clockwise
       double rotation = 2 * M_PI - atEntry->box_orientation;
       double sinVal = sin(rotation);
       double cosVal = cos(rotation);

       float playerBoxDistX = x - atEntry->x;
       float playerBoxDistY = y - atEntry->y;

       float rotPlayerX = float(atEntry->x + playerBoxDistX * cosVal - playerBoxDistY * sinVal);
       float rotPlayerY = float(atEntry->y + playerBoxDistY * cosVal + playerBoxDistX * sinVal);

       // box edges are parallel to coordiante axis, so we can treat every dimension independently :D
       float dz = z - atEntry->z;
       float dx = rotPlayerX - atEntry->x;
       float dy = rotPlayerY - atEntry->y;
       if ((fabs(dx) > atEntry->box_x / 2 + delta) ||
               (fabs(dy) > atEntry->box_y / 2 + delta) ||
               (fabs(dz) > atEntry->box_z / 2 + delta))
           return false;
    */
    let rotation = 2.0 * PI - yaw;
    let sin = rotation.sin();
    let cos = rotation.cos();

    let player_box_dist_x = player.x - square.x;
    let player_box_dist_y = player.y - square.y;

    let player_rotated_x = square.x + player_box_dist_x * cos - player_box_dist_y * sin;
    let player_rotated_y = square.y + player_box_dist_y * cos - player_box_dist_x * sin;

    let dx = player_rotated_x - square.x;
    let dy = player_rotated_y - square.y;
    let dz = player.z - square.z;

    const DELTA: f32 = 2.0;

    !(dx.abs() > length / 2.0 + DELTA
        || dy.abs() > width / 2.0 + DELTA
        || dz.abs() > height / 2.0 + DELTA)
}

/// Distance within which chat in say (`/say`) can be heard.
pub const SAY: f32 = 25.0;
/// Distance within which chat in emotes (`/emote`) can be heard.
pub const EMOTE: f32 = 25.0;
/// Distance within which chat in yell (`/yell`) can be heard.
pub const YELL: f32 = 300.0;

/// Maximum range that stealth can be detected.
/// Outside of this range stealth will never be detected.
/// This is valid for both players and creatures.
pub const STEALTH_DETECTION: f32 = 30.0;

/// Distance within which player to player trades can occur.
pub const TRADE: f32 = 11.11;

/// Distance for general interaction like accepting quest, opening vendor screen, or
/// opening auctioneer menu.
pub const INTERACTION: f32 = 5.0;

/// Distance for base melee attack range.
pub const MELEE_ATTACK: f32 = 5.0;
