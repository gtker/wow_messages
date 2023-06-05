use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/msg_channel_start.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/msg_channel_start.wowm#L1):
/// ```text
/// smsg MSG_CHANNEL_START_Server = 0x0139 {
///     u32 spell;
///     u32 duration;
/// }
/// ```
pub struct MSG_CHANNEL_START_Server {
    pub spell: u32,
    pub duration: u32,
}

#[cfg(feature = "print-testcase")]
impl MSG_CHANNEL_START_Server {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_CHANNEL_START_Server {{").unwrap();
        // Members
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    duration = {};", self.duration).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 313_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for MSG_CHANNEL_START_Server {}
impl crate::Message for MSG_CHANNEL_START_Server {
    const OPCODE: u32 = 0x0139;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        MSG_CHANNEL_START_Server::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0139, size: body_size });
        }

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            spell,
            duration,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_CHANNEL_START_Server {}

