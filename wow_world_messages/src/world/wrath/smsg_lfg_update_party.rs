use std::io::{Read, Write};

use crate::wrath::{
    LfgJoinStatus, LfgUpdateType,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update_party.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update_party.wowm#L1):
/// ```text
/// smsg SMSG_LFG_UPDATE_PARTY = 0x0368 {
///     LfgUpdateType update_type;
///     LfgJoinStatus join_status;
///     if (join_status == JOINED) {
///         u8 joined;
///         u8 queued;
///         u8 no_partial_clear;
///         u8 achievements;
///         u8 amount_of_dungeons;
///         u32[amount_of_dungeons] dungeons;
///         CString comment;
///     }
/// }
/// ```
pub struct SMSG_LFG_UPDATE_PARTY {
    pub update_type: LfgUpdateType,
    pub join_status: SMSG_LFG_UPDATE_PARTY_LfgJoinStatus,
}

impl crate::private::Sealed for SMSG_LFG_UPDATE_PARTY {}
impl SMSG_LFG_UPDATE_PARTY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(2..=1287).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // update_type: LfgUpdateType
        let update_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // join_status: LfgJoinStatus
        let join_status = crate::util::read_u8_le(&mut r)?.try_into()?;

        let join_status_if = match join_status {
            LfgJoinStatus::NotJoined => SMSG_LFG_UPDATE_PARTY_LfgJoinStatus::NotJoined,
            LfgJoinStatus::Joined => {
                // joined: u8
                let joined = crate::util::read_u8_le(&mut r)?;

                // queued: u8
                let queued = crate::util::read_u8_le(&mut r)?;

                // no_partial_clear: u8
                let no_partial_clear = crate::util::read_u8_le(&mut r)?;

                // achievements: u8
                let achievements = crate::util::read_u8_le(&mut r)?;

                // amount_of_dungeons: u8
                let amount_of_dungeons = crate::util::read_u8_le(&mut r)?;

                // dungeons: u32[amount_of_dungeons]
                let dungeons = {
                    let mut dungeons = Vec::with_capacity(amount_of_dungeons as usize);
                    for _ in 0..amount_of_dungeons {
                        dungeons.push(crate::util::read_u32_le(&mut r)?);
                    }
                    dungeons
                };

                // comment: CString
                let comment = {
                    let comment = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(comment)?
                };

                SMSG_LFG_UPDATE_PARTY_LfgJoinStatus::Joined {
                    achievements,
                    comment,
                    dungeons,
                    joined,
                    no_partial_clear,
                    queued,
                }
            }
        };

        Ok(Self {
            update_type,
            join_status: join_status_if,
        })
    }

}

impl crate::Message for SMSG_LFG_UPDATE_PARTY {
    const OPCODE: u32 = 0x0368;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_UPDATE_PARTY {{").unwrap();
        // Members
        writeln!(s, "    update_type = {};", self.update_type.as_test_case_value()).unwrap();
        writeln!(s, "    join_status = {};", LfgJoinStatus::try_from(self.join_status.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.join_status {
            crate::wrath::SMSG_LFG_UPDATE_PARTY_LfgJoinStatus::Joined {
                achievements,
                comment,
                dungeons,
                joined,
                no_partial_clear,
                queued,
            } => {
                writeln!(s, "    joined = {};", joined).unwrap();
                writeln!(s, "    queued = {};", queued).unwrap();
                writeln!(s, "    no_partial_clear = {};", no_partial_clear).unwrap();
                writeln!(s, "    achievements = {};", achievements).unwrap();
                writeln!(s, "    amount_of_dungeons = {};", dungeons.len()).unwrap();
                write!(s, "    dungeons = [").unwrap();
                for v in dungeons.as_slice() {
                    write!(s, "{v:#04X}, ").unwrap();
                }
                writeln!(s, "];").unwrap();
                writeln!(s, "    comment = \"{}\";", comment).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 872_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "update_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "join_status", "    ");
        match &self.join_status {
            crate::wrath::SMSG_LFG_UPDATE_PARTY_LfgJoinStatus::Joined {
                achievements,
                comment,
                dungeons,
                joined,
                no_partial_clear,
                queued,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "joined", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "queued", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "no_partial_clear", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "achievements", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_dungeons", "    ");
                if !dungeons.is_empty() {
                    writeln!(s, "    /* dungeons: u32[amount_of_dungeons] start */").unwrap();
                    for (i, v) in dungeons.iter().enumerate() {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("dungeons {i}"), "    ");
                    }
                    writeln!(s, "    /* dungeons: u32[amount_of_dungeons] end */").unwrap();
                }
                crate::util::write_bytes(&mut s, &mut bytes, comment.len() + 1, "comment", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // update_type: LfgUpdateType
        w.write_all(&(self.update_type.as_int().to_le_bytes()))?;

        // join_status: LfgJoinStatus
        w.write_all(&(self.join_status.as_int().to_le_bytes()))?;

        match &self.join_status {
            SMSG_LFG_UPDATE_PARTY_LfgJoinStatus::Joined {
                achievements,
                comment,
                dungeons,
                joined,
                no_partial_clear,
                queued,
            } => {
                // joined: u8
                w.write_all(&joined.to_le_bytes())?;

                // queued: u8
                w.write_all(&queued.to_le_bytes())?;

                // no_partial_clear: u8
                w.write_all(&no_partial_clear.to_le_bytes())?;

                // achievements: u8
                w.write_all(&achievements.to_le_bytes())?;

                // amount_of_dungeons: u8
                w.write_all(&(dungeons.len() as u8).to_le_bytes())?;

                // dungeons: u32[amount_of_dungeons]
                for i in dungeons.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // comment: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(comment.as_bytes().iter().rev().next(), Some(&0_u8), "String `comment` must not be null-terminated.");
                w.write_all(comment.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(872, "SMSG_LFG_UPDATE_PARTY", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_UPDATE_PARTY {}

impl SMSG_LFG_UPDATE_PARTY {
    pub(crate) fn size(&self) -> usize {
        1 // update_type: LfgUpdateType
        + self.join_status.size() // join_status: SMSG_LFG_UPDATE_PARTY_LfgJoinStatus
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_LFG_UPDATE_PARTY_LfgJoinStatus {
    NotJoined,
    Joined {
        achievements: u8,
        comment: String,
        dungeons: Vec<u32>,
        joined: u8,
        no_partial_clear: u8,
        queued: u8,
    },
}

impl Default for SMSG_LFG_UPDATE_PARTY_LfgJoinStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotJoined
    }
}

impl SMSG_LFG_UPDATE_PARTY_LfgJoinStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotJoined => 0,
            Self::Joined { .. } => 1,
        }
    }

}

impl std::fmt::Display for SMSG_LFG_UPDATE_PARTY_LfgJoinStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotJoined => f.write_str("NotJoined"),
            Self::Joined{ .. } => f.write_str("Joined"),
        }
    }
}

impl SMSG_LFG_UPDATE_PARTY_LfgJoinStatus {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Joined {
                comment,
                dungeons,
                ..
            } => {
                1
                + 1 // achievements: u8
                + 1 // amount_of_dungeons: u8
                + comment.len() + 1 // comment: CString
                + dungeons.len() * core::mem::size_of::<u32>() // dungeons: u32[amount_of_dungeons]
                + 1 // joined: u8
                + 1 // no_partial_clear: u8
                + 1 // queued: u8
            }
            _ => 1,
        }
    }
}

