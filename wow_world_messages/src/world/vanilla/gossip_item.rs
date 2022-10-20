use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm#L3):
/// ```text
/// struct GossipItem {
///     u32 id;
///     u8 item_icon;
///     Bool coded;
///     CString message;
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
    pub message: String,
}

impl GossipItem {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // item_icon: u8
        w.write_all(&self.item_icon.to_le_bytes())?;

        // coded: Bool
        w.write_all(u8::from(self.coded).to_le_bytes().as_slice())?;

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
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
        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            id,
            item_icon,
            coded,
            message,
        })
    }

}

impl GossipItem {
    pub(crate) fn size(&self) -> usize {
        4 // id: u32
        + 1 // item_icon: u8
        + 1 // coded: Bool
        + self.message.len() + 1 // message: CString
    }
}

