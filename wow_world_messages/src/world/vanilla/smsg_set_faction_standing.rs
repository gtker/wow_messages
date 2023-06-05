use std::io::{Read, Write};

use crate::vanilla::FactionStanding;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm#L9):
/// ```text
/// smsg SMSG_SET_FACTION_STANDING = 0x0124 {
///     u32 amount_of_faction_standings;
///     FactionStanding[amount_of_faction_standings] faction_standings;
/// }
/// ```
pub struct SMSG_SET_FACTION_STANDING {
    pub faction_standings: Vec<FactionStanding>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_SET_FACTION_STANDING {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SET_FACTION_STANDING {{").unwrap();
        // Members
        writeln!(s, "    amount_of_faction_standings = {};", self.faction_standings.len()).unwrap();
        write!(s, "    faction_standings = [").unwrap();
        for v in self.faction_standings.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    faction = {};", v.faction.as_test_case_value()).unwrap();
            writeln!(s, "    standing = {};", v.standing).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 292_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_faction_standings");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_SET_FACTION_STANDING {}
impl crate::Message for SMSG_SET_FACTION_STANDING {
    const OPCODE: u32 = 0x0124;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0124, size: body_size });
        }

        // amount_of_faction_standings: u32
        let amount_of_faction_standings = crate::util::read_u32_le(&mut r)?;

        // faction_standings: FactionStanding[amount_of_faction_standings]
        let faction_standings = {
            let mut faction_standings = Vec::with_capacity(amount_of_faction_standings as usize);
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
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SET_FACTION_STANDING {}

impl SMSG_SET_FACTION_STANDING {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_faction_standings: u32
        + self.faction_standings.len() * 6 // faction_standings: FactionStanding[amount_of_faction_standings]
    }
}

