use std::io::{Read, Write};
use wow_world_base::ParseError;

use crate::util::{read_c_string_to_vec, read_u64_le};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone)]
pub struct NamedGuid {
    guid: u64,
    name: Option<String>,
}

impl NamedGuid {
    pub fn new(guid: u64, name: Option<String>) -> Option<Self> {
        let name = name.into();
        if let Some(name) = name {
            if guid != 0 {
                Some(Self { guid, name })
            } else {
                None
            }
        } else {
            if guid == 0 {
                Some(Self { guid, name: None })
            } else {
                None
            }
        }
    }

    /// Guid with 0 value.
    ///
    /// Client uses this to mean different things, including things like no target selected.
    pub fn zero() -> Self {
        Self {
            guid: 0,
            name: None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.guid == 0
    }

    pub fn guid(&self) -> u64 {
        self.guid
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|a| a.as_str())
    }

    pub(crate) fn read(r: &mut impl Read) -> Result<Self, ParseError> {
        let guid = read_u64_le(r)?;
        let name = if guid != 0 {
            let name = read_c_string_to_vec(r)?;
            Some(String::from_utf8(name)?)
        } else {
            None
        };

        Ok(Self { guid, name })
    }

    pub(crate) fn write_into_vec(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        w.write_all(&self.guid().to_le_bytes())?;

        if let Some(name) = &self.name {
            w.write_all(name.as_bytes())?;
            w.write_all(&0_u8.to_le_bytes())?;
        }

        Ok(())
    }

    pub(crate) fn size(&self) -> usize {
        core::mem::size_of::<u64>()
            + if let Some(name) = &self.name {
                name.len() + 1 // null byte
            } else {
                0
            }
    }
}

impl std::fmt::Display for NamedGuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}: {}", self.guid, name)
        } else {
            use std::fmt::Write;
            f.write_char('0')
        }
    }
}
