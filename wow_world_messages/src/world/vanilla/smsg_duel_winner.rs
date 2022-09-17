use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::DuelWinnerReason;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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

impl crate::Message for SMSG_DUEL_WINNER {
    const OPCODE: u32 = 0x016b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reason: DuelWinnerReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        // opponent_name: CString
        // Guard against strings that are already null-terminated
        assert_ne!(self.opponent_name.as_bytes().iter().rev().next(), Some(&0u8), "String opponent_name must not be null-terminated.");
        w.write_all(self.opponent_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // initiator_name: CString
        // Guard against strings that are already null-terminated
        assert_ne!(self.initiator_name.as_bytes().iter().rev().next(), Some(&0u8), "String initiator_name must not be null-terminated.");
        w.write_all(self.initiator_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // reason: DuelWinnerReason
        let reason: DuelWinnerReason = crate::util::read_u8_le(r)?.try_into()?;

        // opponent_name: CString
        let opponent_name = crate::util::read_c_string_to_vec(r)?;
        let opponent_name = String::from_utf8(opponent_name)?;

        // initiator_name: CString
        let initiator_name = crate::util::read_c_string_to_vec(r)?;
        let initiator_name = String::from_utf8(initiator_name)?;

        Ok(Self {
            reason,
            opponent_name,
            initiator_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_DUEL_WINNER {}

impl SMSG_DUEL_WINNER {
    pub(crate) fn size(&self) -> usize {
        1 // reason: DuelWinnerReason
        + self.opponent_name.len() + 1 // opponent_name: CString
        + self.initiator_name.len() + 1 // initiator_name: CString
    }
}

