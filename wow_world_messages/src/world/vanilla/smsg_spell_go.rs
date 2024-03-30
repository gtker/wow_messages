use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    CastFlags, SpellCastTargetFlags, SpellCastTargets, SpellMiss, SpellMissInfo, 
    Vector3d,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_go.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_go.wowm#L1):
/// ```text
/// smsg SMSG_SPELL_GO = 0x0132 {
///     PackedGuid cast_item;
///     PackedGuid caster;
///     Spell spell;
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
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_SPELL_GO {
    /// cmangos/vmangos/mangoszero: if cast item is used, set this to guid of cast item, otherwise set it to same as caster.
    pub cast_item: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub flags: SMSG_SPELL_GO_CastFlags,
    pub hits: Vec<Guid>,
    pub misses: Vec<SpellMiss>,
    pub targets: SpellCastTargets,
}

impl crate::private::Sealed for SMSG_SPELL_GO {}
impl SMSG_SPELL_GO {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(12..=4704).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // cast_item: PackedGuid
        let cast_item = crate::util::read_packed_guid(&mut r)?;

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // spell: Spell
        let spell = crate::util::read_u32_le(&mut r)?;

        // flags: CastFlags
        let flags = CastFlags::new(crate::util::read_u16_le(&mut r)?);

        // amount_of_hits: u8
        let amount_of_hits = crate::util::read_u8_le(&mut r)?;

        // hits: Guid[amount_of_hits]
        let hits = {
            let mut hits = Vec::with_capacity(amount_of_hits as usize);
            for _ in 0..amount_of_hits {
                hits.push(crate::util::read_guid(&mut r)?);
            }
            hits
        };

        // amount_of_misses: u8
        let amount_of_misses = crate::util::read_u8_le(&mut r)?;

        // misses: SpellMiss[amount_of_misses]
        let misses = {
            let mut misses = Vec::with_capacity(amount_of_misses as usize);
            for _ in 0..amount_of_misses {
                misses.push(SpellMiss::read(&mut r)?);
            }
            misses
        };

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(&mut r)?;

        let flags_ammo = if flags.is_ammo() {
            // ammo_display_id: u32
            let ammo_display_id = crate::util::read_u32_le(&mut r)?;

            // ammo_inventory_type: u32
            let ammo_inventory_type = crate::util::read_u32_le(&mut r)?;

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
            ammo: flags_ammo,
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

impl crate::Message for SMSG_SPELL_GO {
    const OPCODE: u32 = 0x0132;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SPELL_GO"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELL_GO {{").unwrap();
        // Members
        writeln!(s, "    cast_item = {};", self.cast_item.guid()).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    flags = {};", CastFlags::new(self.flags.as_int()).as_test_case_value()).unwrap();
        writeln!(s, "    amount_of_hits = {};", self.hits.len()).unwrap();
        writeln!(s, "    hits = [").unwrap();
        for v in self.hits.as_slice() {
            write!(s, "{v:#08X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    amount_of_misses = {};", self.misses.len()).unwrap();
        writeln!(s, "    misses = [").unwrap();
        for v in self.misses.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            target = {};", v.target.guid()).unwrap();
            writeln!(s, "            miss_info = {};", v.miss_info.as_test_case_value()).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        // targets: SpellCastTargets
        writeln!(s, "    targets = {{").unwrap();
        // Members
        writeln!(s, "        target_flags = {};", SpellCastTargetFlags::new(self.targets.target_flags.as_int()).as_test_case_value()).unwrap();
        if let Some(if_statement) = &self.targets.target_flags.get_unit() {
            writeln!(s, "        unit_target = {};", if_statement.unit_target.guid()).unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_gameobject() {
            match if_statement {
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Gameobject::Gameobject {
                    gameobject,
                } => {
                    writeln!(s, "        gameobject = {};", gameobject.guid()).unwrap();
                }
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Gameobject::ObjectUnk {
                    object_unk,
                } => {
                    writeln!(s, "        object_unk = {};", object_unk.guid()).unwrap();
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_item() {
            match if_statement {
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Item::Item {
                    item,
                } => {
                    writeln!(s, "        item = {};", item.guid()).unwrap();
                }
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
                    trade_item,
                } => {
                    writeln!(s, "        trade_item = {};", trade_item.guid()).unwrap();
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_source_location() {
            // source: Vector3d
            writeln!(s, "        source = {{").unwrap();
            // Members
            writeln!(s, "            x = {};", if if_statement.source.x.to_string().contains('.') { if_statement.source.x.to_string() } else { format!("{}.0", if_statement.source.x) }).unwrap();
            writeln!(s, "            y = {};", if if_statement.source.y.to_string().contains('.') { if_statement.source.y.to_string() } else { format!("{}.0", if_statement.source.y) }).unwrap();
            writeln!(s, "            z = {};", if if_statement.source.z.to_string().contains('.') { if_statement.source.z.to_string() } else { format!("{}.0", if_statement.source.z) }).unwrap();

            writeln!(s, "        }};").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_dest_location() {
            // destination: Vector3d
            writeln!(s, "        destination = {{").unwrap();
            // Members
            writeln!(s, "            x = {};", if if_statement.destination.x.to_string().contains('.') { if_statement.destination.x.to_string() } else { format!("{}.0", if_statement.destination.x) }).unwrap();
            writeln!(s, "            y = {};", if if_statement.destination.y.to_string().contains('.') { if_statement.destination.y.to_string() } else { format!("{}.0", if_statement.destination.y) }).unwrap();
            writeln!(s, "            z = {};", if if_statement.destination.z.to_string().contains('.') { if_statement.destination.z.to_string() } else { format!("{}.0", if_statement.destination.z) }).unwrap();

            writeln!(s, "        }};").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_string() {
            writeln!(s, "        target_string = \"{}\";", if_statement.target_string).unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_corpse() {
            match if_statement {
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Corpse::Corpse {
                    corpse,
                } => {
                    writeln!(s, "        corpse = {};", corpse.guid()).unwrap();
                }
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Corpse::PvpCorpse {
                    pvp_corpse,
                } => {
                    writeln!(s, "        pvp_corpse = {};", pvp_corpse.guid()).unwrap();
                }
            }
        }


        writeln!(s, "    }};").unwrap();
        if let Some(if_statement) = &self.flags.get_ammo() {
            writeln!(s, "    ammo_display_id = {};", if_statement.ammo_display_id).unwrap();
            writeln!(s, "    ammo_inventory_type = {};", if_statement.ammo_inventory_type).unwrap();
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 306_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.cast_item), "cast_item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.caster), "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_hits", "    ");
        if !self.hits.is_empty() {
            writeln!(s, "    /* hits: Guid[amount_of_hits] start */").unwrap();
            for (i, v) in self.hits.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 8, &format!("hits {i}"), "    ");
            }
            writeln!(s, "    /* hits: Guid[amount_of_hits] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_misses", "    ");
        if !self.misses.is_empty() {
            writeln!(s, "    /* misses: SpellMiss[amount_of_misses] start */").unwrap();
            for (i, v) in self.misses.iter().enumerate() {
                writeln!(s, "    /* misses: SpellMiss[amount_of_misses] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "miss_info", "        ");
                writeln!(s, "    /* misses: SpellMiss[amount_of_misses] {i} end */").unwrap();
            }
            writeln!(s, "    /* misses: SpellMiss[amount_of_misses] end */").unwrap();
        }
        writeln!(s, "    /* targets: SpellCastTargets start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 2, "target_flags", "        ");
        if let Some(if_statement) = &self.targets.target_flags.get_unit() {
            crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.unit_target), "unit_target", "        ");
        }

        if let Some(if_statement) = &self.targets.target_flags.get_gameobject() {
            match if_statement {
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Gameobject::Gameobject {
                    gameobject,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&gameobject), "gameobject", "        ");
                }
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Gameobject::ObjectUnk {
                    object_unk,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&object_unk), "object_unk", "        ");
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_item() {
            match if_statement {
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Item::Item {
                    item,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&item), "item", "        ");
                }
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
                    trade_item,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&trade_item), "trade_item", "        ");
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_source_location() {
            writeln!(s, "    /* source: Vector3d start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "            ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "            ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "            ");
            writeln!(s, "    /* source: Vector3d end */").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_dest_location() {
            writeln!(s, "    /* destination: Vector3d start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "            ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "            ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "            ");
            writeln!(s, "    /* destination: Vector3d end */").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_string() {
            crate::util::write_bytes(&mut s, &mut bytes, if_statement.target_string.len() + 1, "target_string", "        ");
        }

        if let Some(if_statement) = &self.targets.target_flags.get_corpse() {
            match if_statement {
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Corpse::Corpse {
                    corpse,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&corpse), "corpse", "        ");
                }
                crate::vanilla::SpellCastTargets_SpellCastTargetFlags_Corpse::PvpCorpse {
                    pvp_corpse,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&pvp_corpse), "pvp_corpse", "        ");
                }
            }
        }

        writeln!(s, "    /* targets: SpellCastTargets end */").unwrap();
        if let Some(if_statement) = &self.flags.get_ammo() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "ammo_display_id", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "ammo_inventory_type", "    ");
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // cast_item: PackedGuid
        crate::util::write_packed_guid(&self.cast_item, &mut w)?;

        // caster: PackedGuid
        crate::util::write_packed_guid(&self.caster, &mut w)?;

        // spell: Spell
        w.write_all(&self.spell.to_le_bytes())?;

        // flags: CastFlags
        w.write_all(&(self.flags.as_int().to_le_bytes()))?;

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
            i.write_into_vec(&mut w)?;
        }

        // targets: SpellCastTargets
        self.targets.write_into_vec(&mut w)?;

        if let Some(if_statement) = &self.flags.ammo {
            // ammo_display_id: u32
            w.write_all(&if_statement.ammo_display_id.to_le_bytes())?;

            // ammo_inventory_type: u32
            w.write_all(&if_statement.ammo_inventory_type.to_le_bytes())?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(306, "SMSG_SPELL_GO", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELL_GO {}

impl SMSG_SPELL_GO {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.cast_item) // cast_item: PackedGuid
        + crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 4 // spell: Spell
        + self.flags.size() // flags: SMSG_SPELL_GO_CastFlags
        + 1 // amount_of_hits: u8
        + self.hits.len() *  8 // hits: Guid[amount_of_hits]
        + 1 // amount_of_misses: u8
        + self.misses.len() * 9 // misses: SpellMiss[amount_of_misses]
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

    pub const fn new_hidden_combatlog() -> Self {
        Self {
            inner: CastFlags::HIDDEN_COMBATLOG,
            ammo: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_hidden_combatlog(mut self) -> Self {
        self.inner |= CastFlags::HIDDEN_COMBATLOG;
        self
    }

    pub const fn get_hidden_combatlog(&self) -> bool {
        (self.inner & CastFlags::HIDDEN_COMBATLOG) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_hidden_combatlog(mut self) -> Self {
        self.inner &= CastFlags::HIDDEN_COMBATLOG.reverse_bits();
        self
    }

    pub const fn new_unknown2() -> Self {
        Self {
            inner: CastFlags::UNKNOWN2,
            ammo: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown2(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN2;
        self
    }

    pub const fn get_unknown2(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN2) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown2(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN2.reverse_bits();
        self
    }

    pub const fn new_unknown3() -> Self {
        Self {
            inner: CastFlags::UNKNOWN3,
            ammo: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown3(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN3;
        self
    }

    pub const fn get_unknown3(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN3) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown3(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN3.reverse_bits();
        self
    }

    pub const fn new_unknown4() -> Self {
        Self {
            inner: CastFlags::UNKNOWN4,
            ammo: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown4(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN4;
        self
    }

    pub const fn get_unknown4(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN4) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown4(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN4.reverse_bits();
        self
    }

    pub const fn new_unknown5() -> Self {
        Self {
            inner: CastFlags::UNKNOWN5,
            ammo: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown5(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN5;
        self
    }

    pub const fn get_unknown5(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN5) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown5(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN5.reverse_bits();
        self
    }

    pub const fn new_ammo(ammo: SMSG_SPELL_GO_CastFlags_Ammo) -> Self {
        Self {
            inner: CastFlags::AMMO,
            ammo: Some(ammo),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_ammo(mut self, ammo: SMSG_SPELL_GO_CastFlags_Ammo) -> Self {
        self.inner |= CastFlags::AMMO;
        self.ammo = Some(ammo);
        self
    }

    pub const fn get_ammo(&self) -> Option<&SMSG_SPELL_GO_CastFlags_Ammo> {
        self.ammo.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_ammo(mut self) -> Self {
        self.inner &= CastFlags::AMMO.reverse_bits();
        self.ammo = None;
        self
    }

    pub const fn new_unknown7() -> Self {
        Self {
            inner: CastFlags::UNKNOWN7,
            ammo: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown7(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN7;
        self
    }

    pub const fn get_unknown7(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN7) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown7(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN7.reverse_bits();
        self
    }

    pub const fn new_unknown8() -> Self {
        Self {
            inner: CastFlags::UNKNOWN8,
            ammo: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown8(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN8;
        self
    }

    pub const fn get_unknown8(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN8) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown8(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN8.reverse_bits();
        self
    }

    pub const fn new_unknown9() -> Self {
        Self {
            inner: CastFlags::UNKNOWN9,
            ammo: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown9(mut self) -> Self {
        self.inner |= CastFlags::UNKNOWN9;
        self
    }

    pub const fn get_unknown9(&self) -> bool {
        (self.inner & CastFlags::UNKNOWN9) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown9(mut self) -> Self {
        self.inner &= CastFlags::UNKNOWN9.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u16 {
        self.inner
    }

}
impl SMSG_SPELL_GO_CastFlags {
    pub(crate) const fn size(&self) -> usize {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_GO_CastFlags_Ammo {
    pub ammo_display_id: u32,
    pub ammo_inventory_type: u32,
}

impl SMSG_SPELL_GO_CastFlags_Ammo {
    pub(crate) const fn size(&self) -> usize {
        4 // ammo_display_id: u32
        + 4 // ammo_inventory_type: u32
    }
}

