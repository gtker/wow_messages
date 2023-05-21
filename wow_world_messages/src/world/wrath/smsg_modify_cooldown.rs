use std::io::{Read, Write};

use crate::Guid;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_modify_cooldown.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_modify_cooldown.wowm#L1):
/// ```text
/// smsg SMSG_MODIFY_COOLDOWN = 0x0491 {
///     u32 spell;
///     Guid player;
///     Milliseconds cooldown;
/// }
/// ```
pub struct SMSG_MODIFY_COOLDOWN {
    pub spell: u32,
    pub player: Guid,
    pub cooldown: Duration,
}

impl crate::private::Sealed for SMSG_MODIFY_COOLDOWN {}
impl SMSG_MODIFY_COOLDOWN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0491, size: body_size });
        }

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // cooldown: Milliseconds
        let cooldown = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            spell,
            player,
            cooldown,
        })
    }

}

impl crate::Message for SMSG_MODIFY_COOLDOWN {
    const OPCODE: u32 = 0x0491;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MODIFY_COOLDOWN {{").unwrap();
        // Members
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    cooldown = {};", self.cooldown.as_millis()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 18_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1169_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "cooldown", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // cooldown: Milliseconds
        w.write_all((self.cooldown.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MODIFY_COOLDOWN {}

