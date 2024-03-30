use std::io::{Read, Write};

use crate::tbc::{
    InstanceResetFailedReason, Map,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm#L13):
/// ```text
/// smsg SMSG_INSTANCE_RESET_FAILED = 0x031F {
///     (u32)InstanceResetFailedReason reason;
///     Map map;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_INSTANCE_RESET_FAILED {
    pub reason: InstanceResetFailedReason,
    pub map: Map,
}

impl crate::private::Sealed for SMSG_INSTANCE_RESET_FAILED {}
impl SMSG_INSTANCE_RESET_FAILED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // reason: InstanceResetFailedReason
        let reason = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            reason,
            map,
        })
    }

}

impl crate::Message for SMSG_INSTANCE_RESET_FAILED {
    const OPCODE: u32 = 0x031f;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_INSTANCE_RESET_FAILED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_INSTANCE_RESET_FAILED {{").unwrap();
        // Members
        writeln!(s, "    reason = {};", self.reason.as_test_case_value()).unwrap();
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 799_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "reason", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // reason: InstanceResetFailedReason
        w.write_all(&u32::from(self.reason.as_int()).to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(799, "SMSG_INSTANCE_RESET_FAILED", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INSTANCE_RESET_FAILED {}

