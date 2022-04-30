use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SpellSchool, SpellSchoolError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_SPELLDAMAGESHIELD {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub damage: u32,
    pub school: SpellSchool,
}

impl ServerMessageWrite for SMSG_SPELLDAMAGESHIELD {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_SPELLDAMAGESHIELD {
    const OPCODE: u16 = 0x024f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_SPELLDAMAGESHIELDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim_guid: Guid
        let victim_guid = Guid::read(r)?;

        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // school: SpellSchool
        let school = SpellSchool::read_u32_le(r)?;

        Ok(Self {
            victim_guid,
            caster_guid,
            damage,
            school,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // victim_guid: Guid
        self.victim_guid.write(w)?;

        // caster_guid: Guid
        self.caster_guid.write(w)?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // school: SpellSchool
        self.school.write_u32_le(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim_guid: Guid
        let victim_guid = Guid::tokio_read(r).await?;

        // caster_guid: Guid
        let caster_guid = Guid::tokio_read(r).await?;

        // damage: u32
        let damage = crate::util::tokio_read_u32_le(r).await?;

        // school: SpellSchool
        let school = SpellSchool::tokio_read_u32_le(r).await?;

        Ok(Self {
            victim_guid,
            caster_guid,
            damage,
            school,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // victim_guid: Guid
        self.victim_guid.tokio_write(w).await?;

        // caster_guid: Guid
        self.caster_guid.tokio_write(w).await?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes()).await?;

        // school: SpellSchool
        self.school.tokio_write_u32_le(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim_guid: Guid
        let victim_guid = Guid::astd_read(r).await?;

        // caster_guid: Guid
        let caster_guid = Guid::astd_read(r).await?;

        // damage: u32
        let damage = crate::util::astd_read_u32_le(r).await?;

        // school: SpellSchool
        let school = SpellSchool::astd_read_u32_le(r).await?;

        Ok(Self {
            victim_guid,
            caster_guid,
            damage,
            school,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // victim_guid: Guid
        self.victim_guid.astd_write(w).await?;

        // caster_guid: Guid
        self.caster_guid.astd_write(w).await?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes()).await?;

        // school: SpellSchool
        self.school.astd_write_u32_le(w).await?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SPELLDAMAGESHIELD {}

impl MaximumPossibleSized for SMSG_SPELLDAMAGESHIELD {
    fn maximum_possible_size() -> usize {
        8 // victim_guid: Guid
        + 8 // caster_guid: Guid
        + 4 // damage: u32
        + 4 // school: SpellSchool upcasted to u32
    }
}

#[derive(Debug)]
pub enum SMSG_SPELLDAMAGESHIELDError {
    Io(std::io::Error),
    SpellSchool(SpellSchoolError),
}

impl std::error::Error for SMSG_SPELLDAMAGESHIELDError {}
impl std::fmt::Display for SMSG_SPELLDAMAGESHIELDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellSchool(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELLDAMAGESHIELDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellSchoolError> for SMSG_SPELLDAMAGESHIELDError {
    fn from(e: SpellSchoolError) -> Self {
        Self::SpellSchool(e)
    }
}

