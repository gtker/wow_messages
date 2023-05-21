use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/threat/smsg_threat_remove.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/threat/smsg_threat_remove.wowm#L1):
/// ```text
/// smsg SMSG_THREAT_REMOVE = 0x0484 {
///     PackedGuid unit;
///     PackedGuid victim;
/// }
/// ```
pub struct SMSG_THREAT_REMOVE {
    pub unit: Guid,
    pub victim: Guid,
}

impl crate::private::Sealed for SMSG_THREAT_REMOVE {}
impl SMSG_THREAT_REMOVE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=18).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

        // victim: PackedGuid
        let victim = crate::util::read_packed_guid(&mut r)?;

        Ok(Self {
            unit,
            victim,
        })
    }

}

impl crate::Message for SMSG_THREAT_REMOVE {
    const OPCODE: u32 = 0x0484;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_THREAT_REMOVE {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        writeln!(s, "    victim = {};", self.victim.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1156_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.unit), "unit", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.victim), "victim", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

        // victim: PackedGuid
        crate::util::write_packed_guid(&self.victim, &mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1156, "SMSG_THREAT_REMOVE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_THREAT_REMOVE {}

impl SMSG_THREAT_REMOVE {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + crate::util::packed_guid_size(&self.victim) // victim: PackedGuid
    }
}

