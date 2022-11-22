mod triggers;

use crate::extended::shared::{area_trigger, tbc_wrath_trigger, verify_trigger};
use crate::geometry::{distance_between, is_within_distance, is_within_square};
use crate::tbc::position::Position;
use crate::tbc::trigger::triggers::TRIGGERS;
use crate::tbc::Vector3d;

tbc_wrath_trigger!();

area_trigger!();

verify_trigger!();
