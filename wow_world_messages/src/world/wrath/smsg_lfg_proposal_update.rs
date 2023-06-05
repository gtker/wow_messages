use std::io::{Read, Write};

use crate::wrath::LfgProposal;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_proposal_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_proposal_update.wowm#L1):
/// ```text
/// smsg SMSG_LFG_PROPOSAL_UPDATE = 0x0361 {
///     u32 dungeon_id;
///     u8 proposal_state;
///     u32 proposal_id;
///     u32 encounters_finished_mask;
///     u8 silent;
///     u8 amount_of_proposals;
///     LfgProposal[amount_of_proposals] proposals;
/// }
/// ```
pub struct SMSG_LFG_PROPOSAL_UPDATE {
    pub dungeon_id: u32,
    pub proposal_state: u8,
    pub proposal_id: u32,
    pub encounters_finished_mask: u32,
    pub silent: u8,
    pub proposals: Vec<LfgProposal>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_LFG_PROPOSAL_UPDATE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_PROPOSAL_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    dungeon_id = {};", self.dungeon_id).unwrap();
        writeln!(s, "    proposal_state = {};", self.proposal_state).unwrap();
        writeln!(s, "    proposal_id = {};", self.proposal_id).unwrap();
        writeln!(s, "    encounters_finished_mask = {};", self.encounters_finished_mask).unwrap();
        writeln!(s, "    silent = {};", self.silent).unwrap();
        writeln!(s, "    amount_of_proposals = {};", self.proposals.len()).unwrap();
        write!(s, "    proposals = [").unwrap();
        for v in self.proposals.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        role_mask = {};", v.role_mask).unwrap();
            writeln!(s, "        is_current_player = {};", v.is_current_player).unwrap();
            writeln!(s, "        in_dungeon = {};", v.in_dungeon).unwrap();
            writeln!(s, "        in_same_group = {};", v.in_same_group).unwrap();
            writeln!(s, "        has_answered = {};", v.has_answered).unwrap();
            writeln!(s, "        has_accepted = {};", v.has_accepted).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 865_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "proposal_state", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "proposal_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "encounters_finished_mask", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "silent", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_proposals", "    ");
        if !self.proposals.is_empty() {
            writeln!(s, "    /* proposals: LfgProposal[amount_of_proposals] start */").unwrap();
            for (i, v) in self.proposals.iter().enumerate() {
                writeln!(s, "    /* proposals: LfgProposal[amount_of_proposals] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "role_mask", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "is_current_player", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "in_dungeon", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "in_same_group", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "has_answered", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "has_accepted", "        ");
                writeln!(s, "    /* proposals: LfgProposal[amount_of_proposals] {i} end */").unwrap();
            }
            writeln!(s, "    /* proposals: LfgProposal[amount_of_proposals] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_LFG_PROPOSAL_UPDATE {}
impl crate::Message for SMSG_LFG_PROPOSAL_UPDATE {
    const OPCODE: u32 = 0x0361;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_LFG_PROPOSAL_UPDATE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        // proposal_state: u8
        w.write_all(&self.proposal_state.to_le_bytes())?;

        // proposal_id: u32
        w.write_all(&self.proposal_id.to_le_bytes())?;

        // encounters_finished_mask: u32
        w.write_all(&self.encounters_finished_mask.to_le_bytes())?;

        // silent: u8
        w.write_all(&self.silent.to_le_bytes())?;

        // amount_of_proposals: u8
        w.write_all(&(self.proposals.len() as u8).to_le_bytes())?;

        // proposals: LfgProposal[amount_of_proposals]
        for i in self.proposals.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(15..=2319).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0361, size: body_size });
        }

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(&mut r)?;

        // proposal_state: u8
        let proposal_state = crate::util::read_u8_le(&mut r)?;

        // proposal_id: u32
        let proposal_id = crate::util::read_u32_le(&mut r)?;

        // encounters_finished_mask: u32
        let encounters_finished_mask = crate::util::read_u32_le(&mut r)?;

        // silent: u8
        let silent = crate::util::read_u8_le(&mut r)?;

        // amount_of_proposals: u8
        let amount_of_proposals = crate::util::read_u8_le(&mut r)?;

        // proposals: LfgProposal[amount_of_proposals]
        let proposals = {
            let mut proposals = Vec::with_capacity(amount_of_proposals as usize);
            for _ in 0..amount_of_proposals {
                proposals.push(LfgProposal::read(&mut r)?);
            }
            proposals
        };

        Ok(Self {
            dungeon_id,
            proposal_state,
            proposal_id,
            encounters_finished_mask,
            silent,
            proposals,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_PROPOSAL_UPDATE {}

impl SMSG_LFG_PROPOSAL_UPDATE {
    pub(crate) fn size(&self) -> usize {
        4 // dungeon_id: u32
        + 1 // proposal_state: u8
        + 4 // proposal_id: u32
        + 4 // encounters_finished_mask: u32
        + 1 // silent: u8
        + 1 // amount_of_proposals: u8
        + self.proposals.len() * 9 // proposals: LfgProposal[amount_of_proposals]
    }
}

