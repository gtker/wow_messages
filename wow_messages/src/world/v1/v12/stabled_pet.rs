use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:935`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L935):
/// ```text
/// struct StabledPet {
///     u32 pet_number;
///     u32 entry;
///     u32 level;
///     CString name;
///     u32 loyalty;
///     u8 slot;
/// }
/// ```
pub struct StabledPet {
    pub pet_number: u32,
    pub entry: u32,
    pub level: u32,
    pub name: String,
    pub loyalty: u32,
    pub slot: u8,
}

impl ReadableAndWritable for StabledPet {
    type Error = StabledPetError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
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

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
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

impl VariableSized for StabledPet {
    fn size(&self) -> usize {
        4 // pet_number: u32
        + 4 // entry: u32
        + 4 // level: u32
        + self.name.len() + 1 // name: CString and Null Terminator
        + 4 // loyalty: u32
        + 1 // slot: u8
    }
}

impl MaximumPossibleSized for StabledPet {
    fn maximum_possible_size() -> usize {
        4 // pet_number: u32
        + 4 // entry: u32
        + 4 // level: u32
        + 256 // name: CString
        + 4 // loyalty: u32
        + 1 // slot: u8
    }
}

#[derive(Debug)]
pub enum StabledPetError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for StabledPetError {}
impl std::fmt::Display for StabledPetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for StabledPetError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for StabledPetError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

