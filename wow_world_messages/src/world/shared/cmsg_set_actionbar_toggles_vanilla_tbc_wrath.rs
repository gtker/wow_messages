use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_set_actionbar_toggles.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_set_actionbar_toggles.wowm#L3):
/// ```text
/// cmsg CMSG_SET_ACTIONBAR_TOGGLES = 0x02BF {
///     u8 action_bar;
/// }
/// ```
pub struct CMSG_SET_ACTIONBAR_TOGGLES {
    /// Emulators set PLAYER_FIELD_BYTES`2` to this unless it's 0.
    ///
    pub action_bar: u8,
}

impl crate::Message for CMSG_SET_ACTIONBAR_TOGGLES {
    const OPCODE: u32 = 0x02bf;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // action_bar: u8
        w.write_all(&self.action_bar.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02BF, size: body_size as u32 });
        }

        // action_bar: u8
        let action_bar = crate::util::read_u8_le(r)?;

        Ok(Self {
            action_bar,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_SET_ACTIONBAR_TOGGLES {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_SET_ACTIONBAR_TOGGLES {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_SET_ACTIONBAR_TOGGLES {}

