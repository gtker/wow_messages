use crate::tbc::Gold;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm#L29):
/// ```text
/// struct GossipItem {
///     u32 id;
///     u8 item_icon;
///     Bool coded;
///     Gold money_required;
///     CString message;
///     CString accept_text;
/// }
/// ```
pub struct GossipItem {
    /// vmangos: sets to loop index
    ///
    pub id: u32,
    pub item_icon: u8,
    /// vmangos: makes pop up box password
    ///
    pub coded: bool,
    /// mangosone: 2.0.3
    ///
    pub money_required: Gold,
    pub message: String,
    /// mangosone: related to money pop up box, 2.0.3, max 0x800
    ///
    pub accept_text: String,
}

impl GossipItem {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // item_icon: u8
        w.write_all(&self.item_icon.to_le_bytes())?;

        // coded: Bool
        w.write_all(u8::from(self.coded).to_le_bytes().as_slice())?;

        // money_required: Gold
        w.write_all(u32::from(self.money_required.as_int()).to_le_bytes().as_slice())?;

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // accept_text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.accept_text.as_bytes().iter().rev().next(), Some(&0_u8), "String `accept_text` must not be null-terminated.");
        w.write_all(self.accept_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl GossipItem {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // item_icon: u8
        let item_icon = crate::util::read_u8_le(r)?;

        // coded: Bool
        let coded = crate::util::read_u8_le(r)? != 0;

        // money_required: Gold
        let money_required = Gold::new(crate::util::read_u32_le(r)?);

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(message)?
        };

        // accept_text: CString
        let accept_text = {
            let accept_text = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(accept_text)?
        };

        Ok(Self {
            id,
            item_icon,
            coded,
            money_required,
            message,
            accept_text,
        })
    }

}

impl GossipItem {
    pub(crate) fn size(&self) -> usize {
        4 // id: u32
        + 1 // item_icon: u8
        + 1 // coded: Bool
        + 8 // money_required: Gold
        + self.message.len() + 1 // message: CString
        + self.accept_text.len() + 1 // accept_text: CString
    }
}

