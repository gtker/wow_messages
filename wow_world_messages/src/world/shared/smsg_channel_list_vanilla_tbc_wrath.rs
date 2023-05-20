use std::io::{Read, Write};

use crate::shared::channel_member_vanilla_tbc_wrath::ChannelMember;
use wow_world_base::shared::channel_flags_vanilla_tbc_wrath::ChannelFlags;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

impl crate::private::Sealed for SMSG_CHANNEL_LIST {}
impl crate::Message for SMSG_CHANNEL_LIST {
    const OPCODE: u32 = 0x009b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // channel_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // channel_flags: ChannelFlags
        w.write_all(&(self.channel_flags.as_int().to_le_bytes()))?;

        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // members: ChannelMember[amount_of_members]
        for i in self.members.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(6..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x009B, size: body_size });
        }

        // channel_name: CString
        let channel_name = {
            let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel_name)?
        };

        // channel_flags: ChannelFlags
        let channel_flags = ChannelFlags::new(crate::util::read_u8_le(&mut r)?);

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(&mut r)?;

        // members: ChannelMember[amount_of_members]
        let members = {
            let mut members = Vec::with_capacity(amount_of_members as usize);
            for i in 0..amount_of_members {
                members.push(ChannelMember::read(&mut r)?);
            }
            members
        };

        Ok(Self {
            channel_name,
            channel_flags,
            members,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CHANNEL_LIST {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CHANNEL_LIST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CHANNEL_LIST {}

impl SMSG_CHANNEL_LIST {
    pub(crate) fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString
        + 1 // channel_flags: ChannelFlags
        + 4 // amount_of_members: u32
        + self.members.len() * 9 // members: ChannelMember[amount_of_members]
    }
}

