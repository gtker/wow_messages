use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::buyback_slot_vanilla_tbc_wrath::BuybackSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buyback_item.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buyback_item.wowm#L19):
/// ```text
/// cmsg CMSG_BUYBACK_ITEM = 0x0290 {
///     Guid guid;
///     BuybackSlot slot;
/// }
/// ```
pub struct CMSG_BUYBACK_ITEM {
    pub guid: Guid,
    pub slot: BuybackSlot,
}

impl crate::private::Sealed for CMSG_BUYBACK_ITEM {}
impl CMSG_BUYBACK_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0290, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // slot: BuybackSlot
        let slot = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            slot,
        })
    }

}

impl crate::Message for CMSG_BUYBACK_ITEM {
    const OPCODE: u32 = 0x0290;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_BUYBACK_ITEM {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    slot = {};", self.slot.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 656_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "slot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.8 1.9 1.10 1.11 1.12 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // slot: BuybackSlot
        w.write_all(&(self.slot.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BUYBACK_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BUYBACK_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BUYBACK_ITEM {}

