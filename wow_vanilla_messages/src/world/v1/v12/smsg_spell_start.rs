use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{CastFlags};
use crate::world::v1::v12::{SpellCastTargets, SpellCastTargetsError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELL_START {
    pub cast_item: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub flags: SMSG_SPELL_STARTCastFlags,
    pub timer: u32,
    pub targets: SpellCastTargets,
}

impl ServerMessageWrite for SMSG_SPELL_START {}

impl MessageBody for SMSG_SPELL_START {
    const OPCODE: u16 = 0x0131;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_SPELL_STARTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // cast_item: PackedGuid
        let cast_item = Guid::read_packed(r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // flags: CastFlags
        let flags = CastFlags::new(crate::util::read_u16_le(r)?);

        // timer: u32
        let timer = crate::util::read_u32_le(r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        let flags_AMMO = if flags.is_AMMO() {
            // ammo_display_id: u32
            let ammo_display_id = crate::util::read_u32_le(r)?;

            // ammo_inventory_type: u32
            let ammo_inventory_type = crate::util::read_u32_le(r)?;

            Some(SMSG_SPELL_STARTCastFlagsAMMO {
                ammo_display_id,
                ammo_inventory_type,
            })
        }
        else {
            None
        };

        let flags = SMSG_SPELL_STARTCastFlags {
            inner: flags.as_int(),
            ammo: flags_AMMO,
        };

        Ok(Self {
            cast_item,
            caster,
            spell,
            flags,
            timer,
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
        w.write_all(&(self.flags.as_int() as u16).to_le_bytes())?;

        // timer: u32
        w.write_all(&self.timer.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write(w)?;

        if let Some(if_statement) = &self.flags.ammo {
            // ammo_display_id: u32
            w.write_all(&if_statement.ammo_display_id.to_le_bytes())?;

            // ammo_inventory_type: u32
            w.write_all(&if_statement.ammo_inventory_type.to_le_bytes())?;

        }

        Ok(())
    }

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // cast_item: PackedGuid
            let cast_item = Guid::tokio_read_packed(r).await?;

            // caster: PackedGuid
            let caster = Guid::tokio_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::tokio_read_u32_le(r).await?;

            // flags: CastFlags
            let flags = CastFlags::new(crate::util::tokio_read_u16_le(r).await?);

            // timer: u32
            let timer = crate::util::tokio_read_u32_le(r).await?;

            // targets: SpellCastTargets
            let targets = SpellCastTargets::tokio_read(r).await?;

            let flags_AMMO = if flags.is_AMMO() {
                // ammo_display_id: u32
                let ammo_display_id = crate::util::tokio_read_u32_le(r).await?;

                // ammo_inventory_type: u32
                let ammo_inventory_type = crate::util::tokio_read_u32_le(r).await?;

                Some(SMSG_SPELL_STARTCastFlagsAMMO {
                    ammo_display_id,
                    ammo_inventory_type,
                })
            }
            else {
                None
            };

            let flags = SMSG_SPELL_STARTCastFlags {
                inner: flags.as_int(),
                ammo: flags_AMMO,
            };

            Ok(Self {
                cast_item,
                caster,
                spell,
                flags,
                timer,
                targets,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // cast_item: PackedGuid
            self.cast_item.tokio_write_packed(w).await?;

            // caster: PackedGuid
            self.caster.tokio_write_packed(w).await?;

            // spell: u32
            w.write_all(&self.spell.to_le_bytes()).await?;

            // flags: CastFlags
            w.write_all(&(self.flags.as_int() as u16).to_le_bytes()).await?;

            // timer: u32
            w.write_all(&self.timer.to_le_bytes()).await?;

            // targets: SpellCastTargets
            self.targets.tokio_write(w).await?;

            if let Some(if_statement) = &self.flags.ammo {
                // ammo_display_id: u32
                w.write_all(&if_statement.ammo_display_id.to_le_bytes()).await?;

                // ammo_inventory_type: u32
                w.write_all(&if_statement.ammo_inventory_type.to_le_bytes()).await?;

            }

            Ok(())
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // cast_item: PackedGuid
            let cast_item = Guid::astd_read_packed(r).await?;

            // caster: PackedGuid
            let caster = Guid::astd_read_packed(r).await?;

            // spell: u32
            let spell = crate::util::astd_read_u32_le(r).await?;

            // flags: CastFlags
            let flags = CastFlags::new(crate::util::astd_read_u16_le(r).await?);

            // timer: u32
            let timer = crate::util::astd_read_u32_le(r).await?;

            // targets: SpellCastTargets
            let targets = SpellCastTargets::astd_read(r).await?;

            let flags_AMMO = if flags.is_AMMO() {
                // ammo_display_id: u32
                let ammo_display_id = crate::util::astd_read_u32_le(r).await?;

                // ammo_inventory_type: u32
                let ammo_inventory_type = crate::util::astd_read_u32_le(r).await?;

                Some(SMSG_SPELL_STARTCastFlagsAMMO {
                    ammo_display_id,
                    ammo_inventory_type,
                })
            }
            else {
                None
            };

            let flags = SMSG_SPELL_STARTCastFlags {
                inner: flags.as_int(),
                ammo: flags_AMMO,
            };

            Ok(Self {
                cast_item,
                caster,
                spell,
                flags,
                timer,
                targets,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // cast_item: PackedGuid
            self.cast_item.astd_write_packed(w).await?;

            // caster: PackedGuid
            self.caster.astd_write_packed(w).await?;

            // spell: u32
            w.write_all(&self.spell.to_le_bytes()).await?;

            // flags: CastFlags
            w.write_all(&(self.flags.as_int() as u16).to_le_bytes()).await?;

            // timer: u32
            w.write_all(&self.timer.to_le_bytes()).await?;

            // targets: SpellCastTargets
            self.targets.astd_write(w).await?;

            if let Some(if_statement) = &self.flags.ammo {
                // ammo_display_id: u32
                w.write_all(&if_statement.ammo_display_id.to_le_bytes()).await?;

                // ammo_inventory_type: u32
                w.write_all(&if_statement.ammo_inventory_type.to_le_bytes()).await?;

            }

            Ok(())
        })
    }

}

impl SMSG_SPELL_START {
    pub fn size(&self) -> usize {
        0
        + self.cast_item.size() // cast_item: Guid
        + self.caster.size() // caster: Guid
        + 4 // spell: u32
        + self.flags.size() // flags: SMSG_SPELL_STARTCastFlags
        + 4 // timer: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Debug)]
pub enum SMSG_SPELL_STARTError {
    Io(std::io::Error),
    SpellCastTargets(SpellCastTargetsError),
}

impl std::error::Error for SMSG_SPELL_STARTError {}
impl std::fmt::Display for SMSG_SPELL_STARTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellCastTargets(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELL_STARTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellCastTargetsError> for SMSG_SPELL_STARTError {
    fn from(e: SpellCastTargetsError) -> Self {
        Self::SpellCastTargets(e)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SMSG_SPELL_STARTCastFlags {
    inner: u16,
    ammo: Option<SMSG_SPELL_STARTCastFlagsAMMO>,
}

impl SMSG_SPELL_STARTCastFlags {
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

    pub const fn new_AMMO(ammo: SMSG_SPELL_STARTCastFlagsAMMO) -> Self {
        Self {
            inner: CastFlags::AMMO,
            ammo: Some(ammo),
        }
    }

    pub fn set_AMMO(&mut self, ammo: SMSG_SPELL_STARTCastFlagsAMMO) -> Self {
        self.inner |= CastFlags::AMMO;
        self.ammo = Some(ammo);
        self.clone()
    }

    pub const fn get_AMMO(&self) -> Option<&SMSG_SPELL_STARTCastFlagsAMMO> {
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

    pub(crate) const fn as_int(&self) -> u16 {
        self.inner
    }

}
impl SMSG_SPELL_STARTCastFlags {
    pub fn size(&self) -> usize {
        2 // inner
        + {
            if let Some(s) = &self.ammo {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_SPELL_STARTCastFlagsAMMO {
    pub ammo_display_id: u32,
    pub ammo_inventory_type: u32,
}

impl SMSG_SPELL_STARTCastFlagsAMMO {
    pub fn size(&self) -> usize {
        4 // ammo_display_id: u32
        + 4 // ammo_inventory_type: u32
    }
}

