use std::io::{Read, Write};

use crate::Guid;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_boot_proposal_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_boot_proposal_update.wowm#L1):
/// ```text
/// smsg SMSG_LFG_BOOT_PROPOSAL_UPDATE = 0x036D {
///     Bool vote_in_progress;
///     Bool did_vote;
///     Bool agreed_with_kick;
///     Guid victim;
///     u32 total_votes;
///     u32 votes_agree;
///     Seconds time_left;
///     u32 votes_needed;
///     CString reason;
/// }
/// ```
pub struct SMSG_LFG_BOOT_PROPOSAL_UPDATE {
    pub vote_in_progress: bool,
    pub did_vote: bool,
    pub agreed_with_kick: bool,
    pub victim: Guid,
    pub total_votes: u32,
    pub votes_agree: u32,
    pub time_left: Duration,
    pub votes_needed: u32,
    pub reason: String,
}

#[cfg(feature = "print-testcase")]
impl SMSG_LFG_BOOT_PROPOSAL_UPDATE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_BOOT_PROPOSAL_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    vote_in_progress = {};", if self.vote_in_progress { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    did_vote = {};", if self.did_vote { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    agreed_with_kick = {};", if self.agreed_with_kick { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    victim = {};", self.victim.guid()).unwrap();
        writeln!(s, "    total_votes = {};", self.total_votes).unwrap();
        writeln!(s, "    votes_agree = {};", self.votes_agree).unwrap();
        writeln!(s, "    time_left = {};", self.time_left.as_secs()).unwrap();
        writeln!(s, "    votes_needed = {};", self.votes_needed).unwrap();
        writeln!(s, "    reason = \"{}\";", self.reason).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 877_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "vote_in_progress", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "did_vote", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "agreed_with_kick", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "victim", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "total_votes", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "votes_agree", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_left", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "votes_needed", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.reason.len() + 1, "reason", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_LFG_BOOT_PROPOSAL_UPDATE {}
impl crate::Message for SMSG_LFG_BOOT_PROPOSAL_UPDATE {
    const OPCODE: u32 = 0x036d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_LFG_BOOT_PROPOSAL_UPDATE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vote_in_progress: Bool
        w.write_all(u8::from(self.vote_in_progress).to_le_bytes().as_slice())?;

        // did_vote: Bool
        w.write_all(u8::from(self.did_vote).to_le_bytes().as_slice())?;

        // agreed_with_kick: Bool
        w.write_all(u8::from(self.agreed_with_kick).to_le_bytes().as_slice())?;

        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        // total_votes: u32
        w.write_all(&self.total_votes.to_le_bytes())?;

        // votes_agree: u32
        w.write_all(&self.votes_agree.to_le_bytes())?;

        // time_left: Seconds
        w.write_all((self.time_left.as_secs() as u32).to_le_bytes().as_slice())?;

        // votes_needed: u32
        w.write_all(&self.votes_needed.to_le_bytes())?;

        // reason: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.reason.as_bytes().iter().rev().next(), Some(&0_u8), "String `reason` must not be null-terminated.");
        w.write_all(self.reason.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(28..=283).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036D, size: body_size });
        }

        // vote_in_progress: Bool
        let vote_in_progress = crate::util::read_u8_le(&mut r)? != 0;

        // did_vote: Bool
        let did_vote = crate::util::read_u8_le(&mut r)? != 0;

        // agreed_with_kick: Bool
        let agreed_with_kick = crate::util::read_u8_le(&mut r)? != 0;

        // victim: Guid
        let victim = crate::util::read_guid(&mut r)?;

        // total_votes: u32
        let total_votes = crate::util::read_u32_le(&mut r)?;

        // votes_agree: u32
        let votes_agree = crate::util::read_u32_le(&mut r)?;

        // time_left: Seconds
        let time_left = Duration::from_secs(crate::util::read_u32_le(&mut r)?.into());

        // votes_needed: u32
        let votes_needed = crate::util::read_u32_le(&mut r)?;

        // reason: CString
        let reason = {
            let reason = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(reason)?
        };

        Ok(Self {
            vote_in_progress,
            did_vote,
            agreed_with_kick,
            victim,
            total_votes,
            votes_agree,
            time_left,
            votes_needed,
            reason,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_BOOT_PROPOSAL_UPDATE {}

impl SMSG_LFG_BOOT_PROPOSAL_UPDATE {
    pub(crate) fn size(&self) -> usize {
        1 // vote_in_progress: Bool
        + 1 // did_vote: Bool
        + 1 // agreed_with_kick: Bool
        + 8 // victim: Guid
        + 4 // total_votes: u32
        + 4 // votes_agree: u32
        + 4 // time_left: Seconds
        + 4 // votes_needed: u32
        + self.reason.len() + 1 // reason: CString
    }
}

