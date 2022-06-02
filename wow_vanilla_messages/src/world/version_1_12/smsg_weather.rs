use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::WeatherChangeType;
use crate::world::version_1_12::WeatherType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_weather.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_weather.wowm#L15):
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

impl ServerMessage for SMSG_WEATHER {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // weather_type: WeatherType
        w.write_all(&(self.weather_type.as_int() as u32).to_le_bytes())?;

        // grade: f32
        w.write_all(&self.grade.to_le_bytes())?;

        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        // change: WeatherChangeType
        w.write_all(&(self.change.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02f4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        13
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // weather_type: WeatherType
        let weather_type: WeatherType = crate::util::read_u32_le(r)?.try_into()?;

        // grade: f32
        let grade = crate::util::read_f32_le(r)?;
        // sound_id: u32
        let sound_id = crate::util::read_u32_le(r)?;

        // change: WeatherChangeType
        let change: WeatherChangeType = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            weather_type,
            grade,
            sound_id,
            change,
        })
    }

}

