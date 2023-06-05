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

#[cfg(feature = "print-testcase")]
impl SMSG_DUEL_WINNER {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_DUEL_WINNER {{").unwrap();
        // Members
        writeln!(s, "    reason = {};", self.reason.as_test_case_value()).unwrap();
        writeln!(s, "    opponent_name = \"{}\";", self.opponent_name).unwrap();
        writeln!(s, "    initiator_name = \"{}\";", self.initiator_name).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 363_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "reason", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.opponent_name.len() + 1, "opponent_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.initiator_name.len() + 1, "initiator_name", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_DUEL_WINNER {}
impl crate::Message for SMSG_DUEL_WINNER {
    const OPCODE: u32 = 0x016b;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_DUEL_WINNER::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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
        let reason = crate::util::read_u8_le(&mut r)?.try_into()?;

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

