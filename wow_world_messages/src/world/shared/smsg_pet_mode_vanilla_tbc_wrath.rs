use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::pet_command_state_vanilla_tbc_wrath::PetCommandState;
use wow_world_base::shared::pet_enabled_vanilla_tbc_wrath::PetEnabled;
use wow_world_base::shared::pet_react_state_vanilla_tbc_wrath::PetReactState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_mode.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_mode.wowm#L8):
/// ```text
/// smsg SMSG_PET_MODE = 0x017A {
///     Guid guid;
///     PetReactState react_state;
///     PetCommandState command_state;
///     u8 unknown1;
///     PetEnabled pet_enabled;
/// }
/// ```
pub struct SMSG_PET_MODE {
    pub guid: Guid,
    pub react_state: PetReactState,
    pub command_state: PetCommandState,
    /// vmangos sets to 0.
    ///
    pub unknown1: u8,
    pub pet_enabled: PetEnabled,
}

impl crate::private::Sealed for SMSG_PET_MODE {}
impl crate::Message for SMSG_PET_MODE {
    const OPCODE: u32 = 0x017a;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // react_state: PetReactState
        w.write_all(&(self.react_state.as_int().to_le_bytes()))?;

        // command_state: PetCommandState
        w.write_all(&(self.command_state.as_int().to_le_bytes()))?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // pet_enabled: PetEnabled
        w.write_all(&(self.pet_enabled.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x017A, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // react_state: PetReactState
        let react_state: PetReactState = crate::util::read_u8_le(&mut r)?.try_into()?;

        // command_state: PetCommandState
        let command_state: PetCommandState = crate::util::read_u8_le(&mut r)?.try_into()?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // pet_enabled: PetEnabled
        let pet_enabled: PetEnabled = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            react_state,
            command_state,
            unknown1,
            pet_enabled,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PET_MODE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PET_MODE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_MODE {}

