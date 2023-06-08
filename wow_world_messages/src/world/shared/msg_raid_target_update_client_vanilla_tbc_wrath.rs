use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::raid_target_index_vanilla_tbc_wrath::RaidTargetIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/raid_target.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/raid_target.wowm#L35):
/// ```text
/// cmsg MSG_RAID_TARGET_UPDATE_Client = 0x0321 {
///     RaidTargetIndex target_index;
///     if (target_index != REQUEST_ICONS) {
///         Guid target;
///     }
/// }
/// ```
pub struct MSG_RAID_TARGET_UPDATE_Client {
    pub target_index: MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex,
}

impl crate::private::Sealed for MSG_RAID_TARGET_UPDATE_Client {}
impl crate::Message for MSG_RAID_TARGET_UPDATE_Client {
    const OPCODE: u32 = 0x0321;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_RAID_TARGET_UPDATE_Client {{").unwrap();
        // Members
        writeln!(s, "    target_index = {};", crate::vanilla::RaidTargetIndex::try_from(self.target_index.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.target_index {
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown0 {
                target,
            } => {
                writeln!(s, "    target = {};", target.guid()).unwrap();
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown1 {
                target,
            } => {
                writeln!(s, "    target = {};", target.guid()).unwrap();
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown2 {
                target,
            } => {
                writeln!(s, "    target = {};", target.guid()).unwrap();
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown3 {
                target,
            } => {
                writeln!(s, "    target = {};", target.guid()).unwrap();
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown4 {
                target,
            } => {
                writeln!(s, "    target = {};", target.guid()).unwrap();
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown5 {
                target,
            } => {
                writeln!(s, "    target = {};", target.guid()).unwrap();
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown6 {
                target,
            } => {
                writeln!(s, "    target = {};", target.guid()).unwrap();
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown7 {
                target,
            } => {
                writeln!(s, "    target = {};", target.guid()).unwrap();
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown8 {
                target,
            } => {
                writeln!(s, "    target = {};", target.guid()).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 801_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "target_index", "    ");
        match &self.target_index {
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown0 {
                target,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown1 {
                target,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown2 {
                target,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown3 {
                target,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown4 {
                target,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown5 {
                target,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown6 {
                target,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown7 {
                target,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
            }
            crate::vanilla::MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown8 {
                target,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");
            }
            _ => {}
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
        // target_index: RaidTargetIndex
        w.write_all(&(self.target_index.as_int().to_le_bytes()))?;

        match &self.target_index {
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown0 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown1 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown2 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown3 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown4 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown5 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown6 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown7 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown8 {
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0321, size: body_size });
        }

        // target_index: RaidTargetIndex
        let target_index = crate::util::read_u8_le(&mut r)?.try_into()?;

        let target_index_if = match target_index {
            RaidTargetIndex::Unknown0 => {
                // target: Guid
                let target = crate::util::read_guid(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown0 {
                    target,
                }
            }
            RaidTargetIndex::Unknown1 => {
                // target: Guid
                let target = crate::util::read_guid(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown1 {
                    target,
                }
            }
            RaidTargetIndex::Unknown2 => {
                // target: Guid
                let target = crate::util::read_guid(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown2 {
                    target,
                }
            }
            RaidTargetIndex::Unknown3 => {
                // target: Guid
                let target = crate::util::read_guid(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown3 {
                    target,
                }
            }
            RaidTargetIndex::Unknown4 => {
                // target: Guid
                let target = crate::util::read_guid(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown4 {
                    target,
                }
            }
            RaidTargetIndex::Unknown5 => {
                // target: Guid
                let target = crate::util::read_guid(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown5 {
                    target,
                }
            }
            RaidTargetIndex::Unknown6 => {
                // target: Guid
                let target = crate::util::read_guid(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown6 {
                    target,
                }
            }
            RaidTargetIndex::Unknown7 => {
                // target: Guid
                let target = crate::util::read_guid(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown7 {
                    target,
                }
            }
            RaidTargetIndex::Unknown8 => {
                // target: Guid
                let target = crate::util::read_guid(&mut r)?;

                MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::Unknown8 {
                    target,
                }
            }
            RaidTargetIndex::RequestIcons => MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex::RequestIcons,
        };

        Ok(Self {
            target_index: target_index_if,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_RAID_TARGET_UPDATE_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_RAID_TARGET_UPDATE_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_RAID_TARGET_UPDATE_Client {}

impl MSG_RAID_TARGET_UPDATE_Client {
    pub(crate) const fn size(&self) -> usize {
        self.target_index.size() // target_index: MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    Unknown0 {
        target: Guid,
    },
    Unknown1 {
        target: Guid,
    },
    Unknown2 {
        target: Guid,
    },
    Unknown3 {
        target: Guid,
    },
    Unknown4 {
        target: Guid,
    },
    Unknown5 {
        target: Guid,
    },
    Unknown6 {
        target: Guid,
    },
    Unknown7 {
        target: Guid,
    },
    Unknown8 {
        target: Guid,
    },
    RequestIcons,
}

impl Default for MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    fn default() -> Self {
        // First enumerator without any fields
        Self::RequestIcons
    }
}

impl MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Unknown0 { .. } => 0,
            Self::Unknown1 { .. } => 1,
            Self::Unknown2 { .. } => 2,
            Self::Unknown3 { .. } => 3,
            Self::Unknown4 { .. } => 4,
            Self::Unknown5 { .. } => 5,
            Self::Unknown6 { .. } => 6,
            Self::Unknown7 { .. } => 7,
            Self::Unknown8 { .. } => 8,
            Self::RequestIcons => 255,
        }
    }

}

impl std::fmt::Display for MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown0{ .. } => f.write_str("Unknown0"),
            Self::Unknown1{ .. } => f.write_str("Unknown1"),
            Self::Unknown2{ .. } => f.write_str("Unknown2"),
            Self::Unknown3{ .. } => f.write_str("Unknown3"),
            Self::Unknown4{ .. } => f.write_str("Unknown4"),
            Self::Unknown5{ .. } => f.write_str("Unknown5"),
            Self::Unknown6{ .. } => f.write_str("Unknown6"),
            Self::Unknown7{ .. } => f.write_str("Unknown7"),
            Self::Unknown8{ .. } => f.write_str("Unknown8"),
            Self::RequestIcons => f.write_str("RequestIcons"),
        }
    }
}

impl MSG_RAID_TARGET_UPDATE_Client_RaidTargetIndex {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Unknown0 {
                ..
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown1 {
                ..
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown2 {
                ..
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown3 {
                ..
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown4 {
                ..
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown5 {
                ..
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown6 {
                ..
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown7 {
                ..
            } => {
                1
                + 8 // target: Guid
            }
            Self::Unknown8 {
                ..
            } => {
                1
                + 8 // target: Guid
            }
            _ => 1,
        }
    }
}

