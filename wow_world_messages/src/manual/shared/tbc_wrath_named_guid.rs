use crate::errors::ParseErrorKind;
use std::io::{Read, Write};
use std::num::NonZeroU64;

use crate::util::{read_c_string_to_vec, read_u64_le};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone)]
pub struct NamedGuid {
    guid: u64,
    name: Option<String>,
}

impl NamedGuid {
    /// Constructs a named guid with a non-zero guid value.
    ///
    /// Can only construct objects with a non-zero guid. Use [`Self::zero`] to create a zero guid.
    pub const fn new(guid: NonZeroU64, name: String) -> Self {
        Self {
            guid: guid.get(),
            name: Some(name),
        }
    }

    /// Guid with 0 value.
    ///
    /// Client uses this to mean different things, including things like no target selected.
    pub const fn zero() -> Self {
        Self {
            guid: 0,
            name: None,
        }
    }

    pub const fn is_zero(&self) -> bool {
        self.guid == 0
    }

    pub const fn guid(&self) -> u64 {
        self.guid
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub(crate) fn read(r: &mut impl Read) -> Result<Self, ParseErrorKind> {
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
