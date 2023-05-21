use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_socket_gems.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_socket_gems.wowm#L1):
/// ```text
/// cmsg CMSG_SOCKET_GEMS = 0x0347 {
///     Guid item;
///     Guid[3] gems;
/// }
/// ```
pub struct CMSG_SOCKET_GEMS {
    pub item: Guid,
    pub gems: [Guid; 3],
}

impl crate::private::Sealed for CMSG_SOCKET_GEMS {}
impl CMSG_SOCKET_GEMS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 32 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0347, size: body_size });
        }

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // gems: Guid[3]
        let gems = {
            let mut gems = [Guid::default(); 3];
            for i in gems.iter_mut() {
                *i = crate::util::read_guid(&mut r)?;
            }
            gems
        };

        Ok(Self {
            item,
            gems,
        })
    }

}

impl crate::Message for CMSG_SOCKET_GEMS {
    const OPCODE: u32 = 0x0347;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SOCKET_GEMS {{").unwrap();
        // Members
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        write!(s, "    gems = [").unwrap();
        for v in self.gems.as_slice() {
            write!(s, "{v:#08X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 36_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 839_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
        writeln!(s, "    /* gems: Guid[3] start */").unwrap();
        for (i, v) in self.gems.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 8, &format!("gems {i}"), "    ");
        }
        writeln!(s, "    /* gems: Guid[3] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // gems: Guid[3]
        for i in self.gems.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SOCKET_GEMS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SOCKET_GEMS {}

