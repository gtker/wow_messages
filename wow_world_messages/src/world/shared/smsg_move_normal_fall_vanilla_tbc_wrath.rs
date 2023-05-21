use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_move_normal_fall.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_move_normal_fall.wowm#L3):
/// ```text
/// smsg SMSG_MOVE_NORMAL_FALL = 0x00F3 {
///     PackedGuid guid;
///     u32 counter;
/// }
/// ```
pub struct SMSG_MOVE_NORMAL_FALL {
    pub guid: Guid,
    pub counter: u32,
}

impl crate::private::Sealed for SMSG_MOVE_NORMAL_FALL {}
impl SMSG_MOVE_NORMAL_FALL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x00F3, size: body_size });
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            counter,
        })
    }

}

impl crate::Message for SMSG_MOVE_NORMAL_FALL {
    const OPCODE: u32 = 0x00f3;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MOVE_NORMAL_FALL {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    counter = {};", self.counter).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 243_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "counter", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MOVE_NORMAL_FALL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOVE_NORMAL_FALL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOVE_NORMAL_FALL {}

impl SMSG_MOVE_NORMAL_FALL {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // counter: u32
    }
}

