use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_uninvite_guid.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_uninvite_guid.wowm#L7):
/// ```text
/// cmsg CMSG_GROUP_UNINVITE_GUID = 0x0076 {
///     Guid guid;
///     CString reason;
/// }
/// ```
pub struct CMSG_GROUP_UNINVITE_GUID {
    pub guid: Guid,
    pub reason: String,
}

impl crate::private::Sealed for CMSG_GROUP_UNINVITE_GUID {}
impl CMSG_GROUP_UNINVITE_GUID {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0076, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // reason: CString
        let reason = {
            let reason = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(reason)?
        };

        Ok(Self {
            guid,
            reason,
        })
    }

}

impl crate::Message for CMSG_GROUP_UNINVITE_GUID {
    const OPCODE: u32 = 0x0076;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GROUP_UNINVITE_GUID {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    reason = \"{}\";", self.reason).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 118_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.reason.len() + 1, "reason", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // reason: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.reason.as_bytes().iter().rev().next(), Some(&0_u8), "String `reason` must not be null-terminated.");
        w.write_all(self.reason.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GROUP_UNINVITE_GUID {}

impl CMSG_GROUP_UNINVITE_GUID {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.reason.len() + 1 // reason: CString
    }
}

