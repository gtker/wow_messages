use std::io::Read;

use crate::util::read_u32_le;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Copy, Clone)]
pub struct VariableItemRandomProperty {
    item_random_property_id: u32,
    item_suffix_factor: Option<u32>,
}

impl VariableItemRandomProperty {
    pub fn new(item_random_property_id: u32, item_suffix_factor: Option<u32>) -> Option<Self> {
        if let Some(suffix) = item_suffix_factor {
            if item_random_property_id != 0 {
                Some(Self {
                    item_random_property_id,
                    item_suffix_factor: Some(suffix),
                })
            } else {
                None
            }
        } else {
            if item_random_property_id == 0 {
                Some(Self::zero())
            } else {
                None
            }
        }
    }

    /// No random property.
    pub fn zero() -> Self {
        Self {
            item_random_property_id: 0,
            item_suffix_factor: None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.item_random_property_id == 0
    }

    pub const fn item_random_property_id(&self) -> u32 {
        self.item_random_property_id
    }

    pub const fn item_suffix_factor(&self) -> Option<u32> {
        self.item_suffix_factor
    }

    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        w.write_all(&self.item_random_property_id.to_le_bytes())?;
        if let Some(suffix) = self.item_suffix_factor {
            w.write_all(&suffix.to_le_bytes())?;
        }

        Ok(())
    }

    pub(crate) fn read(r: &mut impl Read) -> Result<Self, std::io::Error> {
        let item_random_property_id = read_u32_le(r)?;
        let item_suffix_factor = if item_random_property_id != 0 {
            Some(read_u32_le(r)?)
        } else {
            None
        };

        Ok(Self {
            item_random_property_id,
            item_suffix_factor,
        })
    }

    pub(crate) fn size(&self) -> usize {
        4 + if self.item_suffix_factor.is_some() {
            4
        } else {
            0
        }
    }
}

impl std::fmt::Display for VariableItemRandomProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(suffix) = &self.item_suffix_factor {
            write!(f, "{}, {}", self.item_random_property_id, suffix)
        } else {
            use std::fmt::Write;
            f.write_char('0')
        }
    }
}
