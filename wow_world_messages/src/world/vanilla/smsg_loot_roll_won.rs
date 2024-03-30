use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::RollVote;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_roll_won.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_roll_won.wowm#L1):
/// ```text
/// smsg SMSG_LOOT_ROLL_WON = 0x029F {
///     Guid looted_target;
///     u32 loot_slot;
///     Item item;
///     u32 item_random_suffix;
///     u32 item_random_property_id;
///     Guid winning_player;
///     u8 winning_roll;
///     RollVote vote;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_LOOT_ROLL_WON {
    pub looted_target: Guid,
    pub loot_slot: u32,
    pub item: u32,
    /// vmangos/mangoszero: not used ?
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub winning_player: Guid,
    /// rollnumber related to SMSG_LOOT_ROLL
    pub winning_roll: u8,
    /// Rolltype related to SMSG_LOOT_ROLL
    pub vote: RollVote,
}

impl crate::private::Sealed for SMSG_LOOT_ROLL_WON {}
impl SMSG_LOOT_ROLL_WON {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 34 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // looted_target: Guid
        let looted_target = crate::util::read_guid(&mut r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(&mut r)?;

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        // winning_player: Guid
        let winning_player = crate::util::read_guid(&mut r)?;

        // winning_roll: u8
        let winning_roll = crate::util::read_u8_le(&mut r)?;

        // vote: RollVote
        let vote = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            looted_target,
            loot_slot,
            item,
            item_random_suffix,
            item_random_property_id,
            winning_player,
            winning_roll,
            vote,
        })
    }

}

impl crate::Message for SMSG_LOOT_ROLL_WON {
    const OPCODE: u32 = 0x029f;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LOOT_ROLL_WON"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOOT_ROLL_WON {{").unwrap();
        // Members
        writeln!(s, "    looted_target = {};", self.looted_target.guid()).unwrap();
        writeln!(s, "    loot_slot = {};", self.loot_slot).unwrap();
        writeln!(s, "    item = {};", self.item).unwrap();
        writeln!(s, "    item_random_suffix = {};", self.item_random_suffix).unwrap();
        writeln!(s, "    item_random_property_id = {};", self.item_random_property_id).unwrap();
        writeln!(s, "    winning_player = {};", self.winning_player.guid()).unwrap();
        writeln!(s, "    winning_roll = {};", self.winning_roll).unwrap();
        writeln!(s, "    vote = {};", self.vote.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 36_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 671_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "looted_target", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "loot_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_suffix", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_property_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "winning_player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "winning_roll", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "vote", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        34
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // looted_target: Guid
        w.write_all(&self.looted_target.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // winning_player: Guid
        w.write_all(&self.winning_player.guid().to_le_bytes())?;

        // winning_roll: u8
        w.write_all(&self.winning_roll.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(671, "SMSG_LOOT_ROLL_WON", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_ROLL_WON {}

