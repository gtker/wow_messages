use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::Vector3d;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_set_projectile_position.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_set_projectile_position.wowm#L1):
/// ```text
/// smsg SMSG_SET_PROJECTILE_POSITION = 0x04BF {
///     Guid caster;
///     u8 amount_of_casts;
///     Vector3d position;
/// }
/// ```
pub struct SMSG_SET_PROJECTILE_POSITION {
    pub caster: Guid,
    pub amount_of_casts: u8,
    pub position: Vector3d,
}

impl crate::private::Sealed for SMSG_SET_PROJECTILE_POSITION {}
impl SMSG_SET_PROJECTILE_POSITION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 21 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x04BF, size: body_size });
        }

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // amount_of_casts: u8
        let amount_of_casts = crate::util::read_u8_le(&mut r)?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        Ok(Self {
            caster,
            amount_of_casts,
            position,
        })
    }

}

impl crate::Message for SMSG_SET_PROJECTILE_POSITION {
    const OPCODE: u32 = 0x04bf;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SET_PROJECTILE_POSITION {{").unwrap();
        // Members
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    amount_of_casts = {};", self.amount_of_casts).unwrap();
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.position.x.to_string().contains('.') { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "    {}", if self.position.y.to_string().contains('.') { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();
        writeln!(s, "    {}", if self.position.z.to_string().contains('.') { self.position.z.to_string() } else { format!("{}.0", self.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 23_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1215_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_casts", "    ");
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
        21
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // amount_of_casts: u8
        w.write_all(&self.amount_of_casts.to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SET_PROJECTILE_POSITION {}

