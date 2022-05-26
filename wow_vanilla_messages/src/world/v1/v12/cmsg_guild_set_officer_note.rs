use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GUILD_SET_OFFICER_NOTE {
    pub player_name: String,
    pub note: String,
}

impl CMSG_GUILD_SET_OFFICER_NOTE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // player_name: CString
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // note: CString
        w.write_all(self.note.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
}

impl ClientMessage for CMSG_GUILD_SET_OFFICER_NOTE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player_name: CString
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // note: CString
        w.write_all(self.note.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0235;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_name: CString
        let player_name = crate::util::read_c_string_to_vec(r)?;
        let player_name = String::from_utf8(player_name)?;

        // note: CString
        let note = crate::util::read_c_string_to_vec(r)?;
        let note = String::from_utf8(note)?;

        Ok(Self {
            player_name,
            note,
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
            // player_name: CString
            let player_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let player_name = String::from_utf8(player_name)?;

            // note: CString
            let note = crate::util::tokio_read_c_string_to_vec(r).await?;
            let note = String::from_utf8(note)?;

            Ok(Self {
                player_name,
                note,
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
            // player_name: CString
            let player_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let player_name = String::from_utf8(player_name)?;

            // note: CString
            let note = crate::util::astd_read_c_string_to_vec(r).await?;
            let note = String::from_utf8(note)?;

            Ok(Self {
                player_name,
                note,
            })
        })
    }

}

impl CMSG_GUILD_SET_OFFICER_NOTE {
    pub fn size(&self) -> usize {
        0
        + self.player_name.len() + 1 // player_name: CString
        + self.note.len() + 1 // note: CString
    }
}

