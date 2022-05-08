use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GOSSIP_SELECT_OPTION {
    pub guid: Guid,
    pub gossip_list_id: u32,
    pub unknown: Option<CMSG_GOSSIP_SELECT_OPTION_unknown>,
}

impl ClientMessageWrite for CMSG_GOSSIP_SELECT_OPTION {}

impl MessageBody for CMSG_GOSSIP_SELECT_OPTION {
    const OPCODE: u16 = 0x017c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GOSSIP_SELECT_OPTIONError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // gossip_list_id: u32
        let gossip_list_id = crate::util::read_u32_le(r)?;

        // optional unknown
        let current_size = {
            0
            + 8 // guid: Guid
            + 4 // gossip_list_id: u32
        };
        let unknown = if current_size < body_size as usize {
            // code: CString
            let code = crate::util::read_c_string_to_vec(r)?;
            let code = String::from_utf8(code)?;

            Some(CMSG_GOSSIP_SELECT_OPTION_unknown {
                code,
            })
        } else {
            None
        };

        Ok(Self {
            guid,
            gossip_list_id,
            unknown,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // gossip_list_id: u32
        w.write_all(&self.gossip_list_id.to_le_bytes())?;

        // optional unknown
        if let Some(v) = &self.unknown {
            // code: CString
            w.write_all(v.code.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // gossip_list_id: u32
            let gossip_list_id = crate::util::tokio_read_u32_le(r).await?;

            // optional unknown
            let current_size = {
                0
                + 8 // guid: Guid
                + 4 // gossip_list_id: u32
            };
            let unknown = if current_size < body_size as usize {
                // code: CString
                let code = crate::util::tokio_read_c_string_to_vec(r).await?;
                let code = String::from_utf8(code)?;

                Some(CMSG_GOSSIP_SELECT_OPTION_unknown {
                    code,
                })
            } else {
                None
            };

            Ok(Self {
                guid,
                gossip_list_id,
                unknown,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // gossip_list_id: u32
            w.write_all(&self.gossip_list_id.to_le_bytes()).await?;

            // optional unknown
            if let Some(v) = &self.unknown {
                // code: CString
                w.write_all(v.code.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // gossip_list_id: u32
            let gossip_list_id = crate::util::astd_read_u32_le(r).await?;

            // optional unknown
            let current_size = {
                0
                + 8 // guid: Guid
                + 4 // gossip_list_id: u32
            };
            let unknown = if current_size < body_size as usize {
                // code: CString
                let code = crate::util::astd_read_c_string_to_vec(r).await?;
                let code = String::from_utf8(code)?;

                Some(CMSG_GOSSIP_SELECT_OPTION_unknown {
                    code,
                })
            } else {
                None
            };

            Ok(Self {
                guid,
                gossip_list_id,
                unknown,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.astd_write(w).await?;

            // gossip_list_id: u32
            w.write_all(&self.gossip_list_id.to_le_bytes()).await?;

            // optional unknown
            if let Some(v) = &self.unknown {
                // code: CString
                w.write_all(v.code.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

            }

            Ok(())
        })
    }

}

impl VariableSized for CMSG_GOSSIP_SELECT_OPTION {
    fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + 4 // gossip_list_id: u32
        + if let Some(unknown) = &self.unknown {
            0
            + unknown.code.len() + 1 // code: CString
        } else {
            0
        }
    }
}

impl MaximumPossibleSized for CMSG_GOSSIP_SELECT_OPTION {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 4 // gossip_list_id: u32
    }
}

#[derive(Debug)]
pub enum CMSG_GOSSIP_SELECT_OPTIONError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GOSSIP_SELECT_OPTIONError {}
impl std::fmt::Display for CMSG_GOSSIP_SELECT_OPTIONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GOSSIP_SELECT_OPTIONError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GOSSIP_SELECT_OPTIONError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CMSG_GOSSIP_SELECT_OPTION_unknown {
    pub code: String,
}

impl CMSG_GOSSIP_SELECT_OPTION_unknown {
    pub fn size(&self) -> usize {
        self.code.len() + 1 // code: CString
    }
}

