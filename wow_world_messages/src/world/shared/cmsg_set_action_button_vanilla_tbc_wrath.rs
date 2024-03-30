use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_set_action_button.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_set_action_button.wowm#L1):
/// ```text
/// cmsg CMSG_SET_ACTION_BUTTON = 0x0128 {
///     u8 button;
///     u16 action;
///     u8 misc;
///     u8 action_type;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_SET_ACTION_BUTTON {
    pub button: u8,
    pub action: u16,
    pub misc: u8,
    pub action_type: u8,
}

impl crate::private::Sealed for CMSG_SET_ACTION_BUTTON {}
impl CMSG_SET_ACTION_BUTTON {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 5 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // button: u8
        let button = crate::util::read_u8_le(&mut r)?;

        // action: u16
        let action = crate::util::read_u16_le(&mut r)?;

        // misc: u8
        let misc = crate::util::read_u8_le(&mut r)?;

        // action_type: u8
        let action_type = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            button,
            action,
            misc,
            action_type,
        })
    }

}

impl crate::Message for CMSG_SET_ACTION_BUTTON {
    const OPCODE: u32 = 0x0128;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_SET_ACTION_BUTTON"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_ACTION_BUTTON {{").unwrap();
        // Members
        writeln!(s, "    button = {};", self.button).unwrap();
        writeln!(s, "    action = {};", self.action).unwrap();
        writeln!(s, "    misc = {};", self.misc).unwrap();
        writeln!(s, "    action_type = {};", self.action_type).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 9_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 296_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "button", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "action", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "misc", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "action_type", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // button: u8
        w.write_all(&self.button.to_le_bytes())?;

        // action: u16
        w.write_all(&self.action.to_le_bytes())?;

        // misc: u8
        w.write_all(&self.misc.to_le_bytes())?;

        // action_type: u8
        w.write_all(&self.action_type.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(296, "CMSG_SET_ACTION_BUTTON", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SET_ACTION_BUTTON {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_ACTION_BUTTON {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_ACTION_BUTTON {}

