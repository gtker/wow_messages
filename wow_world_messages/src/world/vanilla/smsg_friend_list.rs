use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    Area, Class, Friend, FriendStatus,
};
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

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

impl crate::private::Sealed for SMSG_FRIEND_LIST {}
impl SMSG_FRIEND_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=5377).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0067, size: body_size });
        }

        // amount_of_friends: u8
        let amount_of_friends = crate::util::read_u8_le(&mut r)?;

        // friends: Friend[amount_of_friends]
        let friends = {
            let mut friends = Vec::with_capacity(amount_of_friends as usize);
            for _ in 0..amount_of_friends {
                friends.push(Friend::read(&mut r)?);
            }
            friends
        };

        Ok(Self {
            friends,
        })
    }

}

impl crate::Message for SMSG_FRIEND_LIST {
    const OPCODE: u32 = 0x0067;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_FRIEND_LIST {{").unwrap();
        // Members
        writeln!(s, "    amount_of_friends = {};", self.friends.len()).unwrap();
        write!(s, "    friends = [").unwrap();
        for v in self.friends.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "        status = {};", FriendStatus::try_from(v.status.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.status {
                crate::vanilla::Friend_FriendStatus::Online {
                    area,
                    class,
                    level,
                } => {
                    writeln!(s, "        area = {};", area.as_test_case_value()).unwrap();
                    writeln!(s, "        level = {};", level.as_int()).unwrap();
                    writeln!(s, "        class = {};", class.as_test_case_value()).unwrap();
                }
                crate::vanilla::Friend_FriendStatus::Afk {
                    area,
                    class,
                    level,
                } => {
                    writeln!(s, "        area = {};", area.as_test_case_value()).unwrap();
                    writeln!(s, "        level = {};", level.as_int()).unwrap();
                    writeln!(s, "        class = {};", class.as_test_case_value()).unwrap();
                }
                crate::vanilla::Friend_FriendStatus::Unknown3 {
                    area,
                    class,
                    level,
                } => {
                    writeln!(s, "        area = {};", area.as_test_case_value()).unwrap();
                    writeln!(s, "        level = {};", level.as_int()).unwrap();
                    writeln!(s, "        class = {};", class.as_test_case_value()).unwrap();
                }
                crate::vanilla::Friend_FriendStatus::Dnd {
                    area,
                    class,
                    level,
                } => {
                    writeln!(s, "        area = {};", area.as_test_case_value()).unwrap();
                    writeln!(s, "        level = {};", level.as_int()).unwrap();
                    writeln!(s, "        class = {};", class.as_test_case_value()).unwrap();
                }
                _ => {}
            }


            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 103_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_friends", "    ");
        if !self.friends.is_empty() {
            writeln!(s, "    /* friends: Friend[amount_of_friends] start */").unwrap();
            for (i, v) in self.friends.iter().enumerate() {
                writeln!(s, "    /* friends: Friend[amount_of_friends] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "status", "        ");
                match &v.status {
                    crate::vanilla::Friend_FriendStatus::Online {
                        area,
                        class,
                        level,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "level", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "class", "        ");
                    }
                    crate::vanilla::Friend_FriendStatus::Afk {
                        area,
                        class,
                        level,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "level", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "class", "        ");
                    }
                    crate::vanilla::Friend_FriendStatus::Unknown3 {
                        area,
                        class,
                        level,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "level", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "class", "        ");
                    }
                    crate::vanilla::Friend_FriendStatus::Dnd {
                        area,
                        class,
                        level,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "level", "        ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "class", "        ");
                    }
                    _ => {}
                }

                writeln!(s, "    /* friends: Friend[amount_of_friends] {i} end */").unwrap();
            }
            writeln!(s, "    /* friends: Friend[amount_of_friends] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_friends: u8
        w.write_all(&(self.friends.len() as u8).to_le_bytes())?;

        // friends: Friend[amount_of_friends]
        for i in self.friends.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
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

