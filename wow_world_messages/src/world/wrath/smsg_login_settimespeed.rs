use std::io::{Read, Write};

use crate::DateTime;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm#L30):
/// ```text
/// smsg SMSG_LOGIN_SETTIMESPEED = 0x0042 {
///     DateTime datetime;
///     f32 timescale;
///     u32 unknown1;
/// }
/// ```
pub struct SMSG_LOGIN_SETTIMESPEED {
    /// Current server datetime.
    ///
    /// Running the client with `-console` verifies that this message in this format sets the correct datetime. [`SMSG_QUERY_TIME_RESPONSE`](crate::tbc::SMSG_QUERY_TIME_RESPONSE) will not set this.
    ///
    pub datetime: DateTime,
    /// How many minutes should pass by every second.
    ///
    /// vmangos/cmangos/mangoszero set this to 0.01666667. This means that 1/60 minutes pass every second (one second passes every second). Setting this to 1.0 will make the client advance one minute every second.
    ///
    pub timescale: f32,
    /// arcemu/azerothcore/mangostwo: Set to 0.
    ///
    pub unknown1: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_LOGIN_SETTIMESPEED {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOGIN_SETTIMESPEED {{").unwrap();
        // Members
        writeln!(s, "    datetime = {};", self.datetime.as_int()).unwrap();
        writeln!(s, "    {}", if self.timescale.to_string().contains(".") { self.timescale.to_string() } else { format!("{}.0", self.timescale) }).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 66_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "datetime");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.1.2 3.2 3.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_LOGIN_SETTIMESPEED {}
impl crate::Message for SMSG_LOGIN_SETTIMESPEED {
    const OPCODE: u32 = 0x0042;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // datetime: DateTime
        w.write_all(&self.datetime.as_int().to_le_bytes())?;

        // timescale: f32
        w.write_all(&self.timescale.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0042, size: body_size });
        }

        // datetime: DateTime
        let datetime = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // timescale: f32
        let timescale = crate::util::read_f32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            datetime,
            timescale,
            unknown1,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOGIN_SETTIMESPEED {}

