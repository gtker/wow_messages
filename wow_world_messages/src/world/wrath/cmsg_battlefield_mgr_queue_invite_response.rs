use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_mgr_queue_invite_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_mgr_queue_invite_response.wowm#L1):
/// ```text
/// cmsg CMSG_BATTLEFIELD_MGR_QUEUE_INVITE_RESPONSE = 0x04E2 {
///     u32 battle_id;
///     Bool accepted;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_MGR_QUEUE_INVITE_RESPONSE {
    pub battle_id: u32,
    pub accepted: bool,
}

#[cfg(feature = "print-testcase")]
impl CMSG_BATTLEFIELD_MGR_QUEUE_INVITE_RESPONSE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_BATTLEFIELD_MGR_QUEUE_INVITE_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    battle_id = {};", self.battle_id).unwrap();
        writeln!(s, "    accepted = {};", if self.accepted { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 11_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1250_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "battle_id");
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

impl crate::private::Sealed for CMSG_BATTLEFIELD_MGR_QUEUE_INVITE_RESPONSE {}
impl crate::Message for CMSG_BATTLEFIELD_MGR_QUEUE_INVITE_RESPONSE {
    const OPCODE: u32 = 0x04e2;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // accepted: Bool
        w.write_all(u8::from(self.accepted).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04E2, size: body_size });
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(&mut r)?;

        // accepted: Bool
        let accepted = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            battle_id,
            accepted,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BATTLEFIELD_MGR_QUEUE_INVITE_RESPONSE {}

