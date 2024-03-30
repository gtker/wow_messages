use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_ejected.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_ejected.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_EJECTED = 0x04E6 {
///     u32 battle_id;
///     u8 reason;
///     u8 battle_status;
///     u8 relocated;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_BATTLEFIELD_MGR_EJECTED {
    pub battle_id: u32,
    pub reason: u8,
    pub battle_status: u8,
    pub relocated: u8,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_MGR_EJECTED {}
impl SMSG_BATTLEFIELD_MGR_EJECTED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 7 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(&mut r)?;

        // reason: u8
        let reason = crate::util::read_u8_le(&mut r)?;

        // battle_status: u8
        let battle_status = crate::util::read_u8_le(&mut r)?;

        // relocated: u8
        let relocated = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            battle_id,
            reason,
            battle_status,
            relocated,
        })
    }

}

impl crate::Message for SMSG_BATTLEFIELD_MGR_EJECTED {
    const OPCODE: u32 = 0x04e6;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_BATTLEFIELD_MGR_EJECTED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BATTLEFIELD_MGR_EJECTED {{").unwrap();
        // Members
        writeln!(s, "    battle_id = {};", self.battle_id).unwrap();
        writeln!(s, "    reason = {};", self.reason).unwrap();
        writeln!(s, "    battle_status = {};", self.battle_status).unwrap();
        writeln!(s, "    relocated = {};", self.relocated).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 9_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1254_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "battle_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "reason", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "battle_status", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "relocated", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        7
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // reason: u8
        w.write_all(&self.reason.to_le_bytes())?;

        // battle_status: u8
        w.write_all(&self.battle_status.to_le_bytes())?;

        // relocated: u8
        w.write_all(&self.relocated.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1254, "SMSG_BATTLEFIELD_MGR_EJECTED", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_EJECTED {}

