use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::BattlegroundPlayerPosition;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/msg_battleground_player_positions_server.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/msg_battleground_player_positions_server.wowm#L9):
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
    /// **This field is not in the struct, but is written as this constant value.**
    pub const AMOUNT_OF_CARRIERS_VALUE: u32 = 0x00;

}

impl ServerMessage for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
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
    const OPCODE: u16 = 0x02e9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
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

impl MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_carriers: u32
        + 4 // amount_of_flag_carriers: u32
        + self.flag_carriers.len() * 16 // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
    }
}

