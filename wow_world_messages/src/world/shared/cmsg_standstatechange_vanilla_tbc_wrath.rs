use std::convert::{TryFrom, TryInto};
use crate::shared::unit_stand_state_vanilla_tbc_wrath::UnitStandState;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_standstatechange.wowm#L3):
/// ```text
/// cmsg CMSG_STANDSTATECHANGE = 0x0101 {
///     (u32)UnitStandState animation_state;
/// }
/// ```
pub struct CMSG_STANDSTATECHANGE {
    pub animation_state: UnitStandState,
}

impl crate::Message for CMSG_STANDSTATECHANGE {
    const OPCODE: u32 = 0x0101;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // animation_state: UnitStandState
        w.write_all(&(self.animation_state.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0101, size: body_size as u32 });
        }

        // animation_state: UnitStandState
        let animation_state: UnitStandState = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            animation_state,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_STANDSTATECHANGE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_STANDSTATECHANGE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_STANDSTATECHANGE {}

