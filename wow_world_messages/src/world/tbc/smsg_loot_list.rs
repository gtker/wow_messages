use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_list.wowm#L1):
/// ```text
/// smsg SMSG_LOOT_LIST = 0x03F8 {
///     Guid creature;
///     PackedGuid master_looter;
///     PackedGuid group_looter;
/// }
/// ```
pub struct SMSG_LOOT_LIST {
    pub creature: Guid,
    pub master_looter: Guid,
    pub group_looter: Guid,
}

#[cfg(feature = "print-testcase")]
impl SMSG_LOOT_LIST {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOOT_LIST {{").unwrap();
        // Members
        writeln!(s, "    creature = {};", self.creature.guid()).unwrap();
        writeln!(s, "    master_looter = {};", self.master_looter.guid()).unwrap();
        writeln!(s, "    group_looter = {};", self.group_looter.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1016_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "creature");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_LOOT_LIST {}
impl crate::Message for SMSG_LOOT_LIST {
    const OPCODE: u32 = 0x03f8;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // creature: Guid
        w.write_all(&self.creature.guid().to_le_bytes())?;

        // master_looter: PackedGuid
        crate::util::write_packed_guid(&self.master_looter, &mut w)?;

        // group_looter: PackedGuid
        crate::util::write_packed_guid(&self.group_looter, &mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(12..=26).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03F8, size: body_size });
        }

        // creature: Guid
        let creature = crate::util::read_guid(&mut r)?;

        // master_looter: PackedGuid
        let master_looter = crate::util::read_packed_guid(&mut r)?;

        // group_looter: PackedGuid
        let group_looter = crate::util::read_packed_guid(&mut r)?;

        Ok(Self {
            creature,
            master_looter,
            group_looter,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_LIST {}

impl SMSG_LOOT_LIST {
    pub(crate) const fn size(&self) -> usize {
        8 // creature: Guid
        + crate::util::packed_guid_size(&self.master_looter) // master_looter: PackedGuid
        + crate::util::packed_guid_size(&self.group_looter) // group_looter: PackedGuid
    }
}

