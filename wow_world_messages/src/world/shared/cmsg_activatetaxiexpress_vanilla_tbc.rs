use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxiexpress.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxiexpress.wowm#L1):
/// ```text
/// cmsg CMSG_ACTIVATETAXIEXPRESS = 0x0312 {
///     Guid guid;
///     u32 total_cost;
///     u32 node_count;
///     u32[node_count] nodes;
/// }
/// ```
pub struct CMSG_ACTIVATETAXIEXPRESS {
    pub guid: Guid,
    /// vmangos/mangosone: Never used.
    pub total_cost: u32,
    pub nodes: Vec<u32>,
}

impl crate::private::Sealed for CMSG_ACTIVATETAXIEXPRESS {}
impl CMSG_ACTIVATETAXIEXPRESS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(16..=10240).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // total_cost: u32
        let total_cost = crate::util::read_u32_le(&mut r)?;

        // node_count: u32
        let node_count = crate::util::read_u32_le(&mut r)?;

        // nodes: u32[node_count]
        let nodes = {
            let mut nodes = Vec::with_capacity(node_count as usize);
            for _ in 0..node_count {
                nodes.push(crate::util::read_u32_le(&mut r)?);
            }
            nodes
        };

        Ok(Self {
            guid,
            total_cost,
            nodes,
        })
    }

}

impl crate::Message for CMSG_ACTIVATETAXIEXPRESS {
    const OPCODE: u32 = 0x0312;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_ACTIVATETAXIEXPRESS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ACTIVATETAXIEXPRESS {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    total_cost = {};", self.total_cost).unwrap();
        writeln!(s, "    node_count = {};", self.nodes.len()).unwrap();
        writeln!(s, "    nodes = [").unwrap();
        for v in self.nodes.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 786_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "total_cost", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "node_count", "    ");
        if !self.nodes.is_empty() {
            writeln!(s, "    /* nodes: u32[node_count] start */").unwrap();
            for (i, v) in self.nodes.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("nodes {i}"), "    ");
            }
            writeln!(s, "    /* nodes: u32[node_count] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // total_cost: u32
        w.write_all(&self.total_cost.to_le_bytes())?;

        // node_count: u32
        w.write_all(&(self.nodes.len() as u32).to_le_bytes())?;

        // nodes: u32[node_count]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(786, "CMSG_ACTIVATETAXIEXPRESS", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ACTIVATETAXIEXPRESS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ACTIVATETAXIEXPRESS {}

impl CMSG_ACTIVATETAXIEXPRESS {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // total_cost: u32
        + 4 // node_count: u32
        + self.nodes.len() * core::mem::size_of::<u32>() // nodes: u32[node_count]
    }
}

