use std::convert::{TryFrom, TryInto};
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:303`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L303):
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
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // delay: u32
        w.write_all(&self.delay.to_le_bytes())?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        Ok(())
    }
}

impl NpcTextUpdateEmote {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // delay: u32
        let delay = crate::util::read_u32_le(r)?;

        // emote: u32
        let emote = crate::util::read_u32_le(r)?;

        Ok(Self {
            delay,
            emote,
        })
    }

}

