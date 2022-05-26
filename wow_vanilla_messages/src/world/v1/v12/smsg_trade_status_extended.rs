use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::TradeSlot;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TRADE_STATUS_EXTENDED {
    pub self_player: u8,
    pub trade_slot_count1: u32,
    pub trade_slot_count2: u32,
    pub money_in_trade: u32,
    pub spell_on_lowest_slot: u32,
    pub trade_slots: [TradeSlot; 7],
}

impl SMSG_TRADE_STATUS_EXTENDED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 444], std::io::Error> {
        let mut array_w = [0u8; 444];
        let mut w = array_w.as_mut_slice();
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
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_TRADE_STATUS_EXTENDED {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(444);
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
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
    const OPCODE: u16 = 0x0121;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        444
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
        for i in 0..7 {
            trade_slots[i] = TradeSlot::read(r)?;
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

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // self_player: u8
            let self_player = crate::util::tokio_read_u8_le(r).await?;

            // trade_slot_count1: u32
            let trade_slot_count1 = crate::util::tokio_read_u32_le(r).await?;

            // trade_slot_count2: u32
            let trade_slot_count2 = crate::util::tokio_read_u32_le(r).await?;

            // money_in_trade: u32
            let money_in_trade = crate::util::tokio_read_u32_le(r).await?;

            // spell_on_lowest_slot: u32
            let spell_on_lowest_slot = crate::util::tokio_read_u32_le(r).await?;

            // trade_slots: TradeSlot[7]
            let mut trade_slots = [TradeSlot::default(); 7];
            for i in 0..7 {
                trade_slots[i] = TradeSlot::tokio_read(r).await?;
            }

            Ok(Self {
                self_player,
                trade_slot_count1,
                trade_slot_count2,
                money_in_trade,
                spell_on_lowest_slot,
                trade_slots,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // self_player: u8
            let self_player = crate::util::astd_read_u8_le(r).await?;

            // trade_slot_count1: u32
            let trade_slot_count1 = crate::util::astd_read_u32_le(r).await?;

            // trade_slot_count2: u32
            let trade_slot_count2 = crate::util::astd_read_u32_le(r).await?;

            // money_in_trade: u32
            let money_in_trade = crate::util::astd_read_u32_le(r).await?;

            // spell_on_lowest_slot: u32
            let spell_on_lowest_slot = crate::util::astd_read_u32_le(r).await?;

            // trade_slots: TradeSlot[7]
            let mut trade_slots = [TradeSlot::default(); 7];
            for i in 0..7 {
                trade_slots[i] = TradeSlot::astd_read(r).await?;
            }

            Ok(Self {
                self_player,
                trade_slot_count1,
                trade_slot_count2,
                money_in_trade,
                spell_on_lowest_slot,
                trade_slots,
            })
        })
    }

}

