use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_list.wowm#L1):
/// ```text
/// smsg SMSG_LOOT_LIST = 0x03F8 {
///     Guid creature;
///     PackedGuid master_looter;
///     PackedGuid group_looter;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_LOOT_LIST {
    pub creature: Guid,
    pub master_looter: Guid,
    pub group_looter: Guid,
}

impl crate::private::Sealed for SMSG_LOOT_LIST {}
impl SMSG_LOOT_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(10..=26).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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

impl crate::Message for SMSG_LOOT_LIST {
    const OPCODE: u32 = 0x03f8;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LOOT_LIST"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOOT_LIST {{").unwrap();
        // Members
        writeln!(s, "    creature = {};", self.creature.guid()).unwrap();
        writeln!(s, "    master_looter = {};", self.master_looter.guid()).unwrap();
        writeln!(s, "    group_looter = {};", self.group_looter.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1016_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "creature", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.master_looter), "master_looter", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.group_looter), "group_looter", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1016, "SMSG_LOOT_LIST", body_size, a))
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

