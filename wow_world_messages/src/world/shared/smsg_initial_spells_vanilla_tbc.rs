use std::io::{Read, Write};

use crate::shared::cooldown_spell_vanilla_tbc_wrath::CooldownSpell;
use crate::shared::initial_spell_vanilla_tbc::InitialSpell;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm#L22):
/// ```text
/// smsg SMSG_INITIAL_SPELLS = 0x012A {
///     u8 unknown1;
///     u16 spell_count;
///     InitialSpell[spell_count] initial_spells;
///     u16 cooldown_count;
///     CooldownSpell[cooldown_count] cooldowns;
/// }
/// ```
pub struct SMSG_INITIAL_SPELLS {
    /// cmangos/mangoszero: sets to 0
    pub unknown1: u8,
    pub initial_spells: Vec<InitialSpell>,
    pub cooldowns: Vec<CooldownSpell>,
}

impl crate::private::Sealed for SMSG_INITIAL_SPELLS {}
impl SMSG_INITIAL_SPELLS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=1179653).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // spell_count: u16
        let spell_count = crate::util::read_u16_le(&mut r)?;

        // initial_spells: InitialSpell[spell_count]
        let initial_spells = {
            let mut initial_spells = Vec::with_capacity(spell_count as usize);
            for _ in 0..spell_count {
                initial_spells.push(InitialSpell::read(&mut r)?);
            }
            initial_spells
        };

        // cooldown_count: u16
        let cooldown_count = crate::util::read_u16_le(&mut r)?;

        // cooldowns: CooldownSpell[cooldown_count]
        let cooldowns = {
            let mut cooldowns = Vec::with_capacity(cooldown_count as usize);
            for _ in 0..cooldown_count {
                cooldowns.push(CooldownSpell::read(&mut r)?);
            }
            cooldowns
        };

        Ok(Self {
            unknown1,
            initial_spells,
            cooldowns,
        })
    }

}

impl crate::Message for SMSG_INITIAL_SPELLS {
    const OPCODE: u32 = 0x012a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_INITIAL_SPELLS"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // spell_count: u16
        w.write_all(&(self.initial_spells.len() as u16).to_le_bytes())?;

        // initial_spells: InitialSpell[spell_count]
        for i in self.initial_spells.iter() {
            i.write_into_vec(&mut w)?;
        }

        // cooldown_count: u16
        w.write_all(&(self.cooldowns.len() as u16).to_le_bytes())?;

        // cooldowns: CooldownSpell[cooldown_count]
        for i in self.cooldowns.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(298, "SMSG_INITIAL_SPELLS", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_INITIAL_SPELLS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INITIAL_SPELLS {}

impl SMSG_INITIAL_SPELLS {
    pub(crate) fn size(&self) -> usize {
        1 // unknown1: u8
        + 2 // spell_count: u16
        + self.initial_spells.len() * 4 // initial_spells: InitialSpell[spell_count]
        + 2 // cooldown_count: u16
        + self.cooldowns.len() * 14 // cooldowns: CooldownSpell[cooldown_count]
    }
}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_INITIAL_SPELLS;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_INITIAL_SPELLS, expected: &SMSG_INITIAL_SPELLS) {
        assert_eq!(t.unknown1, expected.unknown1);
        assert_eq!(t.initial_spells, expected.initial_spells);
        assert_eq!(t.cooldowns, expected.cooldowns);
    }

    const RAW0: [u8; 169] = [ 0x00, 0xA7, 0x2A, 0x01, 0x00, 0x28, 0x00, 0x4E, 0x00,
         0x00, 0x00, 0x51, 0x00, 0x00, 0x00, 0x6B, 0x00, 0x00, 0x00, 0xC4, 0x00,
         0x00, 0x00, 0xC6, 0x00, 0x00, 0x00, 0xC9, 0x00, 0x00, 0x00, 0xCB, 0x00,
         0x00, 0x00, 0xCC, 0x00, 0x00, 0x00, 0x0A, 0x02, 0x00, 0x00, 0x9C, 0x02,
         0x00, 0x00, 0x4E, 0x09, 0x00, 0x00, 0x99, 0x09, 0x00, 0x00, 0xAF, 0x09,
         0x00, 0x00, 0xEA, 0x0B, 0x00, 0x00, 0x25, 0x0D, 0x00, 0x00, 0xB5, 0x14,
         0x00, 0x00, 0x59, 0x18, 0x00, 0x00, 0x66, 0x18, 0x00, 0x00, 0x67, 0x18,
         0x00, 0x00, 0x4D, 0x19, 0x00, 0x00, 0x4E, 0x19, 0x00, 0x00, 0xCB, 0x19,
         0x00, 0x00, 0x62, 0x1C, 0x00, 0x00, 0x63, 0x1C, 0x00, 0x00, 0xBB, 0x1C,
         0x00, 0x00, 0xC2, 0x20, 0x00, 0x00, 0x21, 0x22, 0x00, 0x00, 0x75, 0x23,
         0x00, 0x00, 0x76, 0x23, 0x00, 0x00, 0x9C, 0x23, 0x00, 0x00, 0xA5, 0x23,
         0x00, 0x00, 0x75, 0x50, 0x00, 0x00, 0x76, 0x50, 0x00, 0x00, 0x77, 0x50,
         0x00, 0x00, 0x78, 0x50, 0x00, 0x00, 0x80, 0x51, 0x00, 0x00, 0x93, 0x54,
         0x00, 0x00, 0x94, 0x54, 0x00, 0x00, 0x0B, 0x56, 0x00, 0x00, 0x1A, 0x59,
         0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_INITIAL_SPELLS {
        SMSG_INITIAL_SPELLS {
            unknown1: 0x0,
            initial_spells: vec![
                InitialSpell {
                    spell_id: 0x4E,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x51,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x6B,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xC4,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xC6,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xC9,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xCB,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xCC,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x20A,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x29C,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x94E,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x999,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x9AF,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xBEA,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xD25,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x14B5,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1859,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1866,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1867,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x194D,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x194E,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x19CB,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1C62,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1C63,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1CBB,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x20C2,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x2221,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x2375,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x2376,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x239C,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x23A5,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5075,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5076,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5077,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5078,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5180,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5493,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5494,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x560B,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x591A,
                    unknown1: 0x0,
                },
            ],
            cooldowns: vec![ ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm` line 54.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_initial_spells0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_INITIAL_SPELLS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_INITIAL_SPELLS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm` line 54.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_initial_spells0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_INITIAL_SPELLS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_INITIAL_SPELLS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm` line 54.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_initial_spells0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_INITIAL_SPELLS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_INITIAL_SPELLS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_INITIAL_SPELLS;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_INITIAL_SPELLS, expected: &SMSG_INITIAL_SPELLS) {
        assert_eq!(t.unknown1, expected.unknown1);
        assert_eq!(t.initial_spells, expected.initial_spells);
        assert_eq!(t.cooldowns, expected.cooldowns);
    }

    const RAW0: [u8; 169] = [ 0x00, 0xA7, 0x2A, 0x01, 0x00, 0x28, 0x00, 0x4E, 0x00,
         0x00, 0x00, 0x51, 0x00, 0x00, 0x00, 0x6B, 0x00, 0x00, 0x00, 0xC4, 0x00,
         0x00, 0x00, 0xC6, 0x00, 0x00, 0x00, 0xC9, 0x00, 0x00, 0x00, 0xCB, 0x00,
         0x00, 0x00, 0xCC, 0x00, 0x00, 0x00, 0x0A, 0x02, 0x00, 0x00, 0x9C, 0x02,
         0x00, 0x00, 0x4E, 0x09, 0x00, 0x00, 0x99, 0x09, 0x00, 0x00, 0xAF, 0x09,
         0x00, 0x00, 0xEA, 0x0B, 0x00, 0x00, 0x25, 0x0D, 0x00, 0x00, 0xB5, 0x14,
         0x00, 0x00, 0x59, 0x18, 0x00, 0x00, 0x66, 0x18, 0x00, 0x00, 0x67, 0x18,
         0x00, 0x00, 0x4D, 0x19, 0x00, 0x00, 0x4E, 0x19, 0x00, 0x00, 0xCB, 0x19,
         0x00, 0x00, 0x62, 0x1C, 0x00, 0x00, 0x63, 0x1C, 0x00, 0x00, 0xBB, 0x1C,
         0x00, 0x00, 0xC2, 0x20, 0x00, 0x00, 0x21, 0x22, 0x00, 0x00, 0x75, 0x23,
         0x00, 0x00, 0x76, 0x23, 0x00, 0x00, 0x9C, 0x23, 0x00, 0x00, 0xA5, 0x23,
         0x00, 0x00, 0x75, 0x50, 0x00, 0x00, 0x76, 0x50, 0x00, 0x00, 0x77, 0x50,
         0x00, 0x00, 0x78, 0x50, 0x00, 0x00, 0x80, 0x51, 0x00, 0x00, 0x93, 0x54,
         0x00, 0x00, 0x94, 0x54, 0x00, 0x00, 0x0B, 0x56, 0x00, 0x00, 0x1A, 0x59,
         0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_INITIAL_SPELLS {
        SMSG_INITIAL_SPELLS {
            unknown1: 0x0,
            initial_spells: vec![
                InitialSpell {
                    spell_id: 0x4E,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x51,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x6B,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xC4,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xC6,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xC9,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xCB,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xCC,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x20A,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x29C,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x94E,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x999,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x9AF,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xBEA,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0xD25,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x14B5,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1859,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1866,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1867,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x194D,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x194E,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x19CB,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1C62,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1C63,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x1CBB,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x20C2,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x2221,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x2375,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x2376,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x239C,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x23A5,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5075,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5076,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5077,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5078,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5180,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5493,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x5494,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x560B,
                    unknown1: 0x0,
                },
                InitialSpell {
                    spell_id: 0x591A,
                    unknown1: 0x0,
                },
            ],
            cooldowns: vec![ ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm` line 54.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_initial_spells0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_INITIAL_SPELLS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_INITIAL_SPELLS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm` line 54.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_initial_spells0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_INITIAL_SPELLS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_INITIAL_SPELLS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm` line 54.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_initial_spells0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_INITIAL_SPELLS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_INITIAL_SPELLS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

