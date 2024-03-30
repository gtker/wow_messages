use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::EncounterFrame;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_update_instance_encounter_unit.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_instance_encounter_unit.wowm#L15):
/// ```text
/// smsg SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT = 0x0214 {
///     EncounterFrame frame;
///     if (frame == ENGAGE
///         || frame == DISENGAGE
///         || frame == UPDATE_PRIORITY) {
///         PackedGuid guid;
///         u8 parameter1;
///     }
///     else if (frame == ADD_TIMER
///         || frame == ENABLE_OBJECTIVE
///         || frame == DISABLE_OBJECTIVE) {
///         u8 parameter2;
///     }
///     else if (frame == UPDATE_OBJECTIVE) {
///         u8 parameter3;
///         u8 parameter4;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    Engage {
        guid: Guid,
        parameter1: u8,
    },
    Disengage {
        guid: Guid,
        parameter1: u8,
    },
    UpdatePriority {
        guid: Guid,
        parameter1: u8,
    },
    AddTimer {
        parameter2: u8,
    },
    EnableObjective {
        parameter2: u8,
    },
    UpdateObjective {
        parameter3: u8,
        parameter4: u8,
    },
    DisableObjective {
        parameter2: u8,
    },
    RefreshFrames,
}

impl crate::private::Sealed for SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {}
impl SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=14).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // frame: EncounterFrame
        let frame = crate::util::read_u32_le(&mut r)?.try_into()?;

        let frame_if = match frame {
            EncounterFrame::Engage => {
                // guid: PackedGuid
                let guid = crate::util::read_packed_guid(&mut r)?;

                // parameter1: u8
                let parameter1 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::Engage {
                    guid,
                    parameter1,
                }
            }
            EncounterFrame::Disengage => {
                // guid: PackedGuid
                let guid = crate::util::read_packed_guid(&mut r)?;

                // parameter1: u8
                let parameter1 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::Disengage {
                    guid,
                    parameter1,
                }
            }
            EncounterFrame::UpdatePriority => {
                // guid: PackedGuid
                let guid = crate::util::read_packed_guid(&mut r)?;

                // parameter1: u8
                let parameter1 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::UpdatePriority {
                    guid,
                    parameter1,
                }
            }
            EncounterFrame::AddTimer => {
                // parameter2: u8
                let parameter2 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::AddTimer {
                    parameter2,
                }
            }
            EncounterFrame::EnableObjective => {
                // parameter2: u8
                let parameter2 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::EnableObjective {
                    parameter2,
                }
            }
            EncounterFrame::UpdateObjective => {
                // parameter3: u8
                let parameter3 = crate::util::read_u8_le(&mut r)?;

                // parameter4: u8
                let parameter4 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::UpdateObjective {
                    parameter3,
                    parameter4,
                }
            }
            EncounterFrame::DisableObjective => {
                // parameter2: u8
                let parameter2 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::DisableObjective {
                    parameter2,
                }
            }
            EncounterFrame::RefreshFrames => SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::RefreshFrames,
        };

        Ok(frame_if)
    }

}

impl crate::Message for SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    const OPCODE: u32 = 0x0214;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {{").unwrap();
        // Members
        writeln!(s, "    frame = {};", EncounterFrame::try_from(self.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self {
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::Engage {
                guid,
                parameter1,
            } => {
                writeln!(s, "    guid = {};", guid.guid()).unwrap();
                writeln!(s, "    parameter1 = {};", parameter1).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::Disengage {
                guid,
                parameter1,
            } => {
                writeln!(s, "    guid = {};", guid.guid()).unwrap();
                writeln!(s, "    parameter1 = {};", parameter1).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::UpdatePriority {
                guid,
                parameter1,
            } => {
                writeln!(s, "    guid = {};", guid.guid()).unwrap();
                writeln!(s, "    parameter1 = {};", parameter1).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::AddTimer {
                parameter2,
            } => {
                writeln!(s, "    parameter2 = {};", parameter2).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::EnableObjective {
                parameter2,
            } => {
                writeln!(s, "    parameter2 = {};", parameter2).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::UpdateObjective {
                parameter3,
                parameter4,
            } => {
                writeln!(s, "    parameter3 = {};", parameter3).unwrap();
                writeln!(s, "    parameter4 = {};", parameter4).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::DisableObjective {
                parameter2,
            } => {
                writeln!(s, "    parameter2 = {};", parameter2).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 532_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "frame", "    ");
        match &self {
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::Engage {
                guid,
                parameter1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid), "guid", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "parameter1", "    ");
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::Disengage {
                guid,
                parameter1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid), "guid", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "parameter1", "    ");
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::UpdatePriority {
                guid,
                parameter1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&guid), "guid", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "parameter1", "    ");
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::AddTimer {
                parameter2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "parameter2", "    ");
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::EnableObjective {
                parameter2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "parameter2", "    ");
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::UpdateObjective {
                parameter3,
                parameter4,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "parameter3", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "parameter4", "    ");
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::DisableObjective {
                parameter2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "parameter2", "    ");
            }
            _ => {}
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
        // frame: EncounterFrame
        w.write_all(&(self.as_int().to_le_bytes()))?;

        match &self {
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::Engage {
                guid,
                parameter1,
            } => {
                // guid: PackedGuid
                crate::util::write_packed_guid(&guid, &mut w)?;

                // parameter1: u8
                w.write_all(&parameter1.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::Disengage {
                guid,
                parameter1,
            } => {
                // guid: PackedGuid
                crate::util::write_packed_guid(&guid, &mut w)?;

                // parameter1: u8
                w.write_all(&parameter1.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::UpdatePriority {
                guid,
                parameter1,
            } => {
                // guid: PackedGuid
                crate::util::write_packed_guid(&guid, &mut w)?;

                // parameter1: u8
                w.write_all(&parameter1.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::AddTimer {
                parameter2,
            } => {
                // parameter2: u8
                w.write_all(&parameter2.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::EnableObjective {
                parameter2,
            } => {
                // parameter2: u8
                w.write_all(&parameter2.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::UpdateObjective {
                parameter3,
                parameter4,
            } => {
                // parameter3: u8
                w.write_all(&parameter3.to_le_bytes())?;

                // parameter4: u8
                w.write_all(&parameter4.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT::DisableObjective {
                parameter2,
            } => {
                // parameter2: u8
                w.write_all(&parameter2.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(532, "SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {}

impl SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    pub(crate) const fn size(&self) -> usize {
        (match self {
            Self::Engage {
                guid,
                ..
            } => {
                4
                + crate::util::packed_guid_size(&guid) // guid: PackedGuid
                + 1 // parameter1: u8
            }
            Self::Disengage {
                guid,
                ..
            } => {
                4
                + crate::util::packed_guid_size(&guid) // guid: PackedGuid
                + 1 // parameter1: u8
            }
            Self::UpdatePriority {
                guid,
                ..
            } => {
                4
                + crate::util::packed_guid_size(&guid) // guid: PackedGuid
                + 1 // parameter1: u8
            }
            Self::AddTimer {
                ..
            } => {
                4
                + 1 // parameter2: u8
            }
            Self::EnableObjective {
                ..
            } => {
                4
                + 1 // parameter2: u8
            }
            Self::UpdateObjective {
                ..
            } => {
                4
                + 1 // parameter3: u8
                + 1 // parameter4: u8
            }
            Self::DisableObjective {
                ..
            } => {
                4
                + 1 // parameter2: u8
            }
            _ => 4,
        }) // frame: SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT
    }
}

impl Default for SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    fn default() -> Self {
        // First enumerator without any fields
        Self::RefreshFrames
    }
}

impl SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Engage { .. } => 0,
            Self::Disengage { .. } => 1,
            Self::UpdatePriority { .. } => 2,
            Self::AddTimer { .. } => 3,
            Self::EnableObjective { .. } => 4,
            Self::UpdateObjective { .. } => 5,
            Self::DisableObjective { .. } => 6,
            Self::RefreshFrames => 7,
        }
    }

}

impl std::fmt::Display for SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Engage{ .. } => f.write_str("Engage"),
            Self::Disengage{ .. } => f.write_str("Disengage"),
            Self::UpdatePriority{ .. } => f.write_str("UpdatePriority"),
            Self::AddTimer{ .. } => f.write_str("AddTimer"),
            Self::EnableObjective{ .. } => f.write_str("EnableObjective"),
            Self::UpdateObjective{ .. } => f.write_str("UpdateObjective"),
            Self::DisableObjective{ .. } => f.write_str("DisableObjective"),
            Self::RefreshFrames => f.write_str("RefreshFrames"),
        }
    }
}

