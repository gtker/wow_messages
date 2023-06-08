use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_master_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_master_list.wowm#L3):
/// ```text
/// smsg SMSG_LOOT_MASTER_LIST = 0x02A4 {
///     u8 amount_of_players;
///     Guid[amount_of_players] guids;
/// }
/// ```
pub struct SMSG_LOOT_MASTER_LIST {
    pub guids: Vec<Guid>,
}

impl crate::private::Sealed for SMSG_LOOT_MASTER_LIST {}
impl crate::Message for SMSG_LOOT_MASTER_LIST {
    const OPCODE: u32 = 0x02a4;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOOT_MASTER_LIST {{").unwrap();
        // Members
        writeln!(s, "    amount_of_players = {};", self.guids.len()).unwrap();
        write!(s, "    guids = [").unwrap();
        for v in self.guids.as_slice() {
            write!(s, "{v:#08X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 676_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_players", "    ");
        if !self.guids.is_empty() {
            writeln!(s, "    /* guids: Guid[amount_of_players] start */").unwrap();
            for (i, v) in self.guids.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 8, &format!("guids {i}"), "    ");
            }
            writeln!(s, "    /* guids: Guid[amount_of_players] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_players: u8
        w.write_all(&(self.guids.len() as u8).to_le_bytes())?;

        // guids: Guid[amount_of_players]
        for i in self.guids.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=2049).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A4, size: body_size });
        }

        // amount_of_players: u8
        let amount_of_players = crate::util::read_u8_le(&mut r)?;

        // guids: Guid[amount_of_players]
        let guids = {
            let mut guids = Vec::with_capacity(amount_of_players as usize);
            for _ in 0..amount_of_players {
                guids.push(crate::util::read_guid(&mut r)?);
            }
            guids
        };

        Ok(Self {
            guids,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_MASTER_LIST {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_MASTER_LIST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_MASTER_LIST {}

impl SMSG_LOOT_MASTER_LIST {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_players: u8
        + self.guids.len() *  8 // guids: Guid[amount_of_players]
    }
}

