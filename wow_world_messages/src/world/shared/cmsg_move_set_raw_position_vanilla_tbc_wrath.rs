use std::io::{Read, Write};

use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// vmangos/mangoszero: write in client console: setrawpos x y z o. For now, it is implemented like worldport but on the same map. Consider using `MSG_MOVE_SET_RAW_POSITION_ACK`.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_raw_position.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_raw_position.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_SET_RAW_POSITION = 0x00E1 {
///     Vector3d position;
///     f32 orientation;
/// }
/// ```
pub struct CMSG_MOVE_SET_RAW_POSITION {
    pub position: Vector3d,
    pub orientation: f32,
}

#[cfg(feature = "print-testcase")]
impl CMSG_MOVE_SET_RAW_POSITION {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_MOVE_SET_RAW_POSITION {{").unwrap();
        // Members
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.position.x.to_string().contains(".") { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "    {}", if self.position.y.to_string().contains(".") { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();
        writeln!(s, "    {}", if self.position.z.to_string().contains(".") { self.position.z.to_string() } else { format!("{}.0", self.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    {}", if self.orientation.to_string().contains(".") { self.orientation.to_string() } else { format!("{}.0", self.orientation) }).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 22_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 225_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
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
        writeln!(s, "    versions = \"1.12 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_MOVE_SET_RAW_POSITION {}
impl crate::Message for CMSG_MOVE_SET_RAW_POSITION {
    const OPCODE: u32 = 0x00e1;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00E1, size: body_size });
        }

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            position,
            orientation,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MOVE_SET_RAW_POSITION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_MOVE_SET_RAW_POSITION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MOVE_SET_RAW_POSITION {}

