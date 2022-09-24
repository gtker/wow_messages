use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Reply to [`CMSG_QUERY_TIME`](crate::world::vanilla::CMSG_QUERY_TIME).
/// [`CMSG_QUERY_TIME`](crate::world::vanilla::CMSG_QUERY_TIME) and this reply does not actually appear to set the time. Instead [`SMSG_LOGIN_SETTIMESPEED`](crate::world::vanilla::SMSG_LOGIN_SETTIMESPEED) seems to correctly set the time. Running the client with `-console` will print the date when [`SMSG_LOGIN_SETTIMESPEED`](crate::world::vanilla::SMSG_LOGIN_SETTIMESPEED) is received, but not when this message is received.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_query_time_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_query_time_response.wowm#L3):
/// ```text
/// smsg SMSG_QUERY_TIME_RESPONSE = 0x01CF {
///     u32 time;
/// }
/// ```
pub struct SMSG_QUERY_TIME_RESPONSE {
    /// Seconds since 1970, 1st of January (Unix Time).
    ///
    pub time: u32,
}

impl crate::Message for SMSG_QUERY_TIME_RESPONSE {
    const OPCODE: u32 = 0x01cf;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // time: u32
        w.write_all(&self.time.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // time: u32
        let time = crate::util::read_u32_le(r)?;

        Ok(Self {
            time,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_QUERY_TIME_RESPONSE {}

#[cfg(test)]
mod test {
    use super::SMSG_QUERY_TIME_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 8] = [ 0x00, 0x06, 0xCF, 0x01, 0x94, 0x98, 0x50, 0x61, ];

    // Generated from `wow_message_parser/wowm/world/queries/smsg_query_time_response.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_QUERY_TIME_RESPONSE0() {
        let expected = SMSG_QUERY_TIME_RESPONSE {
            time: 0x61509894,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_QUERY_TIME_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_QUERY_TIME_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.time, expected.time);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_query_time_response.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_QUERY_TIME_RESPONSE0() {
        let expected = SMSG_QUERY_TIME_RESPONSE {
            time: 0x61509894,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_QUERY_TIME_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_QUERY_TIME_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.time, expected.time);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_query_time_response.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_QUERY_TIME_RESPONSE0() {
        let expected = SMSG_QUERY_TIME_RESPONSE {
            time: 0x61509894,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_QUERY_TIME_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_QUERY_TIME_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.time, expected.time);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

