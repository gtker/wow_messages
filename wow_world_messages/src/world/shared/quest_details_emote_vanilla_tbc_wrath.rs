use std::io::{Read, Write};

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm#L1):
/// ```text
/// struct QuestDetailsEmote {
///     u32 emote;
///     Milliseconds emote_delay;
/// }
/// ```
pub struct QuestDetailsEmote {
    pub emote: u32,
    pub emote_delay: Duration,
}

impl QuestDetailsEmote {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // emote_delay: Milliseconds
        w.write_all((self.emote_delay.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
}

impl QuestDetailsEmote {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // emote: u32
        let emote = crate::util::read_u32_le(&mut r)?;

        // emote_delay: Milliseconds
        let emote_delay = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            emote,
            emote_delay,
        })
    }

}

