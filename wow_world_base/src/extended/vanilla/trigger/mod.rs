mod triggers;

use crate::extended::shared::{area_trigger, vanilla_trigger, verify_trigger};
use crate::geometry::{is_within_distance, is_within_square};
use crate::vanilla::position::Position;
use crate::vanilla::trigger::triggers::TRIGGERS;

vanilla_trigger!();

area_trigger!();

verify_trigger!();
