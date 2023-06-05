use std::io::{Read, Write};

use crate::vanilla::Area;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent by the client whenever it reaches a new area.
///
/// The client does not send an accurate area. For example when going to Sen'jin Village, the client will send `DUROTAR` (0x0E) and not `SENJIN_VILLAGE` (0x16F).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm#L1):
/// ```text
/// cmsg CMSG_ZONEUPDATE = 0x01F4 {
///     Area area;
/// }
/// ```
pub struct CMSG_ZONEUPDATE {
    pub area: Area,
}

#[cfg(feature = "print-testcase")]
impl CMSG_ZONEUPDATE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ZONEUPDATE {{").unwrap();
        // Members
        writeln!(s, "    area = {};", self.area.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 500_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_ZONEUPDATE {}
impl crate::Message for CMSG_ZONEUPDATE {
    const OPCODE: u32 = 0x01f4;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_ZONEUPDATE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F4, size: body_size });
        }

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            area,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ZONEUPDATE {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_ZONEUPDATE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_ZONEUPDATE, expected: &CMSG_ZONEUPDATE) {
        assert_eq!(t.area, expected.area);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0xF4, 0x01, 0x00, 0x00, 0x65, 0x06, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_ZONEUPDATE {
        CMSG_ZONEUPDATE {
            area: Area::Orgrimmar,
        }

    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_zoneupdate0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ZONEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ZONEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_zoneupdate0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ZONEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ZONEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_zoneupdate0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ZONEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ZONEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

