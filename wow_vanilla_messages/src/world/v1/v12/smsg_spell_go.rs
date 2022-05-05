use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{CastFlags};
use crate::world::v1::v12::{SpellCastTargets, SpellCastTargetsError};
use crate::world::v1::v12::{SpellMiss, SpellMissError};
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
pub struct SMSG_SPELL_GO {
    pub cast_item: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub flags: SMSG_SPELL_GOCastFlags,
    pub hits: Vec<Guid>,
    pub misses: Vec<SpellMiss>,
    pub targets: SpellCastTargets,
}

impl ServerMessageWrite for SMSG_SPELL_GO {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_SPELL_GO {
    const OPCODE: u16 = 0x0132;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_SPELL_GOError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // cast_item: PackedGuid
        let cast_item = Guid::read_packed(r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // flags: CastFlags
        let flags = CastFlags::read(r)?;

        // amount_of_hits: u8
        let amount_of_hits = crate::util::read_u8_le(r)?;

        // hits: Guid[amount_of_hits]
        let mut hits = Vec::with_capacity(amount_of_hits as usize);
        for i in 0..amount_of_hits {
            hits.push(Guid::read(r)?);
        }

        // amount_of_misses: u8
        let amount_of_misses = crate::util::read_u8_le(r)?;

        // misses: SpellMiss[amount_of_misses]
        let mut misses = Vec::with_capacity(amount_of_misses as usize);
        for i in 0..amount_of_misses {
            misses.push(SpellMiss::read(r)?);
        }

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        let flags_AMMO = if flags.is_AMMO() {
            // ammo_display_id: u32
            let ammo_display_id = crate::util::read_u32_le(r)?;

            // ammo_inventory_type: u32
            let ammo_inventory_type = crate::util::read_u32_le(r)?;

            Some(SMSG_SPELL_GOCastFlagsAMMO {
                ammo_display_id,
                ammo_inventory_type,
            })
        } else {
            None
        };

        let flags = SMSG_SPELL_GOCastFlags {
            inner: flags.as_u16(),
            ammo: flags_AMMO,
        };

        Ok(Self {
            cast_item,
            caster,
            spell,
            flags,
            hits,
            misses,
            targets,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // cast_item: PackedGuid
        self.cast_item.write_packed(w)?;

        // caster: PackedGuid
        self.caster.write_packed(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // flags: CastFlags
        self.flags.write(w)?;

        // amount_of_hits: u8
        w.write_all(&(self.hits.len() as u8).to_le_bytes())?;

        // hits: Guid[amount_of_hits]
        for i in self.hits.iter() {
            i.write(w)?;
        }

        // amount_of_misses: u8
        w.write_all(&(self.misses.len() as u8).to_le_bytes())?;

        // misses: SpellMiss[amount_of_misses]
        for i in self.misses.iter() {
            i.write(w)?;
        }

        // targets: SpellCastTargets
        self.targets.write(w)?;

        if let Some(s) = &self.flags.ammo {
            s.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // cast_item: PackedGuid
        let cast_item = Guid::tokio_read_packed(r).await?;

        // caster: PackedGuid
        let caster = Guid::tokio_read_packed(r).await?;

        // spell: u32
        let spell = crate::util::tokio_read_u32_le(r).await?;

        // flags: CastFlags
        let flags = CastFlags::tokio_read(r).await?;

        // amount_of_hits: u8
        let amount_of_hits = crate::util::tokio_read_u8_le(r).await?;

        // hits: Guid[amount_of_hits]
        let mut hits = Vec::with_capacity(amount_of_hits as usize);
        for i in 0..amount_of_hits {
            hits.push(Guid::tokio_read(r).await?);
        }

        // amount_of_misses: u8
        let amount_of_misses = crate::util::tokio_read_u8_le(r).await?;

        // misses: SpellMiss[amount_of_misses]
        let mut misses = Vec::with_capacity(amount_of_misses as usize);
        for i in 0..amount_of_misses {
            misses.push(SpellMiss::tokio_read(r).await?);
        }

        // targets: SpellCastTargets
        let targets = SpellCastTargets::tokio_read(r).await?;

        let flags_AMMO = if flags.is_AMMO() {
            // ammo_display_id: u32
            let ammo_display_id = crate::util::tokio_read_u32_le(r).await?;

            // ammo_inventory_type: u32
            let ammo_inventory_type = crate::util::tokio_read_u32_le(r).await?;

            Some(SMSG_SPELL_GOCastFlagsAMMO {
                ammo_display_id,
                ammo_inventory_type,
            })
        } else {
            None
        };

        let flags = SMSG_SPELL_GOCastFlags {
            inner: flags.as_u16(),
            ammo: flags_AMMO,
        };

        Ok(Self {
            cast_item,
            caster,
            spell,
            flags,
            hits,
            misses,
            targets,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // cast_item: PackedGuid
        self.cast_item.tokio_write_packed(w).await?;

        // caster: PackedGuid
        self.caster.tokio_write_packed(w).await?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes()).await?;

        // flags: CastFlags
        self.flags.tokio_write(w).await?;

        // amount_of_hits: u8
        w.write_all(&(self.hits.len() as u8).to_le_bytes()).await?;

        // hits: Guid[amount_of_hits]
        for i in self.hits.iter() {
            i.tokio_write(w).await?;
        }

        // amount_of_misses: u8
        w.write_all(&(self.misses.len() as u8).to_le_bytes()).await?;

        // misses: SpellMiss[amount_of_misses]
        for i in self.misses.iter() {
            i.tokio_write(w).await?;
        }

        // targets: SpellCastTargets
        self.targets.tokio_write(w).await?;

        if let Some(s) = &self.flags.ammo {
            s.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // cast_item: PackedGuid
        let cast_item = Guid::astd_read_packed(r).await?;

        // caster: PackedGuid
        let caster = Guid::astd_read_packed(r).await?;

        // spell: u32
        let spell = crate::util::astd_read_u32_le(r).await?;

        // flags: CastFlags
        let flags = CastFlags::astd_read(r).await?;

        // amount_of_hits: u8
        let amount_of_hits = crate::util::astd_read_u8_le(r).await?;

        // hits: Guid[amount_of_hits]
        let mut hits = Vec::with_capacity(amount_of_hits as usize);
        for i in 0..amount_of_hits {
            hits.push(Guid::astd_read(r).await?);
        }

        // amount_of_misses: u8
        let amount_of_misses = crate::util::astd_read_u8_le(r).await?;

        // misses: SpellMiss[amount_of_misses]
        let mut misses = Vec::with_capacity(amount_of_misses as usize);
        for i in 0..amount_of_misses {
            misses.push(SpellMiss::astd_read(r).await?);
        }

        // targets: SpellCastTargets
        let targets = SpellCastTargets::astd_read(r).await?;

        let flags_AMMO = if flags.is_AMMO() {
            // ammo_display_id: u32
            let ammo_display_id = crate::util::astd_read_u32_le(r).await?;

            // ammo_inventory_type: u32
            let ammo_inventory_type = crate::util::astd_read_u32_le(r).await?;

            Some(SMSG_SPELL_GOCastFlagsAMMO {
                ammo_display_id,
                ammo_inventory_type,
            })
        } else {
            None
        };

        let flags = SMSG_SPELL_GOCastFlags {
            inner: flags.as_u16(),
            ammo: flags_AMMO,
        };

        Ok(Self {
            cast_item,
            caster,
            spell,
            flags,
            hits,
            misses,
            targets,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // cast_item: PackedGuid
        self.cast_item.astd_write_packed(w).await?;

        // caster: PackedGuid
        self.caster.astd_write_packed(w).await?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes()).await?;

        // flags: CastFlags
        self.flags.astd_write(w).await?;

        // amount_of_hits: u8
        w.write_all(&(self.hits.len() as u8).to_le_bytes()).await?;

        // hits: Guid[amount_of_hits]
        for i in self.hits.iter() {
            i.astd_write(w).await?;
        }

        // amount_of_misses: u8
        w.write_all(&(self.misses.len() as u8).to_le_bytes()).await?;

        // misses: SpellMiss[amount_of_misses]
        for i in self.misses.iter() {
            i.astd_write(w).await?;
        }

        // targets: SpellCastTargets
        self.targets.astd_write(w).await?;

        if let Some(s) = &self.flags.ammo {
            s.astd_write(w).await?;
        }

        Ok(())
    }

}

impl VariableSized for SMSG_SPELL_GO {
    fn size(&self) -> usize {
        self.cast_item.size() // cast_item: PackedGuid
        + self.caster.size() // caster: PackedGuid
        + 4 // spell: u32
        + self.flags.size() // flags: CastFlags and subfields
        + 1 // amount_of_hits: u8
        + self.hits.iter().fold(0, |acc, _| acc + 8) // hits: Guid[amount_of_hits]
        + 1 // amount_of_misses: u8
        + self.misses.iter().fold(0, |acc, x| acc + SpellMiss::size()) // misses: SpellMiss[amount_of_misses]
        + self.targets.size() // targets: SpellCastTargets
    }
}

impl MaximumPossibleSized for SMSG_SPELL_GO {
    fn maximum_possible_size() -> usize {
        9 // cast_item: PackedGuid
        + 9 // caster: PackedGuid
        + 4 // spell: u32
        + CastFlags::maximum_possible_size() // flags: CastFlags
        + 1 // amount_of_hits: u8
        + 255 * 8 // hits: Guid[amount_of_hits]
        + 1 // amount_of_misses: u8
        + 255 * SpellMiss::maximum_possible_size() // misses: SpellMiss[amount_of_misses]
        + SpellCastTargets::maximum_possible_size() // targets: SpellCastTargets
    }
}

#[derive(Debug)]
pub enum SMSG_SPELL_GOError {
    Io(std::io::Error),
    SpellCastTargets(SpellCastTargetsError),
    SpellMiss(SpellMissError),
}

impl std::error::Error for SMSG_SPELL_GOError {}
impl std::fmt::Display for SMSG_SPELL_GOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellCastTargets(i) => i.fmt(f),
            Self::SpellMiss(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELL_GOError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellCastTargetsError> for SMSG_SPELL_GOError {
    fn from(e: SpellCastTargetsError) -> Self {
        Self::SpellCastTargets(e)
    }
}

impl From<SpellMissError> for SMSG_SPELL_GOError {
    fn from(e: SpellMissError) -> Self {
        Self::SpellMiss(e)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SMSG_SPELL_GOCastFlags {
    inner: u16,
    ammo: Option<SMSG_SPELL_GOCastFlagsAMMO>,
}

impl From<&SMSG_SPELL_GOCastFlags> for CastFlags {
    fn from(e: &SMSG_SPELL_GOCastFlags) -> Self {
        Self::new(e.inner)
    }
}

impl SMSG_SPELL_GOCastFlags {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFlags = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFlags = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFlags = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            ammo: None,
        }
    }

    pub const fn new_NONE() -> Self {
        Self {
            inner: CastFlags::NONE,
            ammo: None,
        }
    }

    pub fn set_NONE(&mut self) -> Self {
        self.inner |= CastFlags::NONE;
        self.clone()
    }

    pub const fn get_NONE(&self) -> bool {
        // Underlying value is 0
        self.inner == CastFlags::NONE
    }

    pub fn clear_NONE(&mut self) -> Self {
        self.inner &= CastFlags::NONE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_HIDDEN_COMBATLOG() -> Self {
        Self {
            inner: CastFlags::HIDDEN_COMBATLOG,
            ammo: None,
        }
    }

    pub fn set_HIDDEN_COMBATLOG(&mut self) -> Self {
        self.inner |= CastFlags::HIDDEN_COMBATLOG;
        self.clone()
    }

    pub const fn get_HIDDEN_COMBATLOG(&self) -> bool {
        (self.inner & CastFlags::HIDDEN_COMBATLOG) != 0
    }

    pub fn clear_HIDDEN_COMBATLOG(&mut self) -> Self {
        self.inner &= CastFlags::HIDDEN_COMBATLOG.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN2() -> Self {
        Self {
            inner: CastFlags::UNKNOWN2,
            ammo: None,
        }
    }

    pub fn set_UNKNOWN2(&mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN2;
        self.clone()
    }

    pub const fn get_UNKNOWN2(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN2) != 0
    }

    pub fn clear_UNKNOWN2(&mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN2.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN3() -> Self {
        Self {
            inner: CastFlags::UNKNOWN3,
            ammo: None,
        }
    }

    pub fn set_UNKNOWN3(&mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN3;
        self.clone()
    }

    pub const fn get_UNKNOWN3(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN3) != 0
    }

    pub fn clear_UNKNOWN3(&mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN3.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN4() -> Self {
        Self {
            inner: CastFlags::UNKNOWN4,
            ammo: None,
        }
    }

    pub fn set_UNKNOWN4(&mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN4;
        self.clone()
    }

    pub const fn get_UNKNOWN4(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN4) != 0
    }

    pub fn clear_UNKNOWN4(&mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN4.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN5() -> Self {
        Self {
            inner: CastFlags::UNKNOWN5,
            ammo: None,
        }
    }

    pub fn set_UNKNOWN5(&mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN5;
        self.clone()
    }

    pub const fn get_UNKNOWN5(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN5) != 0
    }

    pub fn clear_UNKNOWN5(&mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN5.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_AMMO(ammo: SMSG_SPELL_GOCastFlagsAMMO) -> Self {
        Self {
            inner: CastFlags::AMMO,
            ammo: Some(ammo),
        }
    }

    pub fn set_AMMO(&mut self, ammo: SMSG_SPELL_GOCastFlagsAMMO) -> Self {
        self.inner |= CastFlags::AMMO;
        self.ammo = Some(ammo);
        self.clone()
    }

    pub const fn get_AMMO(&self) -> Option<&SMSG_SPELL_GOCastFlagsAMMO> {
        self.ammo.as_ref()
    }

    pub fn clear_AMMO(&mut self) -> Self {
        self.inner &= CastFlags::AMMO.reverse_bits();
        self.ammo = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN7() -> Self {
        Self {
            inner: CastFlags::UNKNOWN7,
            ammo: None,
        }
    }

    pub fn set_UNKNOWN7(&mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN7;
        self.clone()
    }

    pub const fn get_UNKNOWN7(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN7) != 0
    }

    pub fn clear_UNKNOWN7(&mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN7.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN8() -> Self {
        Self {
            inner: CastFlags::UNKNOWN8,
            ammo: None,
        }
    }

    pub fn set_UNKNOWN8(&mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN8;
        self.clone()
    }

    pub const fn get_UNKNOWN8(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN8) != 0
    }

    pub fn clear_UNKNOWN8(&mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN8.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_UNKNOWN9() -> Self {
        Self {
            inner: CastFlags::UNKNOWN9,
            ammo: None,
        }
    }

    pub fn set_UNKNOWN9(&mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN9;
        self.clone()
    }

    pub const fn get_UNKNOWN9(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN9) != 0
    }

    pub fn clear_UNKNOWN9(&mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN9.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

}
impl VariableSized for SMSG_SPELL_GOCastFlags {
    fn size(&self) -> usize {
        2 // inner: CastFlags (u16)
        + {
            if let Some(s) = &self.ammo {
                s.size()
            } else {
                0
            }
        }
    }
}

impl MaximumPossibleSized for SMSG_SPELL_GOCastFlags {
    fn maximum_possible_size() -> usize {
        2 // inner: CastFlags (u16)
        + SMSG_SPELL_GOCastFlagsAMMO::maximum_possible_size() // AMMO enumerator
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_SPELL_GOCastFlagsAMMO {
    pub ammo_display_id: u32,
    pub ammo_inventory_type: u32,
}

impl VariableSized for SMSG_SPELL_GOCastFlagsAMMO {
    fn size(&self) -> usize {
        4 // ammo_display_id: u32
        + 4 // ammo_inventory_type: u32
    }
}

impl MaximumPossibleSized for SMSG_SPELL_GOCastFlagsAMMO {
    fn maximum_possible_size() -> usize {
        4 // ammo_display_id: u32
        + 4 // ammo_inventory_type: u32
    }
}

impl SMSG_SPELL_GOCastFlagsAMMO {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.ammo_display_id.to_le_bytes())?;

        w.write_all(&self.ammo_inventory_type.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.ammo_display_id.to_le_bytes()).await?;

        w.write_all(&self.ammo_inventory_type.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.ammo_display_id.to_le_bytes()).await?;

        w.write_all(&self.ammo_inventory_type.to_le_bytes()).await?;

        Ok(())
    }

}

