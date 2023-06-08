use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_taxinode_status.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_taxinode_status.wowm#L3):
/// ```text
/// smsg SMSG_TAXINODE_STATUS = 0x01AB {
///     Guid guid;
///     Bool taxi_mask_node_known;
/// }
/// ```
pub struct SMSG_TAXINODE_STATUS {
    pub guid: Guid,
    pub taxi_mask_node_known: bool,
}

#[cfg(feature = "print-testcase")]
impl SMSG_TAXINODE_STATUS {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TAXINODE_STATUS {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    taxi_mask_node_known = {};", if self.taxi_mask_node_known { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 11_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 427_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "taxi_mask_node_known", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_TAXINODE_STATUS {}
impl crate::Message for SMSG_TAXINODE_STATUS {
    const OPCODE: u32 = 0x01ab;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_TAXINODE_STATUS::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // taxi_mask_node_known: Bool
        w.write_all(u8::from(self.taxi_mask_node_known).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01AB, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // taxi_mask_node_known: Bool
        let taxi_mask_node_known = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            guid,
            taxi_mask_node_known,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_TAXINODE_STATUS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TAXINODE_STATUS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TAXINODE_STATUS {}

