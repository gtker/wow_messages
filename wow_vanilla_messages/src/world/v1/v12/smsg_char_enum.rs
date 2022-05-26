use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Character, CharacterError};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_CHAR_ENUM {
    pub characters: Vec<Character>,
}

impl SMSG_CHAR_ENUM {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // amount_of_characters: u8
        w.write_all(&(self.characters.len() as u8).to_le_bytes())?;

        // characters: Character[amount_of_characters]
        for i in self.characters.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_CHAR_ENUM {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // amount_of_characters: u8
        w.write_all(&(self.characters.len() as u8).to_le_bytes())?;

        // characters: Character[amount_of_characters]
        for i in self.characters.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
    const OPCODE: u16 = 0x003b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_CHAR_ENUMError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_characters: u8
        let amount_of_characters = crate::util::read_u8_le(r)?;

        // characters: Character[amount_of_characters]
        let mut characters = Vec::with_capacity(amount_of_characters as usize);
        for i in 0..amount_of_characters {
            characters.push(Character::read(r)?);
        }

        Ok(Self {
            characters,
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // amount_of_characters: u8
            let amount_of_characters = crate::util::tokio_read_u8_le(r).await?;

            // characters: Character[amount_of_characters]
            let mut characters = Vec::with_capacity(amount_of_characters as usize);
            for i in 0..amount_of_characters {
                characters.push(Character::tokio_read(r).await?);
            }

            Ok(Self {
                characters,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // amount_of_characters: u8
            let amount_of_characters = crate::util::astd_read_u8_le(r).await?;

            // characters: Character[amount_of_characters]
            let mut characters = Vec::with_capacity(amount_of_characters as usize);
            for i in 0..amount_of_characters {
                characters.push(Character::astd_read(r).await?);
            }

            Ok(Self {
                characters,
            })
        })
    }

}

impl SMSG_CHAR_ENUM {
    pub fn size(&self) -> usize {
        0
        + 1 // amount_of_characters: u8
        + self.characters.iter().fold(0, |acc, x| acc + x.size()) // characters: Character[amount_of_characters]
    }
}

#[derive(Debug)]
pub enum SMSG_CHAR_ENUMError {
    Io(std::io::Error),
    Character(CharacterError),
}

impl std::error::Error for SMSG_CHAR_ENUMError {}
impl std::fmt::Display for SMSG_CHAR_ENUMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Character(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_CHAR_ENUMError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<CharacterError> for SMSG_CHAR_ENUMError {
    fn from(e: CharacterError) -> Self {
        Self::Character(e)
    }
}

