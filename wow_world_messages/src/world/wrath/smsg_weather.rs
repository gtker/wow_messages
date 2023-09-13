use std::io::{Read, Write};

use crate::wrath::{
    WeatherChangeType, WeatherType,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_weather.wowm:70`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_weather.wowm#L70):
/// ```text
/// smsg SMSG_WEATHER = 0x02F4 {
///     WeatherType weather_type;
///     f32 grade;
///     WeatherChangeType change;
/// }
/// ```
pub struct SMSG_WEATHER {
    pub weather_type: WeatherType,
    pub grade: f32,
    pub change: WeatherChangeType,
}

impl crate::private::Sealed for SMSG_WEATHER {}
impl SMSG_WEATHER {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // weather_type: WeatherType
        let weather_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        // grade: f32
        let grade = crate::util::read_f32_le(&mut r)?;

        // change: WeatherChangeType
        let change = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            weather_type,
            grade,
            change,
        })
    }

}

impl crate::Message for SMSG_WEATHER {
    const OPCODE: u32 = 0x02f4;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_WEATHER"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_WEATHER {{").unwrap();
        // Members
        writeln!(s, "    weather_type = {};", self.weather_type.as_test_case_value()).unwrap();
        writeln!(s, "    grade = {}", if self.grade.to_string().contains('.') { self.grade.to_string() } else { format!("{}.0", self.grade) }).unwrap();
        writeln!(s, "    change = {};", self.change.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 11_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 756_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "weather_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "grade", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "change", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // weather_type: WeatherType
        w.write_all(&(self.weather_type.as_int().to_le_bytes()))?;

        // grade: f32
        w.write_all(&self.grade.to_le_bytes())?;

        // change: WeatherChangeType
        w.write_all(&(self.change.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(756, "SMSG_WEATHER", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_WEATHER {}

