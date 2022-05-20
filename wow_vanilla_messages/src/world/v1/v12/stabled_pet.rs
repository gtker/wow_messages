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
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(273);
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

        Ok(w)
    }
}

impl StabledPet {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, StabledPetError> {
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

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, StabledPetError> {
        // pet_number: u32
        let pet_number = crate::util::tokio_read_u32_le(r).await?;

        // entry: u32
        let entry = crate::util::tokio_read_u32_le(r).await?;

        // level: u32
        let level = crate::util::tokio_read_u32_le(r).await?;

        // name: CString
        let name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // loyalty: u32
        let loyalty = crate::util::tokio_read_u32_le(r).await?;

        // slot: u8
        let slot = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            pet_number,
            entry,
            level,
            name,
            loyalty,
            slot,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, StabledPetError> {
        // pet_number: u32
        let pet_number = crate::util::astd_read_u32_le(r).await?;

        // entry: u32
        let entry = crate::util::astd_read_u32_le(r).await?;

        // level: u32
        let level = crate::util::astd_read_u32_le(r).await?;

        // name: CString
        let name = crate::util::astd_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // loyalty: u32
        let loyalty = crate::util::astd_read_u32_le(r).await?;

        // slot: u8
        let slot = crate::util::astd_read_u8_le(r).await?;

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

