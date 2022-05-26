use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_TOGGLE_PVP {
    pub set: Option<CMSG_TOGGLE_PVPset>,
}

impl CMSG_TOGGLE_PVP {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // optional set
        if let Some(v) = &self.set {
            // enable_pvp: u8
            w.write_all(&v.enable_pvp.to_le_bytes())?;

        }

        Ok(w)
    }
}

impl ClientMessage for CMSG_TOGGLE_PVP {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // optional set
        if let Some(v) = &self.set {
            // enable_pvp: u8
            w.write_all(&v.enable_pvp.to_le_bytes())?;

        }

        Ok(w)
    }
    const OPCODE: u16 = 0x0253;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // optional set
        let current_size = {
            0
        };
        let set = if current_size < body_size as usize {
            // enable_pvp: u8
            let enable_pvp = crate::util::read_u8_le(r)?;

            Some(CMSG_TOGGLE_PVPset {
                enable_pvp,
            })
        } else {
            None
        };

        Ok(Self {
            set,
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
            // optional set
            let current_size = {
                0
            };
            let set = if current_size < body_size as usize {
                // enable_pvp: u8
                let enable_pvp = crate::util::tokio_read_u8_le(r).await?;

                Some(CMSG_TOGGLE_PVPset {
                    enable_pvp,
                })
            } else {
                None
            };

            Ok(Self {
                set,
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
            // optional set
            let current_size = {
                0
            };
            let set = if current_size < body_size as usize {
                // enable_pvp: u8
                let enable_pvp = crate::util::astd_read_u8_le(r).await?;

                Some(CMSG_TOGGLE_PVPset {
                    enable_pvp,
                })
            } else {
                None
            };

            Ok(Self {
                set,
            })
        })
    }

}

impl CMSG_TOGGLE_PVP {
    pub fn size(&self) -> usize {
        0
        + if let Some(set) = &self.set {
            0
            + 1 // enable_pvp: u8
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CMSG_TOGGLE_PVPset {
    pub enable_pvp: u8,
}

impl CMSG_TOGGLE_PVPset {
    pub(crate) fn size(&self) -> usize {
        1 // enable_pvp: u8
    }

}

