use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::ItemTextQuery;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_text_query_response.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_text_query_response.wowm#L17):
/// ```text
/// smsg SMSG_ITEM_TEXT_QUERY_RESPONSE = 0x0244 {
///     ItemTextQuery query;
///     if (query == HAS_TEXT) {
///         Guid item;
///         CString text;
///     }
/// }
/// ```
pub struct SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub query: SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery,
}

impl crate::private::Sealed for SMSG_ITEM_TEXT_QUERY_RESPONSE {}
impl SMSG_ITEM_TEXT_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=265).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // query: ItemTextQuery
        let query = crate::util::read_u8_le(&mut r)?.try_into()?;

        let query_if = match query {
            ItemTextQuery::HasText => {
                // item: Guid
                let item = crate::util::read_guid(&mut r)?;

                // text: CString
                let text = {
                    let text = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(text)?
                };

                SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery::HasText {
                    item,
                    text,
                }
            }
            ItemTextQuery::NoText => SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery::NoText,
        };

        Ok(Self {
            query: query_if,
        })
    }

}

impl crate::Message for SMSG_ITEM_TEXT_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0244;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ITEM_TEXT_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    query = {};", ItemTextQuery::try_from(self.query.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.query {
            crate::wrath::SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery::HasText {
                item,
                text,
            } => {
                writeln!(s, "    item = {};", item.guid()).unwrap();
                writeln!(s, "    text = \"{}\";", text).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 580_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "query", "    ");
        match &self.query {
            crate::wrath::SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery::HasText {
                item,
                text,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, text.len() + 1, "text", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // query: ItemTextQuery
        w.write_all(&(self.query.as_int().to_le_bytes()))?;

        match &self.query {
            SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery::HasText {
                item,
                text,
            } => {
                // item: Guid
                w.write_all(&item.guid().to_le_bytes())?;

                // text: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(text.as_bytes().iter().next_back(), Some(&0_u8), "String `text` must not be null-terminated.");
                w.write_all(text.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(580, "SMSG_ITEM_TEXT_QUERY_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ITEM_TEXT_QUERY_RESPONSE {}

impl SMSG_ITEM_TEXT_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        self.query.size() // query: SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery {
    HasText {
        item: Guid,
        text: String,
    },
    NoText,
}

impl Default for SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NoText
    }
}

impl SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::HasText { .. } => 0,
            Self::NoText => 1,
        }
    }

}

impl std::fmt::Display for SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HasText{ .. } => f.write_str("HasText"),
            Self::NoText => f.write_str("NoText"),
        }
    }
}

impl SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::HasText {
                text,
                ..
            } => {
                1
                + 8 // item: Guid
                + text.len() + 1 // text: CString
            }
            _ => 1,
        }
    }
}

