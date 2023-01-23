use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::shared::arena_team_event_tbc_wrath::ArenaTeamEvent;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_team_event.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_team_event.wowm#L24):
/// ```text
/// smsg SMSG_ARENA_TEAM_EVENT = 0x0357 {
///     ArenaTeamEvent event;
///     if (event == JOIN) {
///         CString joiner_name;
///         CString arena_team_name1;
///         Guid joiner;
///     }
///     else if (event == LEAVE) {
///         CString leaver_name;
///         Guid leaver;
///     }
///     else if (event == REMOVE) {
///         CString kicked_player_name;
///         CString arena_team_name2;
///         CString kicker_name;
///     }
///     else if (event == LEADER_IS
///         || event == DISBANDED) {
///         CString leader_name;
///         CString arena_team_name3;
///     }
///     else if (event == LEADER_CHANGED) {
///         CString old_leader;
///         CString new_leader;
///     }
///     u8 amount_of_strings;
///     CString[amount_of_strings] string;
/// }
/// ```
pub struct SMSG_ARENA_TEAM_EVENT {
    pub event: SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent,
    pub string: Vec<String>,
}

impl crate::Message for SMSG_ARENA_TEAM_EVENT {
    const OPCODE: u32 = 0x0357;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // event: ArenaTeamEvent
        w.write_all(&(self.event.as_int() as u8).to_le_bytes())?;

        match &self.event {
            SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Join {
                arena_team_name1,
                joiner,
                joiner_name,
            } => {
                // joiner_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(joiner_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `joiner_name` must not be null-terminated.");
                w.write_all(joiner_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // arena_team_name1: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(arena_team_name1.as_bytes().iter().rev().next(), Some(&0_u8), "String `arena_team_name1` must not be null-terminated.");
                w.write_all(arena_team_name1.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // joiner: Guid
                w.write_all(&joiner.guid().to_le_bytes())?;

            }
            SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Leave {
                leaver,
                leaver_name,
            } => {
                // leaver_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(leaver_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `leaver_name` must not be null-terminated.");
                w.write_all(leaver_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // leaver: Guid
                w.write_all(&leaver.guid().to_le_bytes())?;

            }
            SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Remove {
                arena_team_name2,
                kicked_player_name,
                kicker_name,
            } => {
                // kicked_player_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(kicked_player_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `kicked_player_name` must not be null-terminated.");
                w.write_all(kicked_player_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // arena_team_name2: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(arena_team_name2.as_bytes().iter().rev().next(), Some(&0_u8), "String `arena_team_name2` must not be null-terminated.");
                w.write_all(arena_team_name2.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // kicker_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(kicker_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `kicker_name` must not be null-terminated.");
                w.write_all(kicker_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::LeaderIs {
                arena_team_name3,
                leader_name,
            } => {
                // leader_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(leader_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `leader_name` must not be null-terminated.");
                w.write_all(leader_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // arena_team_name3: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(arena_team_name3.as_bytes().iter().rev().next(), Some(&0_u8), "String `arena_team_name3` must not be null-terminated.");
                w.write_all(arena_team_name3.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::LeaderChanged {
                new_leader,
                old_leader,
            } => {
                // old_leader: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(old_leader.as_bytes().iter().rev().next(), Some(&0_u8), "String `old_leader` must not be null-terminated.");
                w.write_all(old_leader.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // new_leader: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(new_leader.as_bytes().iter().rev().next(), Some(&0_u8), "String `new_leader` must not be null-terminated.");
                w.write_all(new_leader.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Disbanded {
                arena_team_name3,
                leader_name,
            } => {
                // leader_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(leader_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `leader_name` must not be null-terminated.");
                w.write_all(leader_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // arena_team_name3: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(arena_team_name3.as_bytes().iter().rev().next(), Some(&0_u8), "String `arena_team_name3` must not be null-terminated.");
                w.write_all(arena_team_name3.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
        }

        // amount_of_strings: u8
        w.write_all(&(self.string.len() as u8).to_le_bytes())?;

        // string: CString[amount_of_strings]
        for i in self.string.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=66306).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0357, size: body_size as u32 });
        }

        // event: ArenaTeamEvent
        let event: ArenaTeamEvent = crate::util::read_u8_le(r)?.try_into()?;

        let event_if = match event {
            ArenaTeamEvent::Join => {
                // joiner_name: CString
                let joiner_name = crate::util::read_c_string_to_vec(r)?;
                let joiner_name = String::from_utf8(joiner_name)?;

                // arena_team_name1: CString
                let arena_team_name1 = crate::util::read_c_string_to_vec(r)?;
                let arena_team_name1 = String::from_utf8(arena_team_name1)?;

                // joiner: Guid
                let joiner = Guid::read(r)?;

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Join {
                    arena_team_name1,
                    joiner,
                    joiner_name,
                }
            }
            ArenaTeamEvent::Leave => {
                // leaver_name: CString
                let leaver_name = crate::util::read_c_string_to_vec(r)?;
                let leaver_name = String::from_utf8(leaver_name)?;

                // leaver: Guid
                let leaver = Guid::read(r)?;

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Leave {
                    leaver,
                    leaver_name,
                }
            }
            ArenaTeamEvent::Remove => {
                // kicked_player_name: CString
                let kicked_player_name = crate::util::read_c_string_to_vec(r)?;
                let kicked_player_name = String::from_utf8(kicked_player_name)?;

                // arena_team_name2: CString
                let arena_team_name2 = crate::util::read_c_string_to_vec(r)?;
                let arena_team_name2 = String::from_utf8(arena_team_name2)?;

                // kicker_name: CString
                let kicker_name = crate::util::read_c_string_to_vec(r)?;
                let kicker_name = String::from_utf8(kicker_name)?;

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Remove {
                    arena_team_name2,
                    kicked_player_name,
                    kicker_name,
                }
            }
            ArenaTeamEvent::LeaderIs => {
                // leader_name: CString
                let leader_name = crate::util::read_c_string_to_vec(r)?;
                let leader_name = String::from_utf8(leader_name)?;

                // arena_team_name3: CString
                let arena_team_name3 = crate::util::read_c_string_to_vec(r)?;
                let arena_team_name3 = String::from_utf8(arena_team_name3)?;

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::LeaderIs {
                    arena_team_name3,
                    leader_name,
                }
            }
            ArenaTeamEvent::LeaderChanged => {
                // old_leader: CString
                let old_leader = crate::util::read_c_string_to_vec(r)?;
                let old_leader = String::from_utf8(old_leader)?;

                // new_leader: CString
                let new_leader = crate::util::read_c_string_to_vec(r)?;
                let new_leader = String::from_utf8(new_leader)?;

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::LeaderChanged {
                    new_leader,
                    old_leader,
                }
            }
            ArenaTeamEvent::Disbanded => {
                // leader_name: CString
                let leader_name = crate::util::read_c_string_to_vec(r)?;
                let leader_name = String::from_utf8(leader_name)?;

                // arena_team_name3: CString
                let arena_team_name3 = crate::util::read_c_string_to_vec(r)?;
                let arena_team_name3 = String::from_utf8(arena_team_name3)?;

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Disbanded {
                    arena_team_name3,
                    leader_name,
                }
            }
        };

        // amount_of_strings: u8
        let amount_of_strings = crate::util::read_u8_le(r)?;

        // string: CString[amount_of_strings]
        let mut string = Vec::with_capacity(amount_of_strings as usize);
        for i in 0..amount_of_strings {
            let s = crate::util::read_c_string_to_vec(r)?;
            string.push(String::from_utf8(s)?);
        }

        Ok(Self {
            event: event_if,
            string,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_ARENA_TEAM_EVENT {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ARENA_TEAM_EVENT {}

impl SMSG_ARENA_TEAM_EVENT {
    pub(crate) fn size(&self) -> usize {
        self.event.size() // event: SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent
        + 1 // amount_of_strings: u8
        + self.string.iter().fold(0, |acc, x| acc + x.len() + 1) // string: CString[amount_of_strings]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent {
    Join {
        arena_team_name1: String,
        joiner: Guid,
        joiner_name: String,
    },
    Leave {
        leaver: Guid,
        leaver_name: String,
    },
    Remove {
        arena_team_name2: String,
        kicked_player_name: String,
        kicker_name: String,
    },
    LeaderIs {
        arena_team_name3: String,
        leader_name: String,
    },
    LeaderChanged {
        new_leader: String,
        old_leader: String,
    },
    Disbanded {
        arena_team_name3: String,
        leader_name: String,
    },
}

impl Default for SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Join {
            arena_team_name1: Default::default(),
            joiner: Default::default(),
            joiner_name: Default::default(),
        }
    }
}

impl SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Join { .. } => 3,
            Self::Leave { .. } => 4,
            Self::Remove { .. } => 5,
            Self::LeaderIs { .. } => 6,
            Self::LeaderChanged { .. } => 7,
            Self::Disbanded { .. } => 8,
        }
    }

}

impl SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Join {
                arena_team_name1,
                joiner,
                joiner_name,
            } => {
                1
                + arena_team_name1.len() + 1 // arena_team_name1: CString
                + 8 // joiner: Guid
                + joiner_name.len() + 1 // joiner_name: CString
            }
            Self::Leave {
                leaver,
                leaver_name,
            } => {
                1
                + 8 // leaver: Guid
                + leaver_name.len() + 1 // leaver_name: CString
            }
            Self::Remove {
                arena_team_name2,
                kicked_player_name,
                kicker_name,
            } => {
                1
                + arena_team_name2.len() + 1 // arena_team_name2: CString
                + kicked_player_name.len() + 1 // kicked_player_name: CString
                + kicker_name.len() + 1 // kicker_name: CString
            }
            Self::LeaderIs {
                arena_team_name3,
                leader_name,
            } => {
                1
                + arena_team_name3.len() + 1 // arena_team_name3: CString
                + leader_name.len() + 1 // leader_name: CString
            }
            Self::LeaderChanged {
                new_leader,
                old_leader,
            } => {
                1
                + new_leader.len() + 1 // new_leader: CString
                + old_leader.len() + 1 // old_leader: CString
            }
            Self::Disbanded {
                arena_team_name3,
                leader_name,
            } => {
                1
                + arena_team_name3.len() + 1 // arena_team_name3: CString
                + leader_name.len() + 1 // leader_name: CString
            }
        }
    }
}

