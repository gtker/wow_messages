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

#[cfg(feature = "print-testcase")]
impl SMSG_MODIFY_COOLDOWN {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MODIFY_COOLDOWN {{").unwrap();
        // Members
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    cooldown = {};", self.cooldown.as_millis()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 20_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1169_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_MODIFY_COOLDOWN {}
impl crate::Message for SMSG_MODIFY_COOLDOWN {
    const OPCODE: u32 = 0x0491;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0491, size: body_size });
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MODIFY_COOLDOWN {}

