mod triggers;

use crate::extended::shared::{area_trigger, vanilla_trigger, verify_trigger};
use crate::geometry::{distance_between, is_within_distance, is_within_square};
use crate::vanilla::position::Position;
use crate::vanilla::trigger::triggers::TRIGGERS;
use crate::vanilla::Vector3d;

vanilla_trigger!();

area_trigger!();

verify_trigger!();
