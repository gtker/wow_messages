use crate::Guid;
use crate::wrath::EncounterFrame;
use std::io::{Write, Read};

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

impl crate::Message for SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT {
    const OPCODE: u32 = 0x0214;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // frame: EncounterFrame
        w.write_all(&(self.frame.as_int() as u32).to_le_bytes())?;

        match &self.frame {
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::Engage {
                guid,
                parameter1,
            } => {
                // guid: PackedGuid
                guid.write_packed_guid_into_vec(w)?;

                // parameter1: u8
                w.write_all(&parameter1.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::Disengage {
                guid,
                parameter1,
            } => {
                // guid: PackedGuid
                guid.write_packed_guid_into_vec(w)?;

                // parameter1: u8
                w.write_all(&parameter1.to_le_bytes())?;

            }
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::UpdatePriority {
                guid,
                parameter1,
            } => {
                // guid: PackedGuid
                guid.write_packed_guid_into_vec(w)?;

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
            SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::RefreshFrames => {}
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=14).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0214, size: body_size as u32 });
        }

        // frame: EncounterFrame
        let frame: EncounterFrame = crate::util::read_u32_le(r)?.try_into()?;

        let frame_if = match frame {
            EncounterFrame::Engage => {
                // guid: PackedGuid
                let guid = Guid::read_packed(r)?;

                // parameter1: u8
                let parameter1 = crate::util::read_u8_le(r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::Engage {
                    guid,
                    parameter1,
                }
            }
            EncounterFrame::Disengage => {
                // guid: PackedGuid
                let guid = Guid::read_packed(r)?;

                // parameter1: u8
                let parameter1 = crate::util::read_u8_le(r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::Disengage {
                    guid,
                    parameter1,
                }
            }
            EncounterFrame::UpdatePriority => {
                // guid: PackedGuid
                let guid = Guid::read_packed(r)?;

                // parameter1: u8
                let parameter1 = crate::util::read_u8_le(r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::UpdatePriority {
                    guid,
                    parameter1,
                }
            }
            EncounterFrame::AddTimer => {
                // parameter2: u8
                let parameter2 = crate::util::read_u8_le(r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::AddTimer {
                    parameter2,
                }
            }
            EncounterFrame::EnableObjective => {
                // parameter2: u8
                let parameter2 = crate::util::read_u8_le(r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::EnableObjective {
                    parameter2,
                }
            }
            EncounterFrame::UpdateObjective => {
                // parameter3: u8
                let parameter3 = crate::util::read_u8_le(r)?;

                // parameter4: u8
                let parameter4 = crate::util::read_u8_le(r)?;

                SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame::UpdateObjective {
                    parameter3,
                    parameter4,
                }
            }
            EncounterFrame::DisableObjective => {
                // parameter2: u8
                let parameter2 = crate::util::read_u8_le(r)?;

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
    pub(crate) fn size(&self) -> usize {
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

impl SMSG_UPDATE_INSTANCE_ENCOUNTER_UNIT_EncounterFrame {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Engage {
                guid,
                parameter1,
            } => {
                4
                + guid.size() // guid: Guid
                + 1 // parameter1: u8
            }
            Self::Disengage {
                guid,
                parameter1,
            } => {
                4
                + guid.size() // guid: Guid
                + 1 // parameter1: u8
            }
            Self::UpdatePriority {
                guid,
                parameter1,
            } => {
                4
                + guid.size() // guid: Guid
                + 1 // parameter1: u8
            }
            Self::AddTimer {
                parameter2,
            } => {
                4
                + 1 // parameter2: u8
            }
            Self::EnableObjective {
                parameter2,
            } => {
                4
                + 1 // parameter2: u8
            }
            Self::UpdateObjective {
                parameter3,
                parameter4,
            } => {
                4
                + 1 // parameter3: u8
                + 1 // parameter4: u8
            }
            Self::DisableObjective {
                parameter2,
            } => {
                4
                + 1 // parameter2: u8
            }
            Self::RefreshFrames => {
                4
            }
        }
    }
}

