use std::convert::{TryFrom, TryInto};
use crate::world::shared::channel_member_vanilla_tbc_wrath::ChannelMember;
use crate::world::shared::channel_flags_vanilla_tbc_wrath::ChannelFlags;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_list.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_list.wowm#L29):
/// ```text
/// smsg SMSG_CHANNEL_LIST = 0x009B {
///     CString channel_name;
///     ChannelFlags channel_flags;
///     u32 amount_of_members;
///     ChannelMember[amount_of_members] members;
/// }
/// ```
pub struct SMSG_CHANNEL_LIST {
    pub channel_name: String,
    pub channel_flags: ChannelFlags,
    pub members: Vec<ChannelMember>,
}

impl crate::Message for SMSG_CHANNEL_LIST {
    const OPCODE: u32 = 0x009b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // channel_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // channel_flags: ChannelFlags
        w.write_all(&(self.channel_flags.as_int() as u8).to_le_bytes())?;

        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // members: ChannelMember[amount_of_members]
        for i in self.members.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x009B, size: body_size as u32 });
        }

        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        // channel_flags: ChannelFlags
        let channel_flags = ChannelFlags::new(crate::util::read_u8_le(r)?);

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(r)?;

        // members: ChannelMember[amount_of_members]
        let mut members = Vec::with_capacity(amount_of_members as usize);
        for i in 0..amount_of_members {
            members.push(ChannelMember::read(r)?);
        }

        Ok(Self {
            channel_name,
            channel_flags,
            members,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_CHANNEL_LIST {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_CHANNEL_LIST {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CHANNEL_LIST {}

impl SMSG_CHANNEL_LIST {
    pub(crate) fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString
        + 1 // channel_flags: ChannelFlags
        + 4 // amount_of_members: u32
        + self.members.len() * 9 // members: ChannelMember[amount_of_members]
    }
}

