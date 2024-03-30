use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_set_actionbar_toggles.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_set_actionbar_toggles.wowm#L3):
/// ```text
/// cmsg CMSG_SET_ACTIONBAR_TOGGLES = 0x02BF {
///     u8 action_bar;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_SET_ACTIONBAR_TOGGLES {
    /// Emulators set PLAYER_FIELD_BYTES`2` to this unless it's 0.
    pub action_bar: u8,
}

impl crate::private::Sealed for CMSG_SET_ACTIONBAR_TOGGLES {}
impl CMSG_SET_ACTIONBAR_TOGGLES {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // action_bar: u8
        let action_bar = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            action_bar,
        })
    }

}

impl crate::Message for CMSG_SET_ACTIONBAR_TOGGLES {
    const OPCODE: u32 = 0x02bf;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SET_ACTIONBAR_TOGGLES"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_ACTIONBAR_TOGGLES {{").unwrap();
        // Members
        writeln!(s, "    action_bar = {};", self.action_bar).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 703_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "action_bar", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // action_bar: u8
        w.write_all(&self.action_bar.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(703, "CMSG_SET_ACTIONBAR_TOGGLES", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SET_ACTIONBAR_TOGGLES {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_ACTIONBAR_TOGGLES {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_ACTIONBAR_TOGGLES {}

