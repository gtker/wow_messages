use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Class;
use crate::world::v1::v12::Gender;
use crate::world::v1::v12::Race;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_CHAR_CREATE {
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub gender: Gender,
    pub skin: u8,
    pub face: u8,
    pub hairstyle: u8,
    pub haircolor: u8,
    pub facialhair: u8,
    pub outfit_id: u8,
}

impl CMSG_CHAR_CREATE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&(self.race.as_int() as u8).to_le_bytes())?;

        // class: Class
        w.write_all(&(self.class.as_int() as u8).to_le_bytes())?;

        // gender: Gender
        w.write_all(&(self.gender.as_int() as u8).to_le_bytes())?;

        // skin: u8
        w.write_all(&self.skin.to_le_bytes())?;

        // face: u8
        w.write_all(&self.face.to_le_bytes())?;

        // hairstyle: u8
        w.write_all(&self.hairstyle.to_le_bytes())?;

        // haircolor: u8
        w.write_all(&self.haircolor.to_le_bytes())?;

        // facialhair: u8
        w.write_all(&self.facialhair.to_le_bytes())?;

        // outfit_id: u8
        w.write_all(&self.outfit_id.to_le_bytes())?;

        Ok(w)
    }
}

impl ClientMessage for CMSG_CHAR_CREATE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&(self.race.as_int() as u8).to_le_bytes())?;

        // class: Class
        w.write_all(&(self.class.as_int() as u8).to_le_bytes())?;

        // gender: Gender
        w.write_all(&(self.gender.as_int() as u8).to_le_bytes())?;

        // skin: u8
        w.write_all(&self.skin.to_le_bytes())?;

        // face: u8
        w.write_all(&self.face.to_le_bytes())?;

        // hairstyle: u8
        w.write_all(&self.hairstyle.to_le_bytes())?;

        // haircolor: u8
        w.write_all(&self.haircolor.to_le_bytes())?;

        // facialhair: u8
        w.write_all(&self.facialhair.to_le_bytes())?;

        // outfit_id: u8
        w.write_all(&self.outfit_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0036;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_CHAR_CREATEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // race: Race
        let race: Race = crate::util::read_u8_le(r)?.try_into()?;

        // class: Class
        let class: Class = crate::util::read_u8_le(r)?.try_into()?;

        // gender: Gender
        let gender: Gender = crate::util::read_u8_le(r)?.try_into()?;

        // skin: u8
        let skin = crate::util::read_u8_le(r)?;

        // face: u8
        let face = crate::util::read_u8_le(r)?;

        // hairstyle: u8
        let hairstyle = crate::util::read_u8_le(r)?;

        // haircolor: u8
        let haircolor = crate::util::read_u8_le(r)?;

        // facialhair: u8
        let facialhair = crate::util::read_u8_le(r)?;

        // outfit_id: u8
        let outfit_id = crate::util::read_u8_le(r)?;

        Ok(Self {
            name,
            race,
            class,
            gender,
            skin,
            face,
            hairstyle,
            haircolor,
            facialhair,
            outfit_id,
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
            // name: CString
            let name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // race: Race
            let race: Race = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // class: Class
            let class: Class = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // gender: Gender
            let gender: Gender = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // skin: u8
            let skin = crate::util::tokio_read_u8_le(r).await?;

            // face: u8
            let face = crate::util::tokio_read_u8_le(r).await?;

            // hairstyle: u8
            let hairstyle = crate::util::tokio_read_u8_le(r).await?;

            // haircolor: u8
            let haircolor = crate::util::tokio_read_u8_le(r).await?;

            // facialhair: u8
            let facialhair = crate::util::tokio_read_u8_le(r).await?;

            // outfit_id: u8
            let outfit_id = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                name,
                race,
                class,
                gender,
                skin,
                face,
                hairstyle,
                haircolor,
                facialhair,
                outfit_id,
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
            // name: CString
            let name = crate::util::astd_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // race: Race
            let race: Race = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // class: Class
            let class: Class = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // gender: Gender
            let gender: Gender = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // skin: u8
            let skin = crate::util::astd_read_u8_le(r).await?;

            // face: u8
            let face = crate::util::astd_read_u8_le(r).await?;

            // hairstyle: u8
            let hairstyle = crate::util::astd_read_u8_le(r).await?;

            // haircolor: u8
            let haircolor = crate::util::astd_read_u8_le(r).await?;

            // facialhair: u8
            let facialhair = crate::util::astd_read_u8_le(r).await?;

            // outfit_id: u8
            let outfit_id = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                name,
                race,
                class,
                gender,
                skin,
                face,
                hairstyle,
                haircolor,
                facialhair,
                outfit_id,
            })
        })
    }

}

impl CMSG_CHAR_CREATE {
    pub fn size(&self) -> usize {
        0
        + self.name.len() + 1 // name: CString
        + 1 // race: Race
        + 1 // class: Class
        + 1 // gender: Gender
        + 1 // skin: u8
        + 1 // face: u8
        + 1 // hairstyle: u8
        + 1 // haircolor: u8
        + 1 // facialhair: u8
        + 1 // outfit_id: u8
    }
}

#[derive(Debug)]
pub enum CMSG_CHAR_CREATEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for CMSG_CHAR_CREATEError {}
impl std::fmt::Display for CMSG_CHAR_CREATEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_CHAR_CREATEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for CMSG_CHAR_CREATEError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_CHAR_CREATEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

