use std::io::{Read, Write};

use crate::wrath::{
    Area, Map, Vector3d,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Inform the client of a their hearthstone location.
///
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

#[cfg(feature = "print-testcase")]
impl SMSG_BINDPOINTUPDATE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BINDPOINTUPDATE {{").unwrap();
        // Members
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.position.x.to_string().contains(".") { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "    {}", if self.position.y.to_string().contains(".") { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();
        writeln!(s, "    {}", if self.position.z.to_string().contains(".") { self.position.z.to_string() } else { format!("{}.0", self.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    area = {};", self.area.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 24_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 341_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_BINDPOINTUPDATE {}
impl crate::Message for SMSG_BINDPOINTUPDATE {
    const OPCODE: u32 = 0x0155;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0155, size: body_size });
        }

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BINDPOINTUPDATE {}

