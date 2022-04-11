use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Language, LanguageError};
use crate::world::v1::v12::NpcTextUpdateEmote;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct NpcTextUpdate {
    pub probability: f32,
    pub texts: [String; 2],
    pub language: Language,
    pub emotes: [NpcTextUpdateEmote; 3],
}

impl ReadableAndWritable for NpcTextUpdate {
    type Error = NpcTextUpdateError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // probability: f32
        let probability = crate::util::read_f32_le(r)?;
        // texts: CString[2]
        let mut texts = Vec::with_capacity(2 as usize);
        for i in 0..2 {
            let s = crate::util::read_c_string_to_vec(r)?;
            texts[i] = String::from_utf8(s)?;
        }
        let texts = texts.try_into().unwrap();

        // language: Language
        let language = Language::read(r)?;

        // emotes: NpcTextUpdateEmote[3]
        let mut emotes = Vec::with_capacity(3 as usize);
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

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // probability: f32
        w.write_all(&self.probability.to_le_bytes())?;

        // texts: CString[2]
        for i in self.texts.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        // language: Language
        self.language.write(w)?;

        // emotes: NpcTextUpdateEmote[3]
        for i in self.emotes.iter() {
            i.write(w)?;
        }

        Ok(())
    }

}

impl VariableSized for NpcTextUpdate {
    fn size(&self) -> usize {
        4 // probability: f32
        + self.texts.iter().fold(0, |acc, x| acc + x.len() + 1) // texts: CString[2]
        + Language::size() // language: Language
        + 3 * NpcTextUpdateEmote::size() // emotes: NpcTextUpdateEmote[3]
    }
}

impl MaximumPossibleSized for NpcTextUpdate {
    fn maximum_possible_size() -> usize {
        4 // probability: f32
        + 2 * 256 // texts: CString[2]
        + Language::maximum_possible_size() // language: Language
        + 3 * NpcTextUpdateEmote::maximum_possible_size() // emotes: NpcTextUpdateEmote[3]
    }
}

#[derive(Debug)]
pub enum NpcTextUpdateError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Language(LanguageError),
}

impl std::error::Error for NpcTextUpdateError {}
impl std::fmt::Display for NpcTextUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Language(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for NpcTextUpdateError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for NpcTextUpdateError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<LanguageError> for NpcTextUpdateError {
    fn from(e: LanguageError) -> Self {
        Self::Language(e)
    }
}

