use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    SpellCastTargetFlags, SpellCastTargets, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_cast_spell.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_cast_spell.wowm#L1):
/// ```text
/// cmsg CMSG_PET_CAST_SPELL = 0x01F0 {
///     Guid guid;
///     u32 id;
///     SpellCastTargets targets;
/// }
/// ```
pub struct CMSG_PET_CAST_SPELL {
    pub guid: Guid,
    pub id: u32,
    pub targets: SpellCastTargets,
}

impl crate::private::Sealed for CMSG_PET_CAST_SPELL {}
impl CMSG_PET_CAST_SPELL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(14..=330).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x01F0, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(&mut r)?;

        Ok(Self {
            guid,
            id,
            targets,
        })
    }

}

impl crate::Message for CMSG_PET_CAST_SPELL {
    const OPCODE: u32 = 0x01f0;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PET_CAST_SPELL {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    id = {};", self.id).unwrap();
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

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 496_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
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


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PET_CAST_SPELL {}

impl CMSG_PET_CAST_SPELL {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // id: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}

