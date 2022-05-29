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

impl ServerMessage for SMSG_FRIEND_LIST {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_friends: u8
        w.write_all(&(self.friends.len() as u8).to_le_bytes())?;

        // friends: Friend[amount_of_friends]
        for i in self.friends.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0067;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
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

}

impl SMSG_FRIEND_LIST {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_friends: u8
        + self.friends.iter().fold(0, |acc, x| acc + x.size()) // friends: Friend[amount_of_friends]
    }
}

