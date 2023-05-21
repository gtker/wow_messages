use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    SpellCastTargetFlags, SpellCastTargets, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_use_item.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_use_item.wowm#L10):
/// ```text
/// cmsg CMSG_USE_ITEM = 0x00AB {
///     u8 bag_index;
///     u8 bag_slot;
///     u8 spell_index;
///     u8 cast_count;
///     Guid item;
///     SpellCastTargets targets;
/// }
/// ```
pub struct CMSG_USE_ITEM {
    pub bag_index: u8,
    pub bag_slot: u8,
    pub spell_index: u8,
    /// mangosone: next cast if exists (single or not)
    pub cast_count: u8,
    pub item: Guid,
    pub targets: SpellCastTargets,
}

impl crate::private::Sealed for CMSG_USE_ITEM {}
impl CMSG_USE_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(16..=332).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00AB, size: body_size });
        }

        // bag_index: u8
        let bag_index = crate::util::read_u8_le(&mut r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(&mut r)?;

        // spell_index: u8
        let spell_index = crate::util::read_u8_le(&mut r)?;

        // cast_count: u8
        let cast_count = crate::util::read_u8_le(&mut r)?;

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(&mut r)?;

        Ok(Self {
            bag_index,
            bag_slot,
            spell_index,
            cast_count,
            item,
            targets,
        })
    }

}

impl crate::Message for CMSG_USE_ITEM {
    const OPCODE: u32 = 0x00ab;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_USE_ITEM {{").unwrap();
        // Members
        writeln!(s, "    bag_index = {};", self.bag_index).unwrap();
        writeln!(s, "    bag_slot = {};", self.bag_slot).unwrap();
        writeln!(s, "    spell_index = {};", self.spell_index).unwrap();
        writeln!(s, "    cast_count = {};", self.cast_count).unwrap();
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        // targets: SpellCastTargets
        writeln!(s, "    targets = {{").unwrap();
        // Members
        writeln!(s, "        target_flags = {};", SpellCastTargetFlags::new(self.targets.target_flags.as_int()).as_test_case_value()).unwrap();
        if let Some(if_statement) = &self.targets.target_flags.get_unit() {
            match if_statement {
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                    unit_target,
                } => {
                    writeln!(s, "        unit_target = {};", unit_target.guid()).unwrap();
                }
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Unit::UnitMinipet {
                    unit_minipet,
                } => {
                    writeln!(s, "        unit_minipet = {};", unit_minipet.guid()).unwrap();
                }
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Unit::UnitEnemy {
                    unit_enemy,
                } => {
                    writeln!(s, "        unit_enemy = {};", unit_enemy.guid()).unwrap();
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_gameobject() {
            match if_statement {
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Gameobject::Gameobject {
                    gameobject,
                } => {
                    writeln!(s, "        gameobject = {};", gameobject.guid()).unwrap();
                }
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Gameobject::Locked {
                    locked,
                } => {
                    writeln!(s, "        locked = {};", locked.guid()).unwrap();
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_item() {
            match if_statement {
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Item::Item {
                    item,
                } => {
                    writeln!(s, "        item = {};", item.guid()).unwrap();
                }
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
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
            writeln!(s, "    {}", if if_statement.source.x.to_string().contains('.') { if_statement.source.x.to_string() } else { format!("{}.0", if_statement.source.x) }).unwrap();
            writeln!(s, "    {}", if if_statement.source.y.to_string().contains('.') { if_statement.source.y.to_string() } else { format!("{}.0", if_statement.source.y) }).unwrap();
            writeln!(s, "    {}", if if_statement.source.z.to_string().contains('.') { if_statement.source.z.to_string() } else { format!("{}.0", if_statement.source.z) }).unwrap();

            writeln!(s, "    }};").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_dest_location() {
            // destination: Vector3d
            writeln!(s, "        destination = {{").unwrap();
            // Members
            writeln!(s, "    {}", if if_statement.destination.x.to_string().contains('.') { if_statement.destination.x.to_string() } else { format!("{}.0", if_statement.destination.x) }).unwrap();
            writeln!(s, "    {}", if if_statement.destination.y.to_string().contains('.') { if_statement.destination.y.to_string() } else { format!("{}.0", if_statement.destination.y) }).unwrap();
            writeln!(s, "    {}", if if_statement.destination.z.to_string().contains('.') { if_statement.destination.z.to_string() } else { format!("{}.0", if_statement.destination.z) }).unwrap();

            writeln!(s, "    }};").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_string() {
            writeln!(s, "        target_string = \"{}\";", if_statement.target_string).unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_corpse_ally() {
            match if_statement {
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_CorpseAlly::CorpseAlly {
                    corpse_ally,
                } => {
                    writeln!(s, "        corpse_ally = {};", corpse_ally.guid()).unwrap();
                }
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_CorpseAlly::CorpseEnemy {
                    corpse_enemy,
                } => {
                    writeln!(s, "        corpse_enemy = {};", corpse_enemy.guid()).unwrap();
                }
            }
        }


        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 171_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_index", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "spell_index", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "cast_count", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
        writeln!(s, "    /* targets: SpellCastTargets start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "target_flags", "        ");
        if let Some(if_statement) = &self.targets.target_flags.get_unit() {
            match if_statement {
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                    unit_target,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&unit_target), "unit_target", "        ");
                }
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Unit::UnitMinipet {
                    unit_minipet,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&unit_minipet), "unit_minipet", "        ");
                }
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Unit::UnitEnemy {
                    unit_enemy,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&unit_enemy), "unit_enemy", "        ");
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_gameobject() {
            match if_statement {
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Gameobject::Gameobject {
                    gameobject,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&gameobject), "gameobject", "        ");
                }
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Gameobject::Locked {
                    locked,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&locked), "locked", "        ");
                }
            }
        }

        if let Some(if_statement) = &self.targets.target_flags.get_item() {
            match if_statement {
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Item::Item {
                    item,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&item), "item", "        ");
                }
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_Item::TradeItem {
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

        if let Some(if_statement) = &self.targets.target_flags.get_corpse_ally() {
            match if_statement {
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_CorpseAlly::CorpseAlly {
                    corpse_ally,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&corpse_ally), "corpse_ally", "        ");
                }
                crate::tbc::SpellCastTargets_SpellCastTargetFlags_CorpseAlly::CorpseEnemy {
                    corpse_enemy,
                } => {
                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&corpse_enemy), "corpse_enemy", "        ");
                }
            }
        }

        writeln!(s, "    /* targets: SpellCastTargets end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // spell_index: u8
        w.write_all(&self.spell_index.to_le_bytes())?;

        // cast_count: u8
        w.write_all(&self.cast_count.to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_USE_ITEM {}

impl CMSG_USE_ITEM {
    pub(crate) fn size(&self) -> usize {
        1 // bag_index: u8
        + 1 // bag_slot: u8
        + 1 // spell_index: u8
        + 1 // cast_count: u8
        + 8 // item: Guid
        + self.targets.size() // targets: SpellCastTargets
    }
}

