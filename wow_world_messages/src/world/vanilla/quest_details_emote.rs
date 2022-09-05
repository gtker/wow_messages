use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm#L3):
/// ```text
/// struct QuestDetailsEmote {
///     u32 emote;
///     u32 emote_delay_in_msecs;
/// }
/// ```
pub struct QuestDetailsEmote {
    pub emote: u32,
    pub emote_delay_in_msecs: u32,
}

impl QuestDetailsEmote {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // emote_delay_in_msecs: u32
        w.write_all(&self.emote_delay_in_msecs.to_le_bytes())?;

        Ok(())
    }
}

impl QuestDetailsEmote {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // emote: u32
        let emote = crate::util::read_u32_le(r)?;

        // emote_delay_in_msecs: u32
        let emote_delay_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            emote,
            emote_delay_in_msecs,
        })
    }

}

