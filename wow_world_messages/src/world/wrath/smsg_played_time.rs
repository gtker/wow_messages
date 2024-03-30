use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_played_time.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_played_time.wowm#L9):
/// ```text
/// smsg SMSG_PLAYED_TIME = 0x01CD {
///     u32 total_played_time;
///     u32 level_played_time;
///     Bool show_on_ui;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PLAYED_TIME {
    /// Time played in total (seconds)
    pub total_played_time: u32,
    /// Time played on this level (seconds)
    pub level_played_time: u32,
    /// Whether this is a silent query or the client should show it on the UI (chat box).
    /// Send back the value received in [`CMSG_PLAYED_TIME`](crate::wrath::CMSG_PLAYED_TIME)
    pub show_on_ui: bool,
}

impl crate::private::Sealed for SMSG_PLAYED_TIME {}
impl SMSG_PLAYED_TIME {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // total_played_time: u32
        let total_played_time = crate::util::read_u32_le(&mut r)?;

        // level_played_time: u32
        let level_played_time = crate::util::read_u32_le(&mut r)?;

        // show_on_ui: Bool
        let show_on_ui = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            total_played_time,
            level_played_time,
            show_on_ui,
        })
    }

}

impl crate::Message for SMSG_PLAYED_TIME {
    const OPCODE: u32 = 0x01cd;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PLAYED_TIME"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PLAYED_TIME {{").unwrap();
        // Members
        writeln!(s, "    total_played_time = {};", self.total_played_time).unwrap();
        writeln!(s, "    level_played_time = {};", self.level_played_time).unwrap();
        writeln!(s, "    show_on_ui = {};", if self.show_on_ui { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 11_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 461_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "total_played_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "level_played_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "show_on_ui", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // total_played_time: u32
        w.write_all(&self.total_played_time.to_le_bytes())?;

        // level_played_time: u32
        w.write_all(&self.level_played_time.to_le_bytes())?;

        // show_on_ui: Bool
        w.write_all(u8::from(self.show_on_ui).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(461, "SMSG_PLAYED_TIME", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAYED_TIME {}

