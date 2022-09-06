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

pub(crate) fn header_set(header: &mut Vec<u32>, values: &mut BTreeMap<u16, u32>, bit: u16) {
    let index = bit / 32;
    let offset = bit % 32;

    if index >= header.len() as u16 {
        let extras = index - header.len() as u16;
        for _ in 0..=extras {
            header.push(0);
        }
    }

    header[index as usize] |= 1 << offset;
}

pub(crate) fn write_into_vec(
    v: &mut Vec<u8>,
    header: &[u32],
    values: &BTreeMap<u16, u32>,
) -> Result<(), std::io::Error> {
    v.write_all(&[header.len() as u8])?;

    for h in header {
        v.write_all(h.to_le_bytes().as_slice())?;
    }

    for value in values.values() {
        v.write_all(&value.to_le_bytes())?;
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

#[macro_export]
macro_rules! update_item {
    ($name:ident, $type_value:expr) => {
        #[derive(Debug, Clone, Default, PartialEq)]
        pub struct $name {
            header: Vec<u32>,
            values: BTreeMap<u16, u32>,
        }

        impl $name {
            pub fn new() -> Self {
                const OBJECT_FIELD_TYPE: u16 = 2;

                let mut header = vec![];
                let mut values = BTreeMap::new();

                crate::helper::update_mask_common::header_set(
                    &mut header,
                    &mut values,
                    OBJECT_FIELD_TYPE,
                );
                values.insert(
                    OBJECT_FIELD_TYPE,
                    crate::helper::update_mask_common::OBJECT | $type_value,
                );

                Self { header, values }
            }

            fn from_inners(header: Vec<u32>, values: BTreeMap<u16, u32>) -> Self {
                Self { header, values }
            }

            pub(crate) fn header_set(&mut self, bit: u16) {
                crate::helper::update_mask_common::header_set(
                    &mut self.header,
                    &mut self.values,
                    bit,
                );
            }

            pub(crate) fn write_into_vec(&self, v: &mut Vec<u8>) -> Result<(), std::io::Error> {
                crate::helper::update_mask_common::write_into_vec(v, &self.header, &self.values)
            }
        }
    };
}

pub use update_item;
