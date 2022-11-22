mod triggers;

use crate::extended::shared::{area_trigger, tbc_wrath_trigger, verify_trigger};
use crate::geometry::{distance_between, is_within_distance, is_within_square};
use crate::wrath::position::Position;
use crate::wrath::trigger::triggers::TRIGGERS;
use crate::wrath::Vector3d;

tbc_wrath_trigger!();

area_trigger!();

verify_trigger!();
