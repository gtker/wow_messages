use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_add_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_add_item.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_ADD_ITEM = 0x019A {
///     u32 required_item_id;
///     u32 items_required;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_ADD_ITEM {
    pub required_item_id: u32,
    pub items_required: u32,
}

impl crate::private::Sealed for SMSG_QUESTUPDATE_ADD_ITEM {}
impl SMSG_QUESTUPDATE_ADD_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x019A, size: body_size });
        }

        // required_item_id: u32
        let required_item_id = crate::util::read_u32_le(&mut r)?;

        // items_required: u32
        let items_required = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            required_item_id,
            items_required,
        })
    }

}

impl crate::Message for SMSG_QUESTUPDATE_ADD_ITEM {
    const OPCODE: u32 = 0x019a;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTUPDATE_ADD_ITEM {{").unwrap();
        // Members
        writeln!(s, "    required_item_id = {};", self.required_item_id).unwrap();
        writeln!(s, "    items_required = {};", self.items_required).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 410_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "required_item_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "items_required", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // required_item_id: u32
        w.write_all(&self.required_item_id.to_le_bytes())?;

        // items_required: u32
        w.write_all(&self.items_required.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_QUESTUPDATE_ADD_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTUPDATE_ADD_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTUPDATE_ADD_ITEM {}

