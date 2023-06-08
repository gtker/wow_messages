use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    ClientCastFlags, ClientMovementData, MovementInfo, SpellCastTargets,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_use_item.wowm:37`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_use_item.wowm#L37):
/// ```text
/// cmsg CMSG_USE_ITEM = 0x00AB {
///     u8 bag_index;
///     u8 bag_slot;
///     u8 spell_index;
///     u8 cast_count;
///     u32 spell;
///     Guid item;
///     u32 glyph_index;
///     ClientCastFlags cast_flags;
///     if (cast_flags == EXTRA) {
///         f32 elevation;
///         f32 speed;
///         ClientMovementData movement_data;
///         if (movement_data == PRESENT) {
///             u32 opcode;
///             PackedGuid guid;
///             MovementInfo info;
///         }
///     }
///     SpellCastTargets targets;
/// }
/// ```
pub struct CMSG_USE_ITEM {
    pub bag_index: u8,
    pub bag_slot: u8,
    pub spell_index: u8,
    /// mangosone: next cast if exists (single or not)
    pub cast_count: u8,
    pub spell: u32,
    pub item: Guid,
    pub glyph_index: u32,
    pub cast_flags: CMSG_USE_ITEM_ClientCastFlags,
    pub targets: SpellCastTargets,
}

impl crate::private::Sealed for CMSG_USE_ITEM {}
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
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    glyph_index = {};", self.glyph_index).unwrap();
        writeln!(s, "    cast_flags = {};", crate::wrath::ClientCastFlags::try_from(self.cast_flags.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.cast_flags {
            crate::wrath::CMSG_USE_ITEM_ClientCastFlags::Extra {
                elevation,
                movement_data,
                speed,
            } => {
                writeln!(s, "    {}", if elevation.to_string().contains(".") { elevation.to_string() } else { format!("{}.0", elevation) }).unwrap();
                writeln!(s, "    {}", if speed.to_string().contains(".") { speed.to_string() } else { format!("{}.0", speed) }).unwrap();
                writeln!(s, "    movement_data = {};", crate::wrath::ClientMovementData::try_from(movement_data.as_int()).unwrap().as_test_case_value()).unwrap();
                match &movement_data {
                    crate::wrath::CMSG_USE_ITEM_ClientMovementData::Present {
                        guid,
                        info,
                        opcode,
                    } => {
                        writeln!(s, "    opcode = {};", opcode).unwrap();
                        writeln!(s, "    guid = {};", guid.guid()).unwrap();
                        // info: MovementInfo
                        writeln!(s, "    info = {{").unwrap();
                        // Members
                        writeln!(s, "        flags = {};", crate::wrath::MovementFlags::new(info.flags.as_int()).as_test_case_value()).unwrap();
                        writeln!(s, "        timestamp = {};", info.timestamp).unwrap();
                        // position: Vector3d
                        writeln!(s, "        position = {{").unwrap();
                        // Members
                        writeln!(s, "    {}", if info.position.x.to_string().contains(".") { info.position.x.to_string() } else { format!("{}.0", info.position.x) }).unwrap();
                        writeln!(s, "    {}", if info.position.y.to_string().contains(".") { info.position.y.to_string() } else { format!("{}.0", info.position.y) }).unwrap();
                        writeln!(s, "    {}", if info.position.z.to_string().contains(".") { info.position.z.to_string() } else { format!("{}.0", info.position.z) }).unwrap();

                        writeln!(s, "    }};").unwrap();
                        writeln!(s, "    {}", if info.orientation.to_string().contains(".") { info.orientation.to_string() } else { format!("{}.0", info.orientation) }).unwrap();
                        if let Some(if_statement) = &info.flags.get_on_transport_and_interpolated_movement() {
                            match if_statement {
                                crate::wrath::MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                                    transport_info,
                                    transport_time,
                                } => {
                                    // transport_info: TransportInfo
                                    writeln!(s, "        transport_info = {{").unwrap();
                                    // Members
                                    writeln!(s, "            guid = {};", transport_info.guid.guid()).unwrap();
                                    // position: Vector3d
                                    writeln!(s, "            position = {{").unwrap();
                                    // Members
                                    writeln!(s, "    {}", if transport_info.position.x.to_string().contains(".") { transport_info.position.x.to_string() } else { format!("{}.0", transport_info.position.x) }).unwrap();
                                    writeln!(s, "    {}", if transport_info.position.y.to_string().contains(".") { transport_info.position.y.to_string() } else { format!("{}.0", transport_info.position.y) }).unwrap();
                                    writeln!(s, "    {}", if transport_info.position.z.to_string().contains(".") { transport_info.position.z.to_string() } else { format!("{}.0", transport_info.position.z) }).unwrap();

                                    writeln!(s, "    }};").unwrap();
                                    writeln!(s, "    {}", if transport_info.orientation.to_string().contains(".") { transport_info.orientation.to_string() } else { format!("{}.0", transport_info.orientation) }).unwrap();
                                    writeln!(s, "            timestamp = {};", transport_info.timestamp).unwrap();
                                    writeln!(s, "            seat = {};", transport_info.seat).unwrap();

                                    writeln!(s, "    }};").unwrap();
                                    writeln!(s, "        transport_time = {};", transport_time).unwrap();
                                }
                                crate::wrath::MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                                    transport,
                                } => {
                                    // transport: TransportInfo
                                    writeln!(s, "        transport = {{").unwrap();
                                    // Members
                                    writeln!(s, "            guid = {};", transport.guid.guid()).unwrap();
                                    // position: Vector3d
                                    writeln!(s, "            position = {{").unwrap();
                                    // Members
                                    writeln!(s, "    {}", if transport.position.x.to_string().contains(".") { transport.position.x.to_string() } else { format!("{}.0", transport.position.x) }).unwrap();
                                    writeln!(s, "    {}", if transport.position.y.to_string().contains(".") { transport.position.y.to_string() } else { format!("{}.0", transport.position.y) }).unwrap();
                                    writeln!(s, "    {}", if transport.position.z.to_string().contains(".") { transport.position.z.to_string() } else { format!("{}.0", transport.position.z) }).unwrap();

                                    writeln!(s, "    }};").unwrap();
                                    writeln!(s, "    {}", if transport.orientation.to_string().contains(".") { transport.orientation.to_string() } else { format!("{}.0", transport.orientation) }).unwrap();
                                    writeln!(s, "            timestamp = {};", transport.timestamp).unwrap();
                                    writeln!(s, "            seat = {};", transport.seat).unwrap();

                                    writeln!(s, "    }};").unwrap();
                                }
                            }
                        }

                        if let Some(if_statement) = &info.flags.get_swimming() {
                            match if_statement {
                                crate::wrath::MovementInfo_MovementFlags_Swimming::Swimming {
                                    pitch1,
                                } => {
                                    writeln!(s, "    {}", if pitch1.to_string().contains(".") { pitch1.to_string() } else { format!("{}.0", pitch1) }).unwrap();
                                }
                                crate::wrath::MovementInfo_MovementFlags_Swimming::Flying {
                                    pitch2,
                                } => {
                                    writeln!(s, "    {}", if pitch2.to_string().contains(".") { pitch2.to_string() } else { format!("{}.0", pitch2) }).unwrap();
                                }
                                crate::wrath::MovementInfo_MovementFlags_Swimming::AlwaysAllowPitching {
                                    pitch3,
                                } => {
                                    writeln!(s, "    {}", if pitch3.to_string().contains(".") { pitch3.to_string() } else { format!("{}.0", pitch3) }).unwrap();
                                }
                            }
                        }

                        writeln!(s, "    {}", if info.fall_time.to_string().contains(".") { info.fall_time.to_string() } else { format!("{}.0", info.fall_time) }).unwrap();
                        if let Some(if_statement) = &info.flags.get_falling() {
                            writeln!(s, "    {}", if if_statement.z_speed.to_string().contains(".") { if_statement.z_speed.to_string() } else { format!("{}.0", if_statement.z_speed) }).unwrap();
                            writeln!(s, "    {}", if if_statement.cos_angle.to_string().contains(".") { if_statement.cos_angle.to_string() } else { format!("{}.0", if_statement.cos_angle) }).unwrap();
                            writeln!(s, "    {}", if if_statement.sin_angle.to_string().contains(".") { if_statement.sin_angle.to_string() } else { format!("{}.0", if_statement.sin_angle) }).unwrap();
                            writeln!(s, "    {}", if if_statement.xy_speed.to_string().contains(".") { if_statement.xy_speed.to_string() } else { format!("{}.0", if_statement.xy_speed) }).unwrap();
                        }

                        if let Some(if_statement) = &info.flags.get_spline_elevation() {
                            writeln!(s, "    {}", if if_statement.spline_elevation.to_string().contains(".") { if_statement.spline_elevation.to_string() } else { format!("{}.0", if_statement.spline_elevation) }).unwrap();
                        }


                        writeln!(s, "    }};").unwrap();
                    }
                    _ => {}
                }

            }
            _ => {}
        }

        // targets: SpellCastTargets
        writeln!(s, "    targets = {{").unwrap();
        // Members
        writeln!(s, "        target_flags = {};", crate::wrath::SpellCastTargetFlags::new(self.targets.target_flags.as_int()).as_test_case_value()).unwrap();
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
            writeln!(s, "    {}", if if_statement.source.x.to_string().contains(".") { if_statement.source.x.to_string() } else { format!("{}.0", if_statement.source.x) }).unwrap();
            writeln!(s, "    {}", if if_statement.source.y.to_string().contains(".") { if_statement.source.y.to_string() } else { format!("{}.0", if_statement.source.y) }).unwrap();
            writeln!(s, "    {}", if if_statement.source.z.to_string().contains(".") { if_statement.source.z.to_string() } else { format!("{}.0", if_statement.source.z) }).unwrap();

            writeln!(s, "    }};").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_dest_location() {
            // destination: Vector3d
            writeln!(s, "        destination = {{").unwrap();
            // Members
            writeln!(s, "    {}", if if_statement.destination.x.to_string().contains(".") { if_statement.destination.x.to_string() } else { format!("{}.0", if_statement.destination.x) }).unwrap();
            writeln!(s, "    {}", if if_statement.destination.y.to_string().contains(".") { if_statement.destination.y.to_string() } else { format!("{}.0", if_statement.destination.y) }).unwrap();
            writeln!(s, "    {}", if if_statement.destination.z.to_string().contains(".") { if_statement.destination.z.to_string() } else { format!("{}.0", if_statement.destination.z) }).unwrap();

            writeln!(s, "    }};").unwrap();
        }

        if let Some(if_statement) = &self.targets.target_flags.get_string() {
            writeln!(s, "        target_string = \"{}\";", if_statement.target_string).unwrap();
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
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "glyph_index", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "cast_flags", "    ");
        match &self.cast_flags {
            crate::wrath::CMSG_USE_ITEM_ClientCastFlags::Extra {
                elevation,
                movement_data,
                speed,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "elevation", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "speed", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "movement_data", "    ");
                match &movement_data {
                    crate::wrath::CMSG_USE_ITEM_ClientMovementData::Present {
                        guid,
                        info,
                        opcode,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "opcode", "    ");
                        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid), "guid", "    ");
                        writeln!(s, "    /* info: MovementInfo start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 6, "flags", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "        ");
                        writeln!(s, "    /* position: Vector3d start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "            ");
                        writeln!(s, "    /* position: Vector3d end */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "        ");
                        if let Some(if_statement) = &info.flags.get_on_transport_and_interpolated_movement() {
                            match if_statement {
                                crate::wrath::MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransportAndInterpolatedMovement {
                                    transport_info,
                                    transport_time,
                                } => {
                                    writeln!(s, "    /* transport_info: TransportInfo start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport_info.guid), "guid", "            ");
                                    writeln!(s, "    /* position: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 1, "seat", "            ");
                                    writeln!(s, "    /* transport_info: TransportInfo end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "transport_time", "        ");
                                }
                                crate::wrath::MovementInfo_MovementFlags_OnTransportAndInterpolatedMovement::OnTransport {
                                    transport,
                                } => {
                                    writeln!(s, "    /* transport: TransportInfo start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&transport.guid), "guid", "            ");
                                    writeln!(s, "    /* position: Vector3d start */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "                ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "                ");
                                    writeln!(s, "    /* position: Vector3d end */").unwrap();
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "orientation", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "timestamp", "            ");
                                    crate::util::write_bytes(&mut s, &mut bytes, 1, "seat", "            ");
                                    writeln!(s, "    /* transport: TransportInfo end */").unwrap();
                                }
                            }
                        }

                        if let Some(if_statement) = &info.flags.get_swimming() {
                            match if_statement {
                                crate::wrath::MovementInfo_MovementFlags_Swimming::Swimming {
                                    pitch1,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch1", "        ");
                                }
                                crate::wrath::MovementInfo_MovementFlags_Swimming::Flying {
                                    pitch2,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch2", "        ");
                                }
                                crate::wrath::MovementInfo_MovementFlags_Swimming::AlwaysAllowPitching {
                                    pitch3,
                                } => {
                                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pitch3", "        ");
                                }
                            }
                        }

                        crate::util::write_bytes(&mut s, &mut bytes, 4, "fall_time", "        ");
                        if let Some(if_statement) = &info.flags.get_falling() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "z_speed", "        ");
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "cos_angle", "        ");
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "sin_angle", "        ");
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "xy_speed", "        ");
                        }

                        if let Some(if_statement) = &info.flags.get_spline_elevation() {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "spline_elevation", "        ");
                        }

                        writeln!(s, "    /* info: MovementInfo end */").unwrap();
                    }
                    _ => {}
                }

            }
            _ => {}
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


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
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

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // glyph_index: u32
        w.write_all(&self.glyph_index.to_le_bytes())?;

        // cast_flags: ClientCastFlags
        w.write_all(&(self.cast_flags.as_int().to_le_bytes()))?;

        match &self.cast_flags {
            CMSG_USE_ITEM_ClientCastFlags::Extra {
                elevation,
                movement_data,
                speed,
            } => {
                // elevation: f32
                w.write_all(&elevation.to_le_bytes())?;

                // speed: f32
                w.write_all(&speed.to_le_bytes())?;

                // movement_data: ClientMovementData
                w.write_all(&(movement_data.as_int().to_le_bytes()))?;

                match &movement_data {
                    CMSG_USE_ITEM_ClientMovementData::Present {
                        guid,
                        info,
                        opcode,
                    } => {
                        // opcode: u32
                        w.write_all(&opcode.to_le_bytes())?;

                        // guid: PackedGuid
                        crate::util::write_packed_guid(&guid, &mut w)?;

                        // info: MovementInfo
                        info.write_into_vec(&mut w)?;

                    }
                    _ => {}
                }

            }
            _ => {}
        }

        // targets: SpellCastTargets
        self.targets.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(25..=433).contains(&body_size) {
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

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // glyph_index: u32
        let glyph_index = crate::util::read_u32_le(&mut r)?;

        // cast_flags: ClientCastFlags
        let cast_flags = crate::util::read_u8_le(&mut r)?.try_into()?;

        let cast_flags_if = match cast_flags {
            ClientCastFlags::None => CMSG_USE_ITEM_ClientCastFlags::None,
            ClientCastFlags::Extra => {
                // elevation: f32
                let elevation = crate::util::read_f32_le(&mut r)?;

                // speed: f32
                let speed = crate::util::read_f32_le(&mut r)?;

                // movement_data: ClientMovementData
                let movement_data = crate::util::read_u8_le(&mut r)?.try_into()?;

                let movement_data_if = match movement_data {
                    ClientMovementData::NotPresent => CMSG_USE_ITEM_ClientMovementData::NotPresent,
                    ClientMovementData::Present => {
                        // opcode: u32
                        let opcode = crate::util::read_u32_le(&mut r)?;

                        // guid: PackedGuid
                        let guid = crate::util::read_packed_guid(&mut r)?;

                        // info: MovementInfo
                        let info = MovementInfo::read(&mut r)?;

                        CMSG_USE_ITEM_ClientMovementData::Present {
                            guid,
                            info,
                            opcode,
                        }
                    }
                };

                CMSG_USE_ITEM_ClientCastFlags::Extra {
                    elevation,
                    movement_data: movement_data_if,
                    speed,
                }
            }
        };

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(&mut r)?;

        Ok(Self {
            bag_index,
            bag_slot,
            spell_index,
            cast_count,
            spell,
            item,
            glyph_index,
            cast_flags: cast_flags_if,
            targets,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_USE_ITEM {}

impl CMSG_USE_ITEM {
    pub(crate) fn size(&self) -> usize {
        1 // bag_index: u8
        + 1 // bag_slot: u8
        + 1 // spell_index: u8
        + 1 // cast_count: u8
        + 4 // spell: u32
        + 8 // item: Guid
        + 4 // glyph_index: u32
        + self.cast_flags.size() // cast_flags: CMSG_USE_ITEM_ClientCastFlags
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CMSG_USE_ITEM_ClientMovementData {
    NotPresent,
    Present {
        guid: Guid,
        info: MovementInfo,
        opcode: u32,
    },
}

impl Default for CMSG_USE_ITEM_ClientMovementData {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotPresent
    }
}

impl CMSG_USE_ITEM_ClientMovementData {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotPresent => 0,
            Self::Present { .. } => 1,
        }
    }

}

impl std::fmt::Display for CMSG_USE_ITEM_ClientMovementData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotPresent => f.write_str("NotPresent"),
            Self::Present{ .. } => f.write_str("Present"),
        }
    }
}

impl CMSG_USE_ITEM_ClientMovementData {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Present {
                guid,
                info,
                ..
            } => {
                1
                + crate::util::packed_guid_size(&guid) // guid: PackedGuid
                + info.size() // info: MovementInfo
                + 4 // opcode: u32
            }
            _ => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CMSG_USE_ITEM_ClientCastFlags {
    None,
    Extra {
        elevation: f32,
        movement_data: CMSG_USE_ITEM_ClientMovementData,
        speed: f32,
    },
}

impl Default for CMSG_USE_ITEM_ClientCastFlags {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl CMSG_USE_ITEM_ClientCastFlags {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Extra { .. } => 2,
        }
    }

}

impl std::fmt::Display for CMSG_USE_ITEM_ClientCastFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Extra{ .. } => f.write_str("Extra"),
        }
    }
}

impl CMSG_USE_ITEM_ClientCastFlags {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Extra {
                movement_data,
                ..
            } => {
                1
                + 4 // elevation: f32
                + movement_data.size() // movement_data: CMSG_USE_ITEM_ClientMovementData
                + 4 // speed: f32
            }
            _ => 1,
        }
    }
}

