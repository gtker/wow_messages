use std::io::{Read, Write};

use crate::tbc::{
    Map, Vector3d,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_death_release_loc.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_death_release_loc.wowm#L1):
/// ```text
/// smsg SMSG_DEATH_RELEASE_LOC = 0x0378 {
///     Map map;
///     Vector3d position;
/// }
/// ```
pub struct SMSG_DEATH_RELEASE_LOC {
    pub map: Map,
    pub position: Vector3d,
}

impl crate::private::Sealed for SMSG_DEATH_RELEASE_LOC {}
impl SMSG_DEATH_RELEASE_LOC {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        Ok(Self {
            map,
            position,
        })
    }

}

impl crate::Message for SMSG_DEATH_RELEASE_LOC {
    const OPCODE: u32 = 0x0378;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_DEATH_RELEASE_LOC {{").unwrap();
        // Members
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.position.x.to_string().contains('.') { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "    {}", if self.position.y.to_string().contains('.') { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();
        writeln!(s, "    {}", if self.position.z.to_string().contains('.') { self.position.z.to_string() } else { format!("{}.0", self.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 18_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 888_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        writeln!(s, "    /* position: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
        writeln!(s, "    /* position: Vector3d end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(888, "SMSG_DEATH_RELEASE_LOC", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DEATH_RELEASE_LOC {}

