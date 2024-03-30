use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_socket_gems_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_socket_gems_result.wowm#L1):
/// ```text
/// smsg SMSG_SOCKET_GEMS_RESULT = 0x050B {
///     Guid item;
///     u32[3] sockets;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SOCKET_GEMS_RESULT {
    pub item: Guid,
    pub sockets: [u32; 3],
}

impl crate::private::Sealed for SMSG_SOCKET_GEMS_RESULT {}
impl SMSG_SOCKET_GEMS_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 20 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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

impl crate::Message for SMSG_SOCKET_GEMS_RESULT {
    const OPCODE: u32 = 0x050b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SOCKET_GEMS_RESULT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SOCKET_GEMS_RESULT {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    sockets = [").unwrap();
        for v in self.sockets.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 22_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1291_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
        writeln!(s, "    /* sockets: u32[3] start */").unwrap();
        for (i, v) in self.sockets.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("sockets {i}"), "    ");
        }
        writeln!(s, "    /* sockets: u32[3] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1291, "SMSG_SOCKET_GEMS_RESULT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SOCKET_GEMS_RESULT {}

