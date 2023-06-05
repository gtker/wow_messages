use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autoequip_item_slot.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOEQUIP_ITEM_SLOT = 0x010F {
///     Guid guid;
///     u8 destination_slot;
/// }
/// ```
pub struct CMSG_AUTOEQUIP_ITEM_SLOT {
    pub guid: Guid,
    pub destination_slot: u8,
}

#[cfg(feature = "print-testcase")]
impl CMSG_AUTOEQUIP_ITEM_SLOT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUTOEQUIP_ITEM_SLOT {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    destination_slot = {};", self.destination_slot).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 13_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 271_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "destination_slot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_AUTOEQUIP_ITEM_SLOT {}
impl crate::Message for CMSG_AUTOEQUIP_ITEM_SLOT {
    const OPCODE: u32 = 0x010f;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_AUTOEQUIP_ITEM_SLOT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // destination_slot: u8
        w.write_all(&self.destination_slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x010F, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // destination_slot: u8
        let destination_slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            guid,
            destination_slot,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUTOEQUIP_ITEM_SLOT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUTOEQUIP_ITEM_SLOT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUTOEQUIP_ITEM_SLOT {}

