use std::io::{Read, Write};

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_countdown.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_countdown.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_COUNTDOWN = 0x02B7 {
///     Seconds time;
/// }
/// ```
pub struct SMSG_DUEL_COUNTDOWN {
    pub time: Duration,
}

#[cfg(feature = "print-testcase")]
impl SMSG_DUEL_COUNTDOWN {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_DUEL_COUNTDOWN {{").unwrap();
        // Members
        writeln!(s, "    time = {};", self.time.as_secs()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 695_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "time");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_DUEL_COUNTDOWN {}
impl crate::Message for SMSG_DUEL_COUNTDOWN {
    const OPCODE: u32 = 0x02b7;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // time: Seconds
        w.write_all((self.time.as_secs() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02B7, size: body_size });
        }

        // time: Seconds
        let time = Duration::from_secs(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            time,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_DUEL_COUNTDOWN {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DUEL_COUNTDOWN {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DUEL_COUNTDOWN {}

