mod triggers;

use crate::extended::shared::{area_trigger, tbc_wrath_trigger, verify_trigger};
use crate::wrath::position::Position;
use crate::wrath::trigger::triggers::TRIGGERS;

tbc_wrath_trigger!(Position);

area_trigger!(Position);

verify_trigger!(
    &'static (AreaTrigger, &'static [Trigger]),
    TRIGGERS,
    Position
);
