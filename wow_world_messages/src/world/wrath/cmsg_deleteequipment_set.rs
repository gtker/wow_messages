use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_deleteequipment_set.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_deleteequipment_set.wowm#L1):
/// ```text
/// cmsg CMSG_DELETEEQUIPMENT_SET = 0x013E {
///     PackedGuid set;
/// }
/// ```
pub struct CMSG_DELETEEQUIPMENT_SET {
    pub set: Guid,
}

impl crate::private::Sealed for CMSG_DELETEEQUIPMENT_SET {}
impl CMSG_DELETEEQUIPMENT_SET {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x013E, size: body_size });
        }

        // set: PackedGuid
        let set = crate::util::read_packed_guid(&mut r)?;

        Ok(Self {
            set,
        })
    }

}

impl crate::Message for CMSG_DELETEEQUIPMENT_SET {
    const OPCODE: u32 = 0x013e;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_DELETEEQUIPMENT_SET {{").unwrap();
        // Members
        writeln!(s, "    set = {};", self.set.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 318_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.set), "set", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // set: PackedGuid
        crate::util::write_packed_guid(&self.set, &mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_DELETEEQUIPMENT_SET {}

impl CMSG_DELETEEQUIPMENT_SET {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.set) // set: PackedGuid
    }
}

