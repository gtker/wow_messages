use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// vmangos: Nostalrius: ignored by client if unit is not loaded
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_player_object_sound.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_player_object_sound.wowm#L3):
/// ```text
/// smsg SMSG_PLAY_OBJECT_SOUND = 0x0278 {
///     u32 sound_id;
///     Guid guid;
/// }
/// ```
pub struct SMSG_PLAY_OBJECT_SOUND {
    pub sound_id: u32,
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_PLAY_OBJECT_SOUND {}
impl SMSG_PLAY_OBJECT_SOUND {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0278, size: body_size });
        }

        // sound_id: u32
        let sound_id = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            sound_id,
            guid,
        })
    }

}

impl crate::Message for SMSG_PLAY_OBJECT_SOUND {
    const OPCODE: u32 = 0x0278;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PLAY_OBJECT_SOUND {{").unwrap();
        // Members
        writeln!(s, "    sound_id = {};", self.sound_id).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 632_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "sound_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PLAY_OBJECT_SOUND {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PLAY_OBJECT_SOUND {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAY_OBJECT_SOUND {}

