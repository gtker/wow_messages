use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/msg_random_roll_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/msg_random_roll_client.wowm#L3):
/// ```text
/// cmsg MSG_RANDOM_ROLL_Client = 0x01FB {
///     u32 minimum;
///     u32 maximum;
/// }
/// ```
pub struct MSG_RANDOM_ROLL_Client {
    pub minimum: u32,
    pub maximum: u32,
}

impl crate::private::Sealed for MSG_RANDOM_ROLL_Client {}
impl MSG_RANDOM_ROLL_Client {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // minimum: u32
        let minimum = crate::util::read_u32_le(&mut r)?;

        // maximum: u32
        let maximum = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            minimum,
            maximum,
        })
    }

}

impl crate::Message for MSG_RANDOM_ROLL_Client {
    const OPCODE: u32 = 0x01fb;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_RANDOM_ROLL_Client {{").unwrap();
        // Members
        writeln!(s, "    minimum = {};", self.minimum).unwrap();
        writeln!(s, "    maximum = {};", self.maximum).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 507_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "minimum", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "maximum", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes())?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(507, "MSG_RANDOM_ROLL_Client", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_RANDOM_ROLL_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_RANDOM_ROLL_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_RANDOM_ROLL_Client {}

