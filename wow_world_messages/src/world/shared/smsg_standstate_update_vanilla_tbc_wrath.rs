use wow_world_base::shared::unit_stand_state_vanilla_tbc_wrath::UnitStandState;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_standstate_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_standstate_update.wowm#L3):
/// ```text
/// smsg SMSG_STANDSTATE_UPDATE = 0x029D {
///     UnitStandState state;
/// }
/// ```
pub struct SMSG_STANDSTATE_UPDATE {
    pub state: UnitStandState,
}

impl crate::Message for SMSG_STANDSTATE_UPDATE {
    const OPCODE: u32 = 0x029d;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // state: UnitStandState
        w.write_all(&u8::from(self.state.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x029D, size: body_size as u32 });
        }

        // state: UnitStandState
        let state: UnitStandState = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            state,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_STANDSTATE_UPDATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_STANDSTATE_UPDATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_STANDSTATE_UPDATE {}

