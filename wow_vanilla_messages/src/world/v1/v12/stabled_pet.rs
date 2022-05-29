use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct StabledPet {
    pub pet_number: u32,
    pub entry: u32,
    pub level: u32,
    pub name: String,
    pub loyalty: u32,
    pub slot: u8,
}

impl StabledPet {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        // entry: u32
        w.write_all(&self.entry.to_le_bytes())?;

        // level: u32
        w.write_all(&self.level.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // loyalty: u32
        w.write_all(&self.loyalty.to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
}

impl StabledPet {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // pet_number: u32
        let pet_number = crate::util::read_u32_le(r)?;

        // entry: u32
        let entry = crate::util::read_u32_le(r)?;

        // level: u32
        let level = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // loyalty: u32
        let loyalty = crate::util::read_u32_le(r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            pet_number,
            entry,
            level,
            name,
            loyalty,
            slot,
        })
    }

}

impl StabledPet {
    pub fn size(&self) -> usize {
        0
        + 4 // pet_number: u32
        + 4 // entry: u32
        + 4 // level: u32
        + self.name.len() + 1 // name: CString
        + 4 // loyalty: u32
        + 1 // slot: u8
    }
}

