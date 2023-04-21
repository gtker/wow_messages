mod triggers;

use crate::extended::shared::{area_trigger, tbc_wrath_trigger, verify_trigger};
use crate::geometry::{is_within_distance, is_within_square};
use crate::tbc::position::Position;
use crate::tbc::trigger::triggers::TRIGGERS;

tbc_wrath_trigger!();

area_trigger!();

verify_trigger!();
