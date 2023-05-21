use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfg_set_boot_vote.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfg_set_boot_vote.wowm#L1):
/// ```text
/// cmsg CMSG_LFG_SET_BOOT_VOTE = 0x036C {
///     Bool agree_to_kick_player;
/// }
/// ```
pub struct CMSG_LFG_SET_BOOT_VOTE {
    pub agree_to_kick_player: bool,
}

impl crate::private::Sealed for CMSG_LFG_SET_BOOT_VOTE {}
impl CMSG_LFG_SET_BOOT_VOTE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x036C, size: body_size });
        }

        // agree_to_kick_player: Bool
        let agree_to_kick_player = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            agree_to_kick_player,
        })
    }

}

impl crate::Message for CMSG_LFG_SET_BOOT_VOTE {
    const OPCODE: u32 = 0x036c;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LFG_SET_BOOT_VOTE {{").unwrap();
        // Members
        writeln!(s, "    agree_to_kick_player = {};", if self.agree_to_kick_player { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 876_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "agree_to_kick_player", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // agree_to_kick_player: Bool
        w.write_all(u8::from(self.agree_to_kick_player).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LFG_SET_BOOT_VOTE {}

