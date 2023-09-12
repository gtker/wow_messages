use std::io::{Read, Write};

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_instance_lock_warning_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_instance_lock_warning_query.wowm#L1):
/// ```text
/// smsg SMSG_INSTANCE_LOCK_WARNING_QUERY = 0x0147 {
///     Milliseconds time;
///     u32 encounter_mask;
///     u8 unknown;
/// }
/// ```
pub struct SMSG_INSTANCE_LOCK_WARNING_QUERY {
    pub time: Duration,
    pub encounter_mask: u32,
    pub unknown: u8,
}

impl crate::private::Sealed for SMSG_INSTANCE_LOCK_WARNING_QUERY {}
impl SMSG_INSTANCE_LOCK_WARNING_QUERY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // time: Milliseconds
        let time = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        // encounter_mask: u32
        let encounter_mask = crate::util::read_u32_le(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            time,
            encounter_mask,
            unknown,
        })
    }

}

impl crate::Message for SMSG_INSTANCE_LOCK_WARNING_QUERY {
    const OPCODE: u32 = 0x0147;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_INSTANCE_LOCK_WARNING_QUERY"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_INSTANCE_LOCK_WARNING_QUERY {{").unwrap();
        // Members
        writeln!(s, "    time = {};", self.time.as_millis()).unwrap();
        writeln!(s, "    encounter_mask = {};", self.encounter_mask).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 11_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 327_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "encounter_mask", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // time: Milliseconds
        w.write_all((self.time.as_millis() as u32).to_le_bytes().as_slice())?;

        // encounter_mask: u32
        w.write_all(&self.encounter_mask.to_le_bytes())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(327, "SMSG_INSTANCE_LOCK_WARNING_QUERY", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_INSTANCE_LOCK_WARNING_QUERY {}

