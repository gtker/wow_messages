use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::BattlegroundPlayerPosition;
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:1060`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L1060):
/// ```text
/// smsg MSG_BATTLEGROUND_PLAYER_POSITIONS_Server = 0x2E9 {
///     u32 amount_of_carriers = 0;
///     u32 amount_of_flag_carriers;
///     BattlegroundPlayerPosition[amount_of_flag_carriers] flag_carriers;
/// }
/// ```
pub struct MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    pub amount_of_flag_carriers: u32,
    pub flag_carriers: Vec<BattlegroundPlayerPosition>,
}

impl WorldServerMessageWrite for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    const OPCODE: u16 = 0x2e9;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
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

impl WorldMessageBody for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
            amount_of_flag_carriers,
            flag_carriers,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_carriers: u32
        w.write_all(&Self::AMOUNT_OF_CARRIERS_VALUE.to_le_bytes())?;

        // amount_of_flag_carriers: u32
        w.write_all(&(self.flag_carriers.len() as u32).to_le_bytes())?;

        // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
        for i in self.flag_carriers.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    fn size(&self) -> usize {
        4 // amount_of_carriers: u32
        + 4 // amount_of_flag_carriers: u32
        + self.flag_carriers.iter().fold(0, |acc, x| acc + BattlegroundPlayerPosition::size()) // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
    }
}

impl MaximumPossibleSized for MSG_BATTLEGROUND_PLAYER_POSITIONS_Server {
    fn maximum_possible_size() -> usize {
        4 // amount_of_carriers: u32
        + 4 // amount_of_flag_carriers: u32
        + 4294967295 * BattlegroundPlayerPosition::maximum_possible_size() // flag_carriers: BattlegroundPlayerPosition[amount_of_flag_carriers]
    }
}

