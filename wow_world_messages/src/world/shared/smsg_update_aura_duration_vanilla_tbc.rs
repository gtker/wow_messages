use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_update_aura_duration.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_update_aura_duration.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_AURA_DURATION = 0x0137 {
///     u8 aura_slot;
///     u32 aura_duration;
/// }
/// ```
pub struct SMSG_UPDATE_AURA_DURATION {
    pub aura_slot: u8,
    pub aura_duration: u32,
}

impl crate::private::Sealed for SMSG_UPDATE_AURA_DURATION {}
impl SMSG_UPDATE_AURA_DURATION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 5 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // aura_slot: u8
        let aura_slot = crate::util::read_u8_le(&mut r)?;

        // aura_duration: u32
        let aura_duration = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            aura_slot,
            aura_duration,
        })
    }

}

impl crate::Message for SMSG_UPDATE_AURA_DURATION {
    const OPCODE: u32 = 0x0137;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_UPDATE_AURA_DURATION"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_AURA_DURATION {{").unwrap();
        // Members
        writeln!(s, "    aura_slot = {};", self.aura_slot).unwrap();
        writeln!(s, "    aura_duration = {};", self.aura_duration).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 311_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "aura_slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "aura_duration", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // aura_slot: u8
        w.write_all(&self.aura_slot.to_le_bytes())?;

        // aura_duration: u32
        w.write_all(&self.aura_duration.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(311, "SMSG_UPDATE_AURA_DURATION", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_UPDATE_AURA_DURATION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_UPDATE_AURA_DURATION {}

