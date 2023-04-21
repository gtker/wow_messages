mod triggers;

use crate::extended::shared::{area_trigger, tbc_wrath_trigger, verify_trigger};
use crate::tbc::position::Position;
use crate::tbc::trigger::triggers::TRIGGERS;

tbc_wrath_trigger!(Position);

area_trigger!(Position);

verify_trigger!(
    &'static (AreaTrigger, &'static [Trigger]),
    TRIGGERS,
    Position
);
