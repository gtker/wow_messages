use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vector.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vector.wowm#L10):
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

