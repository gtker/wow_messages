use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:236`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L236):
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

impl ItemStat {
    pub fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // stat_type: u32
        w.write_all(&self.stat_type.to_le_bytes())?;

        // value: i32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }
}

impl ItemStat {
    pub fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // stat_type: u32
        let stat_type = crate::util::read_u32_le(r)?;

        // value: i32
        let value = crate::util::read_i32_le(r)?;

        Ok(Self {
            stat_type,
            value,
        })
    }

}

