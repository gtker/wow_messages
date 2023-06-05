use std::io::{Read, Write};

use crate::Guid;
use crate::shared::spell_cooldown_status_vanilla_tbc_wrath::SpellCooldownStatus;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm#L15):
/// ```text
/// smsg SMSG_SPELL_COOLDOWN = 0x0134 {
///     Guid guid;
///     u8 flags;
///     SpellCooldownStatus[-] cooldowns;
/// }
/// ```
pub struct SMSG_SPELL_COOLDOWN {
    pub guid: Guid,
    pub flags: u8,
    pub cooldowns: Vec<SpellCooldownStatus>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_SPELL_COOLDOWN {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELL_COOLDOWN {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        write!(s, "    cooldowns = [").unwrap();
        for v in self.cooldowns.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    id = {};", v.id).unwrap();
            writeln!(s, "    cooldown_time = {};", v.cooldown_time.as_millis()).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 308_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid");
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

impl crate::private::Sealed for SMSG_SPELL_COOLDOWN {}
impl crate::Message for SMSG_SPELL_COOLDOWN {
    const OPCODE: u32 = 0x0134;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        // cooldowns: SpellCooldownStatus[-]
        for i in self.cooldowns.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(9..=65544).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0134, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // flags: u8
        let flags = crate::util::read_u8_le(&mut r)?;

        // cooldowns: SpellCooldownStatus[-]
        let cooldowns = {
            let mut current_size = {
                8 // guid: Guid
                + 1 // flags: u8
            };
            let mut cooldowns = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                cooldowns.push(SpellCooldownStatus::read(&mut r)?);
                current_size += 1;
            }
            cooldowns
        };

        Ok(Self {
            guid,
            flags,
            cooldowns,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELL_COOLDOWN {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELL_COOLDOWN {}

impl SMSG_SPELL_COOLDOWN {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 1 // flags: u8
        + self.cooldowns.len() * 8 // cooldowns: SpellCooldownStatus[-]
    }
}

