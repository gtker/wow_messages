use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm#L13):
/// ```text
/// struct ActionButton {
///     u16 action;
///     u8 action_type;
///     u8 misc;
/// }
/// ```
pub struct ActionButton {
    pub action: u16,
    pub action_type: u8,
    pub misc: u8,
}

impl ActionButton {
    pub(crate) fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // action: u16
        w.write_all(&self.action.to_le_bytes())?;

        // action_type: u8
        w.write_all(&self.action_type.to_le_bytes())?;

        // misc: u8
        w.write_all(&self.misc.to_le_bytes())?;

        Ok(())
    }
}

impl ActionButton {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // action: u16
        let action = crate::util::read_u16_le(r)?;

        // action_type: u8
        let action_type = crate::util::read_u8_le(r)?;

        // misc: u8
        let misc = crate::util::read_u8_le(r)?;

        Ok(Self {
            action,
            action_type,
            misc,
        })
    }

}

