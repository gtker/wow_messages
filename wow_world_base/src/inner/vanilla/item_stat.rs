use std::io::{Read, Write};

use crate::vanilla::ItemStatType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:104`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L104):
/// ```text
/// struct ItemStat {
///     (u32)ItemStatType stat_type;
///     i32 value;
/// }
/// ```
pub struct ItemStat {
    pub stat_type: ItemStatType,
    pub value: i32,
}

