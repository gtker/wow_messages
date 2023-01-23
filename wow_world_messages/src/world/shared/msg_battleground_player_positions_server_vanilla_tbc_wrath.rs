use std::convert::{TryFrom, TryInto};
use crate::world::shared::battleground_player_position_vanilla_tbc_wrath::BattlegroundPlayerPosition;
use std::io::{Write, Read};

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

impl crate::Message for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    const OPCODE: u32 = 0x02e9;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_teammates: u32
        w.write_all(&(self.teammates.len() as u32).to_le_bytes())?;

        // teammates: BattlegroundPlayerPosition[amount_of_teammates]
        for i in self.teammates.iter() {
            i.write_into_vec(w)?;
        }

        // amount_of_carriers: u8
        w.write_all(&(self.carriers.len() as u8).to_le_bytes())?;

        // carriers: BattlegroundPlayerPosition[amount_of_carriers]
        for i in self.carriers.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02E9, size: body_size as u32 });
        }

        // amount_of_teammates: u32
        let amount_of_teammates = crate::util::read_u32_le(r)?;

        // teammates: BattlegroundPlayerPosition[amount_of_teammates]
        let mut teammates = Vec::with_capacity(amount_of_teammates as usize);
        for i in 0..amount_of_teammates {
            teammates.push(BattlegroundPlayerPosition::read(r)?);
        }

        // amount_of_carriers: u8
        let amount_of_carriers = crate::util::read_u8_le(r)?;

        // carriers: BattlegroundPlayerPosition[amount_of_carriers]
        let mut carriers = Vec::with_capacity(amount_of_carriers as usize);
        for i in 0..amount_of_carriers {
            carriers.push(BattlegroundPlayerPosition::read(r)?);
        }

        Ok(Self {
            teammates,
            carriers,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {}

impl MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_teammates: u32
        + self.teammates.len() * 16 // teammates: BattlegroundPlayerPosition[amount_of_teammates]
        + 1 // amount_of_carriers: u8
        + self.carriers.len() * 16 // carriers: BattlegroundPlayerPosition[amount_of_carriers]
    }
}

