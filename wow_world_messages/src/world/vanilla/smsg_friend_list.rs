use crate::vanilla::Friend;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_list.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_list.wowm#L21):
/// ```text
/// smsg SMSG_FRIEND_LIST = 0x0067 {
///     u8 amount_of_friends;
///     Friend[amount_of_friends] friends;
/// }
/// ```
pub struct SMSG_FRIEND_LIST {
    pub friends: Vec<Friend>,
}

impl crate::Message for SMSG_FRIEND_LIST {
    const OPCODE: u32 = 0x0067;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // amount_of_friends: u8
        w.write_all(&(self.friends.len() as u8).to_le_bytes())?;

        // friends: Friend[amount_of_friends]
        for i in self.friends.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=5377).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0067, size: body_size as u32 });
        }

        // amount_of_friends: u8
        let amount_of_friends = crate::util::read_u8_le(r)?;

        // friends: Friend[amount_of_friends]
        let friends = {
            let mut friends = Vec::with_capacity(amount_of_friends as usize);
            for i in 0..amount_of_friends {
                friends.push(Friend::read(r)?);
            }
            friends
        };

        Ok(Self {
            friends,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_FRIEND_LIST {}

impl SMSG_FRIEND_LIST {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_friends: u8
        + self.friends.iter().fold(0, |acc, x| acc + x.size()) // friends: Friend[amount_of_friends]
    }
}

