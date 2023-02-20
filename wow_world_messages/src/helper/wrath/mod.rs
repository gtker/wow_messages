pub use crate::helper::shared::wrath::*;

pub(crate) mod expected;
pub use expected::*;

mod aura_mask;
pub use aura_mask::*;

#[allow(clippy::missing_panics_doc)]
pub(crate) mod update_mask;
pub use update_mask::*;

pub(crate) mod opcode_to_name;

mod achievement_done_array;
pub use achievement_done_array::*;

mod achievement_in_progress_array;
pub use achievement_in_progress_array::*;

mod enchant_mask;
pub use enchant_mask::*;

mod inspect_talent_gear_mask;
pub use inspect_talent_gear_mask::*;

pub use crate::helper::shared::monster_move_spline_vanilla_tbc_wrath::*;

pub use opcode_to_name::*;

pub(crate) const ACHIEVEMENT_SENTINEL_VALUE: u32 = u32::from_le_bytes((-1_i32).to_le_bytes());
