use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vector.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vector.wowm#L9):
/// ```text
/// struct Vector2d {
///     f32 x;
///     f32 y;
/// }
/// ```
pub struct Vector2d {
    pub x: f32,
    pub y: f32,
}

impl Vector2d {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // x: f32
        w.write_all(&self.x.to_le_bytes())?;

        // y: f32
        w.write_all(&self.y.to_le_bytes())?;

        Ok(())
    }
}

impl Vector2d {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // x: f32
        let x = crate::util::read_f32_le(r)?;
        // y: f32
        let y = crate::util::read_f32_le(r)?;
        Ok(Self {
            x,
            y,
        })
    }

}

