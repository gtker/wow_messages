use std::io::{Read, Write};

use crate::Guid;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm#L1):
/// ```text
/// smsg SMSG_LOOT_START_ROLL = 0x02A1 {
///     Guid creature;
///     u32 loot_slot;
///     Item item;
///     u32 item_random_suffix;
///     u32 item_random_property_id;
///     Milliseconds countdown_time;
/// }
/// ```
pub struct SMSG_LOOT_START_ROLL {
    pub creature: Guid,
    pub loot_slot: u32,
    pub item: u32,
    /// vmangos/mangoszero: not used ?
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub countdown_time: Duration,
}

impl crate::private::Sealed for SMSG_LOOT_START_ROLL {}
impl SMSG_LOOT_START_ROLL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 28 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // creature: Guid
        let creature = crate::util::read_guid(&mut r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(&mut r)?;

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        // countdown_time: Milliseconds
        let countdown_time = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            creature,
            loot_slot,
            item,
            item_random_suffix,
            item_random_property_id,
            countdown_time,
        })
    }

}

impl crate::Message for SMSG_LOOT_START_ROLL {
    const OPCODE: u32 = 0x02a1;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LOOT_START_ROLL"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOOT_START_ROLL {{").unwrap();
        // Members
        writeln!(s, "    creature = {};", self.creature.guid()).unwrap();
        writeln!(s, "    loot_slot = {};", self.loot_slot).unwrap();
        writeln!(s, "    item = {};", self.item).unwrap();
        writeln!(s, "    item_random_suffix = {};", self.item_random_suffix).unwrap();
        writeln!(s, "    item_random_property_id = {};", self.item_random_property_id).unwrap();
        writeln!(s, "    countdown_time = {};", self.countdown_time.as_millis()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 30_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 673_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "creature", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "loot_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_suffix", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_property_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "countdown_time", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        28
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // creature: Guid
        w.write_all(&self.creature.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // countdown_time: Milliseconds
        w.write_all((self.countdown_time.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(673, "SMSG_LOOT_START_ROLL", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_START_ROLL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_START_ROLL {}

