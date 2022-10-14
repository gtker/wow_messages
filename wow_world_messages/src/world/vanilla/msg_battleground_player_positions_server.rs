use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::BattlegroundPlayerPosition;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// vmangos/cmangos/mangoszero: Seems to be older versions used to be `amount_of_carriers` followed by array. All versions now just set first to 0 and have second be 0/1/2.
/// vmangos/cmangos/mangoszero: For AB and AV always set to all zero.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/msg_battleground_player_positions.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/msg_battleground_player_positions.wowm#L14):
/// ```text
/// smsg MSG_BATTLEGROUND_PLAYER_POSITIONS_Server = 0x02E9 {
///     u32 amount_of_carriers = 0;
///     u32 amount_of_flag_carriers;
///     BattlegroundPlayerPosition[amount_of_flag_carriers] flag_carriers;
/// }
/// ```
pub struct MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    pub flag_carriers: Vec<BattlegroundPlayerPosition>,
}

impl MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    /// The field `amount_of_carriers` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const AMOUNT_OF_CARRIERS_VALUE: u32 = 0x00;

}

impl crate::Message for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    const OPCODE: u32 = 0x02e9;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_carriers: u32
        w.write_all(&Self::AMOUNT_OF_CARRIERS_VALUE.to_le_bytes())?;

        // amount_of_flag_carriers: u32
        w.write_all(&(self.flag_carriers.len() as u32).to_le_bytes())?;

        // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
        for i in self.flag_carriers.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // amount_of_carriers: u32
        let _amount_of_carriers = crate::util::read_u32_le(r)?;
        // amount_of_carriers is expected to always be 0 (0)

        // amount_of_flag_carriers: u32
        let amount_of_flag_carriers = crate::util::read_u32_le(r)?;

        // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
        let mut flag_carriers = Vec::with_capacity(amount_of_flag_carriers as usize);
        for i in 0..amount_of_flag_carriers {
            flag_carriers.push(BattlegroundPlayerPosition::read(r)?);
        }

        Ok(Self {
            flag_carriers,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {}

impl MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_carriers: u32
        + 4 // amount_of_flag_carriers: u32
        + self.flag_carriers.len() * 16 // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
    }
}

