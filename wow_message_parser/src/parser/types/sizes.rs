use crate::parser::types::version::MajorWorldVersion;
use crate::rust_printer::{tbc_fields, vanilla_fields, wrath_fields};
use std::ops::AddAssign;

#[derive(Debug, Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Sizes {
    minimum: i128,
    maximum: i128,
}

pub(crate) const AURA_MASK_MAX_SIZE: u8 = 4 + 32 * 4;
pub(crate) const AURA_MASK_MIN_SIZE: u8 = 4;

pub(crate) const fn update_mask_max(version: MajorWorldVersion) -> i128 {
    let data = match version {
        MajorWorldVersion::Vanilla => vanilla_fields::FIELDS,
        MajorWorldVersion::BurningCrusade => tbc_fields::FIELDS,
        MajorWorldVersion::Wrath => wrath_fields::FIELDS,
    };

    let mut i = 0;
    let mut biggest = &data[i];
    let size = biggest.size() as i128;

    while i < data.len() {
        if (biggest.offset() as i128 + size) > data[i].offset() as i128 + data[i].size() as i128 {
            biggest = &data[i];
        }
        i += 1;
    }

    let amount_of_bytes_for_data = biggest.offset() as i128 + size;
    let amount_of_mask_blocks_size = core::mem::size_of::<u32>() as i128;

    let mut max_mask_blocks = amount_of_bytes_for_data / 8;
    if (amount_of_bytes_for_data % 8) > 0 {
        max_mask_blocks += 1;
    }

    amount_of_mask_blocks_size + max_mask_blocks + amount_of_bytes_for_data
}

pub(crate) const F32_SIZE: u8 = 4;
pub(crate) const UPDATE_MASK_MIN_SIZE: u8 = 1;
pub(crate) const PACKED_GUID_MAX_SIZE: u8 = 9;
pub(crate) const PACKED_GUID_MIN_SIZE: u8 = 2;
pub(crate) const NAMED_GUID_MIN_SIZE: u8 = 8;
pub(crate) const NAMED_GUID_MAX_SIZE: u16 = 8008;
pub(crate) const GUID_SIZE: u8 = 8;
pub(crate) const GOLD_SIZE: u8 = 4;
pub(crate) const LEVEL_SIZE: u8 = 1;
pub(crate) const LEVEL16_SIZE: u8 = 2;
pub(crate) const LEVEL32_SIZE: u8 = 4;
pub(crate) const SECONDS_SIZE: u8 = 4;
pub(crate) const MILLISECONDS_SIZE: u8 = 4;
pub(crate) const IP_ADDRESS_SIZE: u8 = 4;
pub(crate) const POPULATION_SIZE: u8 = 4;
pub(crate) const VARIABLE_ITEM_RANDOM_PROPERTY_MIN_SIZE: u8 = 4;
pub(crate) const VARIABLE_ITEM_RANDOM_PROPERTY_MAX_SIZE: u8 = 8;
pub(crate) const ADDON_ARRAY_MIN: u8 = 0;
pub(crate) const ADDON_ARRAY_MAX: i128 = usize::MAX as i128;

pub(crate) const DATETIME_SIZE: u8 = 4;

impl Sizes {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn inc(&mut self, minimum: i128, maximum: i128) {
        self.minimum = self.minimum.saturating_add(minimum);
        self.maximum = self.maximum.saturating_add(maximum);
    }

    pub(crate) fn inc_both(&mut self, v: i128) {
        self.inc(v, v);
    }

    pub(crate) fn minimum(&self) -> i128 {
        self.minimum
    }

    pub(crate) fn maximum(&self) -> i128 {
        self.maximum
    }

    pub(crate) fn set_maximum(&mut self, maximum: i128) {
        self.maximum = maximum;
    }

    pub(crate) fn set_minimum(&mut self, minimum: i128) {
        self.minimum = minimum;
    }

    pub(crate) fn is_constant(&self) -> Option<i128> {
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
