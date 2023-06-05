use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::EncounterFrame;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_update_instance_encounter_unit.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_instance_encounter_unit.wowm#L16):
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
pub struct SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    pub frame: SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame,
}

#[cfg(feature = "print-testcase")]
impl SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {{").unwrap();
        // Members
        writeln!(s, "    frame = {};", crate::wrath::EncounterFrame::try_from(self.frame.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.frame {
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::Engage {
                guid,
                parameter1,
            } => {
                writeln!(s, "    guid = {};", guid.guid()).unwrap();
                writeln!(s, "    parameter1 = {};", parameter1).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::Disengage {
                guid,
                parameter1,
            } => {
                writeln!(s, "    guid = {};", guid.guid()).unwrap();
                writeln!(s, "    parameter1 = {};", parameter1).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::UpdatePriority {
                guid,
                parameter1,
            } => {
                writeln!(s, "    guid = {};", guid.guid()).unwrap();
                writeln!(s, "    parameter1 = {};", parameter1).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::AddTimer {
                parameter2,
            } => {
                writeln!(s, "    parameter2 = {};", parameter2).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::EnableObjective {
                parameter2,
            } => {
                writeln!(s, "    parameter2 = {};", parameter2).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::UpdateObjective {
                parameter3,
                parameter4,
            } => {
                writeln!(s, "    parameter3 = {};", parameter3).unwrap();
                writeln!(s, "    parameter4 = {};", parameter4).unwrap();
            }
            crate::wrath::SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::DisableObjective {
                parameter2,
            } => {
                writeln!(s, "    parameter2 = {};", parameter2).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 532_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "frame");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {}
impl crate::Message for SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    const OPCODE: u32 = 0x0214;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // frame: EncounterFrame
        w.write_all(&(self.frame.as_int().to_le_bytes()))?;

        match &self.frame {
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::Engage {
                guid,
                parameter1,
            } => {
                // guid: PackedGuid
                crate::util::write_packed_guid(&guid, &mut w)?;

                // parameter1: u8
                w.write_all(&parameter1.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::Disengage {
                guid,
                parameter1,
            } => {
                // guid: PackedGuid
                crate::util::write_packed_guid(&guid, &mut w)?;

                // parameter1: u8
                w.write_all(&parameter1.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::UpdatePriority {
                guid,
                parameter1,
            } => {
                // guid: PackedGuid
                crate::util::write_packed_guid(&guid, &mut w)?;

                // parameter1: u8
                w.write_all(&parameter1.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::AddTimer {
                parameter2,
            } => {
                // parameter2: u8
                w.write_all(&parameter2.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::EnableObjective {
                parameter2,
            } => {
                // parameter2: u8
                w.write_all(&parameter2.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::UpdateObjective {
                parameter3,
                parameter4,
            } => {
                // parameter3: u8
                w.write_all(&parameter3.to_le_bytes())?;

                // parameter4: u8
                w.write_all(&parameter4.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::DisableObjective {
                parameter2,
            } => {
                // parameter2: u8
                w.write_all(&parameter2.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=14).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0214, size: body_size });
        }

        // frame: EncounterFrame
        let frame = crate::util::read_u32_le(&mut r)?.try_into()?;

        let frame_if = match frame {
            EncounterFrame::Engage => {
                // guid: PackedGuid
                let guid = crate::util::read_packed_guid(&mut r)?;

                // parameter1: u8
                let parameter1 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::Engage {
                    guid,
                    parameter1,
                }
            }
            EncounterFrame::Disengage => {
                // guid: PackedGuid
                let guid = crate::util::read_packed_guid(&mut r)?;

                // parameter1: u8
                let parameter1 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::Disengage {
                    guid,
                    parameter1,
                }
            }
            EncounterFrame::UpdatePriority => {
                // guid: PackedGuid
                let guid = crate::util::read_packed_guid(&mut r)?;

                // parameter1: u8
                let parameter1 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::UpdatePriority {
                    guid,
                    parameter1,
                }
            }
            EncounterFrame::AddTimer => {
                // parameter2: u8
                let parameter2 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::AddTimer {
                    parameter2,
                }
            }
            EncounterFrame::EnableObjective => {
                // parameter2: u8
                let parameter2 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::EnableObjective {
                    parameter2,
                }
            }
            EncounterFrame::UpdateObjective => {
                // parameter3: u8
                let parameter3 = crate::util::read_u8_le(&mut r)?;

                // parameter4: u8
                let parameter4 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::UpdateObjective {
                    parameter3,
                    parameter4,
                }
            }
            EncounterFrame::DisableObjective => {
                // parameter2: u8
                let parameter2 = crate::util::read_u8_le(&mut r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::DisableObjective {
                    parameter2,
                }
            }
            EncounterFrame::RefreshFrames => SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::RefreshFrames,
        };

        Ok(Self {
            frame: frame_if,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {}

impl SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    pub(crate) const fn size(&self) -> usize {
        self.frame.size() // frame: SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame {
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

impl Default for SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame {
    fn default() -> Self {
        // First enumerator without any fields
        Self::RefreshFrames
    }
}

impl SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame {
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

impl std::fmt::Display for SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame {
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

impl SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame {
    pub(crate) const fn size(&self) -> usize {
        match self {
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
        }
    }
}

