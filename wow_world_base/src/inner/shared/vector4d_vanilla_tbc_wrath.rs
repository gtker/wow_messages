use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vector.wowm:4`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vector.wowm#L4):
/// ```text
/// struct Vector4d {
///     f32 x;
///     f32 y;
///     f32 z;
///     f32 orientation;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Vector4d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub orientation: f32,
}

