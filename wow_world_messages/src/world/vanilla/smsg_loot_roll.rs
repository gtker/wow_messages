use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::RollVote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_roll.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_roll.wowm#L1):
/// ```text
/// smsg SMSG_LOOT_ROLL = 0x02A2 {
///     Guid creature;
///     u32 loot_slot;
///     Guid player;
///     u32 item;
///     u32 item_random_suffix;
///     u32 item_random_property_id;
///     u8 roll_number;
///     RollVote vote;
/// }
/// ```
pub struct SMSG_LOOT_ROLL {
    pub creature: Guid,
    pub loot_slot: u32,
    pub player: Guid,
    pub item: u32,
    /// vmangos/mangoszero: not used ?
    ///
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    /// vmangos/cmangos/mangoszero: 0: Need for: `item_name` > 127: you passed on: `item_name`      Roll number
    ///
    pub roll_number: u8,
    pub vote: RollVote,
}

#[cfg(feature = "print-testcase")]
impl SMSG_LOOT_ROLL {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOOT_ROLL {{").unwrap();
        // Members
        writeln!(s, "    creature = {};", self.creature.guid()).unwrap();
        writeln!(s, "    loot_slot = {};", self.loot_slot).unwrap();
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    item = {};", self.item).unwrap();
        writeln!(s, "    item_random_suffix = {};", self.item_random_suffix).unwrap();
        writeln!(s, "    item_random_property_id = {};", self.item_random_property_id).unwrap();
        writeln!(s, "    roll_number = {};", self.roll_number).unwrap();
        writeln!(s, "    vote = {};", self.vote.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 38_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 674_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "creature");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_LOOT_ROLL {}
impl crate::Message for SMSG_LOOT_ROLL {
    const OPCODE: u32 = 0x02a2;

    fn size_without_header(&self) -> u32 {
        34
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // creature: Guid
        w.write_all(&self.creature.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // roll_number: u8
        w.write_all(&self.roll_number.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 34 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A2, size: body_size });
        }

        // creature: Guid
        let creature = crate::util::read_guid(&mut r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(&mut r)?;

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        // roll_number: u8
        let roll_number = crate::util::read_u8_le(&mut r)?;

        // vote: RollVote
        let vote = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            creature,
            loot_slot,
            player,
            item,
            item_random_suffix,
            item_random_property_id,
            roll_number,
            vote,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_ROLL {}

