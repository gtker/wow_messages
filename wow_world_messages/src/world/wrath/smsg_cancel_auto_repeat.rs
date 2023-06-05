use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_cancel_auto_repeat.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_cancel_auto_repeat.wowm#L5):
/// ```text
/// smsg SMSG_CANCEL_AUTO_REPEAT = 0x029C {
///     PackedGuid target;
/// }
/// ```
pub struct SMSG_CANCEL_AUTO_REPEAT {
    pub target: Guid,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CANCEL_AUTO_REPEAT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CANCEL_AUTO_REPEAT {{").unwrap();
        // Members
        writeln!(s, "    target = {};", self.target.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 668_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.target), "target", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_CANCEL_AUTO_REPEAT {}
impl crate::Message for SMSG_CANCEL_AUTO_REPEAT {
    const OPCODE: u32 = 0x029c;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_CANCEL_AUTO_REPEAT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: PackedGuid
        crate::util::write_packed_guid(&self.target, &mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x029C, size: body_size });
        }

        // target: PackedGuid
        let target = crate::util::read_packed_guid(&mut r)?;

        Ok(Self {
            target,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CANCEL_AUTO_REPEAT {}

impl SMSG_CANCEL_AUTO_REPEAT {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.target) // target: PackedGuid
    }
}

