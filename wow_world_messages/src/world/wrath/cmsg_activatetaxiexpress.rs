use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxiexpress.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxiexpress.wowm#L12):
/// ```text
/// cmsg CMSG_ACTIVATETAXIEXPRESS = 0x0312 {
///     Guid guid;
///     u32 node_count;
///     u32[node_count] nodes;
/// }
/// ```
pub struct CMSG_ACTIVATETAXIEXPRESS {
    pub guid: Guid,
    pub nodes: Vec<u32>,
}

#[cfg(feature = "print-testcase")]
impl CMSG_ACTIVATETAXIEXPRESS {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ACTIVATETAXIEXPRESS {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    node_count = {};", self.nodes.len()).unwrap();
        write!(s, "    nodes = [").unwrap();
        for v in self.nodes.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 6).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 786_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_ACTIVATETAXIEXPRESS {}
impl crate::Message for CMSG_ACTIVATETAXIEXPRESS {
    const OPCODE: u32 = 0x0312;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // node_count: u32
        w.write_all(&(self.nodes.len() as u32).to_le_bytes())?;

        // nodes: u32[node_count]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(12..=10240).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0312, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

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
            nodes,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ACTIVATETAXIEXPRESS {}

impl CMSG_ACTIVATETAXIEXPRESS {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // node_count: u32
        + self.nodes.len() * core::mem::size_of::<u32>() // nodes: u32[node_count]
    }
}

