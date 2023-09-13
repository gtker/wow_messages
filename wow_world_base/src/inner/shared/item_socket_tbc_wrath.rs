use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:602`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L602):
/// ```text
/// struct ItemSocket {
///     u32 color;
///     u32 content;
/// }
/// ```
pub struct ItemSocket {
    pub color: u32,
    pub content: u32,
}

