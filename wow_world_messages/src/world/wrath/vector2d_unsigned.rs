use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_poi_query_response.wowm#L9):
/// ```text
/// struct Vector2dUnsigned {
///     u32 x;
///     u32 y;
/// }
/// ```
pub struct Vector2dUnsigned {
    pub x: u32,
    pub y: u32,
}

impl Vector2dUnsigned {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // x: u32
        w.write_all(&self.x.to_le_bytes())?;

        // y: u32
        w.write_all(&self.y.to_le_bytes())?;

        Ok(())
    }
}

impl Vector2dUnsigned {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // x: u32
        let x = crate::util::read_u32_le(&mut r)?;

        // y: u32
        let y = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            x,
            y,
        })
    }

}

