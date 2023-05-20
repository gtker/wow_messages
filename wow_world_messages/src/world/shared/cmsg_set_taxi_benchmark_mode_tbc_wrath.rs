use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent when the client runs `/timetest 1`.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_set_taxi_benchmark_mode.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_set_taxi_benchmark_mode.wowm#L1):
/// ```text
/// cmsg CMSG_SET_TAXI_BENCHMARK_MODE = 0x0389 {
///     u8 mode;
/// }
/// ```
pub struct CMSG_SET_TAXI_BENCHMARK_MODE {
    pub mode: u8,
}

impl crate::private::Sealed for CMSG_SET_TAXI_BENCHMARK_MODE {}
impl crate::Message for CMSG_SET_TAXI_BENCHMARK_MODE {
    const OPCODE: u32 = 0x0389;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // mode: u8
        w.write_all(&self.mode.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0389, size: body_size });
        }

        // mode: u8
        let mode = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            mode,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_TAXI_BENCHMARK_MODE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_TAXI_BENCHMARK_MODE {}

