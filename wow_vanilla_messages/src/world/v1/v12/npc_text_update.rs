use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Language;
use crate::world::v1::v12::NpcTextUpdateEmote;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct NpcTextUpdate {
    pub probability: f32,
    pub texts: [String; 2],
    pub language: Language,
    pub emotes: [NpcTextUpdateEmote; 3],
}

impl NpcTextUpdate {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // probability: f32
        w.write_all(&self.probability.to_le_bytes())?;

        // texts: CString[2]
        for i in self.texts.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        // language: Language
        w.write_all(&(self.language.as_int() as u32).to_le_bytes())?;

        // emotes: NpcTextUpdateEmote[3]
        for i in self.emotes.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
}

impl NpcTextUpdate {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // probability: f32
        let probability = crate::util::read_f32_le(r)?;
        // texts: CString[2]
        let mut texts = Vec::with_capacity(2);
        for i in 0..2 {
            let s = crate::util::read_c_string_to_vec(r)?;
            texts[i] = String::from_utf8(s)?;
        }
        let texts = texts.try_into().unwrap();

        // language: Language
        let language: Language = crate::util::read_u32_le(r)?.try_into()?;

        // emotes: NpcTextUpdateEmote[3]
        let mut emotes = Vec::with_capacity(3);
        for i in 0..3 {
            emotes.push(NpcTextUpdateEmote::read(r)?);
        }
        let emotes = emotes.try_into().unwrap();

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

