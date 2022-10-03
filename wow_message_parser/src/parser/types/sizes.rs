use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Sizes {
    minimum: usize,
    maximum: usize,
}

pub const AURA_MASK_MAX_SIZE: u8 = 4 + 32 * 4;
pub const AURA_MASK_MIN_SIZE: u8 = 4;

const fn update_mask_max() -> u16 {
    let amount_of_bytes_for_data = 0x501; // PLAYER_END
    let amount_of_mask_blocks_size = core::mem::size_of::<u32>() as i32;

    let mut max_mask_blocks = amount_of_bytes_for_data / 8;
    if (amount_of_bytes_for_data % 8) > 0 {
        max_mask_blocks += 1;
    }

    (amount_of_mask_blocks_size + max_mask_blocks + amount_of_bytes_for_data) as u16
}

pub const UPDATE_MASK_MAX_SIZE: u16 = update_mask_max();
pub const UPDATE_MASK_MIN_SIZE: u8 = 1;
pub const PACKED_GUID_MAX_SIZE: u8 = 9;
pub const PACKED_GUID_MIN_SIZE: u8 = 2;
pub const GUID_SIZE: u8 = core::mem::size_of::<u64>() as u8;
pub const BOOL_SIZE: u8 = 1;
pub const DATETIME_SIZE: u8 = 4;

impl Sizes {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn inc(&mut self, minimum: usize, maximum: usize) {
        self.minimum = self.minimum.saturating_add(minimum);
        self.maximum = self.maximum.saturating_add(maximum);
    }

    pub(crate) fn inc_both(&mut self, v: usize) {
        self.inc(v, v);
    }

    pub(crate) fn minimum(&self) -> usize {
        self.minimum
    }

    pub(crate) fn maximum(&self) -> usize {
        self.maximum
    }

    pub(crate) fn set_maximum(&mut self, maximum: usize) {
        self.maximum = maximum;
    }

    pub(crate) fn set_minimum(&mut self, minimum: usize) {
        self.minimum = minimum;
    }

    pub(crate) fn is_constant(&self) -> Option<usize> {
        if self.minimum == self.maximum {
            Some(self.maximum())
        } else {
            None
        }
    }
}

impl AddAssign for Sizes {
    fn add_assign(&mut self, rhs: Self) {
        self.inc(rhs.minimum, rhs.maximum);
    }
}
