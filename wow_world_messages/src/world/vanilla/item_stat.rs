use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L23):
/// ```text
/// struct ItemStat {
///     u32 item_stat_type;
///     i32 item_stat_value;
/// }
/// ```
pub struct ItemStat {
    pub item_stat_type: u32,
    pub item_stat_value: i32,
}

impl ItemStat {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_stat_type: u32
        w.write_all(&self.item_stat_type.to_le_bytes())?;

        // item_stat_value: i32
        w.write_all(&self.item_stat_value.to_le_bytes())?;

        Ok(())
    }
}

impl ItemStat {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_stat_type: u32
        let item_stat_type = crate::util::read_u32_le(r)?;

        // item_stat_value: i32
        let item_stat_value = crate::util::read_i32_le(r)?;

        Ok(Self {
            item_stat_type,
            item_stat_value,
        })
    }

}

