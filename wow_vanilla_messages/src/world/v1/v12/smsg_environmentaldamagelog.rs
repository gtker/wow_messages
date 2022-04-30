use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{EnvironmentalDamageType, EnvironmentalDamageTypeError};
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
pub struct SMSG_ENVIRONMENTALDAMAGELOG {
    pub guid: Guid,
    pub damage_type: EnvironmentalDamageType,
    pub damage: u32,
    pub absorb: u32,
    pub resist: u32,
}

impl ServerMessageWrite for SMSG_ENVIRONMENTALDAMAGELOG {}

impl MessageBody for SMSG_ENVIRONMENTALDAMAGELOG {
    const OPCODE: u16 = 0x01fc;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_ENVIRONMENTALDAMAGELOGError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // damage_type: EnvironmentalDamageType
        let damage_type = EnvironmentalDamageType::read(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // absorb: u32
        let absorb = crate::util::read_u32_le(r)?;

        // resist: u32
        let resist = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            damage_type,
            damage,
            absorb,
            resist,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // damage_type: EnvironmentalDamageType
        self.damage_type.write(w)?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // absorb: u32
        w.write_all(&self.absorb.to_le_bytes())?;

        // resist: u32
        w.write_all(&self.resist.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_ENVIRONMENTALDAMAGELOG {}

impl MaximumPossibleSized for SMSG_ENVIRONMENTALDAMAGELOG {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + EnvironmentalDamageType::size() // damage_type: EnvironmentalDamageType
        + 4 // damage: u32
        + 4 // absorb: u32
        + 4 // resist: u32
    }
}

#[derive(Debug)]
pub enum SMSG_ENVIRONMENTALDAMAGELOGError {
    Io(std::io::Error),
    EnvironmentalDamageType(EnvironmentalDamageTypeError),
}

impl std::error::Error for SMSG_ENVIRONMENTALDAMAGELOGError {}
impl std::fmt::Display for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::EnvironmentalDamageType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<EnvironmentalDamageTypeError> for SMSG_ENVIRONMENTALDAMAGELOGError {
    fn from(e: EnvironmentalDamageTypeError) -> Self {
        Self::EnvironmentalDamageType(e)
    }
}

