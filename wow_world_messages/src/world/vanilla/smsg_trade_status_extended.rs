use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::TradeSlot;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status_extended.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status_extended.wowm#L22):
/// ```text
/// smsg SMSG_TRADE_STATUS_EXTENDED = 0x0121 {
///     u8 self_player;
///     u32 trade_slot_count1;
///     u32 trade_slot_count2;
///     u32 money_in_trade;
///     u32 spell_on_lowest_slot;
///     TradeSlot[7] trade_slots;
/// }
/// ```
pub struct SMSG_TRADE_STATUS_EXTENDED {
    /// cmangos/vmangos/mangoszero: send trader or own trade windows state (last need for proper show spell apply to non-trade slot)
    ///
    pub self_player: u8,
    /// cmangos/vmangos/mangoszero: sets to 7
    /// cmangos/vmangos/mangoszero: trade slots count/number?, = next field in most cases
    ///
    pub trade_slot_count1: u32,
    /// cmangos/vmangos/mangoszero: sets to 7
    /// cmangos/vmangos/mangoszero: trade slots count/number?, = prev field in most cases
    ///
    pub trade_slot_count2: u32,
    pub money_in_trade: u32,
    pub spell_on_lowest_slot: u32,
    /// vmangos/cmangos/mangoszero: All set to same as trade_slot_count* (7), unsure which determines how big this is. Unused slots are 0.
    ///
    pub trade_slots: [TradeSlot; 7],
}

impl crate::Message for SMSG_TRADE_STATUS_EXTENDED {
    const OPCODE: u32 = 0x0121;

    fn size_without_header(&self) -> u32 {
        444
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // self_player: u8
        w.write_all(&self.self_player.to_le_bytes())?;

        // trade_slot_count1: u32
        w.write_all(&self.trade_slot_count1.to_le_bytes())?;

        // trade_slot_count2: u32
        w.write_all(&self.trade_slot_count2.to_le_bytes())?;

        // money_in_trade: u32
        w.write_all(&self.money_in_trade.to_le_bytes())?;

        // spell_on_lowest_slot: u32
        w.write_all(&self.spell_on_lowest_slot.to_le_bytes())?;

        // trade_slots: TradeSlot[7]
        for i in self.trade_slots.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 444 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // self_player: u8
        let self_player = crate::util::read_u8_le(r)?;

        // trade_slot_count1: u32
        let trade_slot_count1 = crate::util::read_u32_le(r)?;

        // trade_slot_count2: u32
        let trade_slot_count2 = crate::util::read_u32_le(r)?;

        // money_in_trade: u32
        let money_in_trade = crate::util::read_u32_le(r)?;

        // spell_on_lowest_slot: u32
        let spell_on_lowest_slot = crate::util::read_u32_le(r)?;

        // trade_slots: TradeSlot[7]
        let mut trade_slots = [TradeSlot::default(); 7];
        for i in trade_slots.iter_mut() {
            *i = TradeSlot::read(r)?;
        }

        Ok(Self {
            self_player,
            trade_slot_count1,
            trade_slot_count2,
            money_in_trade,
            spell_on_lowest_slot,
            trade_slots,
        })
    }

}
impl ServerMessage for SMSG_TRADE_STATUS_EXTENDED {}

