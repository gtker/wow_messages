use std::io::{Read, Write};

use wow_world_base::shared::duel_winner_reason_vanilla_tbc_wrath::DuelWinnerReason;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_winner.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_winner.wowm#L8):
/// ```text
/// smsg SMSG_DUEL_WINNER = 0x016B {
///     DuelWinnerReason reason;
///     CString opponent_name;
///     CString initiator_name;
/// }
/// ```
pub struct SMSG_DUEL_WINNER {
    pub reason: DuelWinnerReason,
    pub opponent_name: String,
    pub initiator_name: String,
}

impl crate::private::Sealed for SMSG_DUEL_WINNER {}
impl crate::Message for SMSG_DUEL_WINNER {
    const OPCODE: u32 = 0x016b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // reason: DuelWinnerReason
        w.write_all(&(self.reason.as_int().to_le_bytes()))?;

        // opponent_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.opponent_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `opponent_name` must not be null-terminated.");
        w.write_all(self.opponent_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // initiator_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.initiator_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `initiator_name` must not be null-terminated.");
        w.write_all(self.initiator_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(3..=513).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x016B, size: body_size });
        }

        // reason: DuelWinnerReason
        let reason: DuelWinnerReason = crate::util::read_u8_le(&mut r)?.try_into()?;

        // opponent_name: CString
        let opponent_name = {
            let opponent_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(opponent_name)?
        };

        // initiator_name: CString
        let initiator_name = {
            let initiator_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(initiator_name)?
        };

        Ok(Self {
            reason,
            opponent_name,
            initiator_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_DUEL_WINNER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DUEL_WINNER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DUEL_WINNER {}

impl SMSG_DUEL_WINNER {
    pub(crate) fn size(&self) -> usize {
        1 // reason: DuelWinnerReason
        + self.opponent_name.len() + 1 // opponent_name: CString
        + self.initiator_name.len() + 1 // initiator_name: CString
    }
}

