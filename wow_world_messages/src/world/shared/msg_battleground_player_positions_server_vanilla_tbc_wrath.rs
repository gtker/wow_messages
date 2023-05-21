use std::io::{Read, Write};

use crate::Guid;
use crate::shared::battleground_player_position_vanilla_tbc_wrath::BattlegroundPlayerPosition;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/msg_battleground_player_positions.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/msg_battleground_player_positions.wowm#L13):
/// ```text
/// smsg MSG_BATTLEGROUND_PLAYER_POSITIONS_Server = 0x02E9 {
///     u32 amount_of_teammates;
///     BattlegroundPlayerPosition[amount_of_teammates] teammates;
///     u8 amount_of_carriers;
///     BattlegroundPlayerPosition[amount_of_carriers] carriers;
/// }
/// ```
pub struct MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    pub teammates: Vec<BattlegroundPlayerPosition>,
    pub carriers: Vec<BattlegroundPlayerPosition>,
}

impl crate::private::Sealed for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {}
impl MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x02E9, size: body_size });
        }

        // amount_of_teammates: u32
        let amount_of_teammates = crate::util::read_u32_le(&mut r)?;

        // teammates: BattlegroundPlayerPosition[amount_of_teammates]
        let teammates = {
            let mut teammates = Vec::with_capacity(amount_of_teammates as usize);
            for _ in 0..amount_of_teammates {
                teammates.push(BattlegroundPlayerPosition::read(&mut r)?);
            }
            teammates
        };

        // amount_of_carriers: u8
        let amount_of_carriers = crate::util::read_u8_le(&mut r)?;

        // carriers: BattlegroundPlayerPosition[amount_of_carriers]
        let carriers = {
            let mut carriers = Vec::with_capacity(amount_of_carriers as usize);
            for _ in 0..amount_of_carriers {
                carriers.push(BattlegroundPlayerPosition::read(&mut r)?);
            }
            carriers
        };

        Ok(Self {
            teammates,
            carriers,
        })
    }

}

impl crate::Message for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    const OPCODE: u32 = 0x02e9;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {{").unwrap();
        // Members
        writeln!(s, "    amount_of_teammates = {};", self.teammates.len()).unwrap();
        write!(s, "    teammates = [").unwrap();
        for v in self.teammates.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        player = {};", v.player.guid()).unwrap();
            writeln!(s, "    {}", if v.position_x.to_string().contains('.') { v.position_x.to_string() } else { format!("{}.0", v.position_x) }).unwrap();
            writeln!(s, "    {}", if v.position_y.to_string().contains('.') { v.position_y.to_string() } else { format!("{}.0", v.position_y) }).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    amount_of_carriers = {};", self.carriers.len()).unwrap();
        write!(s, "    carriers = [").unwrap();
        for v in self.carriers.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        player = {};", v.player.guid()).unwrap();
            writeln!(s, "    {}", if v.position_x.to_string().contains('.') { v.position_x.to_string() } else { format!("{}.0", v.position_x) }).unwrap();
            writeln!(s, "    {}", if v.position_y.to_string().contains('.') { v.position_y.to_string() } else { format!("{}.0", v.position_y) }).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 745_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_teammates", "    ");
        if !self.teammates.is_empty() {
            writeln!(s, "    /* teammates: BattlegroundPlayerPosition[amount_of_teammates] start */").unwrap();
            for (i, v) in self.teammates.iter().enumerate() {
                writeln!(s, "    /* teammates: BattlegroundPlayerPosition[amount_of_teammates] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "position_x", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "position_y", "        ");
                writeln!(s, "    /* teammates: BattlegroundPlayerPosition[amount_of_teammates] {i} end */").unwrap();
            }
            writeln!(s, "    /* teammates: BattlegroundPlayerPosition[amount_of_teammates] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_carriers", "    ");
        if !self.carriers.is_empty() {
            writeln!(s, "    /* carriers: BattlegroundPlayerPosition[amount_of_carriers] start */").unwrap();
            for (i, v) in self.carriers.iter().enumerate() {
                writeln!(s, "    /* carriers: BattlegroundPlayerPosition[amount_of_carriers] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "position_x", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "position_y", "        ");
                writeln!(s, "    /* carriers: BattlegroundPlayerPosition[amount_of_carriers] {i} end */").unwrap();
            }
            writeln!(s, "    /* carriers: BattlegroundPlayerPosition[amount_of_carriers] end */").unwrap();
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
        // amount_of_teammates: u32
        w.write_all(&(self.teammates.len() as u32).to_le_bytes())?;

        // teammates: BattlegroundPlayerPosition[amount_of_teammates]
        for i in self.teammates.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_carriers: u8
        w.write_all(&(self.carriers.len() as u8).to_le_bytes())?;

        // carriers: BattlegroundPlayerPosition[amount_of_carriers]
        for i in self.carriers.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {}

impl MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_teammates: u32
        + self.teammates.len() * 16 // teammates: BattlegroundPlayerPosition[amount_of_teammates]
        + 1 // amount_of_carriers: u8
        + self.carriers.len() * 16 // carriers: BattlegroundPlayerPosition[amount_of_carriers]
    }
}

