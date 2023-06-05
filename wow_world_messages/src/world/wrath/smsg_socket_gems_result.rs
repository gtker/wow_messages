use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_socket_gems_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_socket_gems_result.wowm#L1):
/// ```text
/// smsg SMSG_SOCKET_GEMS_RESULT = 0x050B {
///     Guid item;
///     u32[3] sockets;
/// }
/// ```
pub struct SMSG_SOCKET_GEMS_RESULT {
    pub item: Guid,
    pub sockets: [u32; 3],
}

#[cfg(feature = "print-testcase")]
impl SMSG_SOCKET_GEMS_RESULT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SOCKET_GEMS_RESULT {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        write!(s, "    sockets = [").unwrap();
        for v in self.sockets.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 24_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1291_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "item");
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

impl crate::private::Sealed for SMSG_SOCKET_GEMS_RESULT {}
impl crate::Message for SMSG_SOCKET_GEMS_RESULT {
    const OPCODE: u32 = 0x050b;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // sockets: u32[3]
        for i in self.sockets.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x050B, size: body_size });
        }

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // sockets: u32[3]
        let sockets = {
            let mut sockets = [u32::default(); 3];
            for i in sockets.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            sockets
        };

        Ok(Self {
            item,
            sockets,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SOCKET_GEMS_RESULT {}

