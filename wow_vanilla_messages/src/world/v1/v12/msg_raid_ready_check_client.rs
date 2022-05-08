use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_RAID_READY_CHECK_Client {
    pub answer: Option<MSG_RAID_READY_CHECK_Client_answer>,
}

impl ClientMessageWrite for MSG_RAID_READY_CHECK_Client {}

impl MessageBody for MSG_RAID_READY_CHECK_Client {
    const OPCODE: u16 = 0x0322;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // optional answer
        let current_size = {
            0
        };
        let answer = if current_size < body_size as usize {
            // state: u8
            let state = crate::util::read_u8_le(r)?;

            Some(MSG_RAID_READY_CHECK_Client_answer {
                state,
            })
        } else {
            None
        };

        Ok(Self {
            answer,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // optional answer
        if let Some(v) = &self.answer {
            // state: u8
            w.write_all(&v.state.to_le_bytes())?;

        }

        Ok(())
    }

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
            // optional answer
            let current_size = {
                0
            };
            let answer = if current_size < body_size as usize {
                // state: u8
                let state = crate::util::tokio_read_u8_le(r).await?;

                Some(MSG_RAID_READY_CHECK_Client_answer {
                    state,
                })
            } else {
                None
            };

            Ok(Self {
                answer,
            })
        })
    }

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
            // optional answer
            if let Some(v) = &self.answer {
                // state: u8
                w.write_all(&v.state.to_le_bytes()).await?;

            }

            Ok(())
        })
    }

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
            // optional answer
            let current_size = {
                0
            };
            let answer = if current_size < body_size as usize {
                // state: u8
                let state = crate::util::astd_read_u8_le(r).await?;

                Some(MSG_RAID_READY_CHECK_Client_answer {
                    state,
                })
            } else {
                None
            };

            Ok(Self {
                answer,
            })
        })
    }

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
            // optional answer
            if let Some(v) = &self.answer {
                // state: u8
                w.write_all(&v.state.to_le_bytes()).await?;

            }

            Ok(())
        })
    }

}

impl VariableSized for MSG_RAID_READY_CHECK_Client {
    fn size(&self) -> usize {
        0
        + if let Some(answer) = &self.answer {
            0
            + 1 // state: u8
        } else {
            0
        }
    }
}

impl MaximumPossibleSized for MSG_RAID_READY_CHECK_Client {
    fn maximum_possible_size() -> usize {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MSG_RAID_READY_CHECK_Client_answer {
    pub state: u8,
}

impl MSG_RAID_READY_CHECK_Client_answer {
    pub fn size(&self) -> usize {
        1 // state: u8
    }
}

