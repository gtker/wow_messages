use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:759`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L759):
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

impl NpcTextUpdateEmote {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // delay: u32
        w.write_all(&self.delay.to_le_bytes())?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        Ok(())
    }
}

impl NpcTextUpdateEmote {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // delay: u32
        let delay = crate::util::read_u32_le(&mut r)?;

        // emote: u32
        let emote = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            delay,
            emote,
        })
    }

}

