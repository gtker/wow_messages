use std::io::{Read, Write};

use crate::Guid;
use crate::shared::raid_target_update_vanilla_tbc_wrath::RaidTargetUpdate;
use wow_world_base::shared::raid_target_index_vanilla_tbc_wrath::RaidTargetIndex;
use wow_world_base::shared::raid_target_update_type_vanilla_tbc_wrath::RaidTargetUpdateType;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/raid_target.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/raid_target.wowm#L26):
/// ```text
/// smsg MSG_RAID_TARGET_UPDATE_Server = 0x0321 {
///     RaidTargetUpdateType update_type;
///     if (update_type == FULL) {
///         RaidTargetUpdate[8] raid_targets;
///     }
///     else if (update_type == PARTIAL) {
///         RaidTargetUpdate raid_target;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MSG_RAID_TARGET_UPDATE_Server {
    Partial {
        raid_target: RaidTargetUpdate,
    },
    Full {
        raid_targets: [RaidTargetUpdate; 8],
    },
}

impl crate::private::Sealed for MSG_RAID_TARGET_UPDATE_Server {}
impl MSG_RAID_TARGET_UPDATE_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=73).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // update_type: RaidTargetUpdateType
        let update_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        let update_type_if = match update_type {
            RaidTargetUpdateType::Partial => {
                // raid_target: RaidTargetUpdate
                let raid_target = RaidTargetUpdate::read(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Server::Partial {
                    raid_target,
                }
            }
            RaidTargetUpdateType::Full => {
                // raid_targets: RaidTargetUpdate[8]
                let raid_targets = {
                    let mut raid_targets = [RaidTargetUpdate::default(); 8];
                    for i in raid_targets.iter_mut() {
                        *i = RaidTargetUpdate::read(&mut r)?;
                    }
                    raid_targets
                };

                MSG_RAID_TARGET_UPDATE_Server::Full {
                    raid_targets,
                }
            }
        };

        Ok(update_type_if)
    }

}

impl crate::Message for MSG_RAID_TARGET_UPDATE_Server {
    const OPCODE: u32 = 0x0321;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_RAID_TARGET_UPDATE_Server"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_RAID_TARGET_UPDATE_Server {{").unwrap();
        // Members
        writeln!(s, "    update_type = {};", RaidTargetUpdateType::try_from(self.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self {
            crate::shared::msg_raid_target_update_server_vanilla_tbc_wrath::MSG_RAID_TARGET_UPDATE_Server::Partial {
                raid_target,
            } => {
                // raid_target: RaidTargetUpdate
                writeln!(s, "    raid_target = {{").unwrap();
                // Members
                writeln!(s, "        index = {};", raid_target.index.as_test_case_value()).unwrap();
                writeln!(s, "        guid = {};", raid_target.guid.guid()).unwrap();

                writeln!(s, "    }};").unwrap();
            }
            crate::shared::msg_raid_target_update_server_vanilla_tbc_wrath::MSG_RAID_TARGET_UPDATE_Server::Full {
                raid_targets,
            } => {
                writeln!(s, "    raid_targets = [").unwrap();
                for v in raid_targets.as_slice() {
                    writeln!(s, "        {{").unwrap();
                    // Members
                    writeln!(s, "            index = {};", v.index.as_test_case_value()).unwrap();
                    writeln!(s, "            guid = {};", v.guid.guid()).unwrap();

                    writeln!(s, "        }},").unwrap();
                }
                writeln!(s, "    ];").unwrap();
            }
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 801_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "update_type", "    ");
        match &self {
            crate::shared::msg_raid_target_update_server_vanilla_tbc_wrath::MSG_RAID_TARGET_UPDATE_Server::Partial {
                raid_target,
            } => {
                writeln!(s, "    /* raid_target: RaidTargetUpdate start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "index", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "        ");
                writeln!(s, "    /* raid_target: RaidTargetUpdate end */").unwrap();
            }
            crate::shared::msg_raid_target_update_server_vanilla_tbc_wrath::MSG_RAID_TARGET_UPDATE_Server::Full {
                raid_targets,
            } => {
                writeln!(s, "    /* raid_targets: RaidTargetUpdate[8] start */").unwrap();
                for (i, v) in raid_targets.iter().enumerate() {
                    writeln!(s, "    /* raid_targets: RaidTargetUpdate[8] {i} start */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "index", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "        ");
                    writeln!(s, "    /* raid_targets: RaidTargetUpdate[8] {i} end */").unwrap();
                }
                writeln!(s, "    /* raid_targets: RaidTargetUpdate[8] end */").unwrap();
            }
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // update_type: RaidTargetUpdateType
        w.write_all(&(self.as_int().to_le_bytes()))?;

        match &self {
            MSG_RAID_TARGET_UPDATE_Server::Partial {
                raid_target,
            } => {
                // raid_target: RaidTargetUpdate
                raid_target.write_into_vec(&mut w)?;

            }
            MSG_RAID_TARGET_UPDATE_Server::Full {
                raid_targets,
            } => {
                // raid_targets: RaidTargetUpdate[8]
                for i in raid_targets.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(801, "MSG_RAID_TARGET_UPDATE_Server", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_RAID_TARGET_UPDATE_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_RAID_TARGET_UPDATE_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_RAID_TARGET_UPDATE_Server {}

impl MSG_RAID_TARGET_UPDATE_Server {
    pub(crate) const fn size(&self) -> usize {
        (match self {
            Self::Partial {
                ..
            } => {
                1
                + 9 // raid_target: RaidTargetUpdate
            }
            Self::Full {
                ..
            } => {
                1
                + 8 * 9 // raid_targets: RaidTargetUpdate[8]
            }
        }) // update_type: MSG_RAID_TARGET_UPDATE_Server
    }
}

impl Default for MSG_RAID_TARGET_UPDATE_Server {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Partial {
            raid_target: Default::default(),
        }
    }
}

impl MSG_RAID_TARGET_UPDATE_Server {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Partial { .. } => 0,
            Self::Full { .. } => 1,
        }
    }

}

impl std::fmt::Display for MSG_RAID_TARGET_UPDATE_Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Partial{ .. } => f.write_str("Partial"),
            Self::Full{ .. } => f.write_str("Full"),
        }
    }
}

