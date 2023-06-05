use std::io::{Read, Write};

use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Not implemented in any Wrath emulators.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_dismiss_sound.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_dismiss_sound.wowm#L3):
/// ```text
/// smsg SMSG_PET_DISMISS_SOUND = 0x0325 {
///     u32 sound_id;
///     Vector3d position;
/// }
/// ```
pub struct SMSG_PET_DISMISS_SOUND {
    pub sound_id: u32,
    pub position: Vector3d,
}

#[cfg(feature = "print-testcase")]
impl SMSG_PET_DISMISS_SOUND {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PET_DISMISS_SOUND {{").unwrap();
        // Members
        writeln!(s, "    sound_id = {};", self.sound_id).unwrap();
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.position.x.to_string().contains(".") { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "    {}", if self.position.y.to_string().contains(".") { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();
        writeln!(s, "    {}", if self.position.z.to_string().contains(".") { self.position.z.to_string() } else { format!("{}.0", self.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 20_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 805_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "sound_id");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_PET_DISMISS_SOUND {}
impl crate::Message for SMSG_PET_DISMISS_SOUND {
    const OPCODE: u32 = 0x0325;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0325, size: body_size });
        }

        // sound_id: u32
        let sound_id = crate::util::read_u32_le(&mut r)?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        Ok(Self {
            sound_id,
            position,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PET_DISMISS_SOUND {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PET_DISMISS_SOUND {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_DISMISS_SOUND {}

