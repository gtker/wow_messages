use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Never actually sent in any emulator.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_chat_player_ambiguous.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_chat_player_ambiguous.wowm#L1):
/// ```text
/// smsg SMSG_CHAT_PLAYER_AMBIGUOUS = 0x032D {
///     CString player;
/// }
/// ```
pub struct SMSG_CHAT_PLAYER_AMBIGUOUS {
    pub player: String,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CHAT_PLAYER_AMBIGUOUS {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CHAT_PLAYER_AMBIGUOUS {{").unwrap();
        // Members
        writeln!(s, "    player = \"{}\";", self.player).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 813_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_CHAT_PLAYER_AMBIGUOUS {}
impl crate::Message for SMSG_CHAT_PLAYER_AMBIGUOUS {
    const OPCODE: u32 = 0x032d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player.as_bytes().iter().rev().next(), Some(&0_u8), "String `player` must not be null-terminated.");
        w.write_all(self.player.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x032D, size: body_size });
        }

        // player: CString
        let player = {
            let player = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(player)?
        };

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CHAT_PLAYER_AMBIGUOUS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CHAT_PLAYER_AMBIGUOUS {}

impl SMSG_CHAT_PLAYER_AMBIGUOUS {
    pub(crate) fn size(&self) -> usize {
        self.player.len() + 1 // player: CString
    }
}

