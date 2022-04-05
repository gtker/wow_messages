use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:478`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L478):
/// ```text
/// struct NpcTextUpdateEmote {
///     u32 delay;
///     u32 emote;
/// }
/// ```
pub struct NpcTextUpdateEmote {
    pub delay: u32,
    pub emote: u32,
}

impl ReadableAndWritable for NpcTextUpdateEmote {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // delay: u32
        let delay = crate::util::read_u32_le(r)?;

        // emote: u32
        let emote = crate::util::read_u32_le(r)?;

        Ok(Self {
            delay,
            emote,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // delay: u32
        w.write_all(&self.delay.to_le_bytes())?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for NpcTextUpdateEmote {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for NpcTextUpdateEmote {
    fn maximum_possible_size() -> usize {
        4 // delay: u32
        + 4 // emote: u32
    }
}

