use std::io::{Read, Write};

use crate::vanilla::{
    Language, NpcTextUpdateEmote,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_npc_text_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_npc_text_update.wowm#L1):
/// ```text
/// struct NpcTextUpdate {
///     f32 probability;
///     CString[2] texts;
///     Language language;
///     NpcTextUpdateEmote[3] emotes;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct NpcTextUpdate {
    pub probability: f32,
    pub texts: [String; 2],
    pub language: Language,
    pub emotes: [NpcTextUpdateEmote; 3],
}

impl NpcTextUpdate {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // probability: f32
        w.write_all(&self.probability.to_le_bytes())?;

        // texts: CString[2]
        for i in self.texts.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        // language: Language
        w.write_all(&(self.language.as_int().to_le_bytes()))?;

        // emotes: NpcTextUpdateEmote[3]
        for i in self.emotes.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
}

impl NpcTextUpdate {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // probability: f32
        let probability = crate::util::read_f32_le(&mut r)?;

        // texts: CString[2]
        let texts = {
            let mut texts = [(); 2].map(|_| String::default());
            for i in texts.iter_mut() {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                *i = String::from_utf8(s)?;
            }
            texts
        };

        // language: Language
        let language = crate::util::read_u32_le(&mut r)?.try_into()?;

        // emotes: NpcTextUpdateEmote[3]
        let emotes = {
            let mut emotes = [NpcTextUpdateEmote::default(); 3];
            for i in emotes.iter_mut() {
                *i = NpcTextUpdateEmote::read(&mut r)?;
            }
            emotes
        };

        Ok(Self {
            probability,
            texts,
            language,
            emotes,
        })
    }

}

impl NpcTextUpdate {
    pub(crate) fn size(&self) -> usize {
        4 // probability: f32
        + self.texts.iter().fold(0, |acc, x| acc + x.len() + 1) // texts: CString[2]
        + 4 // language: Language
        + 3 * 8 // emotes: NpcTextUpdateEmote[3]
    }
}

