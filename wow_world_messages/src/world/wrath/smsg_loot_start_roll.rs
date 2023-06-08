use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    Map, RollFlags,
};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_start_roll.wowm#L23):
/// ```text
/// smsg SMSG_LOOT_START_ROLL = 0x02A1 {
///     Guid creature;
///     Map map;
///     u32 loot_slot;
///     u32 item;
///     u32 item_random_suffix;
///     u32 item_random_property_id;
///     Milliseconds countdown_time;
///     RollFlags flags;
/// }
/// ```
pub struct SMSG_LOOT_START_ROLL {
    pub creature: Guid,
    pub map: Map,
    pub loot_slot: u32,
    pub item: u32,
    /// vmangos/mangoszero: not used ?
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub countdown_time: Duration,
    pub flags: RollFlags,
}

impl crate::private::Sealed for SMSG_LOOT_START_ROLL {}
impl crate::Message for SMSG_LOOT_START_ROLL {
    const OPCODE: u32 = 0x02a1;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOOT_START_ROLL {{").unwrap();
        // Members
        writeln!(s, "    creature = {};", self.creature.guid()).unwrap();
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    loot_slot = {};", self.loot_slot).unwrap();
        writeln!(s, "    item = {};", self.item).unwrap();
        writeln!(s, "    item_random_suffix = {};", self.item_random_suffix).unwrap();
        writeln!(s, "    item_random_property_id = {};", self.item_random_property_id).unwrap();
        writeln!(s, "    countdown_time = {};", self.countdown_time.as_millis()).unwrap();
        writeln!(s, "    flags = {};", self.flags.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 35_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 673_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "creature", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "loot_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_suffix", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_property_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "countdown_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "flags", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        33
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // creature: Guid
        w.write_all(&self.creature.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // countdown_time: Milliseconds
        w.write_all((self.countdown_time.as_millis() as u32).to_le_bytes().as_slice())?;

        // flags: RollFlags
        w.write_all(&(self.flags.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 33 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A1, size: body_size });
        }

        // creature: Guid
        let creature = crate::util::read_guid(&mut r)?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        // countdown_time: Milliseconds
        let countdown_time = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        // flags: RollFlags
        let flags = RollFlags::new(crate::util::read_u8_le(&mut r)?);

        Ok(Self {
            creature,
            map,
            loot_slot,
            item,
            item_random_suffix,
            item_random_property_id,
            countdown_time,
            flags,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_START_ROLL {}

