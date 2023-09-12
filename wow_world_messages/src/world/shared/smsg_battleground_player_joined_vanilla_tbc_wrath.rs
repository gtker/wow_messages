use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battleground_player_joined.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battleground_player_joined.wowm#L3):
/// ```text
/// smsg SMSG_BATTLEGROUND_PLAYER_JOINED = 0x02EC {
///     Guid player;
/// }
/// ```
pub struct SMSG_BATTLEGROUND_PLAYER_JOINED {
    pub player: Guid,
}

impl crate::private::Sealed for SMSG_BATTLEGROUND_PLAYER_JOINED {}
impl SMSG_BATTLEGROUND_PLAYER_JOINED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        Ok(Self {
            player,
        })
    }

}

impl crate::Message for SMSG_BATTLEGROUND_PLAYER_JOINED {
    const OPCODE: u32 = 0x02ec;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_BATTLEGROUND_PLAYER_JOINED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BATTLEGROUND_PLAYER_JOINED {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 748_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(748, "SMSG_BATTLEGROUND_PLAYER_JOINED", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_BATTLEGROUND_PLAYER_JOINED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_BATTLEGROUND_PLAYER_JOINED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEGROUND_PLAYER_JOINED {}

