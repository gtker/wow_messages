use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::RollVote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_loot_roll.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_loot_roll.wowm#L1):
/// ```text
/// cmsg CMSG_LOOT_ROLL = 0x02A0 {
///     Guid item;
///     u32 item_slot;
///     RollVote vote;
/// }
/// ```
pub struct CMSG_LOOT_ROLL {
    pub item: Guid,
    pub item_slot: u32,
    pub vote: RollVote,
}

impl crate::private::Sealed for CMSG_LOOT_ROLL {}
impl CMSG_LOOT_ROLL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 13 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x02A0, size: body_size });
        }

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // item_slot: u32
        let item_slot = crate::util::read_u32_le(&mut r)?;

        // vote: RollVote
        let vote = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            item,
            item_slot,
            vote,
        })
    }

}

impl crate::Message for CMSG_LOOT_ROLL {
    const OPCODE: u32 = 0x02a0;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LOOT_ROLL {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    item_slot = {};", self.item_slot).unwrap();
        writeln!(s, "    vote = {};", self.vote.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 17_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 672_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "vote", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // item_slot: u32
        w.write_all(&self.item_slot.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LOOT_ROLL {}

