use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Class, ClassError};
use crate::world::v1::v12::{Race, RaceError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct WhoPlayer {
    pub name: String,
    pub guild: String,
    pub level: u32,
    pub class: Class,
    pub race: Race,
    pub zone_id: u32,
    pub party_status: u32,
}

impl ReadableAndWritable for WhoPlayer {
    type Error = WhoPlayerError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // guild: CString
        let guild = crate::util::read_c_string_to_vec(r)?;
        let guild = String::from_utf8(guild)?;

        // level: u32
        let level = crate::util::read_u32_le(r)?;

        // class: Class
        let class = Class::read(r)?;

        // race: Race
        let race = Race::read(r)?;

        // zone_id: u32
        let zone_id = crate::util::read_u32_le(r)?;

        // party_status: u32
        let party_status = crate::util::read_u32_le(r)?;

        Ok(Self {
            name,
            guild,
            level,
            class,
            race,
            zone_id,
            party_status,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guild: CString
        w.write_all(self.guild.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // level: u32
        w.write_all(&self.level.to_le_bytes())?;

        // class: Class
        self.class.write(w)?;

        // race: Race
        self.race.write(w)?;

        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes())?;

        // party_status: u32
        w.write_all(&self.party_status.to_le_bytes())?;

        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for WhoPlayer {
    type Error = WhoPlayerError;
    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // guild: CString
        let guild = crate::util::tokio_read_c_string_to_vec(r).await?;
        let guild = String::from_utf8(guild)?;

        // level: u32
        let level = crate::util::tokio_read_u32_le(r).await?;

        // class: Class
        let class = Class::tokio_read(r).await?;

        // race: Race
        let race = Race::tokio_read(r).await?;

        // zone_id: u32
        let zone_id = crate::util::tokio_read_u32_le(r).await?;

        // party_status: u32
        let party_status = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            name,
            guild,
            level,
            class,
            race,
            zone_id,
            party_status,
        })
    }
    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // guild: CString
        w.write_all(self.guild.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // level: u32
        w.write_all(&self.level.to_le_bytes()).await?;

        // class: Class
        self.class.tokio_write(w).await?;

        // race: Race
        self.race.tokio_write(w).await?;

        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes()).await?;

        // party_status: u32
        w.write_all(&self.party_status.to_le_bytes()).await?;

        Ok(())
    }
}
impl VariableSized for WhoPlayer {
    fn size(&self) -> usize {
        self.name.len() + 1 // name: CString and Null Terminator
        + self.guild.len() + 1 // guild: CString and Null Terminator
        + 4 // level: u32
        + Class::size() // class: Class
        + Race::size() // race: Race
        + 4 // zone_id: u32
        + 4 // party_status: u32
    }
}

impl MaximumPossibleSized for WhoPlayer {
    fn maximum_possible_size() -> usize {
        256 // name: CString
        + 256 // guild: CString
        + 4 // level: u32
        + Class::maximum_possible_size() // class: Class
        + Race::maximum_possible_size() // race: Race
        + 4 // zone_id: u32
        + 4 // party_status: u32
    }
}

#[derive(Debug)]
pub enum WhoPlayerError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Class(ClassError),
    Race(RaceError),
}

impl std::error::Error for WhoPlayerError {}
impl std::fmt::Display for WhoPlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Class(i) => i.fmt(f),
            Self::Race(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for WhoPlayerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for WhoPlayerError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<ClassError> for WhoPlayerError {
    fn from(e: ClassError) -> Self {
        Self::Class(e)
    }
}

impl From<RaceError> for WhoPlayerError {
    fn from(e: RaceError) -> Self {
        Self::Race(e)
    }
}

