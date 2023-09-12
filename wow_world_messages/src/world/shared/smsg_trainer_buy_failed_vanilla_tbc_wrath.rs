use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::training_failure_reason_vanilla_tbc_wrath::TrainingFailureReason;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// No TBC emulators implement this.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_buy_failed.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_buy_failed.wowm#L18):
/// ```text
/// smsg SMSG_TRAINER_BUY_FAILED = 0x01B4 {
///     Guid guid;
///     u32 id;
///     TrainingFailureReason error;
/// }
/// ```
pub struct SMSG_TRAINER_BUY_FAILED {
    pub guid: Guid,
    pub id: u32,
    pub error: TrainingFailureReason,
}

impl crate::private::Sealed for SMSG_TRAINER_BUY_FAILED {}
impl SMSG_TRAINER_BUY_FAILED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // error: TrainingFailureReason
        let error = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            id,
            error,
        })
    }

}

impl crate::Message for SMSG_TRAINER_BUY_FAILED {
    const OPCODE: u32 = 0x01b4;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_TRAINER_BUY_FAILED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TRAINER_BUY_FAILED {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    error = {};", self.error.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 18_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 436_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "error", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // error: TrainingFailureReason
        w.write_all(&(self.error.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(436, "SMSG_TRAINER_BUY_FAILED", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_TRAINER_BUY_FAILED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TRAINER_BUY_FAILED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TRAINER_BUY_FAILED {}

