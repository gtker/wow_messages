use std::collections::BTreeMap;
use std::io::{Read, Write};

pub const OBJECT: u32 = 0x0001;
pub const ITEM: u32 = 0x0002;
pub const CONTAINER: u32 = 0x0004;
pub const UNIT: u32 = 0x0008;
pub const PLAYER: u32 = 0x0010;
pub const GAMEOBJECT: u32 = 0x0020;
pub const DYNAMICOBJECT: u32 = 0x0040;
pub const CORPSE: u32 = 0x0080;

pub(crate) fn array_set(array: &mut Vec<u32>, bit: u16) {
    let index = bit / 32;
    let offset = bit % 32;

    if index >= array.len() as u16 {
        let extras = index - array.len() as u16;
        for _ in 0..=extras {
            array.push(0);
        }
    }

    array[index as usize] |= 1 << offset;
}

pub(crate) fn array_reset(array: &mut [u32]) {
    for item in array {
        *item = 0;
    }
}

pub(crate) fn array_fill_ones(array: &mut [u32]) {
    for item in array {
        *item = 0xFFFFFFFF;
    }
}

pub(crate) fn has_array_bit_set(array: &[u32], bit: u16) -> bool {
    let index = bit / 32;
    let offset = bit % 32;
    let item = array[index as usize];

    item & (1 << offset) > 0
}

pub(crate) fn has_any_bit_set(array: &[u32]) -> bool {
    array.iter().any(|h| *h != 0)
}

pub(crate) fn write_into_vec(
    v: &mut Vec<u8>,
    header: &[u32],
    dirty_mask: &[u32],
    values: &BTreeMap<u16, u32>,
) -> Result<(), std::io::Error> {
    assert_eq!(header.len(), dirty_mask.len());

    v.write_all(&[header.len() as u8])?;

    for (h, d) in header.iter().zip(dirty_mask) {
        let masked = h & d;
        v.write_all(masked.to_le_bytes().as_slice())?;
    }

    for (&index, value) in values.iter() {
        if has_array_bit_set(header, index) && has_array_bit_set(dirty_mask, index) {
            v.write_all(&value.to_le_bytes())?;
        }
    }

    Ok(())
}

pub(crate) fn read_inner(
    r: &mut impl Read,
) -> Result<(Vec<u32>, BTreeMap<u16, u32>), std::io::Error> {
    let amount_of_blocks = crate::util::read_u8_le(r)?;

    let mut header = Vec::new();
    for _ in 0..amount_of_blocks {
        header.push(crate::util::read_u32_le(r)?);
    }

    let mut values = BTreeMap::new();
    for (index, block) in header.iter().enumerate() {
        for bit in 0..32 {
            if (block & 1 << bit) != 0 {
                values.insert(index as u16 * 32 + bit, crate::util::read_u32_le(r)?);
            }
        }
    }

    Ok((header, values))
}

macro_rules! update_item {
    ($name:ident, $builder_name:ident, $type_value:expr) => {
        #[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
        pub struct $builder_name {
            header: Vec<u32>,
            values: BTreeMap<u16, u32>,
        }

        impl $builder_name {
            pub fn finalize(self) -> $name {
                $name::from_inners(self.header, self.values)
            }

            pub(crate) fn header_set(&mut self, bit: u16) {
                $crate::helper::update_mask_common::array_set(&mut self.header, bit);
            }

            pub fn new() -> Self {
                const OBJECT_FIELD_TYPE: u16 = 2;

                let mut header = vec![];
                let mut values = BTreeMap::new();

                $crate::helper::update_mask_common::array_set(&mut header, OBJECT_FIELD_TYPE);
                values.insert(
                    OBJECT_FIELD_TYPE,
                    $crate::helper::update_mask_common::OBJECT | $type_value,
                );

                Self { header, values }
            }
        }

        #[derive(Debug, Hash, Clone, Default, PartialEq, Eq)]
        pub struct $name {
            header: Vec<u32>,
            dirty_mask: Vec<u32>,
            values: BTreeMap<u16, u32>,
        }

        impl $name {
            pub fn new() -> Self {
                const OBJECT_FIELD_TYPE: u16 = 2;

                let mut header = vec![];
                let mut dirty_mask = vec![];
                let mut values = BTreeMap::new();

                $crate::helper::update_mask_common::array_set(&mut header, OBJECT_FIELD_TYPE);
                $crate::helper::update_mask_common::array_set(&mut dirty_mask, OBJECT_FIELD_TYPE);

                values.insert(
                    OBJECT_FIELD_TYPE,
                    $crate::helper::update_mask_common::OBJECT | $type_value,
                );

                Self {
                    header,
                    dirty_mask,
                    values,
                }
            }

            pub fn builder() -> $builder_name {
                $builder_name::new()
            }

            fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
                Self {
                    header: header.clone(),
                    dirty_mask: header,
                    values,
                }
            }

            pub(crate) fn header_set(&mut self, bit: u16) {
                $crate::helper::update_mask_common::array_set(&mut self.header, bit);
                //Any modification to the header also means we set it dirty
                $crate::helper::update_mask_common::array_set(&mut self.dirty_mask, bit);
            }

            pub fn dirty_reset(&mut self) {
                $crate::helper::update_mask_common::array_reset(&mut self.dirty_mask);
            }

            pub fn mark_fully_dirty(&mut self) {
                $crate::helper::update_mask_common::array_fill_ones(&mut self.dirty_mask);
            }

            pub fn has_any_dirty_fields(&self) -> bool {
                $crate::helper::update_mask_common::has_any_bit_set(&self.dirty_mask)
            }

            pub fn has_specific_bit_set(&self, bit: u16) -> bool {
                $crate::helper::update_mask_common::has_array_bit_set(&self.header, bit)
            }

            pub fn is_bit_dirty(&self, bit: u16) -> bool {
                $crate::helper::update_mask_common::has_array_bit_set(&self.dirty_mask, bit)
            }

            pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
                $crate::helper::update_mask_common::write_into_vec(
                    v,
                    &self.header,
                    &self.dirty_mask,
                    &self.values,
                )
            }
        }
    };
}

pub(crate) use update_item;
