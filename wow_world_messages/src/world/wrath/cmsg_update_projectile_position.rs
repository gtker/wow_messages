use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::Vector3d;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_update_projectile_position.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_update_projectile_position.wowm#L1):
/// ```text
/// cmsg CMSG_UPDATE_PROJECTILE_POSITION = 0x04BE {
///     Guid caster;
///     u32 spell;
///     u8 cast_count;
///     Vector3d position;
/// }
/// ```
pub struct CMSG_UPDATE_PROJECTILE_POSITION {
    pub caster: Guid,
    pub spell: u32,
    pub cast_count: u8,
    pub position: Vector3d,
}

impl crate::private::Sealed for CMSG_UPDATE_PROJECTILE_POSITION {}
impl CMSG_UPDATE_PROJECTILE_POSITION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 25 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x04BE, size: body_size });
        }

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // cast_count: u8
        let cast_count = crate::util::read_u8_le(&mut r)?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        Ok(Self {
            caster,
            spell,
            cast_count,
            position,
        })
    }

}

impl crate::Message for CMSG_UPDATE_PROJECTILE_POSITION {
    const OPCODE: u32 = 0x04be;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_UPDATE_PROJECTILE_POSITION {{").unwrap();
        // Members
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    cast_count = {};", self.cast_count).unwrap();
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.position.x.to_string().contains('.') { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "    {}", if self.position.y.to_string().contains('.') { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();
        writeln!(s, "    {}", if self.position.z.to_string().contains('.') { self.position.z.to_string() } else { format!("{}.0", self.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 29_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1214_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "cast_count", "    ");
        writeln!(s, "    /* position: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
        writeln!(s, "    /* position: Vector3d end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        25
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // cast_count: u8
        w.write_all(&self.cast_count.to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_UPDATE_PROJECTILE_POSITION {}

