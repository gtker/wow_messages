use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::SpellCastTargets;
use crate::world::vanilla::SpellMiss;
use crate::world::vanilla::CastFlags;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_go.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_go.wowm#L1):
/// ```text
/// smsg SMSG_SPELL_GO = 0x0132 {
///     PackedGuid cast_item;
///     PackedGuid caster;
///     u32 spell;
///     CastFlags flags;
///     u8 amount_of_hits;
///     Guid[amount_of_hits] hits;
///     u8 amount_of_misses;
///     SpellMiss[amount_of_misses] misses;
///     SpellCastTargets targets;
///     if (flags & AMMO) {
///         u32 ammo_display_id;
///         u32 ammo_inventory_type;
///     }
/// }
/// ```
pub struct SMSG_SPELL_GO {
    /// cmangos/vmangos/mangoszero: if cast item is used, set this to guid of cast item, otherwise set it to same as caster.
    ///
    pub cast_item: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub flags: SMSG_SPELL_GO_CastFlags,
    pub hits: Vec<Guid>,
    pub misses: Vec<SpellMiss>,
    pub targets: SpellCastTargets,
}

impl crate::Message for SMSG_SPELL_GO {
    const OPCODE: u32 = 0x0132;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // cast_item: PackedGuid
        self.cast_item.write_packed_guid_into_vec(w);

        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(w);

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // flags: CastFlags
        w.write_all(&(self.flags.as_int() as u16).to_le_bytes())?;

        // amount_of_hits: u8
        w.write_all(&(self.hits.len() as u8).to_le_bytes())?;

        // hits: Guid[amount_of_hits]
        for i in self.hits.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        // amount_of_misses: u8
        w.write_all(&(self.misses.len() as u8).to_le_bytes())?;

        // misses: SpellMiss[amount_of_misses]
        for i in self.misses.iter() {
            i.write_into_vec(w)?;
        }

        // targets: SpellCastTargets
        self.targets.write_into_vec(w)?;

        if let Some(if_statement) = &self.flags.ammo {
            // ammo_display_id: u32
            w.write_all(&if_statement.ammo_display_id.to_le_bytes())?;

            // ammo_inventory_type: u32
            w.write_all(&if_statement.ammo_inventory_type.to_le_bytes())?;

        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(14..=5472).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0132, size: body_size as u32 });
        }

        // cast_item: PackedGuid
        let cast_item = Guid::read_packed(r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // flags: CastFlags
        let flags = CastFlags::new(crate::util::read_u16_le(r)?);

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

            Some(SMSG_SPELL_GO_CastFlags_Ammo {
                ammo_display_id,
                ammo_inventory_type,
            })
        }
        else {
            None
        };

        let flags = SMSG_SPELL_GO_CastFlags {
            inner: flags.as_int(),
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

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SPELL_GO {}

impl SMSG_SPELL_GO {
    pub(crate) fn size(&self) -> usize {
        self.cast_item.size() // cast_item: Guid
        + self.caster.size() // caster: Guid
        + 4 // spell: u32
        + self.flags.size() // flags: SMSG_SPELL_GO_CastFlags
        + 1 // amount_of_hits: u8
        + self.hits.iter().fold(0, |acc, _| acc + 8) // hits: Guid[amount_of_hits]
        + 1 // amount_of_misses: u8
        + self.misses.len() * 12 // misses: SpellMiss[amount_of_misses]
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SMSG_SPELL_GO_CastFlags {
    inner: u16,
    ammo: Option<SMSG_SPELL_GO_CastFlags_Ammo>,
}

impl SMSG_SPELL_GO_CastFlags {
    pub const fn new(inner: u16, ammo: Option<SMSG_SPELL_GO_CastFlags_Ammo>,) -> Self {
        Self {
            inner,
            ammo, 
        }
    }

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

    pub fn set_HIDDEN_COMBATLOG(mut self) -> Self {
        self.inner |= CastFlags::HIDDEN_COMBATLOG;
        self
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

    pub fn set_UNKNOWN2(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN2;
        self
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

    pub fn set_UNKNOWN3(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN3;
        self
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

    pub fn set_UNKNOWN4(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN4;
        self
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

    pub fn set_UNKNOWN5(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN5;
        self
    }

    pub const fn get_UNKNOWN5(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN5) != 0
    }

    pub fn clear_UNKNOWN5(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN5.reverse_bits();
        self
    }

    pub const fn new_AMMO(ammo: SMSG_SPELL_GO_CastFlags_Ammo) -> Self {
        Self {
            inner: CastFlags::AMMO,
            ammo: Some(ammo),
        }
    }

    pub fn set_AMMO(mut self, ammo: SMSG_SPELL_GO_CastFlags_Ammo) -> Self {
        self.inner |= CastFlags::AMMO;
        self.ammo = Some(ammo);
        self
    }

    pub const fn get_AMMO(&self) -> Option<&SMSG_SPELL_GO_CastFlags_Ammo> {
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

    pub fn set_UNKNOWN7(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN7;
        self
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

    pub fn set_UNKNOWN8(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN8;
        self
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

    pub fn set_UNKNOWN9(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN9;
        self
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
impl SMSG_SPELL_GO_CastFlags {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SMSG_SPELL_GO_CastFlags_Ammo {
    pub ammo_display_id: u32,
    pub ammo_inventory_type: u32,
}

impl SMSG_SPELL_GO_CastFlags_Ammo {
    pub(crate) fn size(&self) -> usize {
        4 // ammo_display_id: u32
        + 4 // ammo_inventory_type: u32
    }
}

