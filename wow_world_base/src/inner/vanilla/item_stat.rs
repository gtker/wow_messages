use std::io::{Read, Write};

use crate::vanilla::ItemStatType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:107`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L107):
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

impl ItemStat {
    pub fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // stat_type: ItemStatType
        w.write_all(&u32::from(self.stat_type.as_int()).to_le_bytes())?;

        // value: i32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }
}

impl ItemStat {
    pub fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // stat_type: ItemStatType
        let stat_type = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // value: i32
        let value = crate::util::read_i32_le(&mut r)?;

        Ok(Self {
            stat_type,
            value,
        })
    }

}

