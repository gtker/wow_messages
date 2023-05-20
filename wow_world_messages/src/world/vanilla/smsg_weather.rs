use std::io::{Read, Write};

use crate::vanilla::{
    WeatherChangeType, WeatherType,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_weather.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_weather.wowm#L17):
/// ```text
/// smsg SMSG_WEATHER = 0x02F4 {
///     WeatherType weather_type;
///     f32 grade;
///     u32 sound_id;
///     WeatherChangeType change;
/// }
/// ```
pub struct SMSG_WEATHER {
    pub weather_type: WeatherType,
    pub grade: f32,
    pub sound_id: u32,
    pub change: WeatherChangeType,
}

impl crate::private::Sealed for SMSG_WEATHER {}
impl crate::Message for SMSG_WEATHER {
    const OPCODE: u32 = 0x02f4;

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // weather_type: WeatherType
        w.write_all(&(self.weather_type.as_int().to_le_bytes()))?;

        // grade: f32
        w.write_all(&self.grade.to_le_bytes())?;

        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        // change: WeatherChangeType
        w.write_all(&(self.change.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02F4, size: body_size });
        }

        // weather_type: WeatherType
        let weather_type: WeatherType = crate::util::read_u32_le(&mut r)?.try_into()?;

        // grade: f32
        let grade = crate::util::read_f32_le(&mut r)?;

        // sound_id: u32
        let sound_id = crate::util::read_u32_le(&mut r)?;

        // change: WeatherChangeType
        let change: WeatherChangeType = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            weather_type,
            grade,
            sound_id,
            change,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_WEATHER {}

