use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::arena_team_event_tbc_wrath::ArenaTeamEvent;

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

impl crate::private::Sealed for SMSG_ARENA_TEAM_EVENT {}
impl SMSG_ARENA_TEAM_EVENT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(2..=66306).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // event: ArenaTeamEvent
        let event = crate::util::read_u8_le(&mut r)?.try_into()?;

        let event_if = match event {
            ArenaTeamEvent::Join => {
                // joiner_name: CString
                let joiner_name = {
                    let joiner_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(joiner_name)?
                };

                // arena_team_name1: CString
                let arena_team_name1 = {
                    let arena_team_name1 = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(arena_team_name1)?
                };

                // joiner: Guid
                let joiner = crate::util::read_guid(&mut r)?;

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Join {
                    arena_team_name1,
                    joiner,
                    joiner_name,
                }
            }
            ArenaTeamEvent::Leave => {
                // leaver_name: CString
                let leaver_name = {
                    let leaver_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(leaver_name)?
                };

                // leaver: Guid
                let leaver = crate::util::read_guid(&mut r)?;

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Leave {
                    leaver,
                    leaver_name,
                }
            }
            ArenaTeamEvent::Remove => {
                // kicked_player_name: CString
                let kicked_player_name = {
                    let kicked_player_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(kicked_player_name)?
                };

                // arena_team_name2: CString
                let arena_team_name2 = {
                    let arena_team_name2 = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(arena_team_name2)?
                };

                // kicker_name: CString
                let kicker_name = {
                    let kicker_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(kicker_name)?
                };

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Remove {
                    arena_team_name2,
                    kicked_player_name,
                    kicker_name,
                }
            }
            ArenaTeamEvent::LeaderIs => {
                // leader_name: CString
                let leader_name = {
                    let leader_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(leader_name)?
                };

                // arena_team_name3: CString
                let arena_team_name3 = {
                    let arena_team_name3 = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(arena_team_name3)?
                };

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::LeaderIs {
                    arena_team_name3,
                    leader_name,
                }
            }
            ArenaTeamEvent::LeaderChanged => {
                // old_leader: CString
                let old_leader = {
                    let old_leader = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(old_leader)?
                };

                // new_leader: CString
                let new_leader = {
                    let new_leader = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(new_leader)?
                };

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::LeaderChanged {
                    new_leader,
                    old_leader,
                }
            }
            ArenaTeamEvent::Disbanded => {
                // leader_name: CString
                let leader_name = {
                    let leader_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(leader_name)?
                };

                // arena_team_name3: CString
                let arena_team_name3 = {
                    let arena_team_name3 = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(arena_team_name3)?
                };

                SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Disbanded {
                    arena_team_name3,
                    leader_name,
                }
            }
        };

        // amount_of_strings: u8
        let amount_of_strings = crate::util::read_u8_le(&mut r)?;

        // string: CString[amount_of_strings]
        let string = {
            let mut string = Vec::with_capacity(amount_of_strings as usize);
            for _ in 0..amount_of_strings {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                string.push(String::from_utf8(s)?);
            }
            string
        };

        Ok(Self {
            event: event_if,
            string,
        })
    }

}

impl crate::Message for SMSG_ARENA_TEAM_EVENT {
    const OPCODE: u32 = 0x0357;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ARENA_TEAM_EVENT {{").unwrap();
        // Members
        writeln!(s, "    event = {};", ArenaTeamEvent::try_from(self.event.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.event {
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Join {
                arena_team_name1,
                joiner,
                joiner_name,
            } => {
                writeln!(s, "    joiner_name = \"{}\";", joiner_name).unwrap();
                writeln!(s, "    arena_team_name1 = \"{}\";", arena_team_name1).unwrap();
                writeln!(s, "    joiner = {};", joiner.guid()).unwrap();
            }
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Leave {
                leaver,
                leaver_name,
            } => {
                writeln!(s, "    leaver_name = \"{}\";", leaver_name).unwrap();
                writeln!(s, "    leaver = {};", leaver.guid()).unwrap();
            }
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Remove {
                arena_team_name2,
                kicked_player_name,
                kicker_name,
            } => {
                writeln!(s, "    kicked_player_name = \"{}\";", kicked_player_name).unwrap();
                writeln!(s, "    arena_team_name2 = \"{}\";", arena_team_name2).unwrap();
                writeln!(s, "    kicker_name = \"{}\";", kicker_name).unwrap();
            }
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::LeaderIs {
                arena_team_name3,
                leader_name,
            } => {
                writeln!(s, "    leader_name = \"{}\";", leader_name).unwrap();
                writeln!(s, "    arena_team_name3 = \"{}\";", arena_team_name3).unwrap();
            }
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::LeaderChanged {
                new_leader,
                old_leader,
            } => {
                writeln!(s, "    old_leader = \"{}\";", old_leader).unwrap();
                writeln!(s, "    new_leader = \"{}\";", new_leader).unwrap();
            }
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Disbanded {
                arena_team_name3,
                leader_name,
            } => {
                writeln!(s, "    leader_name = \"{}\";", leader_name).unwrap();
                writeln!(s, "    arena_team_name3 = \"{}\";", arena_team_name3).unwrap();
            }
        }

        writeln!(s, "    amount_of_strings = {};", self.string.len()).unwrap();
        write!(s, "    string = [").unwrap();
        for v in self.string.as_slice() {
            write!(s, "\"{v}\", ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 855_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "event", "    ");
        match &self.event {
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Join {
                arena_team_name1,
                joiner,
                joiner_name,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, joiner_name.len() + 1, "joiner_name", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, arena_team_name1.len() + 1, "arena_team_name1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "joiner", "    ");
            }
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Leave {
                leaver,
                leaver_name,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, leaver_name.len() + 1, "leaver_name", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "leaver", "    ");
            }
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Remove {
                arena_team_name2,
                kicked_player_name,
                kicker_name,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, kicked_player_name.len() + 1, "kicked_player_name", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, arena_team_name2.len() + 1, "arena_team_name2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, kicker_name.len() + 1, "kicker_name", "    ");
            }
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::LeaderIs {
                arena_team_name3,
                leader_name,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, leader_name.len() + 1, "leader_name", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, arena_team_name3.len() + 1, "arena_team_name3", "    ");
            }
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::LeaderChanged {
                new_leader,
                old_leader,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, old_leader.len() + 1, "old_leader", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, new_leader.len() + 1, "new_leader", "    ");
            }
            crate::shared::smsg_arena_team_event_tbc_wrath::SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Disbanded {
                arena_team_name3,
                leader_name,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, leader_name.len() + 1, "leader_name", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, arena_team_name3.len() + 1, "arena_team_name3", "    ");
            }
        }

        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_strings", "    ");
        if !self.string.is_empty() {
            writeln!(s, "    /* string: CString[amount_of_strings] start */").unwrap();
            for (i, v) in self.string.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("string {i}"), "    ");
            }
            writeln!(s, "    /* string: CString[amount_of_strings] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // event: ArenaTeamEvent
        w.write_all(&(self.event.as_int().to_le_bytes()))?;

        match &self.event {
            SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent::Join {
                arena_team_name1,
                joiner,
                joiner_name,
            } => {
                // joiner_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(joiner_name.as_bytes().iter().next_back(), Some(&0_u8), "String `joiner_name` must not be null-terminated.");
                w.write_all(joiner_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // arena_team_name1: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(arena_team_name1.as_bytes().iter().next_back(), Some(&0_u8), "String `arena_team_name1` must not be null-terminated.");
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
                assert_ne!(leaver_name.as_bytes().iter().next_back(), Some(&0_u8), "String `leaver_name` must not be null-terminated.");
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
                assert_ne!(kicked_player_name.as_bytes().iter().next_back(), Some(&0_u8), "String `kicked_player_name` must not be null-terminated.");
                w.write_all(kicked_player_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // arena_team_name2: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(arena_team_name2.as_bytes().iter().next_back(), Some(&0_u8), "String `arena_team_name2` must not be null-terminated.");
                w.write_all(arena_team_name2.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // kicker_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(kicker_name.as_bytes().iter().next_back(), Some(&0_u8), "String `kicker_name` must not be null-terminated.");
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
                assert_ne!(leader_name.as_bytes().iter().next_back(), Some(&0_u8), "String `leader_name` must not be null-terminated.");
                w.write_all(leader_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // arena_team_name3: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(arena_team_name3.as_bytes().iter().next_back(), Some(&0_u8), "String `arena_team_name3` must not be null-terminated.");
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
                assert_ne!(old_leader.as_bytes().iter().next_back(), Some(&0_u8), "String `old_leader` must not be null-terminated.");
                w.write_all(old_leader.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // new_leader: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(new_leader.as_bytes().iter().next_back(), Some(&0_u8), "String `new_leader` must not be null-terminated.");
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
                assert_ne!(leader_name.as_bytes().iter().next_back(), Some(&0_u8), "String `leader_name` must not be null-terminated.");
                w.write_all(leader_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // arena_team_name3: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(arena_team_name3.as_bytes().iter().next_back(), Some(&0_u8), "String `arena_team_name3` must not be null-terminated.");
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

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(855, "SMSG_ARENA_TEAM_EVENT", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ARENA_TEAM_EVENT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ARENA_TEAM_EVENT {}

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

impl std::fmt::Display for SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Join{ .. } => f.write_str("Join"),
            Self::Leave{ .. } => f.write_str("Leave"),
            Self::Remove{ .. } => f.write_str("Remove"),
            Self::LeaderIs{ .. } => f.write_str("LeaderIs"),
            Self::LeaderChanged{ .. } => f.write_str("LeaderChanged"),
            Self::Disbanded{ .. } => f.write_str("Disbanded"),
        }
    }
}

impl SMSG_ARENA_TEAM_EVENT_ArenaTeamEvent {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Join {
                arena_team_name1,
                joiner_name,
                ..
            } => {
                1
                + arena_team_name1.len() + 1 // arena_team_name1: CString
                + 8 // joiner: Guid
                + joiner_name.len() + 1 // joiner_name: CString
            }
            Self::Leave {
                leaver_name,
                ..
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

