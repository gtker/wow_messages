use std::io::{Read, Write};

use crate::vanilla::{
    Faction, FactionStanding,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm#L9):
/// ```text
/// smsg SMSG_SET_FACTION_STANDING = 0x0124 {
///     u32 amount_of_faction_standings;
///     FactionStanding[amount_of_faction_standings] faction_standings;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SET_FACTION_STANDING {
    pub faction_standings: Vec<FactionStanding>,
}

impl crate::private::Sealed for SMSG_SET_FACTION_STANDING {}
impl SMSG_SET_FACTION_STANDING {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_faction_standings: u32
        let amount_of_faction_standings = crate::util::read_u32_le(&mut r)?;

        // faction_standings: FactionStanding[amount_of_faction_standings]
        let faction_standings = {
            let mut faction_standings = Vec::with_capacity(amount_of_faction_standings as usize);

            let allocation_size = u64::from(amount_of_faction_standings) * 6;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_faction_standings {
                faction_standings.push(FactionStanding::read(&mut r)?);
            }
            faction_standings
        };

        Ok(Self {
            faction_standings,
        })
    }

}

impl crate::Message for SMSG_SET_FACTION_STANDING {
    const OPCODE: u32 = 0x0124;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SET_FACTION_STANDING"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SET_FACTION_STANDING {{").unwrap();
        // Members
        writeln!(s, "    amount_of_faction_standings = {};", self.faction_standings.len()).unwrap();
        writeln!(s, "    faction_standings = [").unwrap();
        for v in self.faction_standings.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            faction = {};", v.faction.as_test_case_value()).unwrap();
            writeln!(s, "            standing = {};", v.standing).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 292_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_faction_standings", "    ");
        if !self.faction_standings.is_empty() {
            writeln!(s, "    /* faction_standings: FactionStanding[amount_of_faction_standings] start */").unwrap();
            for (i, v) in self.faction_standings.iter().enumerate() {
                writeln!(s, "    /* faction_standings: FactionStanding[amount_of_faction_standings] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 2, "faction", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "standing", "        ");
                writeln!(s, "    /* faction_standings: FactionStanding[amount_of_faction_standings] {i} end */").unwrap();
            }
            writeln!(s, "    /* faction_standings: FactionStanding[amount_of_faction_standings] end */").unwrap();
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
        // amount_of_faction_standings: u32
        w.write_all(&(self.faction_standings.len() as u32).to_le_bytes())?;

        // faction_standings: FactionStanding[amount_of_faction_standings]
        for i in self.faction_standings.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(292, "SMSG_SET_FACTION_STANDING", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SET_FACTION_STANDING {}

impl SMSG_SET_FACTION_STANDING {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_faction_standings: u32
        + self.faction_standings.len() * 6 // faction_standings: FactionStanding[amount_of_faction_standings]
    }
}

