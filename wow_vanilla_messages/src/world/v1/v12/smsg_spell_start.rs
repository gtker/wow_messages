use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{CastFlags};
use crate::world::v1::v12::{SpellCastTargets, SpellCastTargetsError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_start.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_start.wowm#L3):
/// ```text
/// smsg SMSG_SPELL_START = 0x131 {
///     PackedGuid cast_item;
///     PackedGuid caster;
///     u32 spell;
///     CastFlags flags;
///     u32 timer;
///     SpellCastTargets targets;
///     if (flags & AMMO) {
///         u32 ammo_display_id;
///         u32 ammo_inventory_type;
///     }
/// }
/// ```
pub struct SMSG_SPELL_START {
    pub cast_item: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub flags: SMSG_SPELL_STARTCastFlags,
    pub timer: u32,
    pub targets: SpellCastTargets,
}

impl WorldServerMessageWrite for SMSG_SPELL_START {
    const OPCODE: u16 = 0x131;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_SPELL_START {
    type Error = SMSG_SPELL_STARTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // cast_item: PackedGuid
        let cast_item = Guid::read_packed(r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // flags: CastFlags
        let flags = CastFlags::read(r)?;

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
        } else {
            None
        };

        let flags = SMSG_SPELL_STARTCastFlags {
            inner: flags.as_u16(),
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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // cast_item: PackedGuid
        self.cast_item.write_packed(w)?;

        // caster: PackedGuid
        self.caster.write_packed(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // flags: CastFlags
        self.flags.write(w)?;

        // timer: u32
        w.write_all(&self.timer.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write(w)?;

        if let Some(s) = &self.flags.ammo {
            s.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_SPELL_START {
    fn size(&self) -> usize {
        self.cast_item.size() // cast_item: PackedGuid
        + self.caster.size() // caster: PackedGuid
        + 4 // spell: u32
        + self.flags.size() // flags: CastFlags and subfields
        + 4 // timer: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}

impl MaximumPossibleSized for SMSG_SPELL_START {
    fn maximum_possible_size() -> usize {
        9 // cast_item: PackedGuid
        + 9 // caster: PackedGuid
        + 4 // spell: u32
        + CastFlags::maximum_possible_size() // flags: CastFlags
        + 4 // timer: u32
        + SpellCastTargets::maximum_possible_size() // targets: SpellCastTargets
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

impl From<&SMSG_SPELL_STARTCastFlags> for CastFlags {
    fn from(e: &SMSG_SPELL_STARTCastFlags) -> Self {
        Self::new(e.inner)
    }
}

impl SMSG_SPELL_STARTCastFlags {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: CastFlags = self.into();
        a.write(w)?;
        Ok(())
    }

    pub const fn new_NONE() -> Self {
        Self {
            inner: CastFlags::NONE,
            ammo: None,
        }
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

    pub fn clear_UNKNOWN9(&mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN9.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

}
impl VariableSized for SMSG_SPELL_STARTCastFlags {
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

impl MaximumPossibleSized for SMSG_SPELL_STARTCastFlags {
    fn maximum_possible_size() -> usize {
        2 // inner: CastFlags (u16)
        + SMSG_SPELL_STARTCastFlagsAMMO::maximum_possible_size() // AMMO enumerator
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_SPELL_STARTCastFlagsAMMO {
    pub ammo_display_id: u32,
    pub ammo_inventory_type: u32,
}

impl VariableSized for SMSG_SPELL_STARTCastFlagsAMMO {
    fn size(&self) -> usize {
        4 // ammo_display_id: u32
        + 4 // ammo_inventory_type: u32
    }
}

impl MaximumPossibleSized for SMSG_SPELL_STARTCastFlagsAMMO {
    fn maximum_possible_size() -> usize {
        4 // ammo_display_id: u32
        + 4 // ammo_inventory_type: u32
    }
}

impl SMSG_SPELL_STARTCastFlagsAMMO {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.ammo_display_id.to_le_bytes())?;

        w.write_all(&self.ammo_inventory_type.to_le_bytes())?;

        Ok(())
    }
}

