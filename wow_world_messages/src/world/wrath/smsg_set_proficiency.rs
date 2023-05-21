use std::io::{Read, Write};

use crate::wrath::ItemClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_set_proficiency.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_set_proficiency.wowm#L1):
/// ```text
/// smsg SMSG_SET_PROFICIENCY = 0x0127 {
///     ItemClass class;
///     u32 item_sub_class_mask;
/// }
/// ```
pub struct SMSG_SET_PROFICIENCY {
    pub class: ItemClass,
    pub item_sub_class_mask: u32,
}

impl crate::private::Sealed for SMSG_SET_PROFICIENCY {}
impl SMSG_SET_PROFICIENCY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 5 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0127, size: body_size });
        }

        // class: ItemClass
        let class = crate::util::read_u8_le(&mut r)?.try_into()?;

        // item_sub_class_mask: u32
        let item_sub_class_mask = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            class,
            item_sub_class_mask,
        })
    }

}

impl crate::Message for SMSG_SET_PROFICIENCY {
    const OPCODE: u32 = 0x0127;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SET_PROFICIENCY {{").unwrap();
        // Members
        writeln!(s, "    class = {};", self.class.as_test_case_value()).unwrap();
        writeln!(s, "    item_sub_class_mask = {};", self.item_sub_class_mask).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 295_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "class", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_sub_class_mask", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // class: ItemClass
        w.write_all(&(self.class.as_int().to_le_bytes()))?;

        // item_sub_class_mask: u32
        w.write_all(&self.item_sub_class_mask.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SET_PROFICIENCY {}

