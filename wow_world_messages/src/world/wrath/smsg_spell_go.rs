use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    GameobjectCastFlags, Power, SpellCastTargetFlags, SpellCastTargets, SpellMiss, 
    SpellMissInfo, Vector3d,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_go.wowm:47`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_go.wowm#L47):
/// ```text
/// smsg SMSG_SPELL_GO = 0x0132 {
///     PackedGuid cast_item;
///     PackedGuid caster;
///     u8 extra_casts;
///     Spell spell;
///     GameobjectCastFlags flags;
///     u32 timestamp;
///     u8 amount_of_hits;
///     Guid[amount_of_hits] hits;
///     u8 amount_of_misses;
///     SpellMiss[amount_of_misses] misses;
///     SpellCastTargets targets;
///     if (flags & POWER_UPDATE) {
///         (u32)Power power;
///     }
///     if (flags & RUNE_UPDATE) {
///         u8 rune_mask_initial;
///         u8 rune_mask_after_cast;
///         u8[6] rune_cooldowns;
///     }
///     if (flags & ADJUST_MISSILE) {
///         f32 elevation;
///         u32 delay_trajectory;
///     }
///     if (flags & AMMO) {
///         u32 ammo_display_id;
///         u32 ammo_inventory_type;
///     }
///     if (flags & VISUAL_CHAIN) {
///         u32 unknown1;
///         u32 unknown2;
///     }
///     if (flags & DEST_LOCATION) {
///         u8 unknown3;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_SPELL_GO {
    /// cmangos/vmangos/mangoszero: if cast item is used, set this to guid of cast item, otherwise set it to same as caster.
    pub cast_item: Guid,
    pub caster: Guid,
    pub extra_casts: u8,
    pub spell: u32,
    pub flags: SMSG_SPELL_GO_GameobjectCastFlags,
    pub timestamp: u32,
    pub hits: Vec<Guid>,
    pub misses: Vec<SpellMiss>,
    pub targets: SpellCastTargets,
}

impl crate::private::Sealed for SMSG_SPELL_GO {}
impl SMSG_SPELL_GO {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(21..=4980).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // cast_item: PackedGuid
        let cast_item = crate::util::read_packed_guid(&mut r)?;

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // extra_casts: u8
        let extra_casts = crate::util::read_u8_le(&mut r)?;

        // spell: Spell
        let spell = crate::util::read_u32_le(&mut r)?;

        // flags: GameobjectCastFlags
        let flags = GameobjectCastFlags::new(crate::util::read_u32_le(&mut r)?);

        // timestamp: u32
        let timestamp = crate::util::read_u32_le(&mut r)?;

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

        let flags_power_update = if flags.is_power_update() {
            // power: Power
            let power = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

            Some(SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate {
                power,
            })
        }
        else {
            None
        };

        let flags_rune_update = if flags.is_rune_update() {
            // rune_mask_initial: u8
            let rune_mask_initial = crate::util::read_u8_le(&mut r)?;

            // rune_mask_after_cast: u8
            let rune_mask_after_cast = crate::util::read_u8_le(&mut r)?;

            // rune_cooldowns: u8[6]
            let rune_cooldowns = {
                let mut rune_cooldowns = [0_u8; 6];
                r.read_exact(&mut rune_cooldowns)?;
                rune_cooldowns
            };

            Some(SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate {
                rune_cooldowns,
                rune_mask_after_cast,
                rune_mask_initial,
            })
        }
        else {
            None
        };

        let flags_adjust_missile = if flags.is_adjust_missile() {
            // elevation: f32
            let elevation = crate::util::read_f32_le(&mut r)?;

            // delay_trajectory: u32
            let delay_trajectory = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile {
                delay_trajectory,
                elevation,
            })
        }
        else {
            None
        };

        let flags_ammo = if flags.is_ammo() {
            // ammo_display_id: u32
            let ammo_display_id = crate::util::read_u32_le(&mut r)?;

            // ammo_inventory_type: u32
            let ammo_inventory_type = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_SPELL_GO_GameobjectCastFlags_Ammo {
                ammo_display_id,
                ammo_inventory_type,
            })
        }
        else {
            None
        };

        let flags_visual_chain = if flags.is_visual_chain() {
            // unknown1: u32
            let unknown1 = crate::util::read_u32_le(&mut r)?;

            // unknown2: u32
            let unknown2 = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_SPELL_GO_GameobjectCastFlags_VisualChain {
                unknown1,
                unknown2,
            })
        }
        else {
            None
        };

        let flags_dest_location = if flags.is_dest_location() {
            // unknown3: u8
            let unknown3 = crate::util::read_u8_le(&mut r)?;

            Some(SMSG_SPELL_GO_GameobjectCastFlags_DestLocation {
                unknown3,
            })
        }
        else {
            None
        };

        let flags = SMSG_SPELL_GO_GameobjectCastFlags {
            inner: flags.as_int(),
            ammo: flags_ammo,
            dest_location: flags_dest_location,
            power_update: flags_power_update,
            adjust_missile: flags_adjust_missile,
            visual_chain: flags_visual_chain,
            rune_update: flags_rune_update,
        };

        Ok(Self {
            cast_item,
            caster,
            extra_casts,
            spell,
            flags,
            timestamp,
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
        writeln!(s, "    extra_casts = {};", self.extra_casts).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    flags = {};", GameobjectCastFlags::new(self.flags.as_int()).as_test_case_value()).unwrap();
        writeln!(s, "    timestamp = {};", self.timestamp).unwrap();
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
            writeln!(s, "            miss_info = {};", SpellMissInfo::try_from(v.miss_info.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.miss_info {
                crate::shared::spell_miss_tbc_wrath::SpellMiss_SpellMissInfo::Reflect {
                    reflect_result,
                } => {
                    writeln!(s, "            reflect_result = {};", reflect_result).unwrap();
                }
                _ => {}
            }


            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        // targets: SpellCastTargets
        writeln!(s, "    targets = {{").unwrap();
        // Members
        writeln!(s, "        target_flags = {};", SpellCastTargetFlags::new(self.targets.target_flags.as_int()).as_test_case_value()).unwrap();
        if let Some(if_statement) = &self.targets.target_flags.get_unit() {
            match if_statement {
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                    unit_target,
                } => {
                    writeln!(s, "        unit_target = {};", unit_target.guid()).unwrap();
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::UnitMinipet {
                    minipet_target,
                } => {
                    writeln!(s, "        minipet_target = {};", minipet_target.guid()).unwrap();
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::Gameobject {
                    gameobject_target,
                } => {
                    writeln!(s, "        gameobject_target = {};", gameobject_target.guid()).unwrap();
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::CorpseEnemy {
                    enemy_corpse_target,
                } => {
                    writeln!(s, "        enemy_corpse_target = {};", enemy_corpse_target.guid()).unwrap();
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::CorpseAlly {
                    ally_corpse_target,
                } => {
                    writeln!(s, "        ally_corpse_target = {};", ally_corpse_target.guid()).unwrap();
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_item() {
            match if_statement {
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Item::Item {
                    item_target,
                } => {
                    writeln!(s, "        item_target = {};", item_target.guid()).unwrap();
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
                    trade_item_target,
                } => {
                    writeln!(s, "        trade_item_target = {};", trade_item_target.guid()).unwrap();
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


        writeln!(s, "    }};").unwrap();
        if let Some(if_statement) = &self.flags.get_power_update() {
            writeln!(s, "    power = {};", if_statement.power.as_test_case_value()).unwrap();
        }

        if let Some(if_statement) = &self.flags.get_rune_update() {
            writeln!(s, "    rune_mask_initial = {};", if_statement.rune_mask_initial).unwrap();
            writeln!(s, "    rune_mask_after_cast = {};", if_statement.rune_mask_after_cast).unwrap();
            writeln!(s, "    rune_cooldowns = [").unwrap();
            for v in if_statement.rune_cooldowns.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "    ];").unwrap();
        }

        if let Some(if_statement) = &self.flags.get_adjust_missile() {
            writeln!(s, "    elevation = {};", if if_statement.elevation.to_string().contains('.') { if_statement.elevation.to_string() } else { format!("{}.0", if_statement.elevation) }).unwrap();
            writeln!(s, "    delay_trajectory = {};", if_statement.delay_trajectory).unwrap();
        }

        if let Some(if_statement) = &self.flags.get_ammo() {
            writeln!(s, "    ammo_display_id = {};", if_statement.ammo_display_id).unwrap();
            writeln!(s, "    ammo_inventory_type = {};", if_statement.ammo_inventory_type).unwrap();
        }

        if let Some(if_statement) = &self.flags.get_visual_chain() {
            writeln!(s, "    unknown1 = {};", if_statement.unknown1).unwrap();
            writeln!(s, "    unknown2 = {};", if_statement.unknown2).unwrap();
        }

        if let Some(if_statement) = &self.flags.get_dest_location() {
            writeln!(s, "    unknown3 = {};", if_statement.unknown3).unwrap();
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
        crate::util::write_bytes(&mut s, &mut bytes, 1, "extra_casts", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "    ");
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
                match &v.miss_info {
                    crate::shared::spell_miss_tbc_wrath::SpellMiss_SpellMissInfo::Reflect {
                        reflect_result,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "reflect_result", "        ");
                    }
                    _ => {}
                }

                writeln!(s, "    /* misses: SpellMiss[amount_of_misses] {i} end */").unwrap();
            }
            writeln!(s, "    /* misses: SpellMiss[amount_of_misses] end */").unwrap();
        }
        writeln!(s, "    /* targets: SpellCastTargets start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "target_flags", "        ");
        if let Some(if_statement) = &self.targets.target_flags.get_unit() {
            match if_statement {
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                    unit_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&unit_target), "unit_target", "        ");
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::UnitMinipet {
                    minipet_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&minipet_target), "minipet_target", "        ");
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::Gameobject {
                    gameobject_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&gameobject_target), "gameobject_target", "        ");
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::CorpseEnemy {
                    enemy_corpse_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&enemy_corpse_target), "enemy_corpse_target", "        ");
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Unit::CorpseAlly {
                    ally_corpse_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&ally_corpse_target), "ally_corpse_target", "        ");
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_item() {
            match if_statement {
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Item::Item {
                    item_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&item_target), "item_target", "        ");
                }
                crate::wrath::SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
                    trade_item_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&trade_item_target), "trade_item_target", "        ");
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

        writeln!(s, "    /* targets: SpellCastTargets end */").unwrap();
        if let Some(if_statement) = &self.flags.get_power_update() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "power", "    ");
        }

        if let Some(if_statement) = &self.flags.get_rune_update() {
            crate::util::write_bytes(&mut s, &mut bytes, 1, "rune_mask_initial", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "rune_mask_after_cast", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, if_statement.rune_cooldowns.len(), "rune_cooldowns", "    ");
        }

        if let Some(if_statement) = &self.flags.get_adjust_missile() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "elevation", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "delay_trajectory", "    ");
        }

        if let Some(if_statement) = &self.flags.get_ammo() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "ammo_display_id", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "ammo_inventory_type", "    ");
        }

        if let Some(if_statement) = &self.flags.get_visual_chain() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "    ");
        }

        if let Some(if_statement) = &self.flags.get_dest_location() {
            crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown3", "    ");
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
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

        // extra_casts: u8
        w.write_all(&self.extra_casts.to_le_bytes())?;

        // spell: Spell
        w.write_all(&self.spell.to_le_bytes())?;

        // flags: GameobjectCastFlags
        w.write_all(&(self.flags.as_int().to_le_bytes()))?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

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

        if let Some(if_statement) = &self.flags.power_update {
            // power: Power
            w.write_all(&u32::from(if_statement.power.as_int()).to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.rune_update {
            // rune_mask_initial: u8
            w.write_all(&if_statement.rune_mask_initial.to_le_bytes())?;

            // rune_mask_after_cast: u8
            w.write_all(&if_statement.rune_mask_after_cast.to_le_bytes())?;

            // rune_cooldowns: u8[6]
            for i in if_statement.rune_cooldowns.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

        }

        if let Some(if_statement) = &self.flags.adjust_missile {
            // elevation: f32
            w.write_all(&if_statement.elevation.to_le_bytes())?;

            // delay_trajectory: u32
            w.write_all(&if_statement.delay_trajectory.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.ammo {
            // ammo_display_id: u32
            w.write_all(&if_statement.ammo_display_id.to_le_bytes())?;

            // ammo_inventory_type: u32
            w.write_all(&if_statement.ammo_inventory_type.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.visual_chain {
            // unknown1: u32
            w.write_all(&if_statement.unknown1.to_le_bytes())?;

            // unknown2: u32
            w.write_all(&if_statement.unknown2.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.dest_location {
            // unknown3: u8
            w.write_all(&if_statement.unknown3.to_le_bytes())?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(306, "SMSG_SPELL_GO", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELL_GO {}

impl SMSG_SPELL_GO {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.cast_item) // cast_item: PackedGuid
        + crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 1 // extra_casts: u8
        + 4 // spell: Spell
        + self.flags.size() // flags: SMSG_SPELL_GO_GameobjectCastFlags
        + 4 // timestamp: u32
        + 1 // amount_of_hits: u8
        + self.hits.len() *  8 // hits: Guid[amount_of_hits]
        + 1 // amount_of_misses: u8
        + self.misses.iter().fold(0, |acc, x| acc + x.size()) // misses: SpellMiss[amount_of_misses]
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags {
    inner: u32,
    ammo: Option<SMSG_SPELL_GO_GameobjectCastFlags_Ammo>,
    dest_location: Option<SMSG_SPELL_GO_GameobjectCastFlags_DestLocation>,
    power_update: Option<SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate>,
    adjust_missile: Option<SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile>,
    visual_chain: Option<SMSG_SPELL_GO_GameobjectCastFlags_VisualChain>,
    rune_update: Option<SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate>,
}

impl SMSG_SPELL_GO_GameobjectCastFlags {
    pub const fn new(inner: u32, ammo: Option<SMSG_SPELL_GO_GameobjectCastFlags_Ammo>,dest_location: Option<SMSG_SPELL_GO_GameobjectCastFlags_DestLocation>,power_update: Option<SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate>,adjust_missile: Option<SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile>,visual_chain: Option<SMSG_SPELL_GO_GameobjectCastFlags_VisualChain>,rune_update: Option<SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate>,) -> Self {
        Self {
            inner,
            ammo, 
            dest_location, 
            power_update, 
            adjust_missile, 
            visual_chain, 
            rune_update, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.ammo.is_none()
        && self.dest_location.is_none()
        && self.power_update.is_none()
        && self.adjust_missile.is_none()
        && self.visual_chain.is_none()
        && self.rune_update.is_none()
    }

    pub const fn new_lock_player_cast_anim() -> Self {
        Self {
            inner: GameobjectCastFlags::LOCK_PLAYER_CAST_ANIM,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_lock_player_cast_anim(mut self) -> Self {
        self.inner |= GameobjectCastFlags::LOCK_PLAYER_CAST_ANIM;
        self
    }

    pub const fn get_lock_player_cast_anim(&self) -> bool {
        (self.inner & GameobjectCastFlags::LOCK_PLAYER_CAST_ANIM) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_lock_player_cast_anim(mut self) -> Self {
        self.inner &= GameobjectCastFlags::LOCK_PLAYER_CAST_ANIM.reverse_bits();
        self
    }

    pub const fn new_unknown2() -> Self {
        Self {
            inner: GameobjectCastFlags::UNKNOWN2,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown2(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNKNOWN2;
        self
    }

    pub const fn get_unknown2(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNKNOWN2) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown2(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNKNOWN2.reverse_bits();
        self
    }

    pub const fn new_unknown4() -> Self {
        Self {
            inner: GameobjectCastFlags::UNKNOWN4,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown4(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNKNOWN4;
        self
    }

    pub const fn get_unknown4(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNKNOWN4) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown4(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNKNOWN4.reverse_bits();
        self
    }

    pub const fn new_unknown8() -> Self {
        Self {
            inner: GameobjectCastFlags::UNKNOWN8,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown8(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNKNOWN8;
        self
    }

    pub const fn get_unknown8(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNKNOWN8) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown8(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNKNOWN8.reverse_bits();
        self
    }

    pub const fn new_unknown16() -> Self {
        Self {
            inner: GameobjectCastFlags::UNKNOWN16,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown16(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNKNOWN16;
        self
    }

    pub const fn get_unknown16(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNKNOWN16) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown16(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNKNOWN16.reverse_bits();
        self
    }

    pub const fn new_ammo(ammo: SMSG_SPELL_GO_GameobjectCastFlags_Ammo) -> Self {
        Self {
            inner: GameobjectCastFlags::AMMO,
            ammo: Some(ammo),
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_ammo(mut self, ammo: SMSG_SPELL_GO_GameobjectCastFlags_Ammo) -> Self {
        self.inner |= GameobjectCastFlags::AMMO;
        self.ammo = Some(ammo);
        self
    }

    pub const fn get_ammo(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_Ammo> {
        self.ammo.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_ammo(mut self) -> Self {
        self.inner &= GameobjectCastFlags::AMMO.reverse_bits();
        self.ammo = None;
        self
    }

    pub const fn new_dest_location(dest_location: SMSG_SPELL_GO_GameobjectCastFlags_DestLocation) -> Self {
        Self {
            inner: GameobjectCastFlags::DEST_LOCATION,
            ammo: None,
            dest_location: Some(dest_location),
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_dest_location(mut self, dest_location: SMSG_SPELL_GO_GameobjectCastFlags_DestLocation) -> Self {
        self.inner |= GameobjectCastFlags::DEST_LOCATION;
        self.dest_location = Some(dest_location);
        self
    }

    pub const fn get_dest_location(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_DestLocation> {
        self.dest_location.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_dest_location(mut self) -> Self {
        self.inner &= GameobjectCastFlags::DEST_LOCATION.reverse_bits();
        self.dest_location = None;
        self
    }

    pub const fn new_item_caster() -> Self {
        Self {
            inner: GameobjectCastFlags::ITEM_CASTER,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_item_caster(mut self) -> Self {
        self.inner |= GameobjectCastFlags::ITEM_CASTER;
        self
    }

    pub const fn get_item_caster(&self) -> bool {
        (self.inner & GameobjectCastFlags::ITEM_CASTER) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_item_caster(mut self) -> Self {
        self.inner &= GameobjectCastFlags::ITEM_CASTER.reverse_bits();
        self
    }

    pub const fn new_unk200() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK200,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk200(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK200;
        self
    }

    pub const fn get_unk200(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK200) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk200(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK200.reverse_bits();
        self
    }

    pub const fn new_extra_message() -> Self {
        Self {
            inner: GameobjectCastFlags::EXTRA_MESSAGE,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_extra_message(mut self) -> Self {
        self.inner |= GameobjectCastFlags::EXTRA_MESSAGE;
        self
    }

    pub const fn get_extra_message(&self) -> bool {
        (self.inner & GameobjectCastFlags::EXTRA_MESSAGE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_extra_message(mut self) -> Self {
        self.inner &= GameobjectCastFlags::EXTRA_MESSAGE.reverse_bits();
        self
    }

    pub const fn new_power_update(power_update: SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate) -> Self {
        Self {
            inner: GameobjectCastFlags::POWER_UPDATE,
            ammo: None,
            dest_location: None,
            power_update: Some(power_update),
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_power_update(mut self, power_update: SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate) -> Self {
        self.inner |= GameobjectCastFlags::POWER_UPDATE;
        self.power_update = Some(power_update);
        self
    }

    pub const fn get_power_update(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate> {
        self.power_update.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_power_update(mut self) -> Self {
        self.inner &= GameobjectCastFlags::POWER_UPDATE.reverse_bits();
        self.power_update = None;
        self
    }

    pub const fn new_unk2000() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK2000,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk2000(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK2000;
        self
    }

    pub const fn get_unk2000(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK2000) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk2000(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK2000.reverse_bits();
        self
    }

    pub const fn new_unk1000() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK1000,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk1000(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK1000;
        self
    }

    pub const fn get_unk1000(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK1000) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk1000(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK1000.reverse_bits();
        self
    }

    pub const fn new_unk8000() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK8000,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk8000(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK8000;
        self
    }

    pub const fn get_unk8000(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK8000) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk8000(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK8000.reverse_bits();
        self
    }

    pub const fn new_adjust_missile(adjust_missile: SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile) -> Self {
        Self {
            inner: GameobjectCastFlags::ADJUST_MISSILE,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: Some(adjust_missile),
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_adjust_missile(mut self, adjust_missile: SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile) -> Self {
        self.inner |= GameobjectCastFlags::ADJUST_MISSILE;
        self.adjust_missile = Some(adjust_missile);
        self
    }

    pub const fn get_adjust_missile(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile> {
        self.adjust_missile.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_adjust_missile(mut self) -> Self {
        self.inner &= GameobjectCastFlags::ADJUST_MISSILE.reverse_bits();
        self.adjust_missile = None;
        self
    }

    pub const fn new_unk40000() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK40000,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk40000(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK40000;
        self
    }

    pub const fn get_unk40000(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK40000) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk40000(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK40000.reverse_bits();
        self
    }

    pub const fn new_visual_chain(visual_chain: SMSG_SPELL_GO_GameobjectCastFlags_VisualChain) -> Self {
        Self {
            inner: GameobjectCastFlags::VISUAL_CHAIN,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: Some(visual_chain),
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_visual_chain(mut self, visual_chain: SMSG_SPELL_GO_GameobjectCastFlags_VisualChain) -> Self {
        self.inner |= GameobjectCastFlags::VISUAL_CHAIN;
        self.visual_chain = Some(visual_chain);
        self
    }

    pub const fn get_visual_chain(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_VisualChain> {
        self.visual_chain.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_visual_chain(mut self) -> Self {
        self.inner &= GameobjectCastFlags::VISUAL_CHAIN.reverse_bits();
        self.visual_chain = None;
        self
    }

    pub const fn new_rune_update(rune_update: SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate) -> Self {
        Self {
            inner: GameobjectCastFlags::RUNE_UPDATE,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: Some(rune_update),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_rune_update(mut self, rune_update: SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate) -> Self {
        self.inner |= GameobjectCastFlags::RUNE_UPDATE;
        self.rune_update = Some(rune_update);
        self
    }

    pub const fn get_rune_update(&self) -> Option<&SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate> {
        self.rune_update.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_rune_update(mut self) -> Self {
        self.inner &= GameobjectCastFlags::RUNE_UPDATE.reverse_bits();
        self.rune_update = None;
        self
    }

    pub const fn new_unk400000() -> Self {
        Self {
            inner: GameobjectCastFlags::UNK400000,
            ammo: None,
            dest_location: None,
            power_update: None,
            adjust_missile: None,
            visual_chain: None,
            rune_update: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk400000(mut self) -> Self {
        self.inner |= GameobjectCastFlags::UNK400000;
        self
    }

    pub const fn get_unk400000(&self) -> bool {
        (self.inner & GameobjectCastFlags::UNK400000) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk400000(mut self) -> Self {
        self.inner &= GameobjectCastFlags::UNK400000.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_SPELL_GO_GameobjectCastFlags {
    pub(crate) const fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.ammo {
                8
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.dest_location {
                1
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.power_update {
                4
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.adjust_missile {
                8
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.visual_chain {
                8
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.rune_update {
                8
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_Ammo {
    pub ammo_display_id: u32,
    pub ammo_inventory_type: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_DestLocation {
    pub unknown3: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_PowerUpdate {
    pub power: Power,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_AdjustMissile {
    pub delay_trajectory: u32,
    pub elevation: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_VisualChain {
    pub unknown1: u32,
    pub unknown2: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_GO_GameobjectCastFlags_RuneUpdate {
    pub rune_cooldowns: [u8; 6],
    pub rune_mask_after_cast: u8,
    pub rune_mask_initial: u8,
}

