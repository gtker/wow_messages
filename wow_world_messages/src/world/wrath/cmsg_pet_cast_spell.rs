use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    ClientCastFlags, ClientMovementData, MovementInfo, SpellCastTargets,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_cast_spell.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_cast_spell.wowm#L9):
/// ```text
/// cmsg CMSG_PET_CAST_SPELL = 0x01F0 {
///     Guid guid;
///     u8 cast_count;
///     u32 id;
///     ClientCastFlags cast_flags;
///     SpellCastTargets targets;
///     if (cast_flags == EXTRA) {
///         f32 elevation;
///         f32 speed;
///         ClientMovementData movement_data;
///         if (movement_data == PRESENT) {
///             u32 opcode;
///             PackedGuid movement;
///             MovementInfo info;
///         }
///     }
/// }
/// ```
pub struct CMSG_PET_CAST_SPELL {
    pub guid: Guid,
    pub cast_count: u8,
    pub id: u32,
    pub cast_flags: CMSG_PET_CAST_SPELL_ClientCastFlags,
    pub targets: SpellCastTargets,
}

impl crate::private::Sealed for CMSG_PET_CAST_SPELL {}
impl crate::Message for CMSG_PET_CAST_SPELL {
    const OPCODE: u32 = 0x01f0;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // cast_count: u8
        w.write_all(&self.cast_count.to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // cast_flags: ClientCastFlags
        w.write_all(&(self.cast_flags.as_int().to_le_bytes()))?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(&mut w)?;

        match &self.cast_flags {
            CMSG_PET_CAST_SPELL_ClientCastFlags::Extra {
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
                    CMSG_PET_CAST_SPELL_ClientMovementData::Present {
                        info,
                        movement,
                        opcode,
                    } => {
                        // opcode: u32
                        w.write_all(&opcode.to_le_bytes())?;

                        // movement: PackedGuid
                        movement.write_packed_guid_into_vec(&mut w)?;

                        // info: MovementInfo
                        info.write_into_vec(&mut w)?;

                    }
                    _ => {}
                }

            }
            _ => {}
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(18..=426).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F0, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // cast_count: u8
        let cast_count = crate::util::read_u8_le(&mut r)?;

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // cast_flags: ClientCastFlags
        let cast_flags: ClientCastFlags = crate::util::read_u8_le(&mut r)?.try_into()?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(&mut r)?;

        let cast_flags_if = match cast_flags {
            ClientCastFlags::None => CMSG_PET_CAST_SPELL_ClientCastFlags::None,
            ClientCastFlags::Extra => {
                // elevation: f32
                let elevation = crate::util::read_f32_le(&mut r)?;

                // speed: f32
                let speed = crate::util::read_f32_le(&mut r)?;

                // movement_data: ClientMovementData
                let movement_data: ClientMovementData = crate::util::read_u8_le(&mut r)?.try_into()?;

                let movement_data_if = match movement_data {
                    ClientMovementData::NotPresent => CMSG_PET_CAST_SPELL_ClientMovementData::NotPresent,
                    ClientMovementData::Present => {
                        // opcode: u32
                        let opcode = crate::util::read_u32_le(&mut r)?;

                        // movement: PackedGuid
                        let movement = Guid::read_packed(&mut r)?;

                        // info: MovementInfo
                        let info = MovementInfo::read(&mut r)?;

                        CMSG_PET_CAST_SPELL_ClientMovementData::Present {
                            info,
                            movement,
                            opcode,
                        }
                    }
                };

                CMSG_PET_CAST_SPELL_ClientCastFlags::Extra {
                    elevation,
                    movement_data: movement_data_if,
                    speed,
                }
            }
        };

        Ok(Self {
            guid,
            cast_count,
            id,
            cast_flags: cast_flags_if,
            targets,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PET_CAST_SPELL {}

impl CMSG_PET_CAST_SPELL {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 1 // cast_count: u8
        + 4 // id: u32
        + self.cast_flags.size() // cast_flags: CMSG_PET_CAST_SPELL_ClientCastFlags
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CMSG_PET_CAST_SPELL_ClientMovementData {
    NotPresent,
    Present {
        info: MovementInfo,
        movement: Guid,
        opcode: u32,
    },
}

impl Default for CMSG_PET_CAST_SPELL_ClientMovementData {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotPresent
    }
}

impl CMSG_PET_CAST_SPELL_ClientMovementData {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotPresent => 0,
            Self::Present { .. } => 1,
        }
    }

}

impl CMSG_PET_CAST_SPELL_ClientMovementData {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Present {
                info,
                movement,
                opcode,
            } => {
                1
                + info.size() // info: MovementInfo
                + movement.size() // movement: PackedGuid
                + 4 // opcode: u32
            }
            _ => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CMSG_PET_CAST_SPELL_ClientCastFlags {
    None,
    Extra {
        elevation: f32,
        movement_data: CMSG_PET_CAST_SPELL_ClientMovementData,
        speed: f32,
    },
}

impl Default for CMSG_PET_CAST_SPELL_ClientCastFlags {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl CMSG_PET_CAST_SPELL_ClientCastFlags {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Extra { .. } => 2,
        }
    }

}

impl CMSG_PET_CAST_SPELL_ClientCastFlags {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Extra {
                elevation,
                movement_data,
                speed,
            } => {
                1
                + 4 // elevation: f32
                + movement_data.size() // movement_data: CMSG_PET_CAST_SPELL_ClientMovementData
                + 4 // speed: f32
            }
            _ => 1,
        }
    }
}

