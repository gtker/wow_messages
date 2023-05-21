use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_instance_save_created.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_save_created.wowm#L3):
/// ```text
/// smsg SMSG_INSTANCE_SAVE_CREATED = 0x02CB {
///     u32 unknown;
/// }
/// ```
pub struct SMSG_INSTANCE_SAVE_CREATED {
    /// All emulators across all versions set to 0
    pub unknown: u32,
}

impl crate::private::Sealed for SMSG_INSTANCE_SAVE_CREATED {}
impl SMSG_INSTANCE_SAVE_CREATED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unknown,
        })
    }

}

impl crate::Message for SMSG_INSTANCE_SAVE_CREATED {
    const OPCODE: u32 = 0x02cb;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_INSTANCE_SAVE_CREATED {{").unwrap();
        // Members
        writeln!(s, "    unknown = {};", self.unknown).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 715_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(715, "SMSG_INSTANCE_SAVE_CREATED", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_INSTANCE_SAVE_CREATED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INSTANCE_SAVE_CREATED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_INSTANCE_SAVE_CREATED {}

