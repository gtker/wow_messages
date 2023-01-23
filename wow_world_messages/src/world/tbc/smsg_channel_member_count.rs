use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_channel_member_count.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_channel_member_count.wowm#L1):
/// ```text
/// smsg SMSG_CHANNEL_MEMBER_COUNT = 0x03D4 {
///     CString channel;
///     u8 flags;
///     u32 amount_of_members;
/// }
/// ```
pub struct SMSG_CHANNEL_MEMBER_COUNT {
    pub channel: String,
    pub flags: u8,
    pub amount_of_members: u32,
}

impl crate::Message for SMSG_CHANNEL_MEMBER_COUNT {
    const OPCODE: u32 = 0x03d4;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // channel: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel` must not be null-terminated.");
        w.write_all(self.channel.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        // amount_of_members: u32
        w.write_all(&self.amount_of_members.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=261).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03D4, size: body_size as u32 });
        }

        // channel: CString
        let channel = crate::util::read_c_string_to_vec(r)?;
        let channel = String::from_utf8(channel)?;

        // flags: u8
        let flags = crate::util::read_u8_le(r)?;

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(r)?;

        Ok(Self {
            channel,
            flags,
            amount_of_members,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_CHANNEL_MEMBER_COUNT {}

impl SMSG_CHANNEL_MEMBER_COUNT {
    pub(crate) fn size(&self) -> usize {
        self.channel.len() + 1 // channel: CString
        + 1 // flags: u8
        + 4 // amount_of_members: u32
    }
}

