use std::io::{Read, Write};

use crate::tbc::{
    Area, Map, Vector3d,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Inform the client of a their hearthstone location.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_bindpointupdate.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_bindpointupdate.wowm#L1):
/// ```text
/// smsg SMSG_BINDPOINTUPDATE = 0x0155 {
///     Vector3d position;
///     Map map;
///     Area area;
/// }
/// ```
pub struct SMSG_BINDPOINTUPDATE {
    pub position: Vector3d,
    pub map: Map,
    pub area: Area,
}

impl crate::private::Sealed for SMSG_BINDPOINTUPDATE {}
impl SMSG_BINDPOINTUPDATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 20 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // position: Vector3d
        let position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            position,
            map,
            area,
        })
    }

}

impl crate::Message for SMSG_BINDPOINTUPDATE {
    const OPCODE: u32 = 0x0155;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_BINDPOINTUPDATE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BINDPOINTUPDATE {{").unwrap();
        // Members
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.position.x.to_string().contains('.') { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "    {}", if self.position.y.to_string().contains('.') { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();
        writeln!(s, "    {}", if self.position.z.to_string().contains('.') { self.position.z.to_string() } else { format!("{}.0", self.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    area = {};", self.area.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 22_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 341_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    /* position: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
        writeln!(s, "    /* position: Vector3d end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // position: Vector3d
crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.position, &mut w)?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(341, "SMSG_BINDPOINTUPDATE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_BINDPOINTUPDATE {}

