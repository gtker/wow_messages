use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Only exists as a comment in trinitycore/azerothcore.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/smsg_camera_shake.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/smsg_camera_shake.wowm#L1):
/// ```text
/// smsg SMSG_CAMERA_SHAKE = 0x050A {
///     u32 camera_shake_id;
///     u32 unknown;
/// }
/// ```
pub struct SMSG_CAMERA_SHAKE {
    /// SpellEffectCameraShakes.dbc
    pub camera_shake_id: u32,
    pub unknown: u32,
}

impl crate::private::Sealed for SMSG_CAMERA_SHAKE {}
impl crate::Message for SMSG_CAMERA_SHAKE {
    const OPCODE: u32 = 0x050a;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CAMERA_SHAKE {{").unwrap();
        // Members
        writeln!(s, "    camera_shake_id = {};", self.camera_shake_id).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1290_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "camera_shake_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // camera_shake_id: u32
        w.write_all(&self.camera_shake_id.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x050A, size: body_size });
        }

        // camera_shake_id: u32
        let camera_shake_id = crate::util::read_u32_le(&mut r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            camera_shake_id,
            unknown,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CAMERA_SHAKE {}

