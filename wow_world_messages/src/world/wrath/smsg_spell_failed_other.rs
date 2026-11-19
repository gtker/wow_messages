use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::SpellCastResult;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_failed_other.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_failed_other.wowm#L8):
/// ```text
/// smsg SMSG_SPELL_FAILED_OTHER = 0x02A6 {
///     PackedGuid caster;
///     u8 extra_casts;
///     Spell id;
///     SpellCastResult result;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SPELL_FAILED_OTHER {
    pub caster: Guid,
    pub extra_casts: u8,
    pub id: u32,
    pub result: SpellCastResult,
}

impl crate::private::Sealed for SMSG_SPELL_FAILED_OTHER {}
impl SMSG_SPELL_FAILED_OTHER {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(7..=15).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // extra_casts: u8
        let extra_casts = crate::util::read_u8_le(&mut r)?;

        // id: Spell
        let id = crate::util::read_u32_le(&mut r)?;

        // result: SpellCastResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            caster,
            extra_casts,
            id,
            result,
        })
    }

}

impl crate::Message for SMSG_SPELL_FAILED_OTHER {
    const OPCODE: u32 = 0x02a6;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SPELL_FAILED_OTHER"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // caster: PackedGuid
        crate::util::write_packed_guid(&self.caster, &mut w)?;

        // extra_casts: u8
        w.write_all(&self.extra_casts.to_le_bytes())?;

        // id: Spell
        w.write_all(&self.id.to_le_bytes())?;

        // result: SpellCastResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(678, "SMSG_SPELL_FAILED_OTHER", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELL_FAILED_OTHER {}

impl SMSG_SPELL_FAILED_OTHER {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 1 // extra_casts: u8
        + 4 // id: Spell
        + 1 // result: SpellCastResult
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_SPELL_FAILED_OTHER;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 12] = [ 0x00, 0x0A, 0xA6, 0x02, 0x01, 0x01, 0x00, 0x85, 0x00,
         0x00, 0x00, 0x28, ];

    pub(crate) fn expected0() -> SMSG_SPELL_FAILED_OTHER {
        SMSG_SPELL_FAILED_OTHER {
            caster: Guid::new(0x1),
            extra_casts: 0x0,
            id: 0x85,
            result: SpellCastResult::Interrupted,
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spell_failed_other.wowm` line 17.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_spell_failed_other0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELL_FAILED_OTHER(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELL_FAILED_OTHER, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spell_failed_other.wowm` line 17.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_spell_failed_other0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELL_FAILED_OTHER(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELL_FAILED_OTHER, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spell_failed_other.wowm` line 17.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_spell_failed_other0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELL_FAILED_OTHER(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELL_FAILED_OTHER, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

