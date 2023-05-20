use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_set_action_button.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_set_action_button.wowm#L1):
/// ```text
/// cmsg CMSG_SET_ACTION_BUTTON = 0x0128 {
///     u8 button;
///     u16 action;
///     u8 misc;
///     u8 action_type;
/// }
/// ```
pub struct CMSG_SET_ACTION_BUTTON {
    pub button: u8,
    pub action: u16,
    pub misc: u8,
    pub action_type: u8,
}

impl crate::private::Sealed for CMSG_SET_ACTION_BUTTON {}
impl crate::Message for CMSG_SET_ACTION_BUTTON {
    const OPCODE: u32 = 0x0128;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // button: u8
        w.write_all(&self.button.to_le_bytes())?;

        // action: u16
        w.write_all(&self.action.to_le_bytes())?;

        // misc: u8
        w.write_all(&self.misc.to_le_bytes())?;

        // action_type: u8
        w.write_all(&self.action_type.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0128, size: body_size });
        }

        // button: u8
        let button = crate::util::read_u8_le(&mut r)?;

        // action: u16
        let action = crate::util::read_u16_le(&mut r)?;

        // misc: u8
        let misc = crate::util::read_u8_le(&mut r)?;

        // action_type: u8
        let action_type = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            button,
            action,
            misc,
            action_type,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SET_ACTION_BUTTON {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_ACTION_BUTTON {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_ACTION_BUTTON {}

