use std::io::{Read, Write};

use crate::tbc::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_pending.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_pending.wowm#L1):
/// ```text
/// smsg SMSG_TRANSFER_PENDING = 0x003F {
///     Map map;
///     optional has_transport {
///         u32 transport;
///         Map transport_map;
///     }
/// }
/// ```
pub struct SMSG_TRANSFER_PENDING {
    pub map: Map,
    pub has_transport: Option<SMSG_TRANSFER_PENDING_has_transport>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_TRANSFER_PENDING {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TRANSFER_PENDING {{").unwrap();
        // Members
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        if let Some(has_transport) = &self.has_transport {
            writeln!(s, "    transport = {};", has_transport.transport).unwrap();
            writeln!(s, "    transport_map = {};", has_transport.transport_map.as_test_case_value()).unwrap();
        }

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 63_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "map");
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

impl crate::private::Sealed for SMSG_TRANSFER_PENDING {}
impl crate::Message for SMSG_TRANSFER_PENDING {
    const OPCODE: u32 = 0x003f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // optional has_transport
        if let Some(v) = &self.has_transport {
            // transport: u32
            w.write_all(&v.transport.to_le_bytes())?;

            // transport_map: Map
            w.write_all(&(v.transport_map.as_int().to_le_bytes()))?;

        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=12).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x003F, size: body_size });
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // optional has_transport
        let current_size = {
            4 // map: Map
        };
        let has_transport = if current_size < body_size as usize {
            // transport: u32
            let transport = crate::util::read_u32_le(&mut r)?;

            // transport_map: Map
            let transport_map = crate::util::read_u32_le(&mut r)?.try_into()?;

            Some(SMSG_TRANSFER_PENDING_has_transport {
                transport,
                transport_map,
            })
        } else {
            None
        };

        Ok(Self {
            map,
            has_transport,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TRANSFER_PENDING {}

impl SMSG_TRANSFER_PENDING {
    pub(crate) const fn size(&self) -> usize {
        4 // map: Map
        + if let Some(has_transport) = &self.has_transport {
            4 // transport: u32
            + 4 // transport_map: Map
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_TRANSFER_PENDING_has_transport {
    pub transport: u32,
    pub transport_map: Map,
}

