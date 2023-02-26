use crate::Guid;
use crate::wrath::MovementInfo;
use crate::wrath::SpellCastTargets;
use crate::wrath::ClientCastFlags;
use crate::wrath::ClientMovementData;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm#L8):
/// ```text
/// cmsg CMSG_CAST_SPELL = 0x012E {
///     u8 cast_count;
///     u32 spell;
///     ClientCastFlags cast_flags;
///     SpellCastTargets targets;
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
/// }
/// ```
pub struct CMSG_CAST_SPELL {
    pub cast_count: u8,
    pub spell: u32,
    pub cast_flags: CMSG_CAST_SPELL_ClientCastFlags,
    pub targets: SpellCastTargets,
}

impl crate::Message for CMSG_CAST_SPELL {
    const OPCODE: u32 = 0x012e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // cast_count: u8
        w.write_all(&self.cast_count.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // cast_flags: ClientCastFlags
        w.write_all(&(self.cast_flags.as_int() as u8).to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(w)?;

        match &self.cast_flags {
            CMSG_CAST_SPELL_ClientCastFlags::None => {}
            CMSG_CAST_SPELL_ClientCastFlags::Extra {
                elevation,
                movement_data,
                speed,
            } => {
                // elevation: f32
                w.write_all(&elevation.to_le_bytes())?;

                // speed: f32
                w.write_all(&speed.to_le_bytes())?;

                // movement_data: ClientMovementData
                w.write_all(&(movement_data.as_int() as u8).to_le_bytes())?;

                match &movement_data {
                    CMSG_CAST_SPELL_ClientMovementData::NotPresent => {}
                    CMSG_CAST_SPELL_ClientMovementData::Present {
                        guid,
                        info,
                        opcode,
                    } => {
                        // opcode: u32
                        w.write_all(&opcode.to_le_bytes())?;

                        // guid: PackedGuid
                        guid.write_packed_guid_into_vec(w)?;

                        // info: MovementInfo
                        info.write_into_vec(w)?;

                    }
                }

            }
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=414).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x012E, size: body_size as u32 });
        }

        // cast_count: u8
        let cast_count = crate::util::read_u8_le(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // cast_flags: ClientCastFlags
        let cast_flags: ClientCastFlags = crate::util::read_u8_le(r)?.try_into()?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        let cast_flags_if = match cast_flags {
            ClientCastFlags::None => CMSG_CAST_SPELL_ClientCastFlags::None,
            ClientCastFlags::Extra => {
                // elevation: f32
                let elevation = crate::util::read_f32_le(r)?;

                // speed: f32
                let speed = crate::util::read_f32_le(r)?;

                // movement_data: ClientMovementData
                let movement_data: ClientMovementData = crate::util::read_u8_le(r)?.try_into()?;

                let movement_data_if = match movement_data {
                    ClientMovementData::NotPresent => CMSG_CAST_SPELL_ClientMovementData::NotPresent,
                    ClientMovementData::Present => {
                        // opcode: u32
                        let opcode = crate::util::read_u32_le(r)?;

                        // guid: PackedGuid
                        let guid = Guid::read_packed(r)?;

                        // info: MovementInfo
                        let info = MovementInfo::read(r)?;

                        CMSG_CAST_SPELL_ClientMovementData::Present {
                            guid,
                            info,
                            opcode,
                        }
                    }
                };

                CMSG_CAST_SPELL_ClientCastFlags::Extra {
                    elevation,
                    movement_data: movement_data_if,
                    speed,
                }
            }
        };

        Ok(Self {
            cast_count,
            spell,
            cast_flags: cast_flags_if,
            targets,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CAST_SPELL {}

impl CMSG_CAST_SPELL {
    pub(crate) fn size(&self) -> usize {
        1 // cast_count: u8
        + 4 // spell: u32
        + self.cast_flags.size() // cast_flags: CMSG_CAST_SPELL_ClientCastFlags
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CMSG_CAST_SPELL_ClientMovementData {
    NotPresent,
    Present {
        guid: Guid,
        info: MovementInfo,
        opcode: u32,
    },
}

impl Default for CMSG_CAST_SPELL_ClientMovementData {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotPresent
    }
}

impl CMSG_CAST_SPELL_ClientMovementData {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotPresent => 0,
            Self::Present { .. } => 1,
        }
    }

}

impl CMSG_CAST_SPELL_ClientMovementData {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NotPresent => {
                1
            }
            Self::Present {
                guid,
                info,
                opcode,
            } => {
                1
                + guid.size() // guid: Guid
                + info.size() // info: MovementInfo
                + 4 // opcode: u32
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CMSG_CAST_SPELL_ClientCastFlags {
    None,
    Extra {
        elevation: f32,
        movement_data: CMSG_CAST_SPELL_ClientMovementData,
        speed: f32,
    },
}

impl Default for CMSG_CAST_SPELL_ClientCastFlags {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl CMSG_CAST_SPELL_ClientCastFlags {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Extra { .. } => 2,
        }
    }

}

impl CMSG_CAST_SPELL_ClientCastFlags {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::None => {
                1
            }
            Self::Extra {
                elevation,
                movement_data,
                speed,
            } => {
                1
                + 4 // elevation: f32
                + movement_data.size() // movement_data: CMSG_CAST_SPELL_ClientMovementData
                + 4 // speed: f32
            }
        }
    }
}

