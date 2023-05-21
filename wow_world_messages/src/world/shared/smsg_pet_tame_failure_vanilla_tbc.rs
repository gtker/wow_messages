use std::io::{Read, Write};

use wow_world_base::shared::pet_tame_failure_reason_vanilla_tbc::PetTameFailureReason;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm#L24):
/// ```text
/// smsg SMSG_PET_TAME_FAILURE = 0x0173 {
///     PetTameFailureReason reason;
/// }
/// ```
pub struct SMSG_PET_TAME_FAILURE {
    pub reason: PetTameFailureReason,
}

impl crate::private::Sealed for SMSG_PET_TAME_FAILURE {}
impl SMSG_PET_TAME_FAILURE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // reason: PetTameFailureReason
        let reason = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            reason,
        })
    }

}

impl crate::Message for SMSG_PET_TAME_FAILURE {
    const OPCODE: u32 = 0x0173;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PET_TAME_FAILURE {{").unwrap();
        // Members
        writeln!(s, "    reason = {};", self.reason.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 3_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 371_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "reason", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // reason: PetTameFailureReason
        w.write_all(&(self.reason.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(371, "SMSG_PET_TAME_FAILURE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PET_TAME_FAILURE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PET_TAME_FAILURE {}

