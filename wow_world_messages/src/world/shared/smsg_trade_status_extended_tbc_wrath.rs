use std::io::{Read, Write};

use crate::shared::trade_slot_tbc_wrath::TradeSlot;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status_extended.wowm:67`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status_extended.wowm#L67):
/// ```text
/// smsg SMSG_TRADE_STATUS_EXTENDED = 0x0121 {
///     Bool self_player;
///     u32 trade_id;
///     u32 trade_slot_count1;
///     u32 trade_slot_count2;
///     Gold money_in_trade;
///     u32 spell_on_lowest_slot;
///     TradeSlot[7] trade_slots;
/// }
/// ```
pub struct SMSG_TRADE_STATUS_EXTENDED {
    /// cmangos/vmangos/mangoszero: send trader or own trade windows state (last need for proper show spell apply to non-trade slot)
    pub self_player: bool,
    /// added in 2.4.0, this value must be equal to value from TRADE_STATUS_OPEN_WINDOW status packet (different value for different players to block multiple trades?)
    pub trade_id: u32,
    /// cmangos/vmangos/mangoszero: sets to 7
    /// cmangos/vmangos/mangoszero: trade slots count/number?, = next field in most cases
    pub trade_slot_count1: u32,
    /// cmangos/vmangos/mangoszero: sets to 7
    /// cmangos/vmangos/mangoszero: trade slots count/number?, = prev field in most cases
    pub trade_slot_count2: u32,
    pub money_in_trade: Gold,
    pub spell_on_lowest_slot: u32,
    /// vmangos/cmangos/mangoszero: All set to same as trade_slot_count* (7), unsure which determines how big this is. Unused slots are 0.
    pub trade_slots: [TradeSlot; 7],
}

#[cfg(feature = "print-testcase")]
impl SMSG_TRADE_STATUS_EXTENDED {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TRADE_STATUS_EXTENDED {{").unwrap();
        // Members
        writeln!(s, "    self_player = {};", if self.self_player { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    trade_id = {};", self.trade_id).unwrap();
        writeln!(s, "    trade_slot_count1 = {};", self.trade_slot_count1).unwrap();
        writeln!(s, "    trade_slot_count2 = {};", self.trade_slot_count2).unwrap();
        writeln!(s, "    money_in_trade = {};", self.money_in_trade.as_int()).unwrap();
        writeln!(s, "    spell_on_lowest_slot = {};", self.spell_on_lowest_slot).unwrap();
        write!(s, "    trade_slots = [").unwrap();
        for v in self.trade_slots.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        trade_slot_number = {};", v.trade_slot_number).unwrap();
            writeln!(s, "        item = {};", v.item).unwrap();
            writeln!(s, "        display_id = {};", v.display_id).unwrap();
            writeln!(s, "        stack_count = {};", v.stack_count).unwrap();
            writeln!(s, "        wrapped = {};", if v.wrapped { "TRUE" } else { "FALSE" }).unwrap();
            writeln!(s, "        gift_wrapper = {};", v.gift_wrapper.guid()).unwrap();
            writeln!(s, "        enchantment = {};", v.enchantment).unwrap();
            write!(s, "        enchantments_slots = [").unwrap();
            for v in v.enchantments_slots.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "];").unwrap();
            writeln!(s, "        item_creator = {};", v.item_creator.guid()).unwrap();
            writeln!(s, "        spell_charges = {};", v.spell_charges).unwrap();
            writeln!(s, "        item_suffix_factor = {};", v.item_suffix_factor).unwrap();
            writeln!(s, "        item_random_properties_id = {};", v.item_random_properties_id).unwrap();
            writeln!(s, "        lock_id = {};", v.lock_id).unwrap();
            writeln!(s, "        max_durability = {};", v.max_durability).unwrap();
            writeln!(s, "        durability = {};", v.durability).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 534_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 289_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "self_player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "trade_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "trade_slot_count1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "trade_slot_count2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money_in_trade", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_on_lowest_slot", "    ");
        writeln!(s, "    /* trade_slots: TradeSlot[7] start */").unwrap();
        for (i, v) in self.trade_slots.iter().enumerate() {
            writeln!(s, "    /* trade_slots: TradeSlot[7] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 1, "trade_slot_number", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "display_id", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "stack_count", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "wrapped", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 8, "gift_wrapper", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "enchantment", "        ");
            writeln!(s, "    /* enchantments_slots: u32[3] start */").unwrap();
            for (i, v) in v.enchantments_slots.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("enchantments_slots {i}"), "        ");
            }
            writeln!(s, "    /* enchantments_slots: u32[3] end */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 8, "item_creator", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_charges", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item_suffix_factor", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_properties_id", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "lock_id", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "max_durability", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "durability", "        ");
            writeln!(s, "    /* trade_slots: TradeSlot[7] {i} end */").unwrap();
        }
        writeln!(s, "    /* trade_slots: TradeSlot[7] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_TRADE_STATUS_EXTENDED {}
impl crate::Message for SMSG_TRADE_STATUS_EXTENDED {
    const OPCODE: u32 = 0x0121;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_TRADE_STATUS_EXTENDED::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        532
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // self_player: Bool
        w.write_all(u8::from(self.self_player).to_le_bytes().as_slice())?;

        // trade_id: u32
        w.write_all(&self.trade_id.to_le_bytes())?;

        // trade_slot_count1: u32
        w.write_all(&self.trade_slot_count1.to_le_bytes())?;

        // trade_slot_count2: u32
        w.write_all(&self.trade_slot_count2.to_le_bytes())?;

        // money_in_trade: Gold
        w.write_all((self.money_in_trade.as_int()).to_le_bytes().as_slice())?;

        // spell_on_lowest_slot: u32
        w.write_all(&self.spell_on_lowest_slot.to_le_bytes())?;

        // trade_slots: TradeSlot[7]
        for i in self.trade_slots.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 532 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0121, size: body_size });
        }

        // self_player: Bool
        let self_player = crate::util::read_u8_le(&mut r)? != 0;

        // trade_id: u32
        let trade_id = crate::util::read_u32_le(&mut r)?;

        // trade_slot_count1: u32
        let trade_slot_count1 = crate::util::read_u32_le(&mut r)?;

        // trade_slot_count2: u32
        let trade_slot_count2 = crate::util::read_u32_le(&mut r)?;

        // money_in_trade: Gold
        let money_in_trade = Gold::new(crate::util::read_u32_le(&mut r)?);

        // spell_on_lowest_slot: u32
        let spell_on_lowest_slot = crate::util::read_u32_le(&mut r)?;

        // trade_slots: TradeSlot[7]
        let trade_slots = {
            let mut trade_slots = [TradeSlot::default(); 7];
            for i in trade_slots.iter_mut() {
                *i = TradeSlot::read(&mut r)?;
            }
            trade_slots
        };

        Ok(Self {
            self_player,
            trade_id,
            trade_slot_count1,
            trade_slot_count2,
            money_in_trade,
            spell_on_lowest_slot,
            trade_slots,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TRADE_STATUS_EXTENDED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TRADE_STATUS_EXTENDED {}

