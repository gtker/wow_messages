use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_opt_out_of_loot.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_opt_out_of_loot.wowm#L1):
/// ```text
/// cmsg CMSG_OPT_OUT_OF_LOOT = 0x0408 {
///     Bool32 pass_on_loot;
/// }
/// ```
pub struct CMSG_OPT_OUT_OF_LOOT {
    pub pass_on_loot: bool,
}

impl crate::private::Sealed for CMSG_OPT_OUT_OF_LOOT {}
impl CMSG_OPT_OUT_OF_LOOT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // pass_on_loot: Bool32
        let pass_on_loot = crate::util::read_u32_le(&mut r)? != 0;

        Ok(Self {
            pass_on_loot,
        })
    }

}

impl crate::Message for CMSG_OPT_OUT_OF_LOOT {
    const OPCODE: u32 = 0x0408;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_OPT_OUT_OF_LOOT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_OPT_OUT_OF_LOOT {{").unwrap();
        // Members
        writeln!(s, "    pass_on_loot = {};", if self.pass_on_loot { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1032_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "pass_on_loot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // pass_on_loot: Bool32
        w.write_all(u32::from(self.pass_on_loot).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1032, "CMSG_OPT_OUT_OF_LOOT", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_OPT_OUT_OF_LOOT {}

