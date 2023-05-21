use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_play_music.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_play_music.wowm#L3):
/// ```text
/// smsg SMSG_PLAY_MUSIC = 0x0277 {
///     u32 sound_id;
/// }
/// ```
pub struct SMSG_PLAY_MUSIC {
    pub sound_id: u32,
}

impl crate::private::Sealed for SMSG_PLAY_MUSIC {}
impl SMSG_PLAY_MUSIC {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0277, size: body_size });
        }

        // sound_id: u32
        let sound_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            sound_id,
        })
    }

}

impl crate::Message for SMSG_PLAY_MUSIC {
    const OPCODE: u32 = 0x0277;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PLAY_MUSIC {{").unwrap();
        // Members
        writeln!(s, "    sound_id = {};", self.sound_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 631_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "sound_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PLAY_MUSIC {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PLAY_MUSIC {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAY_MUSIC {}

