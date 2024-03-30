use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/smsg_mountspecial_anim.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/smsg_mountspecial_anim.wowm#L3):
/// ```text
/// smsg SMSG_MOUNTSPECIAL_ANIM = 0x0172 {
///     Guid guid;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_MOUNTSPECIAL_ANIM {
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_MOUNTSPECIAL_ANIM {}
impl SMSG_MOUNTSPECIAL_ANIM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}

impl crate::Message for SMSG_MOUNTSPECIAL_ANIM {
    const OPCODE: u32 = 0x0172;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MOUNTSPECIAL_ANIM"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MOUNTSPECIAL_ANIM {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 370_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(370, "SMSG_MOUNTSPECIAL_ANIM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MOUNTSPECIAL_ANIM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOUNTSPECIAL_ANIM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOUNTSPECIAL_ANIM {}

