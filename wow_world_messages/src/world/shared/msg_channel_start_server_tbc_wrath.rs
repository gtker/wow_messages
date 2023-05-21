use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/msg_channel_start.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/msg_channel_start.wowm#L8):
/// ```text
/// smsg MSG_CHANNEL_START_Server = 0x0139 {
///     PackedGuid caster;
///     u32 spell;
///     u32 duration;
/// }
/// ```
pub struct MSG_CHANNEL_START_Server {
    pub caster: Guid,
    pub spell: u32,
    pub duration: u32,
}

impl crate::private::Sealed for MSG_CHANNEL_START_Server {}
impl MSG_CHANNEL_START_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(10..=17).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            caster,
            spell,
            duration,
        })
    }

}

impl crate::Message for MSG_CHANNEL_START_Server {
    const OPCODE: u32 = 0x0139;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_CHANNEL_START_Server {{").unwrap();
        // Members
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    duration = {};", self.duration).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 313_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.caster), "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // caster: PackedGuid
        crate::util::write_packed_guid(&self.caster, &mut w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(313, "MSG_CHANNEL_START_Server", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_CHANNEL_START_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_CHANNEL_START_Server {}

impl MSG_CHANNEL_START_Server {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 4 // spell: u32
        + 4 // duration: u32
    }
}

