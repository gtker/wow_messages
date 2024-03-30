use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_item_text_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_item_text_query.wowm#L1):
/// ```text
/// cmsg CMSG_ITEM_TEXT_QUERY = 0x0243 {
///     u32 item_text_id;
///     u32 mail_id;
///     u32 unknown1;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_ITEM_TEXT_QUERY {
    pub item_text_id: u32,
    /// vmangos/cmangos/mangoszero: this value can be item id in bag, but it is also mail id
    pub mail_id: u32,
    /// vmangos/cmangos/mangoszero: maybe something like state - 0x70000000
    pub unknown1: u32,
}

impl crate::private::Sealed for CMSG_ITEM_TEXT_QUERY {}
impl CMSG_ITEM_TEXT_QUERY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // item_text_id: u32
        let item_text_id = crate::util::read_u32_le(&mut r)?;

        // mail_id: u32
        let mail_id = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            item_text_id,
            mail_id,
            unknown1,
        })
    }

}

impl crate::Message for CMSG_ITEM_TEXT_QUERY {
    const OPCODE: u32 = 0x0243;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_ITEM_TEXT_QUERY"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ITEM_TEXT_QUERY {{").unwrap();
        // Members
        writeln!(s, "    item_text_id = {};", self.item_text_id).unwrap();
        writeln!(s, "    mail_id = {};", self.mail_id).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 579_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_text_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "mail_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // mail_id: u32
        w.write_all(&self.mail_id.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(579, "CMSG_ITEM_TEXT_QUERY", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ITEM_TEXT_QUERY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ITEM_TEXT_QUERY {}

