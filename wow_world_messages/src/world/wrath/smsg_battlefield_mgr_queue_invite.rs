use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_queue_invite.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_queue_invite.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_QUEUE_INVITE = 0x04E1 {
///     u32 battle_id;
///     u8 warmup;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_MGR_QUEUE_INVITE {
    pub battle_id: u32,
    /// Possibly not used.
    pub warmup: u8,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_MGR_QUEUE_INVITE {}
impl SMSG_BATTLEFIELD_MGR_QUEUE_INVITE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 5 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(&mut r)?;

        // warmup: u8
        let warmup = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            battle_id,
            warmup,
        })
    }

}

impl crate::Message for SMSG_BATTLEFIELD_MGR_QUEUE_INVITE {
    const OPCODE: u32 = 0x04e1;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_BATTLEFIELD_MGR_QUEUE_INVITE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BATTLEFIELD_MGR_QUEUE_INVITE {{").unwrap();
        // Members
        writeln!(s, "    battle_id = {};", self.battle_id).unwrap();
        writeln!(s, "    warmup = {};", self.warmup).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1249_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "battle_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "warmup", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // warmup: u8
        w.write_all(&self.warmup.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1249, "SMSG_BATTLEFIELD_MGR_QUEUE_INVITE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_QUEUE_INVITE {}

