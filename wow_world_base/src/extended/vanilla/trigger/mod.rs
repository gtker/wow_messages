mod triggers;

use crate::extended::shared::{area_trigger, verify_trigger};
use crate::vanilla::position::Position;
use crate::vanilla::trigger::triggers::TRIGGERS;
use crate::vanilla::vanilla_trigger;

vanilla_trigger!(Position);

area_trigger!(Position);

verify_trigger!(
    &'static (AreaTrigger, &'static [Trigger]),
    TRIGGERS,
    Position
);
