use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_world_state_ui_timer_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_world_state_ui_timer_update.wowm#L3):
/// ```text
/// smsg SMSG_WORLD_STATE_UI_TIMER_UPDATE = 0x04F7 {
///     u32 time;
/// }
/// ```
pub struct SMSG_WORLD_STATE_UI_TIMER_UPDATE {
    /// Seconds since Unix Epoch
    ///
    pub time: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_WORLD_STATE_UI_TIMER_UPDATE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_WORLD_STATE_UI_TIMER_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    time = {};", self.time).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1271_u32.to_le_bytes();
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
        writeln!(s, "    versions = \"3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_WORLD_STATE_UI_TIMER_UPDATE {}
impl crate::Message for SMSG_WORLD_STATE_UI_TIMER_UPDATE {
    const OPCODE: u32 = 0x04f7;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // time: u32
        w.write_all(&self.time.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04F7, size: body_size });
        }

        // time: u32
        let time = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            time,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_WORLD_STATE_UI_TIMER_UPDATE {}

