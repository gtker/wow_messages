use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_enchant_time_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_enchant_time_update.wowm#L3):
/// ```text
/// smsg SMSG_ITEM_ENCHANT_TIME_UPDATE = 0x01EB {
///     Guid item;
///     u32 slot;
///     u32 duration;
///     Guid player;
/// }
/// ```
pub struct SMSG_ITEM_ENCHANT_TIME_UPDATE {
    pub item: Guid,
    /// Possibly used with EnchantmentSlot enum.
    pub slot: u32,
    pub duration: u32,
    pub player: Guid,
}

impl crate::private::Sealed for SMSG_ITEM_ENCHANT_TIME_UPDATE {}
impl crate::Message for SMSG_ITEM_ENCHANT_TIME_UPDATE {
    const OPCODE: u32 = 0x01eb;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ITEM_ENCHANT_TIME_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    slot = {};", self.slot).unwrap();
        writeln!(s, "    duration = {};", self.duration).unwrap();
        writeln!(s, "    player = {};", self.player.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 26_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 491_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // slot: u32
        w.write_all(&self.slot.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01EB, size: body_size });
        }

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // slot: u32
        let slot = crate::util::read_u32_le(&mut r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        Ok(Self {
            item,
            slot,
            duration,
            player,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ITEM_ENCHANT_TIME_UPDATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ITEM_ENCHANT_TIME_UPDATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ITEM_ENCHANT_TIME_UPDATE {}

