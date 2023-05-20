use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::channel_member_flags_vanilla_tbc_wrath::ChannelMemberFlags;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_list.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_list.wowm#L24):
/// ```text
/// struct ChannelMember {
///     Guid guid;
///     ChannelMemberFlags member_flags;
/// }
/// ```
pub struct ChannelMember {
    pub guid: Guid,
    pub member_flags: ChannelMemberFlags,
}

impl ChannelMember {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // member_flags: ChannelMemberFlags
        w.write_all(&(self.member_flags.as_int().to_le_bytes()))?;

        Ok(())
    }
}

impl ChannelMember {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, std::io::Error> {
        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // member_flags: ChannelMemberFlags
        let member_flags = ChannelMemberFlags::new(crate::util::read_u8_le(&mut r)?);

        Ok(Self {
            guid,
            member_flags,
        })
    }

}

