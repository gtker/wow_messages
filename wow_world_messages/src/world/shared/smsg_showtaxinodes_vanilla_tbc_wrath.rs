use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm#L3):
/// ```text
/// smsg SMSG_SHOWTAXINODES = 0x01A9 {
///     u32 unknown1;
///     Guid guid;
///     u32 nearest_node;
///     u32[-] nodes;
/// }
/// ```
pub struct SMSG_SHOWTAXINODES {
    /// Set to 1 in mangoszero
    pub unknown1: u32,
    pub guid: Guid,
    pub nearest_node: u32,
    pub nodes: Vec<u32>,
}

impl crate::private::Sealed for SMSG_SHOWTAXINODES {}
impl SMSG_SHOWTAXINODES {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(16..=65551).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // nearest_node: u32
        let nearest_node = crate::util::read_u32_le(&mut r)?;

        // nodes: u32[-]
        let nodes = {
            let mut current_size = {
                4 // unknown1: u32
                + 8 // guid: Guid
                + 4 // nearest_node: u32
            };
            let mut nodes = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                nodes.push(crate::util::read_u32_le(&mut r)?);
                current_size += 4;
            }
            nodes
        };

        Ok(Self {
            unknown1,
            guid,
            nearest_node,
            nodes,
        })
    }

}

impl crate::Message for SMSG_SHOWTAXINODES {
    const OPCODE: u32 = 0x01a9;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SHOWTAXINODES"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SHOWTAXINODES {{").unwrap();
        // Members
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    nearest_node = {};", self.nearest_node).unwrap();
        writeln!(s, "    nodes = [").unwrap();
        for v in self.nodes.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 425_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "nearest_node", "    ");
        if !self.nodes.is_empty() {
            writeln!(s, "    /* nodes: u32[-] start */").unwrap();
            for (i, v) in self.nodes.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("nodes {i}"), "    ");
            }
            writeln!(s, "    /* nodes: u32[-] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // nearest_node: u32
        w.write_all(&self.nearest_node.to_le_bytes())?;

        // nodes: u32[-]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(425, "SMSG_SHOWTAXINODES", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SHOWTAXINODES {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SHOWTAXINODES {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SHOWTAXINODES {}

impl SMSG_SHOWTAXINODES {
    pub(crate) fn size(&self) -> usize {
        4 // unknown1: u32
        + 8 // guid: Guid
        + 4 // nearest_node: u32
        + self.nodes.len() * core::mem::size_of::<u32>() // nodes: u32[-]
    }
}

