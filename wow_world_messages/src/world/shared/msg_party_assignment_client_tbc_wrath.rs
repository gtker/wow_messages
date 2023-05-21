use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::party_role_tbc_wrath::PartyRole;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/msg_party_assignment.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/msg_party_assignment.wowm#L8):
/// ```text
/// cmsg MSG_PARTY_ASSIGNMENT_Client = 0x038E {
///     PartyRole role;
///     Bool apply;
///     Guid player;
/// }
/// ```
pub struct MSG_PARTY_ASSIGNMENT_Client {
    pub role: PartyRole,
    pub apply: bool,
    pub player: Guid,
}

impl crate::private::Sealed for MSG_PARTY_ASSIGNMENT_Client {}
impl MSG_PARTY_ASSIGNMENT_Client {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 10 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x038E, size: body_size });
        }

        // role: PartyRole
        let role = crate::util::read_u8_le(&mut r)?.try_into()?;

        // apply: Bool
        let apply = crate::util::read_u8_le(&mut r)? != 0;

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        Ok(Self {
            role,
            apply,
            player,
        })
    }

}

impl crate::Message for MSG_PARTY_ASSIGNMENT_Client {
    const OPCODE: u32 = 0x038e;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_PARTY_ASSIGNMENT_Client {{").unwrap();
        // Members
        writeln!(s, "    role = {};", self.role.as_test_case_value()).unwrap();
        writeln!(s, "    apply = {};", if self.apply { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    player = {};", self.player.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 910_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "role", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "apply", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        10
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // role: PartyRole
        w.write_all(&(self.role.as_int().to_le_bytes()))?;

        // apply: Bool
        w.write_all(u8::from(self.apply).to_le_bytes().as_slice())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_PARTY_ASSIGNMENT_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_PARTY_ASSIGNMENT_Client {}

