use std::io::{Read, Write};

use wow_world_base::shared::raid_group_error_vanilla_tbc_wrath::RaidGroupError;

/// used when player leaves raid group inside instance
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_raid_group_only.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_raid_group_only.wowm#L9):
/// ```text
/// smsg SMSG_RAID_GROUP_ONLY = 0x0286 {
///     u32 homebind_timer;
///     RaidGroupError error;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_RAID_GROUP_ONLY {
    pub homebind_timer: u32,
    pub error: RaidGroupError,
}

impl crate::private::Sealed for SMSG_RAID_GROUP_ONLY {}
impl SMSG_RAID_GROUP_ONLY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // homebind_timer: u32
        let homebind_timer = crate::util::read_u32_le(&mut r)?;

        // error: RaidGroupError
        let error = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            homebind_timer,
            error,
        })
    }

}

impl crate::Message for SMSG_RAID_GROUP_ONLY {
    const OPCODE: u32 = 0x0286;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_RAID_GROUP_ONLY"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_RAID_GROUP_ONLY {{").unwrap();
        // Members
        writeln!(s, "    homebind_timer = {};", self.homebind_timer).unwrap();
        writeln!(s, "    error = {};", self.error.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 646_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "homebind_timer", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "error", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // homebind_timer: u32
        w.write_all(&self.homebind_timer.to_le_bytes())?;

        // error: RaidGroupError
        w.write_all(&(self.error.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(646, "SMSG_RAID_GROUP_ONLY", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_RAID_GROUP_ONLY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_RAID_GROUP_ONLY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_RAID_GROUP_ONLY {}

