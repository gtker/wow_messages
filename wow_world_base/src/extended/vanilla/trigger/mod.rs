mod triggers;

use crate::extended::shared::{area_trigger, vanilla_trigger, verify_trigger};
use crate::vanilla::position::Position;
use crate::vanilla::trigger::triggers::TRIGGERS;

vanilla_trigger!(Position);

area_trigger!(Position);

verify_trigger!(
    &'static (AreaTrigger, &'static [Trigger]),
    TRIGGERS,
    Position
);
