use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Friend;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_FRIEND_LIST {
    pub friends: Vec<Friend>,
}

impl SMSG_FRIEND_LIST {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // amount_of_friends: u8
        w.write_all(&(self.friends.len() as u8).to_le_bytes())?;

        // friends: Friend[amount_of_friends]
        for i in self.friends.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_FRIEND_LIST {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_friends: u8
        w.write_all(&(self.friends.len() as u8).to_le_bytes())?;

        // friends: Friend[amount_of_friends]
        for i in self.friends.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0067;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_friends: u8
        let amount_of_friends = crate::util::read_u8_le(r)?;

        // friends: Friend[amount_of_friends]
        let mut friends = Vec::with_capacity(amount_of_friends as usize);
        for i in 0..amount_of_friends {
            friends.push(Friend::read(r)?);
        }

        Ok(Self {
            friends,
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
            // amount_of_friends: u8
            let amount_of_friends = crate::util::tokio_read_u8_le(r).await?;

            // friends: Friend[amount_of_friends]
            let mut friends = Vec::with_capacity(amount_of_friends as usize);
            for i in 0..amount_of_friends {
                friends.push(Friend::tokio_read(r).await?);
            }

            Ok(Self {
                friends,
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
            // amount_of_friends: u8
            let amount_of_friends = crate::util::astd_read_u8_le(r).await?;

            // friends: Friend[amount_of_friends]
            let mut friends = Vec::with_capacity(amount_of_friends as usize);
            for i in 0..amount_of_friends {
                friends.push(Friend::astd_read(r).await?);
            }

            Ok(Self {
                friends,
            })
        })
    }

}

impl SMSG_FRIEND_LIST {
    pub fn size(&self) -> usize {
        0
        + 1 // amount_of_friends: u8
        + self.friends.iter().fold(0, |acc, x| acc + x.size()) // friends: Friend[amount_of_friends]
    }
}

