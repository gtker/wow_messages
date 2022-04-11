use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_list.wowm#L3):
/// ```text
/// struct ChannelMember {
///     Guid guid;
///     u8 member_flags;
/// }
/// ```
pub struct ChannelMember {
    pub guid: Guid,
    pub member_flags: u8,
}

impl ReadableAndWritable for ChannelMember {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // member_flags: u8
        let member_flags = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            member_flags,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // member_flags: u8
        w.write_all(&self.member_flags.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for ChannelMember {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for ChannelMember {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 1 // member_flags: u8
    }
}

