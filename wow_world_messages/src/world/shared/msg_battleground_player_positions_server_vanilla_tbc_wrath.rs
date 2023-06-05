use std::io::{Read, Write};

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

#[cfg(feature = "print-testcase")]
impl MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    pub fn to_test_case_string(&self) -> String {
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
            writeln!(s, "    player = {};", v.player.guid()).unwrap();
            writeln!(s, "    {}", if v.position_x.to_string().contains(".") { v.position_x.to_string() } else { format!("{}.0", v.position_x) }).unwrap();
            writeln!(s, "    {}", if v.position_y.to_string().contains(".") { v.position_y.to_string() } else { format!("{}.0", v.position_y) }).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    amount_of_carriers = {};", self.carriers.len()).unwrap();
        write!(s, "    carriers = [").unwrap();
        for v in self.carriers.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    player = {};", v.player.guid()).unwrap();
            writeln!(s, "    {}", if v.position_x.to_string().contains(".") { v.position_x.to_string() } else { format!("{}.0", v.position_x) }).unwrap();
            writeln!(s, "    {}", if v.position_y.to_string().contains(".") { v.position_y.to_string() } else { format!("{}.0", v.position_y) }).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 745_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_teammates");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {}
impl crate::Message for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    const OPCODE: u32 = 0x02e9;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(5..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02E9, size: body_size });
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

