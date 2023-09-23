use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent when the client runs `/timetest 1`.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_set_taxi_benchmark_mode.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_set_taxi_benchmark_mode.wowm#L2):
/// ```text
/// cmsg CMSG_SET_TAXI_BENCHMARK_MODE = 0x0389 {
///     u8 mode;
/// }
/// ```
pub struct CMSG_SET_TAXI_BENCHMARK_MODE {
    pub mode: u8,
}

impl crate::private::Sealed for CMSG_SET_TAXI_BENCHMARK_MODE {}
impl CMSG_SET_TAXI_BENCHMARK_MODE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // mode: u8
        let mode = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            mode,
        })
    }

}

impl crate::Message for CMSG_SET_TAXI_BENCHMARK_MODE {
    const OPCODE: u32 = 0x0389;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SET_TAXI_BENCHMARK_MODE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_TAXI_BENCHMARK_MODE {{").unwrap();
        // Members
        writeln!(s, "    mode = {};", self.mode).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 905_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "mode", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // mode: u8
        w.write_all(&self.mode.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(905, "CMSG_SET_TAXI_BENCHMARK_MODE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_TAXI_BENCHMARK_MODE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_TAXI_BENCHMARK_MODE {}

