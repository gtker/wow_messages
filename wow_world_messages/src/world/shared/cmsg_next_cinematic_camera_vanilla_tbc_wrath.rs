use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent by client when cinematic beings.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/cmsg_next_cinematic_camera.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/cmsg_next_cinematic_camera.wowm#L3):
/// ```text
/// cmsg CMSG_NEXT_CINEMATIC_CAMERA = 0x00FB {
/// }
/// ```
pub struct CMSG_NEXT_CINEMATIC_CAMERA {
}

impl crate::private::Sealed for CMSG_NEXT_CINEMATIC_CAMERA {}
impl CMSG_NEXT_CINEMATIC_CAMERA {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x00FB, size: body_size });
        }

        Ok(Self {
        })
    }

}

impl crate::Message for CMSG_NEXT_CINEMATIC_CAMERA {
    const OPCODE: u32 = 0x00fb;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_NEXT_CINEMATIC_CAMERA {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        let [a, b] = 4_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 251_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_NEXT_CINEMATIC_CAMERA {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_NEXT_CINEMATIC_CAMERA {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_NEXT_CINEMATIC_CAMERA {}

