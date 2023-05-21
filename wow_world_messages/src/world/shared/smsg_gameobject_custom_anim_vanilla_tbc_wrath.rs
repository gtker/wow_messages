use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_gameobject_custom_anim.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_gameobject_custom_anim.wowm#L3):
/// ```text
/// smsg SMSG_GAMEOBJECT_CUSTOM_ANIM = 0x00B3 {
///     Guid guid;
///     u32 animation_id;
/// }
/// ```
pub struct SMSG_GAMEOBJECT_CUSTOM_ANIM {
    pub guid: Guid,
    pub animation_id: u32,
}

impl crate::private::Sealed for SMSG_GAMEOBJECT_CUSTOM_ANIM {}
impl SMSG_GAMEOBJECT_CUSTOM_ANIM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // animation_id: u32
        let animation_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            animation_id,
        })
    }

}

impl crate::Message for SMSG_GAMEOBJECT_CUSTOM_ANIM {
    const OPCODE: u32 = 0x00b3;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GAMEOBJECT_CUSTOM_ANIM {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    animation_id = {};", self.animation_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 179_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "animation_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // animation_id: u32
        w.write_all(&self.animation_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(179, "SMSG_GAMEOBJECT_CUSTOM_ANIM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GAMEOBJECT_CUSTOM_ANIM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GAMEOBJECT_CUSTOM_ANIM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GAMEOBJECT_CUSTOM_ANIM {}

