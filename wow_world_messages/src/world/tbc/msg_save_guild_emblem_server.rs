use std::io::{Read, Write};

use crate::tbc::GuildEmblemResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_save_guild_emblem_server.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_save_guild_emblem_server.wowm#L39):
/// ```text
/// smsg MSG_SAVE_GUILD_EMBLEM_Server = 0x01F1 {
///     GuildEmblemResult result;
/// }
/// ```
pub struct MSG_SAVE_GUILD_EMBLEM_Server {
    pub result: GuildEmblemResult,
}

#[cfg(feature = "print-testcase")]
impl MSG_SAVE_GUILD_EMBLEM_Server {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_SAVE_GUILD_EMBLEM_Server {{").unwrap();
        // Members
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 497_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "result");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for MSG_SAVE_GUILD_EMBLEM_Server {}
impl crate::Message for MSG_SAVE_GUILD_EMBLEM_Server {
    const OPCODE: u32 = 0x01f1;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: GuildEmblemResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F1, size: body_size });
        }

        // result: GuildEmblemResult
        let result = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_SAVE_GUILD_EMBLEM_Server {}

