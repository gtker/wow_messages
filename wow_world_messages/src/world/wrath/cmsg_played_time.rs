use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_played_time.wowm:6`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_played_time.wowm#L6):
/// ```text
/// cmsg CMSG_PLAYED_TIME = 0x01CC {
///     Bool show_on_ui;
/// }
/// ```
pub struct CMSG_PLAYED_TIME {
    /// Whether the clients wants it shown on the UI. Just ping it back in [`SMSG_PLAYED_TIME`](crate::wrath::SMSG_PLAYED_TIME)
    pub show_on_ui: bool,
}

#[cfg(feature = "print-testcase")]
impl CMSG_PLAYED_TIME {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PLAYED_TIME {{").unwrap();
        // Members
        writeln!(s, "    show_on_ui = {};", if self.show_on_ui { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 460_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "show_on_ui", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_PLAYED_TIME {}
impl crate::Message for CMSG_PLAYED_TIME {
    const OPCODE: u32 = 0x01cc;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_PLAYED_TIME::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // show_on_ui: Bool
        w.write_all(u8::from(self.show_on_ui).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01CC, size: body_size });
        }

        // show_on_ui: Bool
        let show_on_ui = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            show_on_ui,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PLAYED_TIME {}

