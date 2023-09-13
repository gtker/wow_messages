use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::SpellCooldownStatus;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm#L8):
/// ```text
/// smsg SMSG_SPELL_COOLDOWN = 0x0134 {
///     Guid guid;
///     SpellCooldownStatus[-] cooldowns;
/// }
/// ```
pub struct SMSG_SPELL_COOLDOWN {
    pub guid: Guid,
    pub cooldowns: Vec<SpellCooldownStatus>,
}

impl crate::private::Sealed for SMSG_SPELL_COOLDOWN {}
impl SMSG_SPELL_COOLDOWN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(8..=65543).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // cooldowns: SpellCooldownStatus[-]
        let cooldowns = {
            let mut current_size = {
                8 // guid: Guid
            };
            current_size += 4; // cooldowns_decompressed_size: u32
            let mut cooldowns = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                cooldowns.push(SpellCooldownStatus::read(&mut r)?);
                current_size += 8;
            }
            cooldowns
        };

        Ok(Self {
            guid,
            cooldowns,
        })
    }

}

impl crate::Message for SMSG_SPELL_COOLDOWN {
    const OPCODE: u32 = 0x0134;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SPELL_COOLDOWN"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELL_COOLDOWN {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    cooldowns = [").unwrap();
        for v in self.cooldowns.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            id = {};", v.id).unwrap();
            writeln!(s, "            cooldown_time = {};", v.cooldown_time.as_millis()).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 308_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        if !self.cooldowns.is_empty() {
            writeln!(s, "    /* cooldowns: SpellCooldownStatus[-] start */").unwrap();
            for (i, v) in self.cooldowns.iter().enumerate() {
                writeln!(s, "    /* cooldowns: SpellCooldownStatus[-] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "cooldown_time", "        ");
                writeln!(s, "    /* cooldowns: SpellCooldownStatus[-] {i} end */").unwrap();
            }
            writeln!(s, "    /* cooldowns: SpellCooldownStatus[-] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // cooldowns: SpellCooldownStatus[-]
        for i in self.cooldowns.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(308, "SMSG_SPELL_COOLDOWN", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELL_COOLDOWN {}

impl SMSG_SPELL_COOLDOWN {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.cooldowns.len() * 8 // cooldowns: SpellCooldownStatus[-]
    }
}

