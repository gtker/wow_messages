use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::wrath::ItemTextQuery;
use std::io::{Write, Read};

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

impl crate::Message for SMSG_ITEM_TEXT_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0244;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // query: ItemTextQuery
        w.write_all(&(self.query.as_int() as u8).to_le_bytes())?;

        match &self.query {
            SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery::HasText {
                item,
                text,
            } => {
                // item: Guid
                w.write_all(&item.guid().to_le_bytes())?;

                // text: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(text.as_bytes().iter().rev().next(), Some(&0_u8), "String `text` must not be null-terminated.");
                w.write_all(text.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery::NoText => {}
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=265).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0244, size: body_size as u32 });
        }

        // query: ItemTextQuery
        let query: ItemTextQuery = crate::util::read_u8_le(r)?.try_into()?;

        let query_if = match query {
            ItemTextQuery::HasText => {
                // item: Guid
                let item = Guid::read(r)?;

                // text: CString
                let text = crate::util::read_c_string_to_vec(r)?;
                let text = String::from_utf8(text)?;

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

impl SMSG_ITEM_TEXT_QUERY_RESPONSE_ItemTextQuery {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::HasText {
                item,
                text,
            } => {
                1
                + 8 // item: Guid
                + text.len() + 1 // text: CString
            }
            Self::NoText => {
                1
            }
        }
    }
}

