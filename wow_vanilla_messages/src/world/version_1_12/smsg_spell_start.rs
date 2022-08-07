use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::CastFlags;
use crate::world::version_1_12::SpellCastTargets;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_start.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_start.wowm#L3):
/// ```text
/// smsg SMSG_SPELL_START = 0x0131 {
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
    /// cmangos/vmangos/mangoszero: if cast item is used, set this to guid of cast item, otherwise set it to same as caster.
    ///
    pub cast_item: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub flags: SMSG_SPELL_START_CastFlags,
    pub timer: u32,
    pub targets: SpellCastTargets,
}

impl ServerMessage for SMSG_SPELL_START {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // cast_item: PackedGuid
        self.cast_item.write_packed_guid_into_vec(w);

        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(w);

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // flags: CastFlags
        w.write_all(&(self.flags.as_int() as u16).to_le_bytes())?;

        // timer: u32
        w.write_all(&self.timer.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(w)?;

        if let Some(if_statement) = &self.flags.ammo {
            // ammo_display_id: u32
            w.write_all(&if_statement.ammo_display_id.to_le_bytes())?;

            // ammo_inventory_type: u32
            w.write_all(&if_statement.ammo_inventory_type.to_le_bytes())?;

        }

        Ok(())
    }
    const OPCODE: u16 = 0x0131;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
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

            Some(SMSG_SPELL_START_CastFlags_Ammo {
                ammo_display_id,
                ammo_inventory_type,
            })
        }
        else {
            None
        };

        let flags = SMSG_SPELL_START_CastFlags {
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

}

impl SMSG_SPELL_START {
    pub(crate) fn size(&self) -> usize {
        self.cast_item.size() // cast_item: Guid
        + self.caster.size() // caster: Guid
        + 4 // spell: u32
        + self.flags.size() // flags: SMSG_SPELL_START_CastFlags
        + 4 // timer: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SMSG_SPELL_START_CastFlags {
    inner: u16,
    ammo: Option<SMSG_SPELL_START_CastFlags_Ammo>,
}

impl SMSG_SPELL_START_CastFlags {
    pub const fn empty() -> Self {
        Self {
            inner: 0,
            ammo: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.ammo.is_none()
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

    pub fn clear_HIDDEN_COMBATLOG(mut self) -> Self {
        self.inner &= CastFlags::HIDDEN_COMBATLOG.reverse_bits();
        self
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

    pub fn clear_UNKNOWN2(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN2.reverse_bits();
        self
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

    pub fn clear_UNKNOWN3(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN3.reverse_bits();
        self
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

    pub fn clear_UNKNOWN4(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN4.reverse_bits();
        self
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

    pub fn clear_UNKNOWN5(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN5.reverse_bits();
        self
    }

    pub const fn new_AMMO(ammo: SMSG_SPELL_START_CastFlags_Ammo) -> Self {
        Self {
            inner: CastFlags::AMMO,
            ammo: Some(ammo),
        }
    }

    pub fn set_AMMO(&mut self, ammo: SMSG_SPELL_START_CastFlags_Ammo) -> Self {
        self.inner |= CastFlags::AMMO;
        self.ammo = Some(ammo);
        self.clone()
    }

    pub const fn get_AMMO(&self) -> Option<&SMSG_SPELL_START_CastFlags_Ammo> {
        self.ammo.as_ref()
    }

    pub fn clear_AMMO(mut self) -> Self {
        self.inner &= CastFlags::AMMO.reverse_bits();
        self.ammo = None;
        self
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

    pub fn clear_UNKNOWN7(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN7.reverse_bits();
        self
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

    pub fn clear_UNKNOWN8(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN8.reverse_bits();
        self
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

    pub fn clear_UNKNOWN9(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN9.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u16 {
        self.inner
    }

}
impl SMSG_SPELL_START_CastFlags {
    pub(crate) fn size(&self) -> usize {
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
pub struct SMSG_SPELL_START_CastFlags_Ammo {
    pub ammo_display_id: u32,
    pub ammo_inventory_type: u32,
}

impl SMSG_SPELL_START_CastFlags_Ammo {
    pub(crate) fn size(&self) -> usize {
        4 // ammo_display_id: u32
        + 4 // ammo_inventory_type: u32
    }
}

