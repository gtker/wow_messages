use std::io::{Read, Write};

use crate::wrath::{
    Faction, ForcedReaction,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm#L8):
/// ```text
/// smsg SMSG_SET_FORCED_REACTIONS = 0x02A5 {
///     u32 amount_of_reactions;
///     ForcedReaction[amount_of_reactions] reactions;
/// }
/// ```
pub struct SMSG_SET_FORCED_REACTIONS {
    pub reactions: Vec<ForcedReaction>,
}

impl crate::private::Sealed for SMSG_SET_FORCED_REACTIONS {}
impl SMSG_SET_FORCED_REACTIONS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_reactions: u32
        let amount_of_reactions = crate::util::read_u32_le(&mut r)?;

        // reactions: ForcedReaction[amount_of_reactions]
        let reactions = {
            let mut reactions = Vec::with_capacity(amount_of_reactions as usize);
            for _ in 0..amount_of_reactions {
                reactions.push(ForcedReaction::read(&mut r)?);
            }
            reactions
        };

        Ok(Self {
            reactions,
        })
    }

}

impl crate::Message for SMSG_SET_FORCED_REACTIONS {
    const OPCODE: u32 = 0x02a5;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SET_FORCED_REACTIONS {{").unwrap();
        // Members
        writeln!(s, "    amount_of_reactions = {};", self.reactions.len()).unwrap();
        write!(s, "    reactions = [").unwrap();
        for v in self.reactions.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        faction = {};", v.faction.as_test_case_value()).unwrap();
            writeln!(s, "        reputation_rank = {};", v.reputation_rank).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 677_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_reactions", "    ");
        if !self.reactions.is_empty() {
            writeln!(s, "    /* reactions: ForcedReaction[amount_of_reactions] start */").unwrap();
            for (i, v) in self.reactions.iter().enumerate() {
                writeln!(s, "    /* reactions: ForcedReaction[amount_of_reactions] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 2, "faction", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "reputation_rank", "        ");
                writeln!(s, "    /* reactions: ForcedReaction[amount_of_reactions] {i} end */").unwrap();
            }
            writeln!(s, "    /* reactions: ForcedReaction[amount_of_reactions] end */").unwrap();
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
        // amount_of_reactions: u32
        w.write_all(&(self.reactions.len() as u32).to_le_bytes())?;

        // reactions: ForcedReaction[amount_of_reactions]
        for i in self.reactions.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(677, "SMSG_SET_FORCED_REACTIONS", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SET_FORCED_REACTIONS {}

impl SMSG_SET_FORCED_REACTIONS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_reactions: u32
        + self.reactions.len() * 6 // reactions: ForcedReaction[amount_of_reactions]
    }
}

