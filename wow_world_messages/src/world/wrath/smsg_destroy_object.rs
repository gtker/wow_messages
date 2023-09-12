use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Immediately removes an object from the presence of the player.
///
/// Used by vmangos for logout.
/// azerothcore: If the following bool is true, the client will call `void CGUnit_C::OnDeath()` for this object. `OnDeath()` does for eg trigger death animation and interrupts certain spells/missiles/auras/sounds...
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_destroy_object.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_destroy_object.wowm#L21):
/// ```text
/// smsg SMSG_DESTROY_OBJECT = 0x00AA {
///     Guid guid;
///     Bool target_died;
/// }
/// ```
pub struct SMSG_DESTROY_OBJECT {
    pub guid: Guid,
    pub target_died: bool,
}

impl crate::private::Sealed for SMSG_DESTROY_OBJECT {}
impl SMSG_DESTROY_OBJECT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // target_died: Bool
        let target_died = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            guid,
            target_died,
        })
    }

}

impl crate::Message for SMSG_DESTROY_OBJECT {
    const OPCODE: u32 = 0x00aa;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_DESTROY_OBJECT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_DESTROY_OBJECT {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    target_died = {};", if self.target_died { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 11_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 170_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "target_died", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // target_died: Bool
        w.write_all(u8::from(self.target_died).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(170, "SMSG_DESTROY_OBJECT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DESTROY_OBJECT {}

