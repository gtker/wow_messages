use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_removed_notification.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_removed_notification.wowm#L3):
/// ```text
/// smsg SMSG_AUCTION_REMOVED_NOTIFICATION = 0x028D {
///     u32 item;
///     u32 item_template;
///     u32 random_property_id;
/// }
/// ```
pub struct SMSG_AUCTION_REMOVED_NOTIFICATION {
    pub item: u32,
    pub item_template: u32,
    pub random_property_id: u32,
}

impl crate::private::Sealed for SMSG_AUCTION_REMOVED_NOTIFICATION {}
impl crate::Message for SMSG_AUCTION_REMOVED_NOTIFICATION {
    const OPCODE: u32 = 0x028d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AUCTION_REMOVED_NOTIFICATION {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item).unwrap();
        writeln!(s, "    item_template = {};", self.item_template).unwrap();
        writeln!(s, "    random_property_id = {};", self.random_property_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 653_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_template", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "random_property_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_template: u32
        w.write_all(&self.item_template.to_le_bytes())?;

        // random_property_id: u32
        w.write_all(&self.random_property_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x028D, size: body_size });
        }

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // item_template: u32
        let item_template = crate::util::read_u32_le(&mut r)?;

        // random_property_id: u32
        let random_property_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            item,
            item_template,
            random_property_id,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AUCTION_REMOVED_NOTIFICATION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_AUCTION_REMOVED_NOTIFICATION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AUCTION_REMOVED_NOTIFICATION {}

