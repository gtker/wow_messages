use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/msg_inspect_arena_teams.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/msg_inspect_arena_teams.wowm#L14):
/// ```text
/// cmsg MSG_INSPECT_ARENA_TEAMS_Client = 0x0377 {
///     Guid player;
/// }
/// ```
pub struct MSG_INSPECT_ARENA_TEAMS_Client {
    pub player: Guid,
}

#[cfg(feature = "print-testcase")]
impl MSG_INSPECT_ARENA_TEAMS_Client {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_INSPECT_ARENA_TEAMS_Client {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 887_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "player");
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

impl crate::private::Sealed for MSG_INSPECT_ARENA_TEAMS_Client {}
impl crate::Message for MSG_INSPECT_ARENA_TEAMS_Client {
    const OPCODE: u32 = 0x0377;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0377, size: body_size });
        }

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_INSPECT_ARENA_TEAMS_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_INSPECT_ARENA_TEAMS_Client {}

