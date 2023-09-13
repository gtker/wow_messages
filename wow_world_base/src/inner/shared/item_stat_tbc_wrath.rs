use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:594`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L594):
/// ```text
/// struct ItemStat {
///     u32 stat_type;
///     i32 value;
/// }
/// ```
pub struct ItemStat {
    pub stat_type: u32,
    pub value: i32,
}

