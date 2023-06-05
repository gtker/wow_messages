use std::io::{Read, Write};

use wow_world_base::shared::vector2d_vanilla_tbc_wrath::Vector2d;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_gossip_poi.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_gossip_poi.wowm#L3):
/// ```text
/// smsg SMSG_GOSSIP_POI = 0x0224 {
///     u32 flags;
///     Vector2d position;
///     u32 icon;
///     u32 data;
///     CString location_name;
/// }
/// ```
pub struct SMSG_GOSSIP_POI {
    pub flags: u32,
    pub position: Vector2d,
    pub icon: u32,
    pub data: u32,
    pub location_name: String,
}

#[cfg(feature = "print-testcase")]
impl SMSG_GOSSIP_POI {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GOSSIP_POI {{").unwrap();
        // Members
        writeln!(s, "    flags = {};", self.flags).unwrap();
        // position: Vector2d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.position.x.to_string().contains(".") { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "    {}", if self.position.y.to_string().contains(".") { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    icon = {};", self.icon).unwrap();
        writeln!(s, "    data = {};", self.data).unwrap();
        writeln!(s, "    location_name = \"{}\";", self.location_name).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 548_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");
        writeln!(s, "    /* position: Vector2d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        writeln!(s, "    /* position: Vector2d end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "icon", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "data", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.location_name.len() + 1, "location_name", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_GOSSIP_POI {}
impl crate::Message for SMSG_GOSSIP_POI {
    const OPCODE: u32 = 0x0224;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_GOSSIP_POI::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // position: Vector2d
        self.position.write_into_vec(&mut w)?;

        // icon: u32
        w.write_all(&self.icon.to_le_bytes())?;

        // data: u32
        w.write_all(&self.data.to_le_bytes())?;

        // location_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.location_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `location_name` must not be null-terminated.");
        w.write_all(self.location_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(21..=276).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0224, size: body_size });
        }

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // position: Vector2d
        let position = Vector2d::read(&mut r)?;

        // icon: u32
        let icon = crate::util::read_u32_le(&mut r)?;

        // data: u32
        let data = crate::util::read_u32_le(&mut r)?;

        // location_name: CString
        let location_name = {
            let location_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(location_name)?
        };

        Ok(Self {
            flags,
            position,
            icon,
            data,
            location_name,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GOSSIP_POI {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GOSSIP_POI {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GOSSIP_POI {}

impl SMSG_GOSSIP_POI {
    pub(crate) fn size(&self) -> usize {
        4 // flags: u32
        + 8 // position: Vector2d
        + 4 // icon: u32
        + 4 // data: u32
        + self.location_name.len() + 1 // location_name: CString
    }
}

