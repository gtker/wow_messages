mod player_race;
mod race_class;

use crate::tbc::Race;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

pub use crate::manual::PlayerGender;

pub use player_race::*;
pub use race_class::*;
