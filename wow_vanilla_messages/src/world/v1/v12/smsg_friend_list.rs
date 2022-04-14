use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Friend, FriendError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_FRIEND_LIST {
    pub friends: Vec<Friend>,
}

impl WorldServerMessageWrite for SMSG_FRIEND_LIST {
    const OPCODE: u16 = 0x67;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_FRIEND_LIST {
    type Error = SMSG_FRIEND_LISTError;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_friends: u8
        w.write_all(&(self.friends.len() as u8).to_le_bytes())?;

        // friends: Friend[amount_of_friends]
        for i in self.friends.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_FRIEND_LIST {
    fn size(&self) -> usize {
        1 // amount_of_friends: u8
        + self.friends.iter().fold(0, |acc, x| acc + x.size()) // friends: Friend[amount_of_friends]
    }
}

impl MaximumPossibleSized for SMSG_FRIEND_LIST {
    fn maximum_possible_size() -> usize {
        1 // amount_of_friends: u8
        + 255 * Friend::maximum_possible_size() // friends: Friend[amount_of_friends]
    }
}

#[derive(Debug)]
pub enum SMSG_FRIEND_LISTError {
    Io(std::io::Error),
    Friend(FriendError),
}

impl std::error::Error for SMSG_FRIEND_LISTError {}
impl std::fmt::Display for SMSG_FRIEND_LISTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Friend(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_FRIEND_LISTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<FriendError> for SMSG_FRIEND_LISTError {
    fn from(e: FriendError) -> Self {
        Self::Friend(e)
    }
}

