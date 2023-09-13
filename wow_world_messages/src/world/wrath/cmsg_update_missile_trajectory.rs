use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::Vector3d;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_update_missile_trajectory.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_update_missile_trajectory.wowm#L1):
/// ```text
/// cmsg CMSG_UPDATE_MISSILE_TRAJECTORY = 0x0462 {
///     Guid guid;
///     u32 spell;
///     f32 elevation;
///     f32 speed;
///     Vector3d position;
///     Vector3d target;
/// }
/// ```
pub struct CMSG_UPDATE_MISSILE_TRAJECTORY {
    pub guid: Guid,
    pub spell: u32,
    pub elevation: f32,
    pub speed: f32,
    pub position: Vector3d,
    pub target: Vector3d,
}

impl crate::private::Sealed for CMSG_UPDATE_MISSILE_TRAJECTORY {}
impl CMSG_UPDATE_MISSILE_TRAJECTORY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 44 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // elevation: f32
        let elevation = crate::util::read_f32_le(&mut r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(&mut r)?;

        // position: Vector3d
        let position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        // target: Vector3d
        let target = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        Ok(Self {
            guid,
            spell,
            elevation,
            speed,
            position,
            target,
        })
    }

}

impl crate::Message for CMSG_UPDATE_MISSILE_TRAJECTORY {
    const OPCODE: u32 = 0x0462;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_UPDATE_MISSILE_TRAJECTORY"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_UPDATE_MISSILE_TRAJECTORY {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    elevation = {}", if self.elevation.to_string().contains('.') { self.elevation.to_string() } else { format!("{}.0", self.elevation) }).unwrap();
        writeln!(s, "    speed = {}", if self.speed.to_string().contains('.') { self.speed.to_string() } else { format!("{}.0", self.speed) }).unwrap();
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "        x = {}", if self.position.x.to_string().contains('.') { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "        y = {}", if self.position.y.to_string().contains('.') { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();
        writeln!(s, "        z = {}", if self.position.z.to_string().contains('.') { self.position.z.to_string() } else { format!("{}.0", self.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        // target: Vector3d
        writeln!(s, "    target = {{").unwrap();
        // Members
        writeln!(s, "        x = {}", if self.target.x.to_string().contains('.') { self.target.x.to_string() } else { format!("{}.0", self.target.x) }).unwrap();
        writeln!(s, "        y = {}", if self.target.y.to_string().contains('.') { self.target.y.to_string() } else { format!("{}.0", self.target.y) }).unwrap();
        writeln!(s, "        z = {}", if self.target.z.to_string().contains('.') { self.target.z.to_string() } else { format!("{}.0", self.target.z) }).unwrap();

        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 48_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1122_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "elevation", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "speed", "    ");
        writeln!(s, "    /* position: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
        writeln!(s, "    /* position: Vector3d end */").unwrap();
        writeln!(s, "    /* target: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
        writeln!(s, "    /* target: Vector3d end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        44
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // elevation: f32
        w.write_all(&self.elevation.to_le_bytes())?;

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        // position: Vector3d
crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.position, &mut w)?;

        // target: Vector3d
crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.target, &mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1122, "CMSG_UPDATE_MISSILE_TRAJECTORY", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_UPDATE_MISSILE_TRAJECTORY {}

