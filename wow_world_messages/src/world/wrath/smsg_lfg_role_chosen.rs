use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_role_chosen.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_role_chosen.wowm#L1):
/// ```text
/// smsg SMSG_LFG_ROLE_CHOSEN = 0x02BB {
///     Guid guid;
///     Bool ready;
///     u32 roles;
/// }
/// ```
pub struct SMSG_LFG_ROLE_CHOSEN {
    pub guid: Guid,
    pub ready: bool,
    pub roles: u32,
}

impl crate::private::Sealed for SMSG_LFG_ROLE_CHOSEN {}
impl SMSG_LFG_ROLE_CHOSEN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 13 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // ready: Bool
        let ready = crate::util::read_u8_le(&mut r)? != 0;

        // roles: u32
        let roles = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            ready,
            roles,
        })
    }

}

impl crate::Message for SMSG_LFG_ROLE_CHOSEN {
    const OPCODE: u32 = 0x02bb;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LFG_ROLE_CHOSEN"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_ROLE_CHOSEN {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    ready = {};", if self.ready { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    roles = {};", self.roles).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 15_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 699_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "ready", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "roles", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // ready: Bool
        w.write_all(u8::from(self.ready).to_le_bytes().as_slice())?;

        // roles: u32
        w.write_all(&self.roles.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(699, "SMSG_LFG_ROLE_CHOSEN", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_ROLE_CHOSEN {}

