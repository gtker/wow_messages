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

#[cfg(feature = "print-testcase")]
impl SMSG_WEATHER {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_WEATHER {{").unwrap();
        // Members
        writeln!(s, "    weather_type = {};", self.weather_type.as_test_case_value()).unwrap();
        writeln!(s, "    {}", if self.grade.to_string().contains(".") { self.grade.to_string() } else { format!("{}.0", self.grade) }).unwrap();
        writeln!(s, "    sound_id = {};", self.sound_id).unwrap();
        writeln!(s, "    change = {};", self.change.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 15_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 756_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "weather_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "grade", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "sound_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "change", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_WEATHER {}
impl crate::Message for SMSG_WEATHER {
    const OPCODE: u32 = 0x02f4;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_WEATHER::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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
        let weather_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        // grade: f32
        let grade = crate::util::read_f32_le(&mut r)?;

        // sound_id: u32
        let sound_id = crate::util::read_u32_le(&mut r)?;

        // change: WeatherChangeType
        let change = crate::util::read_u8_le(&mut r)?.try_into()?;

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

