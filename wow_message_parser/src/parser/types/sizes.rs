use crate::parser::types::version::MajorWorldVersion;
use crate::rust_printer::{tbc_fields, vanilla_fields, wrath_fields};
use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Sizes {
    minimum: usize,
    maximum: usize,
}

pub(crate) const AURA_MASK_MAX_SIZE: u8 = 4 + 32 * 4;
pub(crate) const AURA_MASK_MIN_SIZE: u8 = 4;

pub(crate) const fn update_mask_max(version: MajorWorldVersion) -> u16 {
    let data = match version {
        MajorWorldVersion::Vanilla => vanilla_fields::FIELDS,
        MajorWorldVersion::BurningCrusade => tbc_fields::FIELDS,
        MajorWorldVersion::Wrath => wrath_fields::FIELDS.as_slice(),
    };

    let mut i = 0;
    let mut biggest = &data[i];
    while i < data.len() {
        if (biggest.offset() + biggest.size()) > data[i].offset() + data[i].size() {
            biggest = &data[i];
        }
        i += 1;
    }

    let amount_of_bytes_for_data = biggest.offset() + biggest.size();
    let amount_of_mask_blocks_size = core::mem::size_of::<u32>() as i32;

    let mut max_mask_blocks = amount_of_bytes_for_data / 8;
    if (amount_of_bytes_for_data % 8) > 0 {
        max_mask_blocks += 1;
    }

    (amount_of_mask_blocks_size + max_mask_blocks + amount_of_bytes_for_data) as u16
}

pub(crate) const UPDATE_MASK_MIN_SIZE: u8 = 1;
pub(crate) const PACKED_GUID_MAX_SIZE: u8 = 9;
pub(crate) const PACKED_GUID_MIN_SIZE: u8 = 2;
pub(crate) const GUID_SIZE: u8 = core::mem::size_of::<u64>() as u8;
pub(crate) const GOLD_SIZE: u8 = core::mem::size_of::<u32>() as u8;
pub(crate) const LEVEL_SIZE: u8 = core::mem::size_of::<u8>() as u8;
pub(crate) const LEVEL16_SIZE: usize = core::mem::size_of::<u16>();
pub(crate) const LEVEL32_SIZE: usize = core::mem::size_of::<u32>();

pub(crate) const DATETIME_SIZE: u8 = 4;

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
